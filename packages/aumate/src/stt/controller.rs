//! STT (Speech-to-Text) feature for the controller
//!
//! Provides speech-to-text functionality with model management and hotkey support.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::error::Result;
use crate::gui::controller::{AsyncTask, ControllerContext, ControllerFeature, TabInfo};
use crate::ml::{
    DeviceConfig, DownloadProgress, DownloadStatus, ModelInfo, ModelManager, ModelType,
    device_name, is_gpu_available,
};
use crate::stt::{
    AudioRecorder, HotkeyEvent as SttHotkeyEvent, HotkeyManager as SttHotkeyManager, HotkeyMode,
    OutputMode, SttConfig, WhisperEngine,
};

/// Available device options for STT inference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SttDevice {
    /// CPU (always available)
    #[default]
    Cpu,
    /// GPU acceleration (Metal on macOS, CUDA on Linux/Windows)
    Gpu,
}

/// STT feature for speech-to-text
pub struct SttFeature {
    /// STT configuration
    stt_config: SttConfig,
    /// Model manager for downloading/managing models
    stt_model_manager: Option<ModelManager>,
    /// List of available models
    stt_available_models: Vec<ModelInfo>,
    /// Flag indicating models list needs refresh
    stt_models_need_refresh: bool,
    /// Download progress for async downloads
    stt_download_progress: Arc<Mutex<Option<DownloadProgress>>>,
    /// Whether STT is initialized with a valid model
    stt_initialized: bool,
    /// Whether currently recording (shared with hotkey callback for UI display)
    stt_is_recording: Arc<AtomicBool>,
    /// Last transcription result (shared with background thread)
    stt_last_transcription: Arc<Mutex<Option<String>>>,
    /// Last recorded audio data (for debug playback)
    stt_last_audio: Arc<Mutex<Option<crate::stt::AudioData>>>,
    /// Status message
    stt_status: String,
    /// Hotkey manager for global hotkeys
    stt_hotkey_manager: Option<SttHotkeyManager>,
    /// Debug output log (last N messages, shared with hotkey callback)
    stt_debug_log: Arc<Mutex<Vec<String>>>,
    /// Flag indicating transcription is in progress
    stt_transcribing: Arc<AtomicBool>,
    /// Loaded Whisper engine (for preloaded model)
    stt_whisper_engine: Option<WhisperEngine>,
    /// Async task for model loading
    load_model_task: Option<AsyncTask<std::result::Result<WhisperEngine, String>>>,
    /// Selected device for inference
    selected_device: SttDevice,
    /// Whether GPU is available on this system
    gpu_available: bool,
    /// Whether audio playback is in progress
    stt_is_playing: Arc<AtomicBool>,
}

impl SttFeature {
    pub fn new() -> Self {
        Self {
            stt_config: SttConfig::load().unwrap_or_default(),
            stt_model_manager: None,
            stt_available_models: Vec::new(),
            stt_models_need_refresh: true,
            stt_download_progress: Arc::new(Mutex::new(None)),
            stt_initialized: false,
            stt_is_recording: Arc::new(AtomicBool::new(false)),
            stt_last_transcription: Arc::new(Mutex::new(None)),
            stt_last_audio: Arc::new(Mutex::new(None)),
            stt_status: "Not initialized".to_string(),
            stt_hotkey_manager: None,
            stt_debug_log: Arc::new(Mutex::new(Vec::new())),
            stt_transcribing: Arc::new(AtomicBool::new(false)),
            stt_whisper_engine: None,
            load_model_task: None,
            selected_device: SttDevice::default(),
            gpu_available: is_gpu_available(),
            stt_is_playing: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Add a debug message to the log
    fn add_debug_message(&self, message: &str) {
        Self::add_debug_message_to_log(&self.stt_debug_log, message);
    }

    /// Add a debug message to a shared log
    fn add_debug_message_to_log(log: &Arc<Mutex<Vec<String>>>, message: &str) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs =
            SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() % 86400).unwrap_or(0);
        let hours = (secs / 3600) % 24;
        let mins = (secs % 3600) / 60;
        let secs = secs % 60;
        let timestamp = format!("{:02}:{:02}:{:02}", hours, mins, secs);
        let mut log_guard = log.lock().unwrap();
        log_guard.push(format!("[{}] {}", timestamp, message));
        if log_guard.len() > 20 {
            log_guard.remove(0);
        }
    }

