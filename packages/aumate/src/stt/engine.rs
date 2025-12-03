//! Whisper transcription engine
//!
//! Provides speech-to-text transcription using Whisper.

use super::audio::AudioData;
use crate::error::{AumateError, Result};
use std::path::{Path, PathBuf};
use std::time::Instant;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Result of a transcription
#[derive(Debug, Clone)]
pub struct TranscriptionResult {
    /// Transcribed text
    pub text: String,
    /// Detected or specified language
    pub language: Option<String>,
    /// Transcription duration in milliseconds
    pub duration_ms: u64,
}

/// Whisper transcription engine
pub struct WhisperEngine {
    /// Whisper context (loaded model)
    context: Option<WhisperContext>,
    /// Path to the loaded model
    model_path: Option<PathBuf>,
    /// Language to use for transcription (None = auto-detect)
    language: Option<String>,
}

impl WhisperEngine {
    /// Create a new Whisper engine (no model loaded)
    pub fn new() -> Self {
        Self { context: None, model_path: None, language: None }
    }

    /// Load a Whisper model from file
    pub fn load_model(&mut self, path: &Path) -> Result<()> {
        log::info!("Loading Whisper model from: {:?}", path);

        if !path.exists() {
            return Err(AumateError::Other(format!("Model file not found: {:?}", path)));
        }

        let params = WhisperContextParameters::default();
        let context = WhisperContext::new_with_params(
            path.to_str().ok_or_else(|| AumateError::Other("Invalid model path".to_string()))?,
            params,
        )
        .map_err(|e| AumateError::Other(format!("Failed to load Whisper model: {}", e)))?;

        self.context = Some(context);
        self.model_path = Some(path.to_path_buf());

        log::info!("Whisper model loaded successfully");
        Ok(())
    }

    /// Unload the current model
    pub fn unload_model(&mut self) {
        self.context = None;
        self.model_path = None;
        log::info!("Whisper model unloaded");
    }

    /// Check if a model is loaded
    pub fn is_loaded(&self) -> bool {
        self.context.is_some()
    }

    /// Get the path to the loaded model
    pub fn model_path(&self) -> Option<&Path> {
        self.model_path.as_deref()
    }

    /// Set the language for transcription
    pub fn set_language(&mut self, language: Option<String>) {
        self.language = language;
    }

    /// Get the current language setting
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    /// Transcribe audio data
    pub fn transcribe(&self, audio: &AudioData) -> Result<TranscriptionResult> {
        let context = self
            .context
            .as_ref()
            .ok_or_else(|| AumateError::Other("No model loaded".to_string()))?;

        // Prepare audio for Whisper (mono, 16kHz)
        let prepared = audio.prepare_for_whisper();

        let start_time = Instant::now();

        // Create a new state for this transcription
        let mut state = context
            .create_state()
            .map_err(|e| AumateError::Other(format!("Failed to create state: {}", e)))?;

        // Configure transcription parameters
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

        // Set language if specified
        if let Some(ref lang) = self.language {
            params.set_language(Some(lang));
        } else {
            // Auto-detect language
            params.set_language(None);
        }

        // Configure for better real-time experience
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_translate(false);
        params.set_no_context(true);
        params.set_single_segment(true);

        // Run transcription
        state
            .full(params, &prepared.samples)
            .map_err(|e| AumateError::Other(format!("Transcription failed: {}", e)))?;

        // Get number of segments
        let num_segments = state
            .full_n_segments()
            .map_err(|e| AumateError::Other(format!("Failed to get segment count: {}", e)))?;

        // Collect all segment texts
        let mut text = String::new();
        for i in 0..num_segments {
            if let Ok(segment_text) = state.full_get_segment_text(i) {
                text.push_str(&segment_text);
            }
        }

        // Trim whitespace
        let text = text.trim().to_string();

        let duration_ms = start_time.elapsed().as_millis() as u64;

        log::info!(
            "Transcription completed in {}ms: \"{}\"",
            duration_ms,
            if text.len() > 50 { format!("{}...", &text[..50]) } else { text.clone() }
        );

        Ok(TranscriptionResult { text, language: self.language.clone(), duration_ms })
    }
}

impl Default for WhisperEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = WhisperEngine::new();
        assert!(!engine.is_loaded());
        assert!(engine.model_path().is_none());
    }

    #[test]
    fn test_language_setting() {
        let mut engine = WhisperEngine::new();
        assert!(engine.language().is_none());

        engine.set_language(Some("en".to_string()));
        assert_eq!(engine.language(), Some("en"));

        engine.set_language(None);
        assert!(engine.language().is_none());
    }

    #[test]
    fn test_transcribe_without_model() {
        let engine = WhisperEngine::new();
        let audio = AudioData { samples: vec![0.0; 16000], sample_rate: 16000, channels: 1 };

        let result = engine.transcribe(&audio);
        assert!(result.is_err());
    }
}
