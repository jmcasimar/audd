//! Matching logic for entities and fields

use audd_ir::{normalize_identifier, EntitySchema, FieldSchema};

use crate::config::CompareConfig;
use crate::result::{Match, MatchReason};
use crate::semantic::{SemanticMatchDecision, SemanticMatchPipeline};

/// Match entities by name
pub fn match_entities(
    entities_a: &[EntitySchema],
    entities_b: &[EntitySchema],
    config: &CompareConfig,
) -> Vec<Match> {
    let mut candidates = Vec::new();

    for (idx_a, entity_a) in entities_a.iter().enumerate() {
        for (idx_b, entity_b) in entities_b.iter().enumerate() {
            if let Some(m) = try_match_entity(entity_a, entity_b, idx_a, idx_b, config) {
                let tier = match_tier(&entity_a.entity_name, &entity_b.entity_name, &m.reason);
                candidates.push((tier, m));
            }
        }
    }

    assign_one_to_one(candidates, entities_a.len(), entities_b.len())
}

/// Try to match two entities
fn try_match_entity(
    entity_a: &EntitySchema,
    entity_b: &EntitySchema,
    idx_a: usize,
    idx_b: usize,
    config: &CompareConfig,
) -> Option<Match> {
    // Try semantic matching first if enabled
    if config.semantic_match.enabled {
        let pipeline = SemanticMatchPipeline::new(config.semantic_match.clone());
        let result = pipeline.compare(&entity_a.entity_name, &entity_b.entity_name);

        match result.decision {
            SemanticMatchDecision::Match => {
                let details = result.reasons.map(|reasons| {
                    serde_json::to_value(reasons).unwrap_or(serde_json::Value::Null)
                });
                return Some(Match::semantic(
                    entity_a.entity_name.clone(),
                    None,
                    result.final_score,
                    "match".to_string(),
                    details,
                    idx_a,
                    idx_b,
                ));
            }
            SemanticMatchDecision::ProbableMatch => {
                if config.semantic_match.allow_probable_as_match {
                    let details = result.reasons.map(|reasons| {
                        serde_json::to_value(reasons).unwrap_or(serde_json::Value::Null)
                    });
                    return Some(Match::semantic(
                        entity_a.entity_name.clone(),
                        None,
                        result.final_score,
                        "probable_match".to_string(),
                        details,
                        idx_a,
                        idx_b,
                    ));
                }
            }
            SemanticMatchDecision::NoMatch => {
                // Continue to legacy matching strategies
            }
        }
    }

    // Exact name match
    if config.exact_matching && entity_a.entity_name == entity_b.entity_name {
        return Some(Match::exact(
            entity_a.entity_name.clone(),
            None,
            idx_a,
            idx_b,
        ));
    }

    // Normalized name match
    if config.normalized_matching {
        let norm_a = normalize_identifier(&entity_a.entity_name);
        let norm_b = normalize_identifier(&entity_b.entity_name);

        if norm_a == norm_b && entity_a.entity_name != entity_b.entity_name {
            return Some(Match::normalized(
                norm_a,
                None,
                entity_a.entity_name.clone(),
                entity_b.entity_name.clone(),
                idx_a,
                idx_b,
            ));
        }
    }

    // Similarity-based match
    if config.similarity_matching {
        let score = calculate_similarity(&entity_a.entity_name, &entity_b.entity_name);
        if score >= config.similarity_threshold {
            return Some(Match::similarity(
                entity_a.entity_name.clone(),
                None,
                score,
                idx_a,
                idx_b,
            ));
        }
    }

    None
}

/// Match fields within an entity
pub fn match_fields(
    entity_name: &str,
    fields_a: &[FieldSchema],
    fields_b: &[FieldSchema],
    config: &CompareConfig,
) -> Vec<Match> {
    let mut candidates = Vec::new();

    for (idx_a, field_a) in fields_a.iter().enumerate() {
        for (idx_b, field_b) in fields_b.iter().enumerate() {
            if let Some(m) = try_match_field(entity_name, field_a, field_b, idx_a, idx_b, config) {
                let tier = match_tier(&field_a.field_name, &field_b.field_name, &m.reason);
                candidates.push((tier, m));
            }
        }
    }

    assign_one_to_one(candidates, fields_a.len(), fields_b.len())
}