    /// Initialize STT hotkey manager
    fn init_stt_hotkey(&mut self) {
        let hotkey_config = self.stt_config.hotkey.clone();
        let is_recording = self.stt_is_recording.clone();
        let is_transcribing = self.stt_transcribing.clone();
        let debug_log = self.stt_debug_log.clone();
        let last_transcription = self.stt_last_transcription.clone();
        let last_audio = self.stt_last_audio.clone();
        let model_id = self.stt_config.model_id.clone();
        let language = self.stt_config.language.clone();
        let input_device = self.stt_config.input_device.clone();
        let output_mode = self.stt_config.output_mode;

        self.add_debug_message(&format!(
            "Initializing hotkey: {} (mode: {:?})",
            hotkey_config.display_string(),
            hotkey_config.mode
        ));

        // Shared state for the recording thread
        let should_stop = Arc::new(AtomicBool::new(false));
        let should_stop_for_callback = should_stop.clone();

        let mut manager = SttHotkeyManager::new();
        manager.set_config(hotkey_config.clone());
        manager.set_callback(move |event| match event {
            SttHotkeyEvent::RecordStart => {
                // Don't start if already recording or transcribing
                if is_recording.load(Ordering::Relaxed) || is_transcribing.load(Ordering::Relaxed) {
                    return;
                }

                is_recording.store(true, Ordering::Relaxed);
                should_stop_for_callback.store(false, Ordering::Relaxed);

                let msg = format!("Hotkey pressed: {}", hotkey_config.display_string());
                log::info!("STT: {}", msg);
                Self::add_debug_message_to_log(&debug_log, &msg);

                // Clone everything needed for the recording thread
                let is_recording_thread = is_recording.clone();
                let is_transcribing_thread = is_transcribing.clone();
                let should_stop_thread = should_stop_for_callback.clone();
                let debug_log_thread = debug_log.clone();
                let last_transcription_thread = last_transcription.clone();
                let last_audio_thread = last_audio.clone();
                let model_id_thread = model_id.clone();
                let language_thread = language.clone();
                let input_device_thread = input_device.clone();
                let output_mode_thread = output_mode;

                // Spawn recording thread
                thread::spawn(move || {
                    Self::run_recording_thread(
                        is_recording_thread,
                        is_transcribing_thread,
                        should_stop_thread,
                        debug_log_thread,
                        last_transcription_thread,
                        last_audio_thread,
                        model_id_thread,
                        language_thread,
                        input_device_thread,
                        output_mode_thread,
                    );
                });
            }
            SttHotkeyEvent::RecordStop => {
                should_stop_for_callback.store(true, Ordering::Relaxed);
                let msg = format!("Hotkey released: {}", hotkey_config.display_string());
                log::info!("STT: {}", msg);
                Self::add_debug_message_to_log(&debug_log, &msg);
            }
        });

        if let Err(e) = manager.start() {
            let msg = format!("Failed to start hotkey manager: {}", e);
            log::error!("STT: {}", msg);
            self.add_debug_message(&msg);
        } else {
            let msg =
                format!("Hotkey manager started: {}", self.stt_config.hotkey.display_string());
            log::info!("STT: {}", msg);
            self.add_debug_message(&msg);
            self.stt_hotkey_manager = Some(manager);
        }
    }

