//! Audio capture using cpal
//!
//! Provides audio recording functionality for speech-to-text.

use crate::error::{AumateError, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Target sample rate for Whisper (16kHz)
pub const WHISPER_SAMPLE_RATE: u32 = 16000;

/// Audio data captured from microphone
#[derive(Debug, Clone)]
pub struct AudioData {
    /// Audio samples (mono, f32, normalized to [-1.0, 1.0])
    pub samples: Vec<f32>,
    /// Sample rate in Hz
    pub sample_rate: u32,
    /// Number of channels (should be 1 for mono)
    pub channels: u16,
}

impl AudioData {
    /// Get duration in seconds
    pub fn duration_secs(&self) -> f32 {
        if self.sample_rate == 0 {
            0.0
        } else {
            self.samples.len() as f32 / self.sample_rate as f32
        }
    }

    /// Get duration in milliseconds
    pub fn duration_ms(&self) -> u64 {
        (self.duration_secs() * 1000.0) as u64
    }

    /// Resample to target sample rate
    pub fn resample(&self, target_rate: u32) -> Self {
        if self.sample_rate == target_rate {
            return self.clone();
        }

        let ratio = target_rate as f32 / self.sample_rate as f32;
        let new_len = (self.samples.len() as f32 * ratio) as usize;
        let mut resampled = Vec::with_capacity(new_len);

        for i in 0..new_len {
            let src_idx = (i as f32 / ratio) as usize;
            let src_idx = src_idx.min(self.samples.len() - 1);
            resampled.push(self.samples[src_idx]);
        }

        AudioData { samples: resampled, sample_rate: target_rate, channels: self.channels }
    }

    /// Convert stereo to mono by averaging channels
    pub fn to_mono(&self) -> Self {
        if self.channels == 1 {
            return self.clone();
        }

        let mono_samples: Vec<f32> = self
            .samples
            .chunks(self.channels as usize)
            .map(|chunk| chunk.iter().sum::<f32>() / chunk.len() as f32)
            .collect();

        AudioData { samples: mono_samples, sample_rate: self.sample_rate, channels: 1 }
    }

    /// Prepare audio for Whisper (mono, 16kHz)
    pub fn prepare_for_whisper(&self) -> Self {
        let mono = self.to_mono();
        mono.resample(WHISPER_SAMPLE_RATE)
    }
}

/// Audio input device information
#[derive(Debug, Clone)]
pub struct AudioDevice {
    /// Device name
    pub name: String,
    /// Whether this is the default device
    pub is_default: bool,
}

/// Audio recorder for capturing microphone input
pub struct AudioRecorder {
    /// Collected audio samples
    samples: Arc<Mutex<Vec<f32>>>,
    /// Sample rate of the input device
    sample_rate: u32,
    /// Number of channels
    channels: u16,
    /// Whether currently recording
    is_recording: Arc<AtomicBool>,
    /// Active audio stream
    stream: Option<cpal::Stream>,
    /// Selected device name (None = default)
    device_name: Option<String>,
}

impl AudioRecorder {
    /// Create a new audio recorder with default input device
    pub fn new() -> Result<Self> {
        Ok(Self {
            samples: Arc::new(Mutex::new(Vec::new())),
            sample_rate: WHISPER_SAMPLE_RATE,
            channels: 1,
            is_recording: Arc::new(AtomicBool::new(false)),
            stream: None,
            device_name: None,
        })
    }

    /// List available input devices
    pub fn list_input_devices() -> Result<Vec<AudioDevice>> {
        let host = cpal::default_host();
        let default_device = host.default_input_device();
        let default_name = default_device.as_ref().and_then(|d| d.name().ok());

        let mut devices = Vec::new();
        if let Ok(input_devices) = host.input_devices() {
            for device in input_devices {
                if let Ok(name) = device.name() {
                    let is_default = default_name.as_ref() == Some(&name);
                    devices.push(AudioDevice { name, is_default });
                }
            }
        }

        Ok(devices)
    }

    /// Set the input device by name
    pub fn set_input_device(&mut self, name: Option<String>) {
        self.device_name = name;
    }

    /// Get the current input device name
    pub fn get_input_device(&self) -> Option<&str> {
        self.device_name.as_deref()
    }

    /// Check if currently recording
    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::Relaxed)
    }

    /// Start recording audio
    pub fn start_recording(&mut self) -> Result<()> {
        if self.is_recording() {
            return Ok(());
        }

        // Clear previous samples
        self.samples.lock().unwrap().clear();

        // Get audio host and device
        let host = cpal::default_host();
        let device = if let Some(ref name) = self.device_name {
            host.input_devices()
                .map_err(|e| AumateError::Other(format!("Failed to get input devices: {}", e)))?
                .find(|d| d.name().ok().as_ref() == Some(name))
                .ok_or_else(|| AumateError::Other(format!("Device not found: {}", name)))?
        } else {
            host.default_input_device()
                .ok_or_else(|| AumateError::Other("No default input device".to_string()))?
        };

        // Get device config
        let config = device
            .default_input_config()
            .map_err(|e| AumateError::Other(format!("Failed to get input config: {}", e)))?;

        self.sample_rate = config.sample_rate().0;
        self.channels = config.channels();

        // Create audio stream
        let samples = self.samples.clone();
        let is_recording = self.is_recording.clone();

        let err_fn = |err| {
            log::error!("Audio stream error: {}", err);
        };

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if is_recording.load(Ordering::Relaxed) {
                        samples.lock().unwrap().extend_from_slice(data);
                    }
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config.into(),
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    if is_recording.load(Ordering::Relaxed) {
                        let float_samples: Vec<f32> =
                            data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                        samples.lock().unwrap().extend(float_samples);
                    }
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::U16 => device.build_input_stream(
                &config.into(),
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    if is_recording.load(Ordering::Relaxed) {
                        let float_samples: Vec<f32> = data
                            .iter()
                            .map(|&s| (s as f32 / u16::MAX as f32) * 2.0 - 1.0)
                            .collect();
                        samples.lock().unwrap().extend(float_samples);
                    }
                },
                err_fn,
                None,
            ),
            _ => {
                return Err(AumateError::Other("Unsupported sample format".to_string()));
            }
        }
        .map_err(|e| AumateError::Other(format!("Failed to build audio stream: {}", e)))?;

        // Start the stream
        stream
            .play()
            .map_err(|e| AumateError::Other(format!("Failed to start audio stream: {}", e)))?;

        self.is_recording.store(true, Ordering::Relaxed);
        self.stream = Some(stream);

        log::info!("Started recording: {}Hz, {} channels", self.sample_rate, self.channels);

        Ok(())
    }

    /// Stop recording and return captured audio
    pub fn stop_recording(&mut self) -> Result<AudioData> {
        self.is_recording.store(false, Ordering::Relaxed);

        // Drop the stream to stop recording
        self.stream = None;

        // Get captured samples
        let samples = std::mem::take(&mut *self.samples.lock().unwrap());

        log::info!(
            "Stopped recording: {} samples ({:.2}s)",
            samples.len(),
            samples.len() as f32 / self.sample_rate as f32
        );

        Ok(AudioData { samples, sample_rate: self.sample_rate, channels: self.channels })
    }

    /// Get current recording level (RMS amplitude)
    pub fn get_level(&self) -> f32 {
        let samples = self.samples.lock().unwrap();
        if samples.is_empty() {
            return 0.0;
        }

        // Calculate RMS of last 1024 samples
        let window_size = 1024.min(samples.len());
        let start = samples.len() - window_size;
        let sum: f32 = samples[start..].iter().map(|s| s * s).sum();
        (sum / window_size as f32).sqrt()
    }
}