/// Priority of a candidate match (lower is stronger):
/// exact name > normalized name > semantic > similarity.
///
/// The tier is derived from the names themselves, so an identical pair outranks
/// a semantic or similarity candidate even when the semantic pipeline produced it.
fn match_tier(name_a: &str, name_b: &str, reason: &MatchReason) -> u8 {
    if name_a == name_b {
        0
    } else if normalize_identifier(name_a) == normalize_identifier(name_b) {
        1
    } else if matches!(reason, MatchReason::Semantic { .. }) {
        2
    } else {
        3
    }
}

/// Greedy one-to-one assignment over candidate pairs.
///
/// Candidates are taken by tier, then by descending score; remaining ties are
/// broken by index (lower index pair first, independent of which side it comes
/// from) so the result is deterministic and does not depend on A/B order.
/// Each element of A and of B ends up in at most one match. The result is
/// returned ordered by (index_a, index_b).
fn assign_one_to_one(mut candidates: Vec<(u8, Match)>, len_a: usize, len_b: usize) -> Vec<Match> {
    candidates.sort_by(|(tier_x, x), (tier_y, y)| {
        tier_x
            .cmp(tier_y)
            .then_with(|| y.score.total_cmp(&x.score))
            .then_with(|| index_key(x).cmp(&index_key(y)))
    });

    let mut used_a = vec![false; len_a];
    let mut used_b = vec![false; len_b];
    let mut matches = Vec::new();

    for (_, m) in candidates {
        if used_a[m.index_a] || used_b[m.index_b] {
            continue;
        }
        used_a[m.index_a] = true;
        used_b[m.index_b] = true;
        matches.push(m);
    }

    matches.sort_by_key(|m| (m.index_a, m.index_b));
    matches
}

/// Tie-break key that is invariant under swapping A and B, except for the
/// mirrored pair (i, j) / (j, i), which is resolved by `index_a`.
fn index_key(m: &Match) -> (usize, usize, usize) {
    (
        m.index_a.min(m.index_b),
        m.index_a.max(m.index_b),
        m.index_a,
    )
}

/// Try to match two fields
fn try_match_field(
    entity_name: &str,
    field_a: &FieldSchema,
    field_b: &FieldSchema,
    idx_a: usize,
    idx_b: usize,
    config: &CompareConfig,
) -> Option<Match> {
    // Try semantic matching first if enabled
    if config.semantic_match.enabled {
        let pipeline = SemanticMatchPipeline::new(config.semantic_match.clone());
        let result = pipeline.compare(&field_a.field_name, &field_b.field_name);

        match result.decision {
            SemanticMatchDecision::Match => {
                let details = result.reasons.map(|reasons| {
                    serde_json::to_value(reasons).unwrap_or(serde_json::Value::Null)
                });
                return Some(Match::semantic(
                    entity_name.to_string(),
                    Some(field_a.field_name.clone()),
                    result.final_score,
                    "match".to_string(),
                    details,
                    idx_a,
                    idx_b,
                ));
            }
            SemanticMatchDecision::ProbableMatch => {
                if config.semantic_match.allow_probable_as_match {
                    let details = result.reasons.map(|reasons| {
                        serde_json::to_value(reasons).unwrap_or(serde_json::Value::Null)
                    });
                    return Some(Match::semantic(
                        entity_name.to_string(),
                        Some(field_a.field_name.clone()),
                        result.final_score,
                        "probable_match".to_string(),
                        details,
                        idx_a,
                        idx_b,
                    ));
                }
            }
            SemanticMatchDecision::NoMatch => {
                // Continue to legacy matching strategies
            }
        }
    }

    // Exact name match
    if config.exact_matching && field_a.field_name == field_b.field_name {
        return Some(Match::exact(
            entity_name.to_string(),
            Some(field_a.field_name.clone()),
            idx_a,
            idx_b,
        ));
    }

    // Normalized name match
    if config.normalized_matching {
        let norm_a = normalize_identifier(&field_a.field_name);
        let norm_b = normalize_identifier(&field_b.field_name);

        if norm_a == norm_b && field_a.field_name != field_b.field_name {
            return Some(Match::normalized(
                entity_name.to_string(),
                Some(norm_a),
                field_a.field_name.clone(),
                field_b.field_name.clone(),
                idx_a,
                idx_b,
            ));
        }
    }

    // Similarity-based match
    if config.similarity_matching {
        let score = calculate_similarity(&field_a.field_name, &field_b.field_name);
        if score >= config.similarity_threshold {
            return Some(Match::similarity(
                entity_name.to_string(),
                Some(field_a.field_name.clone()),
                score,
                idx_a,
                idx_b,
            ));
        }
    }

    None
}

