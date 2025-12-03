//! Voice Activity Detection (VAD) using Silero VAD ONNX model
//!
//! Detects speech in audio to enable auto-stop on silence.

use crate::error::{AumateError, Result};
use ort::session::builder::GraphOptimizationLevel;
use ort::session::{Session, SessionInputValue};
use ort::value::Value;
use std::path::Path;

/// Sample rate expected by Silero VAD (16kHz)
const VAD_SAMPLE_RATE: i64 = 16000;

/// Number of samples per VAD chunk (512 samples = 32ms at 16kHz)
const CHUNK_SIZE: usize = 512;

/// Voice Activity Detector using Silero VAD
pub struct VoiceActivityDetector {
    /// ONNX Runtime session
    session: Session,
    /// Hidden state (h) for LSTM
    h: Vec<f32>,
    /// Cell state (c) for LSTM
    c: Vec<f32>,
    /// Speech probability threshold
    threshold: f32,
    /// Consecutive silence chunks for auto-stop
    silence_chunks: usize,
    /// Maximum silence duration in chunks before auto-stop
    max_silence_chunks: usize,
}

impl VoiceActivityDetector {
    /// Create a new VAD from a model file
    pub fn new(model_path: &Path) -> Result<Self> {
        // Load model bytes from file
        let model_bytes = std::fs::read(model_path)
            .map_err(|e| AumateError::Other(format!("Failed to read VAD model file: {}", e)))?;

        // Initialize ONNX Runtime session
        let session = Session::builder()
            .map_err(|e| AumateError::Other(format!("Failed to create session builder: {}", e)))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| AumateError::Other(format!("Failed to set optimization level: {}", e)))?
            .commit_from_memory(&model_bytes)
            .map_err(|e| AumateError::Other(format!("Failed to load VAD model: {}", e)))?;

        // Initialize hidden states (2 layers * 64 units)
        let h = vec![0.0f32; 2 * 64];
        let c = vec![0.0f32; 2 * 64];

        Ok(Self {
            session,
            h,
            c,
            threshold: 0.5,
            silence_chunks: 0,
            max_silence_chunks: 47, // ~1.5s at 32ms per chunk
        })
    }

    /// Set the speech probability threshold (0.0 - 1.0)
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold.clamp(0.0, 1.0);
    }

    /// Set the maximum silence duration in milliseconds
    pub fn set_max_silence_ms(&mut self, ms: u32) {
        // Each chunk is 32ms
        self.max_silence_chunks = (ms / 32) as usize;
    }

    /// Reset the hidden states (call when starting a new recording)
    pub fn reset(&mut self) {
        self.h.fill(0.0);
        self.c.fill(0.0);
        self.silence_chunks = 0;
    }

    /// Process a chunk of audio and return speech probability
    ///
    /// Audio should be mono, 16kHz, f32 samples.
    /// Returns speech probability (0.0 - 1.0).
    pub fn process_chunk(&mut self, samples: &[f32]) -> Result<f32> {
        if samples.len() != CHUNK_SIZE {
            return Err(AumateError::Other(format!(
                "VAD expects {} samples, got {}",
                CHUNK_SIZE,
                samples.len()
            )));
        }

        // Create Value tensors using shape tuples
        let input_value = Value::from_array(([1usize, CHUNK_SIZE], samples.to_vec()))
            .map_err(|e| AumateError::Other(format!("Failed to create input value: {}", e)))?;
        let sr_value = Value::from_array(([1usize], vec![VAD_SAMPLE_RATE]))
            .map_err(|e| AumateError::Other(format!("Failed to create sr value: {}", e)))?;
        let h_value = Value::from_array(([2usize, 1, 64], self.h.clone()))
            .map_err(|e| AumateError::Other(format!("Failed to create h value: {}", e)))?;
        let c_value = Value::from_array(([2usize, 1, 64], self.c.clone()))
            .map_err(|e| AumateError::Other(format!("Failed to create c value: {}", e)))?;

        // Run inference
        let outputs = self
            .session
            .run(vec![
                ("input", SessionInputValue::from(input_value)),
                ("sr", SessionInputValue::from(sr_value)),
                ("h", SessionInputValue::from(h_value)),
                ("c", SessionInputValue::from(c_value)),
            ])
            .map_err(|e| AumateError::Other(format!("VAD inference failed: {}", e)))?;

        // Extract outputs
        let output = outputs
            .get("output")
            .ok_or_else(|| AumateError::Other("Missing output tensor".to_string()))?;

        let hn =
            outputs.get("hn").ok_or_else(|| AumateError::Other("Missing hn tensor".to_string()))?;

        let cn =
            outputs.get("cn").ok_or_else(|| AumateError::Other("Missing cn tensor".to_string()))?;

        // Get speech probability
        let output_tensor = output
            .try_extract_tensor::<f32>()
            .map_err(|e| AumateError::Other(format!("Failed to extract output: {}", e)))?;
        let (_, output_data) = output_tensor;
        let prob = output_data.first().copied().unwrap_or(0.0);

        // Update hidden states
        let hn_tensor = hn
            .try_extract_tensor::<f32>()
            .map_err(|e| AumateError::Other(format!("Failed to extract hn: {}", e)))?;
        let (_, hn_data) = hn_tensor;
        self.h = hn_data.to_vec();

        let cn_tensor = cn
            .try_extract_tensor::<f32>()
            .map_err(|e| AumateError::Other(format!("Failed to extract cn: {}", e)))?;
        let (_, cn_data) = cn_tensor;
        self.c = cn_data.to_vec();

        Ok(prob)
    }

    /// Check if audio chunk contains speech
    pub fn is_speech(&mut self, samples: &[f32]) -> Result<bool> {
        let prob = self.process_chunk(samples)?;
        Ok(prob >= self.threshold)
    }

    /// Process audio and check if we should auto-stop due to silence
    ///
    /// Returns (is_speech, should_stop)
    pub fn process_and_check_stop(&mut self, samples: &[f32]) -> Result<(bool, bool)> {
        let is_speech = self.is_speech(samples)?;

        if is_speech {
            self.silence_chunks = 0;
        } else {
            self.silence_chunks += 1;
        }

        let should_stop = self.silence_chunks >= self.max_silence_chunks;

        Ok((is_speech, should_stop))
    }

    /// Get the current speech probability threshold
    pub fn threshold(&self) -> f32 {
        self.threshold
    }

    /// Get chunk size in samples
    pub fn chunk_size() -> usize {
        CHUNK_SIZE
    }

    /// Get sample rate
    pub fn sample_rate() -> u32 {
        VAD_SAMPLE_RATE as u32
    }
}

