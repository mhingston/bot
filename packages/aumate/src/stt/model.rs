//! Model management for STT
//!
//! Handles downloading, storing, and managing Whisper and VAD models.

use crate::error::{AumateError, Result};
use futures_util::StreamExt;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Available Whisper models
pub const WHISPER_MODELS: &[(&str, &str, u64, &str)] = &[
    (
        "whisper-tiny",
        "Whisper Tiny (75 MB)",
        75_000_000,
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",
    ),
    (
        "whisper-base",
        "Whisper Base (142 MB)",
        142_000_000,
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin",
    ),
    (
        "whisper-small",
        "Whisper Small (466 MB)",
        466_000_000,
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",
    ),
    (
        "whisper-medium",
        "Whisper Medium (1.5 GB)",
        1_500_000_000,
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin",
    ),
];

/// Silero VAD model URL
pub const VAD_MODEL_URL: &str =
    "https://github.com/snakers4/silero-vad/raw/master/src/silero_vad/data/silero_vad.onnx";
pub const VAD_MODEL_ID: &str = "silero-vad";
pub const VAD_MODEL_SIZE: u64 = 2_000_000; // ~2MB

/// Information about a model
#[derive(Debug, Clone)]
pub struct ModelInfo {
    /// Model identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Size in bytes
    pub size_bytes: u64,
    /// Download URL
    pub url: String,
    /// Whether the model is downloaded
    pub is_downloaded: bool,
    /// Local file path if downloaded
    pub local_path: Option<PathBuf>,
}

impl ModelInfo {
    /// Get human-readable size string
    pub fn size_display(&self) -> String {
        if self.size_bytes >= 1_000_000_000 {
            format!("{:.1} GB", self.size_bytes as f64 / 1_000_000_000.0)
        } else if self.size_bytes >= 1_000_000 {
            format!("{} MB", self.size_bytes / 1_000_000)
        } else {
            format!("{} KB", self.size_bytes / 1_000)
        }
    }
}

/// Download progress information
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    /// Model being downloaded
    pub model_id: String,
    /// Bytes downloaded so far
    pub downloaded_bytes: u64,
    /// Total bytes to download
    pub total_bytes: u64,
    /// Current status
    pub status: DownloadStatus,
}

impl DownloadProgress {
    /// Get progress as a percentage (0.0 - 1.0)
    pub fn progress(&self) -> f32 {
        if self.total_bytes == 0 {
            0.0
        } else {
            self.downloaded_bytes as f32 / self.total_bytes as f32
        }
    }

    /// Get progress as a percentage string
    pub fn progress_percent(&self) -> String {
        format!("{:.1}%", self.progress() * 100.0)
    }
}

/// Download status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadStatus {
    /// Download not started
    Pending,
    /// Currently downloading
    Downloading,
    /// Download paused
    Paused,
    /// Download completed
    Completed,
    /// Download failed with error message
    Failed(String),
}

/// Model manager for downloading and managing models
pub struct ModelManager {
    /// Directory where models are stored
    models_dir: PathBuf,
    /// Current downloads in progress
    downloads: Arc<Mutex<HashMap<String, DownloadProgress>>>,
}

impl ModelManager {
    /// Create a new model manager
    pub fn new() -> Result<Self> {
        let models_dir = super::get_models_dir()?;
        Ok(Self { models_dir, downloads: Arc::new(Mutex::new(HashMap::new())) })
    }

    /// Get the models directory
    pub fn models_dir(&self) -> &Path {
        &self.models_dir
    }

    /// List all available models
    pub fn list_available_models(&self) -> Vec<ModelInfo> {
        WHISPER_MODELS
            .iter()
            .map(|(id, name, size, url)| {
                let local_path = self.models_dir.join(format!("{}.bin", id));
                let is_downloaded = local_path.exists();
                ModelInfo {
                    id: id.to_string(),
                    name: name.to_string(),
                    size_bytes: *size,
                    url: url.to_string(),
                    is_downloaded,
                    local_path: if is_downloaded { Some(local_path) } else { None },
                }
            })
            .collect()
    }

