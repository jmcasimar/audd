//! Semantic matching pipeline with configurable scoring

use serde::{Deserialize, Serialize};

use crate::config::SemanticMatchConfig;
use super::normalization::{normalize, NormalizedText};
use super::pluralization::{apply_stemming, normalize_plural};
use super::similarity::{calculate_fuzzy_similarity, calculate_ngram_similarity, calculate_token_similarity};
use super::synonyms::check_synonym_match;

/// Decision outcome for semantic matching
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticMatchDecision {
    /// Automatic match (score >= auto_match threshold)
    Match,
    /// Probable match (score >= probable_match threshold)
    ProbableMatch,
    /// No match (score < probable_match threshold)
    NoMatch,
}

/// Individual strategy score with details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrategyScore {
    /// Name of the strategy
    pub strategy: String,
    /// Score from this strategy (0.0 to 1.0)
    pub score: f64,
    /// Optional details about the match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
}

/// Complete result of semantic matching
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticMatchResult {
    /// Original string A
    pub a: String,
    /// Original string B
    pub b: String,
    /// Normalized string A
    pub normalized_a: String,
    /// Normalized string B
    pub normalized_b: String,
    /// Final combined score (0.0 to 1.0)
    pub final_score: f64,
    /// Match decision
    pub decision: SemanticMatchDecision,
    /// Individual strategy scores
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<StrategyScore>>,
}

/// Semantic matching pipeline
pub struct SemanticMatchPipeline {
    config: SemanticMatchConfig,
}

impl SemanticMatchPipeline {
    /// Create a new pipeline with the given configuration
    pub fn new(config: SemanticMatchConfig) -> Self {
        Self { config }
    }

    /// Compare two names using the semantic pipeline
    pub fn compare(&self, a: &str, b: &str) -> SemanticMatchResult {
        if !self.config.enabled {
            // Fallback to simple comparison when disabled
            return self.simple_comparison(a, b);
        }

        // Step 1: Normalize both strings
        let norm_a = normalize(a, &self.config.normalization);
        let norm_b = normalize(b, &self.config.normalization);

        // Step 2: Apply pluralization and stemming
        let tokens_a = self.apply_morphology(norm_a.tokens.clone());
        let tokens_b = self.apply_morphology(norm_b.tokens.clone());

        // Step 3: Calculate scores from each strategy
        let mut reasons = Vec::new();
        let mut weighted_score = 0.0;
        let mut total_weight = 0.0;

        // Strategy 1: Exact normalized match
        let exact_score = self.score_exact_normalized(&norm_a, &norm_b);
        if exact_score > 0.0 {
            weighted_score += exact_score * self.config.weights.exact_normalized;
            total_weight += self.config.weights.exact_normalized;
            if self.config.explain {
                reasons.push(StrategyScore {
                    strategy: "exact_normalized".to_string(),
                    score: exact_score,
                    detail: None,
                });
            }
        }

        // Strategy 2: Plural/stem match
        let plural_stem_score = self.score_plural_stem(&tokens_a, &tokens_b);
        if plural_stem_score > 0.0 {
            weighted_score += plural_stem_score * self.config.weights.plural_stem;
            total_weight += self.config.weights.plural_stem;
            if self.config.explain {
                reasons.push(StrategyScore {
                    strategy: "plural_stem".to_string(),
                    score: plural_stem_score,
                    detail: None,
                });
            }
        }

        // Strategy 3: Synonyms
        if self.config.synonyms.enabled {
            let (synonym_score, detail) = self.score_synonyms(&tokens_a, &tokens_b);
            if synonym_score > 0.0 {
                weighted_score += synonym_score * self.config.weights.synonyms;
                total_weight += self.config.weights.synonyms;
                if self.config.explain {
                    reasons.push(StrategyScore {
                        strategy: "synonyms".to_string(),
                        score: synonym_score,
                        detail,
                    });
                }
            }
        }

        // Strategy 4: Token similarity (always calculate, even if 0)
        if self.config.token_similarity.enabled {
            let (token_score, overlap) = calculate_token_similarity(
                &tokens_a,
                &tokens_b,
                &self.config.token_similarity,
            );
            weighted_score += token_score * self.config.weights.token_similarity;
            total_weight += self.config.weights.token_similarity;
            if self.config.explain && token_score > 0.0 {
                let detail = serde_json::json!({
                    "intersection": overlap.intersection,
                    "only_a": overlap.only_a,
                    "only_b": overlap.only_b,
                });
                reasons.push(StrategyScore {
                    strategy: "token_similarity".to_string(),
                    score: token_score,
                    detail: Some(detail),
                });
            }
        }

        // Strategy 5: Fuzzy character matching (always calculate, even if 0)
        if self.config.fuzzy.enabled {
            let fuzzy_score = calculate_fuzzy_similarity(
                &norm_a.text,
                &norm_b.text,
                &self.config.fuzzy,
            );
            weighted_score += fuzzy_score * self.config.weights.fuzzy_chars;
            total_weight += self.config.weights.fuzzy_chars;
            if self.config.explain && fuzzy_score > 0.0 {
                reasons.push(StrategyScore {
                    strategy: "fuzzy_chars".to_string(),
                    score: fuzzy_score,
                    detail: None,
                });
            }
        }

        // Strategy 6: N-grams (always calculate, even if 0)
        if self.config.ngrams.enabled {
            let ngram_score = calculate_ngram_similarity(
                &norm_a.text,
                &norm_b.text,
                &self.config.ngrams,
            );
            weighted_score += ngram_score * self.config.weights.ngrams;
            total_weight += self.config.weights.ngrams;
            if self.config.explain && ngram_score > 0.0 {
                reasons.push(StrategyScore {
                    strategy: "ngrams".to_string(),
                    score: ngram_score,
                    detail: None,
                });
            }
        }

        // Calculate final score
        let final_score = if total_weight > 0.0 {
            weighted_score / total_weight
        } else {
            0.0
        };

        // Determine decision
        let decision = if final_score >= self.config.thresholds.auto_match {
            SemanticMatchDecision::Match
        } else if final_score >= self.config.thresholds.probable_match {
            SemanticMatchDecision::ProbableMatch
        } else {
            SemanticMatchDecision::NoMatch
        };

        SemanticMatchResult {
            a: a.to_string(),
            b: b.to_string(),
            normalized_a: norm_a.text,
            normalized_b: norm_b.text,
            final_score,
            decision,
            reasons: if self.config.explain {
                Some(reasons)
            } else {
                None
            },
        }
    }

