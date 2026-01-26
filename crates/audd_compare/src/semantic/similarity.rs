//! Similarity metrics for token and n-gram comparison

use std::collections::HashSet;

use crate::config::{FuzzyAlgorithm, FuzzyConfig, NgramConfig, SimilarityMetric, TokenSimilarityConfig};

/// Token overlap information
#[derive(Debug, Clone, PartialEq)]
pub struct TokenOverlap {
    pub intersection: Vec<String>,
    pub only_a: Vec<String>,
    pub only_b: Vec<String>,
}

/// Calculate token similarity using configured metric
pub fn calculate_token_similarity(
    tokens_a: &[String],
    tokens_b: &[String],
    config: &TokenSimilarityConfig,
) -> (f64, TokenOverlap) {
    if !config.enabled {
        return (0.0, TokenOverlap {
            intersection: vec![],
            only_a: tokens_a.to_vec(),
            only_b: tokens_b.to_vec(),
        });
    }

    let set_a: HashSet<&String> = tokens_a.iter().collect();
    let set_b: HashSet<&String> = tokens_b.iter().collect();

    let intersection: Vec<String> = set_a.intersection(&set_b).map(|s| (*s).clone()).collect();
    let only_a: Vec<String> = set_a.difference(&set_b).map(|s| (*s).clone()).collect();
    let only_b: Vec<String> = set_b.difference(&set_a).map(|s| (*s).clone()).collect();

    let score = match config.metric {
        SimilarityMetric::Jaccard => {
            let union_size = set_a.union(&set_b).count();
            if union_size == 0 {
                0.0
            } else {
                intersection.len() as f64 / union_size as f64
            }
        }
        SimilarityMetric::Dice => {
            let total_size = set_a.len() + set_b.len();
            if total_size == 0 {
                0.0
            } else {
                (2.0 * intersection.len() as f64) / total_size as f64
            }
        }
    };

    (score, TokenOverlap {
        intersection,
        only_a,
        only_b,
    })
}

/// Calculate fuzzy character similarity
pub fn calculate_fuzzy_similarity(
    text_a: &str,
    text_b: &str,
    config: &FuzzyConfig,
) -> f64 {
    if !config.enabled || text_a.is_empty() || text_b.is_empty() {
        return 0.0;
    }

    match config.algorithm {
        FuzzyAlgorithm::JaroWinkler => strsim::jaro_winkler(text_a, text_b),
        FuzzyAlgorithm::Levenshtein => {
            let distance = strsim::levenshtein(text_a, text_b);
            let max_len = text_a.len().max(text_b.len());
            if max_len == 0 {
                1.0
            } else {
                1.0 - (distance as f64 / max_len as f64)
            }
        }
        FuzzyAlgorithm::DamerauLevenshtein => {
            let distance = strsim::damerau_levenshtein(text_a, text_b);
            let max_len = text_a.len().max(text_b.len());
            if max_len == 0 {
                1.0
            } else {
                1.0 - (distance as f64 / max_len as f64)
            }
        }
    }
}

/// Generate n-grams from text
pub fn generate_ngrams(text: &str, n: usize) -> HashSet<String> {
    if text.len() < n {
        return HashSet::from([text.to_string()]);
    }

    let chars: Vec<char> = text.chars().collect();
    let mut ngrams = HashSet::new();

    for i in 0..=chars.len().saturating_sub(n) {
        let ngram: String = chars[i..i + n].iter().collect();
        ngrams.insert(ngram);
    }

    ngrams
}

