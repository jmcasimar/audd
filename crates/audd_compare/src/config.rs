//! Configuration for comparison engine

use serde::{Deserialize, Serialize};

/// Configuration for the comparison engine
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompareConfig {
    /// Enable exact name matching
    pub exact_matching: bool,

    /// Enable normalized name matching
    pub normalized_matching: bool,

    /// Enable similarity-based matching
    pub similarity_matching: bool,

    /// Minimum similarity score for a match (0.0 to 1.0)
    pub similarity_threshold: f64,

    /// Check type compatibility
    pub check_type_compatibility: bool,

    /// Check nullability compatibility
    pub check_nullability: bool,

    /// Check constraint compatibility
    pub check_constraints: bool,

    /// Detect normalization collisions
    pub detect_collisions: bool,
}

impl Default for CompareConfig {
    fn default() -> Self {
        Self {
            exact_matching: true,
            normalized_matching: true,
            similarity_matching: false,
            similarity_threshold: 0.7,
            check_type_compatibility: true,
            check_nullability: true,
            check_constraints: true,
            detect_collisions: true,
        }
    }
}

impl CompareConfig {
    /// Create a new configuration with all features enabled
    pub fn all_features() -> Self {
        Self {
            similarity_matching: true,
            ..Default::default()
        }
    }

    /// Create a minimal configuration (exact matching only)
    pub fn minimal() -> Self {
        Self {
            exact_matching: true,
            normalized_matching: false,
            similarity_matching: false,
            similarity_threshold: 0.7,
            check_type_compatibility: true,
            check_nullability: false,
            check_constraints: false,
            detect_collisions: false,
        }
    }

    /// Create a strict configuration (all checks enabled)
    pub fn strict() -> Self {
        Self {
            exact_matching: true,
            normalized_matching: true,
            similarity_matching: false,
            similarity_threshold: 0.9,
            check_type_compatibility: true,
            check_nullability: true,
            check_constraints: true,
            detect_collisions: true,
        }
    }

    /// Set the similarity threshold
    pub fn with_similarity_threshold(mut self, threshold: f64) -> Self {
        self.similarity_threshold = threshold.clamp(0.0, 1.0);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CompareConfig::default();
        assert!(config.exact_matching);
        assert!(config.normalized_matching);
        assert!(!config.similarity_matching);
        assert_eq!(config.similarity_threshold, 0.7);
    }

    #[test]
    fn test_minimal_config() {
        let config = CompareConfig::minimal();
        assert!(config.exact_matching);
        assert!(!config.normalized_matching);
        assert!(!config.similarity_matching);
    }

    #[test]
    fn test_all_features_config() {
        let config = CompareConfig::all_features();
        assert!(config.similarity_matching);
    }

    #[test]
    fn test_with_similarity_threshold() {
        let config = CompareConfig::default().with_similarity_threshold(0.8);
        assert_eq!(config.similarity_threshold, 0.8);

        // Test clamping
        let config = CompareConfig::default().with_similarity_threshold(1.5);
        assert_eq!(config.similarity_threshold, 1.0);

        let config = CompareConfig::default().with_similarity_threshold(-0.5);
        assert_eq!(config.similarity_threshold, 0.0);
    }
}