    /// Background thread that handles recording and transcription
    #[allow(clippy::too_many_arguments)]
    fn run_recording_thread(
        is_recording: Arc<AtomicBool>,
        is_transcribing: Arc<AtomicBool>,
        should_stop: Arc<AtomicBool>,
        debug_log: Arc<Mutex<Vec<String>>>,
        last_transcription: Arc<Mutex<Option<String>>>,
        last_audio: Arc<Mutex<Option<crate::stt::AudioData>>>,
        model_id: String,
        language: Option<String>,
        input_device: Option<String>,
        output_mode: OutputMode,
    ) {
        // Create audio recorder
        let mut recorder = match AudioRecorder::new() {
            Ok(mut r) => {
                r.set_input_device(input_device);
                r
            }
            Err(e) => {
                let msg = format!("Failed to create audio recorder: {}", e);
                log::error!("STT: {}", msg);
                Self::add_debug_message_to_log(&debug_log, &msg);
                is_recording.store(false, Ordering::Relaxed);
                return;
            }
        };

        // Start recording
        if let Err(e) = recorder.start_recording() {
            let msg = format!("Failed to start recording: {}", e);
            log::error!("STT: {}", msg);
            Self::add_debug_message_to_log(&debug_log, &msg);
            is_recording.store(false, Ordering::Relaxed);
            return;
        }

        Self::add_debug_message_to_log(&debug_log, "Recording STARTED");

        // Wait for stop signal
        while !should_stop.load(Ordering::Relaxed) {
            thread::sleep(std::time::Duration::from_millis(10));
        }

        // Stop recording
        let audio_data = match recorder.stop_recording() {
            Ok(data) => data,
            Err(e) => {
                let msg = format!("Failed to stop recording: {}", e);
                log::error!("STT: {}", msg);
                Self::add_debug_message_to_log(&debug_log, &msg);
                is_recording.store(false, Ordering::Relaxed);
                return;
            }
        };

        is_recording.store(false, Ordering::Relaxed);

        let duration_ms = audio_data.duration_ms();
        Self::add_debug_message_to_log(
            &debug_log,
            &format!(
                "Recording STOPPED ({} ms, {} samples)",
                duration_ms,
                audio_data.samples.len()
            ),
        );

        // Store audio for debug playback
        *last_audio.lock().unwrap() = Some(audio_data.clone());

        // Skip if too short
        if duration_ms < 100 {
            Self::add_debug_message_to_log(
                &debug_log,
                "Recording too short, skipping transcription",
            );
            return;
        }

        // Start transcription
        is_transcribing.store(true, Ordering::Relaxed);
        Self::add_debug_message_to_log(&debug_log, "Starting transcription...");

        // Get model path
        let model_manager = match ModelManager::new() {
            Ok(m) => m,
            Err(e) => {
                let msg = format!("Failed to create model manager: {}", e);
                log::error!("STT: {}", msg);
                Self::add_debug_message_to_log(&debug_log, &msg);
                is_transcribing.store(false, Ordering::Relaxed);
                return;
            }
        };

        // Check if model is downloaded
        if !model_manager.is_downloaded(ModelType::Whisper, &model_id) {
            let msg = format!("Model not downloaded: {}", model_id);
            log::error!("STT: {}", msg);
            Self::add_debug_message_to_log(&debug_log, &msg);
            is_transcribing.store(false, Ordering::Relaxed);
            return;
        }

        let model_path = model_manager.model_dir(ModelType::Whisper, &model_id);

        // Load and run Whisper
        let mut engine = WhisperEngine::new();
        engine.set_language(language);

        if let Err(e) = engine.load_model(&model_path) {
            let msg = format!("Failed to load model: {}", e);
            log::error!("STT: {}", msg);
            Self::add_debug_message_to_log(&debug_log, &msg);
            is_transcribing.store(false, Ordering::Relaxed);
            return;
        }

        match engine.transcribe(&audio_data) {
            Ok(result) => {
                let msg = format!(
                    "Transcription complete ({} ms): \"{}\"",
                    result.duration_ms, result.text
                );
                log::info!("STT: {}", msg);
                Self::add_debug_message_to_log(&debug_log, &msg);

                // Store result
                *last_transcription.lock().unwrap() = Some(result.text.clone());

                // Handle output
                Self::handle_output(&result.text, output_mode);
            }
            Err(e) => {
                let msg = format!("Transcription failed: {}", e);
                log::error!("STT: {}", msg);
                Self::add_debug_message_to_log(&debug_log, &msg);
            }
        }

        is_transcribing.store(false, Ordering::Relaxed);
    }

    /// Handle transcription output
    fn handle_output(text: &str, output_mode: OutputMode) {
        if text.is_empty() {
            return;
        }

        match output_mode {
            OutputMode::Keystrokes => {
                #[cfg(feature = "input")]
                {
                    use crate::input::Keyboard;
                    if let Ok(keyboard) = Keyboard::new() {
                        if let Err(e) = keyboard.type_string(text) {
                            log::error!("STT: Failed to type text: {}", e);
                        }
                    }
                }
            }
            OutputMode::Clipboard => {
                #[cfg(feature = "clipboard")]
                {
                    use crate::clipboard;
                    if let Err(e) = clipboard::set_text(text) {
                        log::error!("STT: Failed to copy to clipboard: {}", e);
                    }
                }
            }
            OutputMode::Both => {
                #[cfg(feature = "clipboard")]
                {
                    use crate::clipboard;
                    if let Err(e) = clipboard::set_text(text) {
                        log::error!("STT: Failed to copy to clipboard: {}", e);
                    }
                }
                #[cfg(feature = "input")]
                {
                    use crate::input::Keyboard;
                    if let Ok(keyboard) = Keyboard::new() {
                        #[cfg(target_os = "macos")]
                        let modifiers = vec!["meta".to_string()];
                        #[cfg(not(target_os = "macos"))]
                        let modifiers = vec!["control".to_string()];

                        if let Err(e) = keyboard.key_tap("v", Some(&modifiers)) {
                            log::error!("STT: Failed to paste: {}", e);
                        }
                    }
                }
            }
            OutputMode::Logger => {
                // Logger mode - transcription is already logged, no additional action needed
            }
        }
    }