/// Split audio into VAD-compatible chunks
#[allow(dead_code)]
pub fn split_into_chunks(samples: &[f32]) -> impl Iterator<Item = &[f32]> {
    samples.chunks(CHUNK_SIZE)
}

/// Analyze audio for speech segments
///
/// Returns a list of (start_sample, end_sample) tuples for speech segments.
#[allow(dead_code)]
pub fn detect_speech_segments(
    samples: &[f32],
    model_path: &Path,
    threshold: f32,
) -> Result<Vec<(usize, usize)>> {
    let mut vad = VoiceActivityDetector::new(model_path)?;
    vad.set_threshold(threshold);

    let mut segments = Vec::new();
    let mut in_speech = false;
    let mut speech_start = 0;

    for (chunk_idx, chunk) in samples.chunks(CHUNK_SIZE).enumerate() {
        // Pad last chunk if needed
        let padded: Vec<f32>;
        let chunk = if chunk.len() < CHUNK_SIZE {
            padded = chunk.iter().copied().chain(std::iter::repeat(0.0)).take(CHUNK_SIZE).collect();
            &padded
        } else {
            chunk
        };

        let is_speech = vad.is_speech(chunk)?;

        if is_speech && !in_speech {
            // Speech started
            speech_start = chunk_idx * CHUNK_SIZE;
            in_speech = true;
        } else if !is_speech && in_speech {
            // Speech ended
            let speech_end = chunk_idx * CHUNK_SIZE;
            segments.push((speech_start, speech_end));
            in_speech = false;
        }
    }

    // Handle case where speech extends to the end
    if in_speech {
        segments.push((speech_start, samples.len()));
    }

    Ok(segments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_size() {
        assert_eq!(VoiceActivityDetector::chunk_size(), 512);
    }

    #[test]
    fn test_sample_rate() {
        assert_eq!(VoiceActivityDetector::sample_rate(), 16000);
    }

    #[test]
    fn test_split_into_chunks() {
        let samples = vec![0.0f32; 1024];
        let chunks: Vec<_> = split_into_chunks(&samples).collect();
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].len(), 512);
        assert_eq!(chunks[1].len(), 512);
    }
}