/// Calculate similarity between two strings using Jaro-Winkler
pub fn calculate_similarity(a: &str, b: &str) -> f64 {
    strsim::jaro_winkler(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use audd_ir::CanonicalType;

    fn create_test_entity(name: &str) -> EntitySchema {
        EntitySchema::builder().entity_name(name).build()
    }

    fn create_test_field(name: &str) -> FieldSchema {
        FieldSchema::builder()
            .field_name(name)
            .canonical_type(CanonicalType::Int32)
            .nullable(false)
            .build()
    }

    #[test]
    fn test_exact_entity_match() {
        let mut config = CompareConfig::default();
        config.semantic_match.enabled = false;
        let entities_a = vec![create_test_entity("users")];
        let entities_b = vec![create_test_entity("users")];

        let matches = match_entities(&entities_a, &entities_b, &config);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].entity_name, "users");
        assert!(matches!(matches[0].reason, MatchReason::ExactName));
    }

    #[test]
    fn test_normalized_entity_match() {
        let mut config = CompareConfig::default();
        config.semantic_match.enabled = false;
        let entities_a = vec![create_test_entity("UserTable")];
        let entities_b = vec![create_test_entity("user_table")];

        let matches = match_entities(&entities_a, &entities_b, &config);

        assert_eq!(matches.len(), 1);
        match &matches[0].reason {
            MatchReason::NormalizedName {
                original_a,
                original_b,
            } => {
                assert_eq!(original_a, "UserTable");
                assert_eq!(original_b, "user_table");
            }
            _ => panic!("Expected NormalizedName match"),
        }
    }

    #[test]
    fn test_similarity_entity_match() {
        let mut config = CompareConfig::all_features().with_similarity_threshold(0.8);
        config.semantic_match.enabled = false;
        let entities_a = vec![create_test_entity("users")];
        let entities_b = vec![create_test_entity("user")];

        let matches = match_entities(&entities_a, &entities_b, &config);

        assert_eq!(matches.len(), 1);
        match &matches[0].reason {
            MatchReason::Similarity { score } => {
                assert!(*score >= 0.8);
            }
            _ => panic!("Expected Similarity match"),
        }
    }

    #[test]
    fn test_exact_field_match() {
        let mut config = CompareConfig::default();
        config.semantic_match.enabled = false;
        let fields_a = vec![create_test_field("email")];
        let fields_b = vec![create_test_field("email")];

        let matches = match_fields("users", &fields_a, &fields_b, &config);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].field_name, Some("email".to_string()));
        assert!(matches!(matches[0].reason, MatchReason::ExactName));
    }

    #[test]
    fn test_normalized_field_match() {
        let mut config = CompareConfig::default();
        config.semantic_match.enabled = false;
        let fields_a = vec![create_test_field("firstName")];
        let fields_b = vec![create_test_field("first_name")];

        let matches = match_fields("users", &fields_a, &fields_b, &config);

        assert_eq!(matches.len(), 1);
        match &matches[0].reason {
            MatchReason::NormalizedName {
                original_a,
                original_b,
            } => {
                assert_eq!(original_a, "firstName");
                assert_eq!(original_b, "first_name");
            }
            _ => panic!("Expected NormalizedName match"),
        }
    }

    #[test]
    fn test_exact_field_wins_over_normalized() {
        let config = CompareConfig::default();
        let fields_a = vec![create_test_field("to_date")];
        let fields_b = vec![create_test_field("ToDate"), create_test_field("to_date")];

        let matches = match_fields("history", &fields_a, &fields_b, &config);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].index_b, 1);
    }

    #[test]
    fn test_entities_assigned_one_to_one() {
        let config = CompareConfig::default();
        let entities = vec![
            create_test_entity("Product"),
            create_test_entity("Product_Product"),
        ];

        let matches = match_entities(&entities, &entities, &config);

        assert_eq!(matches.len(), 2);
        assert!(matches.iter().all(|m| m.index_a == m.index_b));
    }

    #[test]
    fn test_no_match_below_threshold() {
        let config = CompareConfig::all_features().with_similarity_threshold(0.95);
        let entities_a = vec![create_test_entity("users")];
        let entities_b = vec![create_test_entity("customers")];

        let matches = match_entities(&entities_a, &entities_b, &config);

        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_calculate_similarity() {
        let score = calculate_similarity("users", "user");
        assert!(score > 0.9);

        let score = calculate_similarity("email", "e_mail");
        assert!(score > 0.8);

        let score = calculate_similarity("abc", "xyz");
        assert!(score < 0.5);
    }
}
