//! Configuration for image template matching

/// Configuration for image template matching
#[derive(Debug, Clone)]
pub struct MatchConfig {
    /// Search at multiple scales (default: true)
    pub search_multiple_scales: bool,
    /// Convert to grayscale for faster matching (default: false)
    pub use_grayscale: bool,
    /// Scale factors to search (default: [1.0, 0.9, 0.8, 0.7, 0.6, 0.5])
    pub scale_steps: Vec<f32>,
    /// Minimum confidence threshold 0.0-1.0 (default: 0.8)
    pub confidence: f32,
    /// Maximum number of results to return (default: 100)
    pub limit: usize,
    /// Use parallel processing (default: true)
    pub parallel: bool,
}

impl Default for MatchConfig {
    fn default() -> Self {
        Self {
            search_multiple_scales: true,
            use_grayscale: false,
            scale_steps: vec![1.0, 0.9, 0.8, 0.7, 0.6, 0.5],
            confidence: 0.8,
            limit: 100,
            parallel: true,
        }
    }
}

impl MatchConfig {
    /// Create a new config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set search_multiple_scales
    pub fn with_multi_scale(mut self, enabled: bool) -> Self {
        self.search_multiple_scales = enabled;
        self
    }

    /// Set use_grayscale
    pub fn with_grayscale(mut self, enabled: bool) -> Self {
        self.use_grayscale = enabled;
        self
    }

    /// Set scale_steps
    pub fn with_scale_steps(mut self, steps: Vec<f32>) -> Self {
        self.scale_steps = steps;
        self
    }

    /// Set confidence threshold
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Set result limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Set parallel processing
    pub fn with_parallel(mut self, enabled: bool) -> Self {
        self.parallel = enabled;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = MatchConfig::default();
        assert!(config.search_multiple_scales);
        assert!(!config.use_grayscale);
        assert_eq!(config.confidence, 0.8);
        assert_eq!(config.limit, 100);
        assert!(config.parallel);
        assert!(!config.scale_steps.is_empty());
    }

    #[test]
    fn test_config_builder() {
        let config = MatchConfig::new()
            .with_confidence(0.9)
            .with_multi_scale(false)
            .with_grayscale(true)
            .with_limit(10);

        assert_eq!(config.confidence, 0.9);
        assert!(!config.search_multiple_scales);
        assert!(config.use_grayscale);
        assert_eq!(config.limit, 10);
    }

    #[test]
    fn test_confidence_clamping() {
        let config = MatchConfig::new().with_confidence(1.5);
        assert_eq!(config.confidence, 1.0);

        let config = MatchConfig::new().with_confidence(-0.5);
        assert_eq!(config.confidence, 0.0);
    }
}
