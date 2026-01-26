//! Configuration for comparison engine

use serde::{Deserialize, Serialize};

/// Language/locale for semantic matching
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MatchLocale {
    /// English language rules
    En,
    /// Spanish language rules
    Es,
    /// Mixed/universal rules
    Mixed,
}

impl Default for MatchLocale {
    fn default() -> Self {
        Self::En
    }
}

/// Unicode normalization form
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum UnicodeNormalization {
    /// Canonical Composition
    NFC,
    /// Compatibility Composition
    NFKC,
}

impl Default for UnicodeNormalization {
    fn default() -> Self {
        Self::NFC
    }
}

/// Similarity metric for token and n-gram comparison
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SimilarityMetric {
    /// Jaccard similarity coefficient
    Jaccard,
    /// Dice coefficient (Sørensen-Dice)
    Dice,
}

impl Default for SimilarityMetric {
    fn default() -> Self {
        Self::Jaccard
    }
}

/// Fuzzy string matching algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FuzzyAlgorithm {
    /// Jaro-Winkler distance
    JaroWinkler,
    /// Levenshtein distance
    Levenshtein,
    /// Damerau-Levenshtein distance
    DamerauLevenshtein,
}

impl Default for FuzzyAlgorithm {
    fn default() -> Self {
        Self::JaroWinkler
    }
}

/// Configuration for semantic name matching
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticMatchConfig {
    /// Enable semantic matching
    pub enabled: bool,

    /// Language/locale for matching rules
    pub locale: MatchLocale,

    /// Enable detailed explanation of matches
    pub explain: bool,

    /// Normalization settings
    pub normalization: NormalizationConfig,

    /// Pluralization settings
    pub pluralization: PluralizationConfig,

    /// Stemming settings
    pub stemming: StemmingConfig,

    /// Synonym dictionary settings
    pub synonyms: SynonymConfig,

    /// Token similarity settings
    pub token_similarity: TokenSimilarityConfig,

    /// Fuzzy character matching settings
    pub fuzzy: FuzzyConfig,

    /// N-gram similarity settings
    pub ngrams: NgramConfig,

    /// Scoring weights for different strategies
    pub weights: ScoringWeights,

    /// Thresholds for match decisions
    pub thresholds: MatchThresholds,

    /// Allow probable matches to be treated as matches
    pub allow_probable_as_match: bool,
}

/// Normalization configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizationConfig {
    /// Remove diacritics (accents)
    pub remove_diacritics: bool,

    /// Unicode normalization form
    pub unicode_normalization: UnicodeNormalization,

    /// Split camelCase/PascalCase into tokens
    pub split_camel_case: bool,

    /// Stopwords to remove (e.g., "tbl", "table", "tmp")
    pub stopwords: Vec<String>,
}

impl Default for NormalizationConfig {
    fn default() -> Self {
        Self {
            remove_diacritics: true,
            unicode_normalization: UnicodeNormalization::NFC,
            split_camel_case: true,
            stopwords: vec![
                "tbl".to_string(),
                "table".to_string(),
                "tmp".to_string(),
                "backup".to_string(),
                "bak".to_string(),
            ],
        }
    }
}

/// Pluralization configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluralizationConfig {
    /// Enable pluralization normalization
    pub enabled: bool,

    /// Exceptions to standard pluralization rules
    pub exceptions: Vec<String>,
}

impl Default for PluralizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            exceptions: vec![],
        }
    }
}

/// Stemming configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StemmingConfig {
    /// Enable simple stemming
    pub enabled: bool,
}

impl Default for StemmingConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Synonym dictionary configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SynonymConfig {
    /// Enable synonym matching
    pub enabled: bool,

    /// Synonym groups (each group is a list of equivalent terms)
    pub groups: Vec<Vec<String>>,
}