/// Calculate n-gram similarity
pub fn calculate_ngram_similarity(
    text_a: &str,
    text_b: &str,
    config: &NgramConfig,
) -> f64 {
    if !config.enabled || text_a.is_empty() || text_b.is_empty() {
        return 0.0;
    }

    let n = config.n.clamp(2, 5);
    let ngrams_a = generate_ngrams(text_a, n);
    let ngrams_b = generate_ngrams(text_b, n);

    if ngrams_a.is_empty() && ngrams_b.is_empty() {
        return 1.0;
    }

    let intersection = ngrams_a.intersection(&ngrams_b).count();

    match config.metric {
        SimilarityMetric::Jaccard => {
            let union = ngrams_a.union(&ngrams_b).count();
            if union == 0 {
                0.0
            } else {
                intersection as f64 / union as f64
            }
        }
        SimilarityMetric::Dice => {
            let total = ngrams_a.len() + ngrams_b.len();
            if total == 0 {
                0.0
            } else {
                (2.0 * intersection as f64) / total as f64
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token_config() -> TokenSimilarityConfig {
        TokenSimilarityConfig {
            enabled: true,
            metric: SimilarityMetric::Jaccard,
        }
    }

    fn fuzzy_config() -> FuzzyConfig {
        FuzzyConfig {
            enabled: true,
            algorithm: FuzzyAlgorithm::JaroWinkler,
        }
    }

    fn ngram_config() -> NgramConfig {
        NgramConfig {
            enabled: true,
            n: 3,
            metric: SimilarityMetric::Jaccard,
        }
    }

    #[test]
    fn test_token_similarity_jaccard() {
        let config = token_config();
        let tokens_a = vec!["user".to_string(), "profile".to_string()];
        let tokens_b = vec!["user".to_string(), "data".to_string()];

        let (score, overlap) = calculate_token_similarity(&tokens_a, &tokens_b, &config);

        assert!(score > 0.0 && score < 1.0);
        assert_eq!(overlap.intersection, vec!["user"]);
        assert_eq!(overlap.only_a.len(), 1);
        assert_eq!(overlap.only_b.len(), 1);
    }

    #[test]
    fn test_token_similarity_identical() {
        let config = token_config();
        let tokens = vec!["user".to_string(), "profile".to_string()];

        let (score, overlap) = calculate_token_similarity(&tokens, &tokens, &config);

        assert_eq!(score, 1.0);
        assert_eq!(overlap.intersection.len(), 2);
        assert!(overlap.only_a.is_empty());
        assert!(overlap.only_b.is_empty());
    }

    #[test]
    fn test_token_similarity_dice() {
        let config = TokenSimilarityConfig {
            enabled: true,
            metric: SimilarityMetric::Dice,
        };
        let tokens_a = vec!["user".to_string(), "profile".to_string()];
        let tokens_b = vec!["user".to_string(), "data".to_string()];

        let (score, _) = calculate_token_similarity(&tokens_a, &tokens_b, &config);
        assert!(score > 0.0 && score < 1.0);
    }

    #[test]
    fn test_fuzzy_similarity_jaro_winkler() {
        let config = fuzzy_config();
        let score = calculate_fuzzy_similarity("users", "user", &config);
        assert!(score > 0.9);

        let score = calculate_fuzzy_similarity("address", "adress", &config);
        assert!(score > 0.9);
    }

    #[test]
    fn test_fuzzy_similarity_levenshtein() {
        let config = FuzzyConfig {
            enabled: true,
            algorithm: FuzzyAlgorithm::Levenshtein,
        };
        let score = calculate_fuzzy_similarity("users", "user", &config);
        assert!(score > 0.7);
    }

    #[test]
    fn test_fuzzy_similarity_identical() {
        let config = fuzzy_config();
        let score = calculate_fuzzy_similarity("user", "user", &config);
        assert_eq!(score, 1.0);
    }

    #[test]
    fn test_generate_ngrams() {
        let ngrams = generate_ngrams("user", 3);
        assert!(ngrams.contains("use"));
        assert!(ngrams.contains("ser"));
        assert_eq!(ngrams.len(), 2);
    }

    #[test]
    fn test_generate_ngrams_short() {
        let ngrams = generate_ngrams("ab", 3);
        assert_eq!(ngrams.len(), 1);
        assert!(ngrams.contains("ab"));
    }

    #[test]
    fn test_ngram_similarity() {
        let config = ngram_config();
        let score = calculate_ngram_similarity("users", "user", &config);
        assert!(score > 0.5);
    }

    #[test]
    fn test_ngram_similarity_identical() {
        let config = ngram_config();
        let score = calculate_ngram_similarity("user", "user", &config);
        assert_eq!(score, 1.0);
    }

    #[test]
    fn test_disabled_token_similarity() {
        let config = TokenSimilarityConfig {
            enabled: false,
            metric: SimilarityMetric::Jaccard,
        };
        let tokens_a = vec!["user".to_string()];
        let tokens_b = vec!["customer".to_string()];
        let (score, _) = calculate_token_similarity(&tokens_a, &tokens_b, &config);
        assert_eq!(score, 0.0);
    }
}