    /// Initialize the STT model manager if not already done
    fn ensure_stt_model_manager(&mut self) {
        if self.stt_model_manager.is_none() {
            match ModelManager::new() {
                Ok(manager) => {
                    self.stt_model_manager = Some(manager);
                    self.stt_models_need_refresh = true;
                    log::info!("STT model manager initialized");
                }
                Err(e) => {
                    log::error!("Failed to create STT model manager: {}", e);
                    self.stt_status = format!("Error: {}", e);
                }
            }
        }
    }

    /// Refresh the available models list
    fn refresh_stt_models(&mut self) {
        if !self.stt_models_need_refresh {
            return;
        }

        if let Some(ref manager) = self.stt_model_manager {
            self.stt_available_models = manager.list_whisper_models();

            // Update status based on model availability
            let has_model = self
                .stt_available_models
                .iter()
                .any(|m| m.is_downloaded && m.id == self.stt_config.model_id);

            if has_model {
                self.stt_status = "Ready".to_string();
                self.stt_initialized = true;
            } else {
                let any_downloaded = self.stt_available_models.iter().any(|m| m.is_downloaded);
                if any_downloaded {
                    self.stt_status = "Model available (select one)".to_string();
                } else {
                    self.stt_status = "No models downloaded".to_string();
                }
                self.stt_initialized = false;
            }
        }

        self.stt_models_need_refresh = false;
    }

    /// Start downloading a model in the background
    fn start_stt_model_download(&mut self, model_id: &str) {
        let Some(ref manager) = self.stt_model_manager else {
            return;
        };

        // Clone what we need for the background thread
        let model_id_owned = model_id.to_string();
        let progress_arc = self.stt_download_progress.clone();

        // Find model info
        let model_info = manager.list_whisper_models().into_iter().find(|m| m.id == model_id);

        let Some(model_info) = model_info else {
            log::error!("Unknown model: {}", model_id);
            return;
        };

        // Set initial progress
        {
            let mut progress = progress_arc.lock().unwrap();
            *progress = Some(DownloadProgress {
                model_id: model_id_owned.clone(),
                current_file: String::new(),
                file_index: 0,
                total_files: model_info.files.len(),
                downloaded_bytes: 0,
                total_bytes: model_info.size_bytes,
                status: DownloadStatus::Pending,
            });
        }

        // Spawn download thread
        let progress_for_thread = progress_arc.clone();
        thread::spawn(move || {
            log::info!("Starting download of model: {}", model_id_owned);

            // Create a new model manager in this thread
            let manager = match ModelManager::new() {
                Ok(m) => m,
                Err(e) => {
                    log::error!("Failed to create model manager: {}", e);
                    let mut progress = progress_for_thread.lock().unwrap();
                    if let Some(ref mut p) = *progress {
                        p.status = DownloadStatus::Failed(e.to_string());
                    }
                    return;
                }
            };

            // Create progress callback
            let progress_callback = progress_for_thread.clone();
            let callback = Box::new(move |p: DownloadProgress| {
                let mut progress = progress_callback.lock().unwrap();
                *progress = Some(p);
            });

            // Download the model
            match manager.download_model_sync(ModelType::Whisper, &model_id_owned, Some(callback)) {
                Ok(path) => {
                    log::info!("Model downloaded to: {:?}", path);
                    let mut progress = progress_for_thread.lock().unwrap();
                    if let Some(ref mut p) = *progress {
                        p.status = DownloadStatus::Completed;
                    }
                }
                Err(e) => {
                    log::error!("Failed to download model: {}", e);
                    let mut progress = progress_for_thread.lock().unwrap();
                    if let Some(ref mut p) = *progress {
                        p.status = DownloadStatus::Failed(e.to_string());
                    }
                }
            }
        });

        self.stt_models_need_refresh = true;
    }

    /// Delete a downloaded model
    fn delete_stt_model(&mut self, model_id: &str) {
        if let Some(ref manager) = self.stt_model_manager {
            if let Err(e) = manager.delete_model(ModelType::Whisper, model_id) {
                log::error!("Failed to delete model {}: {}", model_id, e);
            } else {
                log::info!("Deleted model: {}", model_id);
                self.stt_models_need_refresh = true;
            }
        }
    }