    /// Apply pluralization and stemming to tokens
    fn apply_morphology(&self, tokens: Vec<String>) -> Vec<String> {
        let mut result = tokens;

        if self.config.pluralization.enabled {
            result = normalize_plural(&result, &self.config.pluralization, self.config.locale);
        }

        if self.config.stemming.enabled {
            result = apply_stemming(&result, &self.config.stemming, self.config.locale);
        }

        result
    }

    /// Score exact normalized match
    fn score_exact_normalized(&self, norm_a: &NormalizedText, norm_b: &NormalizedText) -> f64 {
        if norm_a.text == norm_b.text {
            1.0
        } else {
            0.0
        }
    }

    /// Score after plural/stem normalization
    fn score_plural_stem(&self, tokens_a: &[String], tokens_b: &[String]) -> f64 {
        if tokens_a == tokens_b {
            1.0
        } else {
            0.0
        }
    }

    /// Score synonym matches
    fn score_synonyms(&self, tokens_a: &[String], tokens_b: &[String]) -> (f64, Option<serde_json::Value>) {
        if let Some(matches) = check_synonym_match(tokens_a, tokens_b, &self.config.synonyms) {
            let score = if !matches.is_empty() { 1.0 } else { 0.0 };
            let detail = serde_json::json!({
                "matches": matches.iter().map(|(a, b, group)| {
                    serde_json::json!({
                        "token_a": a,
                        "token_b": b,
                        "group_index": group,
                    })
                }).collect::<Vec<_>>(),
            });
            (score, Some(detail))
        } else {
            (0.0, None)
        }
    }