    /// List downloaded models
    pub fn list_downloaded_models(&self) -> Vec<ModelInfo> {
        self.list_available_models().into_iter().filter(|m| m.is_downloaded).collect()
    }

    /// Get the path to a downloaded model
    pub fn get_model_path(&self, model_id: &str) -> Option<PathBuf> {
        let path = self.models_dir.join(format!("{}.bin", model_id));
        if path.exists() { Some(path) } else { None }
    }

    /// Get the path to the VAD model
    pub fn get_vad_model_path(&self) -> Option<PathBuf> {
        let path = self.models_dir.join("silero_vad.onnx");
        if path.exists() { Some(path) } else { None }
    }

    /// Check if VAD model is downloaded
    pub fn is_vad_downloaded(&self) -> bool {
        self.get_vad_model_path().is_some()
    }

    /// Get VAD model info
    pub fn get_vad_model_info(&self) -> ModelInfo {
        let local_path = self.models_dir.join("silero_vad.onnx");
        let is_downloaded = local_path.exists();
        ModelInfo {
            id: VAD_MODEL_ID.to_string(),
            name: "Silero VAD".to_string(),
            size_bytes: VAD_MODEL_SIZE,
            url: VAD_MODEL_URL.to_string(),
            is_downloaded,
            local_path: if is_downloaded { Some(local_path) } else { None },
        }
    }

    /// Get download progress for a model
    pub fn get_download_progress(&self, model_id: &str) -> Option<DownloadProgress> {
        self.downloads.lock().unwrap().get(model_id).cloned()
    }

    /// Download a model (blocking)
    pub fn download_model_sync(
        &self,
        model_id: &str,
        progress_callback: Option<Box<dyn Fn(DownloadProgress) + Send>>,
    ) -> Result<PathBuf> {
        // Find the model
        let model_info =
            self.list_available_models()
                .into_iter()
                .find(|m| m.id == model_id)
                .or_else(|| {
                    if model_id == VAD_MODEL_ID { Some(self.get_vad_model_info()) } else { None }
                })
                .ok_or_else(|| AumateError::Other(format!("Unknown model: {}", model_id)))?;

        // Determine output path
        let filename = if model_id == VAD_MODEL_ID {
            "silero_vad.onnx".to_string()
        } else {
            format!("{}.bin", model_id)
        };
        let output_path = self.models_dir.join(&filename);
        let temp_path = self.models_dir.join(format!("{}.tmp", filename));

        // Initialize progress
        let progress = DownloadProgress {
            model_id: model_id.to_string(),
            downloaded_bytes: 0,
            total_bytes: model_info.size_bytes,
            status: DownloadStatus::Pending,
        };
        self.downloads.lock().unwrap().insert(model_id.to_string(), progress.clone());

        // Create a tokio runtime for the async download
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| AumateError::Other(format!("Failed to create runtime: {}", e)))?;

        let url = model_info.url.clone();
        let downloads = self.downloads.clone();
        let model_id_owned = model_id.to_string();