impl Default for SynonymConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            groups: vec![
                // Example groups - can be customized
                vec!["id".to_string(), "identifier".to_string()],
                vec!["num".to_string(), "number".to_string(), "no".to_string()],
            ],
        }
    }
}

/// Token similarity configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenSimilarityConfig {
    /// Enable token similarity
    pub enabled: bool,

    /// Similarity metric to use
    pub metric: SimilarityMetric,
}

impl Default for TokenSimilarityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metric: SimilarityMetric::Jaccard,
        }
    }
}

/// Fuzzy matching configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FuzzyConfig {
    /// Enable fuzzy matching
    pub enabled: bool,

    /// Fuzzy algorithm to use
    pub algorithm: FuzzyAlgorithm,
}

impl Default for FuzzyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: FuzzyAlgorithm::JaroWinkler,
        }
    }
}

/// N-gram similarity configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NgramConfig {
    /// Enable n-gram similarity
    pub enabled: bool,

    /// Size of n-grams (2-5)
    pub n: usize,

    /// Similarity metric to use
    pub metric: SimilarityMetric,
}

impl Default for NgramConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            n: 3,
            metric: SimilarityMetric::Jaccard,
        }
    }
}

/// Scoring weights for different strategies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoringWeights {
    /// Weight for exact normalized match
    pub exact_normalized: f64,

    /// Weight for plural/stem match
    pub plural_stem: f64,

    /// Weight for synonym match
    pub synonyms: f64,

    /// Weight for token similarity
    pub token_similarity: f64,

    /// Weight for fuzzy character matching
    pub fuzzy_chars: f64,

    /// Weight for n-gram similarity
    pub ngrams: f64,
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            exact_normalized: 0.50,  // If exact match after normalization, strong signal
            plural_stem: 0.45,        // If same after plural/stem, very strong signal
            synonyms: 0.50,           // Synonyms are controlled equivalences, strong signal
            token_similarity: 0.30,   // Token overlap is good indicator
            fuzzy_chars: 0.20,        // Fuzzy can help with typos
            ngrams: 0.15,             // N-grams provide additional signal
        }
    }
}

/// Thresholds for match decisions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchThresholds {
    /// Threshold for automatic match (>= this score)
    pub auto_match: f64,

    /// Threshold for probable match (>= this score, < auto_match)
    pub probable_match: f64,
}

impl Default for MatchThresholds {
    fn default() -> Self {
        Self {
            auto_match: 0.80,
            probable_match: 0.55,
        }
    }
}

impl Default for SemanticMatchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            locale: MatchLocale::En,
            explain: true,
            normalization: NormalizationConfig::default(),
            pluralization: PluralizationConfig::default(),
            stemming: StemmingConfig::default(),
            synonyms: SynonymConfig::default(),
            token_similarity: TokenSimilarityConfig::default(),
            fuzzy: FuzzyConfig::default(),
            ngrams: NgramConfig::default(),
            weights: ScoringWeights::default(),
            thresholds: MatchThresholds::default(),
            allow_probable_as_match: false,
        }
    }
}

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

    /// Semantic matching configuration
    pub semantic_match: SemanticMatchConfig,
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
            semantic_match: SemanticMatchConfig::default(),
        }
    }
}

impl CompareConfig {
    /// Create a new configuration with all features enabled
    pub fn all_features() -> Self {
        Self {
            similarity_matching: true,
            semantic_match: SemanticMatchConfig::default(),
            ..Default::default()
        }
    }

    /// Create a minimal configuration (exact matching only)
    pub fn minimal() -> Self {
        let mut semantic_config = SemanticMatchConfig::default();
        semantic_config.enabled = false;

        Self {
            exact_matching: true,
            normalized_matching: false,
            similarity_matching: false,
            similarity_threshold: 0.7,
            check_type_compatibility: true,
            check_nullability: false,
            check_constraints: false,
            detect_collisions: false,
            semantic_match: semantic_config,
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
            semantic_match: SemanticMatchConfig::default(),
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