    /// Start loading model asynchronously (non-blocking)
    fn start_model_load(&mut self, model_path: PathBuf) {
        let task = AsyncTask::new();
        let callback = task.callback();
        let use_gpu = self.selected_device == SttDevice::Gpu;
        let language = self.stt_config.language.clone();

        self.stt_status = "Loading model...".to_string();
        self.add_debug_message(&format!(
            "Loading model: {:?} (GPU: {})",
            model_path.file_name().unwrap_or_default(),
            use_gpu
        ));

        thread::spawn(move || {
            log::info!("Loading STT model async: {:?} (GPU: {})", model_path, use_gpu);
            let device_config =
                if use_gpu { DeviceConfig::with_gpu() } else { DeviceConfig::cpu_only() };
            let result = match WhisperEngine::with_device(device_config) {
                Ok(mut engine) => {
                    engine.set_language(language);
                    engine.load_model(&model_path).map(|_| engine).map_err(|e| e.to_string())
                }
                Err(e) => Err(e.to_string()),
            };
            callback(result);
        });

        self.load_model_task = Some(task);
    }

    /// Check for async task completion
    fn check_async_tasks(&mut self, ctx: &mut ControllerContext) {
        // Check model loading task
        if let Some(ref task) = self.load_model_task {
            if let Some(result) = task.take() {
                match result {
                    Ok(engine) => {
                        let dev_name = device_name(engine.device());
                        let msg = format!("Model loaded ({}) - Ready", dev_name);
                        self.add_debug_message(&msg);
                        self.stt_whisper_engine = Some(engine);
                        self.stt_status = msg;
                        self.stt_initialized = true;
                        log::info!("STT model loaded successfully on {}", dev_name);
                    }
                    Err(e) => {
                        let msg = format!("Failed to load model: {}", e);
                        self.add_debug_message(&msg);
                        self.stt_status = msg.clone();
                        log::error!("Failed to load STT model: {}", e);
                    }
                }
                self.load_model_task = None;
                ctx.request_repaint();
            }
        }

        // Check download progress and refresh models list when complete
        let download_info = {
            let progress = self.stt_download_progress.lock().unwrap();
            progress
                .as_ref()
                .filter(|p| p.status == DownloadStatus::Completed)
                .map(|p| p.model_id.clone())
        };

        if let Some(completed_model_id) = download_info {
            // Request models refresh
            self.stt_models_need_refresh = true;

            // Check if the model now shows as downloaded in the models list
            let model_confirmed_downloaded = self
                .stt_available_models
                .iter()
                .any(|m| m.id == completed_model_id && m.is_downloaded);

            // Only clear the download progress after the model list confirms the download
            if model_confirmed_downloaded {
                *self.stt_download_progress.lock().unwrap() = None;
            }
        }
    }

    /// Play back the last recorded audio
    fn play_last_audio(&self) {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

        let audio_data = {
            let guard = self.stt_last_audio.lock().unwrap();
            match guard.as_ref() {
                Some(data) => data.clone(),
                None => {
                    self.add_debug_message("No audio to play");
                    return;
                }
            }
        };

        if self.stt_is_playing.load(Ordering::Relaxed) {
            self.add_debug_message("Already playing");
            return;
        }

        let is_playing = self.stt_is_playing.clone();
        let debug_log = self.stt_debug_log.clone();

        self.add_debug_message(&format!(
            "Playing audio: {} ms, {} samples at {} Hz",
            audio_data.duration_ms(),
            audio_data.samples.len(),
            audio_data.sample_rate
        ));

        thread::spawn(move || {
            is_playing.store(true, Ordering::Relaxed);

            let host = cpal::default_host();
            let device = match host.default_output_device() {
                Some(d) => d,
                None => {
                    Self::add_debug_message_to_log(&debug_log, "No output device available");
                    is_playing.store(false, Ordering::Relaxed);
                    return;
                }
            };

            let config = match device.default_output_config() {
                Ok(c) => c,
                Err(e) => {
                    Self::add_debug_message_to_log(
                        &debug_log,
                        &format!("Failed to get output config: {}", e),
                    );
                    is_playing.store(false, Ordering::Relaxed);
                    return;
                }
            };

            let output_sample_rate = config.sample_rate().0;
            let output_channels = config.channels() as usize;

            // Resample if needed
            let resampled = audio_data.resample(output_sample_rate);
            let samples = resampled.samples.clone();
            let sample_idx = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let sample_idx_clone = sample_idx.clone();
            let total_samples = samples.len();

            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => device.build_output_stream(
                    &config.into(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        for frame in data.chunks_mut(output_channels) {
                            let idx = sample_idx_clone.fetch_add(1, Ordering::Relaxed);
                            let sample = if idx < total_samples { samples[idx] } else { 0.0 };
                            for s in frame.iter_mut() {
                                *s = sample;
                            }
                        }
                    },
                    |err| log::error!("Audio playback error: {}", err),
                    None,
                ),
                _ => {
                    Self::add_debug_message_to_log(&debug_log, "Unsupported output format");
                    is_playing.store(false, Ordering::Relaxed);
                    return;
                }
            };

            let stream = match stream {
                Ok(s) => s,
                Err(e) => {
                    Self::add_debug_message_to_log(
                        &debug_log,
                        &format!("Failed to build output stream: {}", e),
                    );
                    is_playing.store(false, Ordering::Relaxed);
                    return;
                }
            };

            if let Err(e) = stream.play() {
                Self::add_debug_message_to_log(
                    &debug_log,
                    &format!("Failed to play stream: {}", e),
                );
                is_playing.store(false, Ordering::Relaxed);
                return;
            }

            // Wait for playback to complete
            let duration_ms = (total_samples as f32 / output_sample_rate as f32 * 1000.0) as u64;
            thread::sleep(std::time::Duration::from_millis(duration_ms + 100));

            Self::add_debug_message_to_log(&debug_log, "Playback complete");
            is_playing.store(false, Ordering::Relaxed);
        });
    }
}