impl Drop for AudioRecorder {
    fn drop(&mut self) {
        self.is_recording.store(false, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_data_duration() {
        let data = AudioData { samples: vec![0.0; 16000], sample_rate: 16000, channels: 1 };
        assert!((data.duration_secs() - 1.0).abs() < 0.001);
        assert_eq!(data.duration_ms(), 1000);
    }

    #[test]
    fn test_audio_data_resample() {
        let data = AudioData { samples: vec![0.0; 48000], sample_rate: 48000, channels: 1 };
        let resampled = data.resample(16000);
        assert_eq!(resampled.sample_rate, 16000);
        assert_eq!(resampled.samples.len(), 16000);
    }

    #[test]
    fn test_audio_data_to_mono() {
        let data = AudioData {
            samples: vec![0.5, 0.5, 1.0, 1.0], // 2 stereo samples
            sample_rate: 16000,
            channels: 2,
        };
        let mono = data.to_mono();
        assert_eq!(mono.channels, 1);
        assert_eq!(mono.samples.len(), 2);
        assert!((mono.samples[0] - 0.5).abs() < 0.001);
        assert!((mono.samples[1] - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_list_input_devices() {
        // This test may fail on systems without audio devices
        let result = AudioRecorder::list_input_devices();
        // Just check it doesn't panic
        let _ = result;
    }
}