    /// Simple comparison for when semantic matching is disabled
    fn simple_comparison(&self, a: &str, b: &str) -> SemanticMatchResult {
        let decision = if a == b {
            SemanticMatchDecision::Match
        } else {
            SemanticMatchDecision::NoMatch
        };

        SemanticMatchResult {
            a: a.to_string(),
            b: b.to_string(),
            normalized_a: a.to_string(),
            normalized_b: b.to_string(),
            final_score: if a == b { 1.0 } else { 0.0 },
            decision,
            reasons: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> SemanticMatchConfig {
        SemanticMatchConfig::default()
    }

    #[test]
    fn test_user_vs_users() {
        let config = default_config();
        let pipeline = SemanticMatchPipeline::new(config);
        let result = pipeline.compare("User", "users");

        assert_eq!(result.decision, SemanticMatchDecision::Match);
        assert!(result.final_score >= 0.92);
    }

    #[test]
    fn test_user_profile_vs_user_profile() {
        let config = default_config();
        let pipeline = SemanticMatchPipeline::new(config);
        let result = pipeline.compare("UserProfile", "user_profile");

        assert_eq!(result.decision, SemanticMatchDecision::Match);
    }

    #[test]
    fn test_normalized_match() {
        let config = default_config();
        let pipeline = SemanticMatchPipeline::new(config);
        let result = pipeline.compare("  USERS  ", "users");

        assert_eq!(result.decision, SemanticMatchDecision::Match);
    }

    #[test]
    fn test_diacritics_match() {
        let config = default_config();
        let pipeline = SemanticMatchPipeline::new(config);
        let result = pipeline.compare("canción", "cancion");

        assert_eq!(result.decision, SemanticMatchDecision::Match);
    }

    #[test]
    fn test_token_reordering() {
        let config = default_config();
        let pipeline = SemanticMatchPipeline::new(config);
        let result = pipeline.compare("sales_order", "order_sales");

        // Should at least be a probable match due to token overlap
        assert!(matches!(
            result.decision,
            SemanticMatchDecision::Match | SemanticMatchDecision::ProbableMatch
        ));
    }

    #[test]
    fn test_fuzzy_typo() {
        let config = default_config();
        let pipeline = SemanticMatchPipeline::new(config);
        let result = pipeline.compare("adress", "address");

        // Fuzzy matching should recognize this as similar (typo with one missing 'd')
        // Even if it's not a perfect match, the score should be reasonably high
        assert!(result.final_score > 0.35, "Expected score > 0.35, got {}", result.final_score);
    }

    #[test]
    fn test_synonyms() {
        let mut config = default_config();
        config.synonyms.groups = vec![vec![
            "customer".to_string(),
            "client".to_string(),
            "user".to_string(),
        ]];
        config.explain = true;  // Enable explanations

        let pipeline = SemanticMatchPipeline::new(config);
        let result = pipeline.compare("customer", "user");

        // Print reasons for debugging
        if let Some(reasons) = &result.reasons {
            eprintln!("Reasons:");
            for r in reasons {
                eprintln!("  - {}: {}", r.strategy, r.score);
            }
        }
        eprintln!("Final score: {}", result.final_score);

        // Synonyms should provide a strong signal
        assert!(result.final_score > 0.4, "Expected score > 0.4, got {}", result.final_score);
    }

    #[test]
    fn test_no_match() {
        let config = default_config();
        let pipeline = SemanticMatchPipeline::new(config);
        let result = pipeline.compare("user", "product");

        assert_eq!(result.decision, SemanticMatchDecision::NoMatch);
    }

    #[test]
    fn test_disabled_semantic_matching() {
        let mut config = default_config();
        config.enabled = false;

        let pipeline = SemanticMatchPipeline::new(config);
        let result = pipeline.compare("User", "users");

        assert_eq!(result.decision, SemanticMatchDecision::NoMatch);
    }

    #[test]
    fn test_explain_reasons() {
        let mut config = default_config();
        config.explain = true;

        let pipeline = SemanticMatchPipeline::new(config);
        let result = pipeline.compare("User", "users");

        assert!(result.reasons.is_some());
        let reasons = result.reasons.unwrap();
        assert!(!reasons.is_empty());
    }

    #[test]
    fn test_no_explain() {
        let mut config = default_config();
        config.explain = false;

        let pipeline = SemanticMatchPipeline::new(config);
        let result = pipeline.compare("User", "users");

        assert!(result.reasons.is_none());
    }
}