impl Default for SttFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerFeature for SttFeature {
    fn id(&self) -> &'static str {
        "stt"
    }

    fn tab_info(&self) -> TabInfo {
        TabInfo::new("stt", "Speech to Text", 40) // After clipboard (30)
    }

    fn render(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        // Initialize model manager if needed
        self.ensure_stt_model_manager();
        self.refresh_stt_models();

        // Check async tasks (model loading, download complete)
        self.check_async_tasks(ctx);

        ui.heading("Speech to Text");
        ui.add_space(8.0);

        // Get current states
        let is_recording = self.stt_is_recording.load(Ordering::Relaxed);
        let is_transcribing = self.stt_transcribing.load(Ordering::Relaxed);

        // Update status based on current state
        if is_recording {
            self.stt_status = "Recording...".to_string();
        } else if is_transcribing {
            self.stt_status = "Transcribing...".to_string();
        } else if self.stt_initialized {
            self.stt_status = "Ready".to_string();
        }

        let is_loading_model = self.load_model_task.is_some();

        // Status section
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Status:");
                let status_color = if is_recording {
                    egui::Color32::RED
                } else if is_transcribing || is_loading_model {
                    egui::Color32::YELLOW
                } else if self.stt_initialized {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::GRAY
                };
                ui.label(egui::RichText::new(&self.stt_status).color(status_color));
            });

            // Show last transcription if available
            let last_text = self.stt_last_transcription.lock().unwrap().clone();
            if let Some(ref text) = last_text {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Last:");
                    ui.label(egui::RichText::new(text).italics());
                });
            }

            // Recording / Transcribing / Loading indicator
            if is_recording {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(egui::RichText::new("Recording...").color(egui::Color32::RED));
                });
            } else if is_loading_model {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Loading model...");
                });
            } else if is_transcribing {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(egui::RichText::new("Transcribing...").color(egui::Color32::YELLOW));
                });
            }
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Settings section
        ui.heading("Settings");
        ui.add_space(8.0);

        let mut config_changed = false;

        ui.group(|ui| {
            // Hotkey display
            ui.horizontal(|ui| {
                ui.label("Hotkey:");
                ui.label(self.stt_config.hotkey.display_string());
            });

            ui.add_space(4.0);

            // Hotkey enabled checkbox
            ui.horizontal(|ui| {
                ui.label("Global Hotkey:");
                let is_running = self.stt_hotkey_manager.as_ref().is_some_and(|m| m.is_running());

                let mut enabled = self.stt_config.hotkey_enabled;
                if ui.checkbox(&mut enabled, "Enabled").changed() {
                    self.stt_config.hotkey_enabled = enabled;
                    config_changed = true;

                    if enabled && !is_running {
                        self.init_stt_hotkey();
                    } else if !enabled && is_running {
                        if let Some(ref mut manager) = self.stt_hotkey_manager {
                            manager.stop();
                        }
                        self.stt_hotkey_manager = None;
                    }
                }

                // Status indicator
                if is_running {
                    ui.label(egui::RichText::new("Running").color(egui::Color32::GREEN));
                } else {
                    ui.label(egui::RichText::new("Stopped").color(egui::Color32::GRAY));
                }
            });

            ui.add_space(8.0);

            // Mode selection
            ui.label("Mode:");
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(
                        self.stt_config.hotkey.mode == HotkeyMode::PushToTalk,
                        "Push to Talk",
                    )
                    .clicked()
                {
                    self.stt_config.hotkey.mode = HotkeyMode::PushToTalk;
                    config_changed = true;
                }
                if ui
                    .selectable_label(self.stt_config.hotkey.mode == HotkeyMode::Toggle, "Toggle")
                    .clicked()
                {
                    self.stt_config.hotkey.mode = HotkeyMode::Toggle;
                    config_changed = true;
                }
            });

            ui.add_space(8.0);

            // Output mode selection
            ui.label("Output:");
            ui.horizontal(|ui| {
                for mode in OutputMode::all() {
                    if ui
                        .selectable_label(self.stt_config.output_mode == *mode, mode.display_name())
                        .clicked()
                    {
                        self.stt_config.output_mode = *mode;
                        config_changed = true;
                    }
                }
            });
        });

        // Save config if changed
        if config_changed {
            if let Err(e) = self.stt_config.save() {
                log::error!("Failed to save STT config: {}", e);
            }
        }

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Models section
        ui.heading("Models");
        ui.add_space(8.0);

        // Check for download progress updates
        let download_progress = self.stt_download_progress.lock().unwrap().clone();

        ui.group(|ui| {
            // Model selection dropdown
            ui.horizontal(|ui| {
                ui.label("Selected model:");
                egui::ComboBox::from_id_salt("stt_model_selector")
                    .selected_text(&self.stt_config.model_id)
                    .show_ui(ui, |ui| {
                        for model in &self.stt_available_models {
                            if model.is_downloaded
                                && ui
                                    .selectable_label(
                                        self.stt_config.model_id == model.id,
                                        &model.id,
                                    )
                                    .clicked()
                            {
                                self.stt_config.model_id = model.id.clone();
                                // Unload engine when switching models
                                self.stt_whisper_engine = None;
                                self.stt_initialized = false;
                                let _ = self.stt_config.save();
                            }
                        }
                    });
            });

            ui.add_space(4.0);

            // Device selection
            ui.horizontal(|ui| {
                ui.label("Device:");

                // CPU option (always available)
                let cpu_selected = self.selected_device == SttDevice::Cpu;
                if ui.selectable_label(cpu_selected, "CPU").clicked()
                    && self.selected_device != SttDevice::Cpu
                {
                    self.selected_device = SttDevice::Cpu;
                    // Unload engine when switching device
                    self.stt_whisper_engine = None;
                    self.stt_initialized = false;
                    self.stt_status = "Device changed - reload model".to_string();
                }

                // GPU option (only if available)
                let gpu_text = if cfg!(target_os = "macos") { "Metal" } else { "CUDA" };

                ui.add_enabled_ui(self.gpu_available, |ui| {
                    let gpu_selected = self.selected_device == SttDevice::Gpu;
                    if ui.selectable_label(gpu_selected, gpu_text).clicked()
                        && self.selected_device != SttDevice::Gpu
                    {
                        self.selected_device = SttDevice::Gpu;
                        // Unload engine when switching device
                        self.stt_whisper_engine = None;
                        self.stt_initialized = false;
                        self.stt_status = "Device changed - reload model".to_string();
                    }
                });

                if !self.gpu_available {
                    ui.label(
                        egui::RichText::new("(GPU not available)")
                            .small()
                            .color(egui::Color32::GRAY),
                    );
                }
            });

            ui.add_space(4.0);

            // Load Model button
            let has_downloaded_model = self
                .stt_available_models
                .iter()
                .any(|m| m.is_downloaded && m.id == self.stt_config.model_id);

            if self.stt_whisper_engine.is_none()
                && has_downloaded_model
                && !is_loading_model
                && ui.button("Load Model").clicked()
            {
                if let Some(ref manager) = self.stt_model_manager {
                    let model_path =
                        manager.model_dir(ModelType::Whisper, &self.stt_config.model_id);
                    self.start_model_load(model_path);
                }
            }

            ui.add_space(8.0);

            // Download progress bar if downloading
            if let Some(ref progress) = download_progress {
                if progress.status == DownloadStatus::Downloading {
                    ui.horizontal(|ui| {
                        ui.label(format!("Downloading {}:", progress.model_id));
                        ui.add(
                            egui::ProgressBar::new(progress.overall_progress())
                                .text(progress.progress_percent()),
                        );
                    });
                    ui.add_space(4.0);
                }
            }

            // Models table
            ui.label("Available models:");
            ui.add_space(4.0);

            egui::Grid::new("stt_models_grid")
                .num_columns(4)
                .striped(true)
                .spacing([8.0, 4.0])
                .show(ui, |ui| {
                    // Header
                    ui.label(egui::RichText::new("Model").strong());
                    ui.label(egui::RichText::new("Size").strong());
                    ui.label(egui::RichText::new("Status").strong());
                    ui.label(egui::RichText::new("Action").strong());
                    ui.end_row();

                    let mut model_to_download: Option<String> = None;
                    let mut model_to_delete: Option<String> = None;

                    for model in &self.stt_available_models {
                        ui.label(&model.name);
                        ui.label(model.size_display());

                        // Status
                        if model.is_downloaded {
                            ui.label(egui::RichText::new("Downloaded").color(egui::Color32::GREEN));
                        } else if let Some(ref progress) = download_progress {
                            if progress.model_id == model.id {
                                match &progress.status {
                                    DownloadStatus::Downloading => {
                                        ui.label(egui::RichText::new(progress.progress_percent()));
                                    }
                                    DownloadStatus::Completed => {
                                        ui.label(
                                            egui::RichText::new("Completed")
                                                .color(egui::Color32::GREEN),
                                        );
                                    }
                                    DownloadStatus::Failed(err) => {
                                        ui.label(
                                            egui::RichText::new("Failed").color(egui::Color32::RED),
                                        )
                                        .on_hover_text(err);
                                    }
                                    _ => {
                                        ui.label("-");
                                    }
                                }
                            } else {
                                ui.label("-");
                            }
                        } else {
                            ui.label("-");
                        }

                        // Action button
                        let is_downloading = download_progress
                            .as_ref()
                            .is_some_and(|p| p.status == DownloadStatus::Downloading);

                        if model.is_downloaded {
                            if ui.small_button("Delete").clicked() {
                                model_to_delete = Some(model.id.clone());
                            }
                        } else if is_downloading {
                            ui.add_enabled(false, egui::Button::new("..."));
                        } else if ui.small_button("Download").clicked() {
                            model_to_download = Some(model.id.clone());
                        }

                        ui.end_row();
                    }

                    // Handle download action
                    if let Some(model_id) = model_to_download {
                        self.start_stt_model_download(&model_id);
                    }

                    // Handle delete action
                    if let Some(model_id) = model_to_delete {
                        self.delete_stt_model(&model_id);
                    }
                });
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Output section
        ui.heading("Output Log");
        ui.add_space(8.0);

        ui.group(|ui| {
            let log = self.stt_debug_log.lock().unwrap();
            if log.is_empty() {
                ui.label(egui::RichText::new("No events yet. Press the hotkey to test.").weak());
            } else {
                egui::ScrollArea::vertical().max_height(150.0).stick_to_bottom(true).show(
                    ui,
                    |ui| {
                        for msg in log.iter() {
                            ui.label(egui::RichText::new(msg).monospace().size(11.0));
                        }
                    },
                );
            }

            ui.add_space(4.0);
            drop(log); // Release the lock before UI interactions

            ui.horizontal(|ui| {
                if ui.small_button("Clear Log").clicked() {
                    self.stt_debug_log.lock().unwrap().clear();
                }

                // Play button for debug playback
                let has_audio = self.stt_last_audio.lock().unwrap().is_some();
                let is_playing = self.stt_is_playing.load(Ordering::Relaxed);

                ui.add_enabled_ui(has_audio && !is_playing, |ui| {
                    if ui.small_button("Play Last Recording").clicked() {
                        self.play_last_audio();
                    }
                });

                if is_playing {
                    ui.spinner();
                    ui.label("Playing...");
                }
            });
        });
    }

    fn initialize(&mut self, _ctx: &mut ControllerContext) -> Result<()> {
        log::info!("STT feature initialized");

        // Start hotkey manager if enabled
        if self.stt_config.hotkey_enabled {
            self.init_stt_hotkey();
        }

        Ok(())
    }

    fn shutdown(&mut self) {
        // Stop hotkey manager if running
        if let Some(ref mut manager) = self.stt_hotkey_manager {
            manager.stop();
        }
        log::info!("STT feature shutdown");
    }
}