        let result = rt.block_on(async {
            // Check for existing partial download
            let start_pos = if temp_path.exists() {
                std::fs::metadata(&temp_path).map(|m| m.len()).unwrap_or(0)
            } else {
                0
            };

            // Build request with range header for resume
            let client = reqwest::Client::new();
            let mut request = client.get(&url);
            if start_pos > 0 {
                request = request.header("Range", format!("bytes={}-", start_pos));
            }

            let response = request
                .send()
                .await
                .map_err(|e| AumateError::Other(format!("Download failed: {}", e)))?;

            if !response.status().is_success()
                && response.status() != reqwest::StatusCode::PARTIAL_CONTENT
            {
                return Err(AumateError::Other(format!(
                    "Download failed with status: {}",
                    response.status()
                )));
            }

            // Get total size
            let total_size = response
                .content_length()
                .map(|len| len + start_pos)
                .unwrap_or(model_info.size_bytes);

            // Update progress
            {
                let mut downloads = downloads.lock().unwrap();
                if let Some(p) = downloads.get_mut(&model_id_owned) {
                    p.downloaded_bytes = start_pos;
                    p.total_bytes = total_size;
                    p.status = DownloadStatus::Downloading;
                }
            }

            // Open file for writing (append if resuming)
            let mut file =
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&temp_path)
                    .map_err(|e| AumateError::Other(format!("Failed to open file: {}", e)))?;

            let mut downloaded = start_pos;
            let mut stream = response.bytes_stream();

            while let Some(chunk_result) = stream.next().await {
                let chunk = chunk_result
                    .map_err(|e| AumateError::Other(format!("Download error: {}", e)))?;

                file.write_all(&chunk)
                    .map_err(|e| AumateError::Other(format!("Write error: {}", e)))?;

                downloaded += chunk.len() as u64;

                // Update progress
                {
                    let mut downloads = downloads.lock().unwrap();
                    if let Some(p) = downloads.get_mut(&model_id_owned) {
                        p.downloaded_bytes = downloaded;
                    }
                }

                // Call progress callback
                if let Some(ref callback) = progress_callback {
                    callback(DownloadProgress {
                        model_id: model_id_owned.clone(),
                        downloaded_bytes: downloaded,
                        total_bytes: total_size,
                        status: DownloadStatus::Downloading,
                    });
                }
            }

            // Rename temp file to final path
            std::fs::rename(&temp_path, &output_path)
                .map_err(|e| AumateError::Other(format!("Failed to rename file: {}", e)))?;

            // Mark as completed
            {
                let mut downloads = downloads.lock().unwrap();
                if let Some(p) = downloads.get_mut(&model_id_owned) {
                    p.status = DownloadStatus::Completed;
                }
            }

            Ok(output_path.clone())
        });

        // Handle error
        if let Err(ref e) = result {
            let mut downloads = self.downloads.lock().unwrap();
            if let Some(p) = downloads.get_mut(model_id) {
                p.status = DownloadStatus::Failed(e.to_string());
            }
        }

        result
    }

    /// Delete a downloaded model
    pub fn delete_model(&self, model_id: &str) -> Result<()> {
        let filename = if model_id == VAD_MODEL_ID {
            "silero_vad.onnx".to_string()
        } else {
            format!("{}.bin", model_id)
        };
        let path = self.models_dir.join(filename);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
        Ok(())
    }

    /// Verify model checksum (if checksum is provided)
    pub fn verify_model(&self, model_id: &str) -> Result<bool> {
        let path = self.get_model_path(model_id);
        if let Some(path) = path {
            // For now, just check if file exists and has non-zero size
            let metadata = std::fs::metadata(&path)?;
            Ok(metadata.len() > 0)
        } else {
            Ok(false)
        }
    }

    /// Calculate SHA-256 hash of a file
    #[allow(dead_code)]
    fn calculate_hash(path: &Path) -> Result<String> {
        let mut file = std::fs::File::open(path)?;
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    }
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new().expect("Failed to create model manager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_info_size_display() {
        let model = ModelInfo {
            id: "test".to_string(),
            name: "Test".to_string(),
            size_bytes: 142_000_000,
            url: "".to_string(),
            is_downloaded: false,
            local_path: None,
        };
        assert_eq!(model.size_display(), "142 MB");
    }

    #[test]
    fn test_download_progress() {
        let progress = DownloadProgress {
            model_id: "test".to_string(),
            downloaded_bytes: 50,
            total_bytes: 100,
            status: DownloadStatus::Downloading,
        };
        assert_eq!(progress.progress(), 0.5);
        assert_eq!(progress.progress_percent(), "50.0%");
    }

    #[test]
    fn test_list_available_models() {
        // This test requires the models directory to exist
        if let Ok(manager) = ModelManager::new() {
            let models = manager.list_available_models();
            assert!(!models.is_empty());
            assert!(models.iter().any(|m| m.id == "whisper-base"));
        }
    }
}
