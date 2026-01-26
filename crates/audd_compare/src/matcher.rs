//! Matching logic for entities and fields

use audd_ir::{normalize_identifier, EntitySchema, FieldSchema};

use crate::config::CompareConfig;
use crate::result::Match;

/// Match entities by name
pub fn match_entities(
    entities_a: &[EntitySchema],
    entities_b: &[EntitySchema],
    config: &CompareConfig,
) -> Vec<Match> {
    let mut matches = Vec::new();

    for (idx_a, entity_a) in entities_a.iter().enumerate() {
        for (idx_b, entity_b) in entities_b.iter().enumerate() {
            if let Some(m) = try_match_entity(entity_a, entity_b, idx_a, idx_b, config) {
                matches.push(m);
            }
        }
    }

    matches
}

/// Try to match two entities
fn try_match_entity(
    entity_a: &EntitySchema,
    entity_b: &EntitySchema,
    idx_a: usize,
    idx_b: usize,
    config: &CompareConfig,
) -> Option<Match> {
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
    let mut matches = Vec::new();

    for (idx_a, field_a) in fields_a.iter().enumerate() {
        for (idx_b, field_b) in fields_b.iter().enumerate() {
            if let Some(m) = try_match_field(entity_name, field_a, field_b, idx_a, idx_b, config) {
                matches.push(m);
            }
        }
    }

    matches
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
        let config = CompareConfig::default();
        let entities_a = vec![create_test_entity("users")];
        let entities_b = vec![create_test_entity("users")];

        let matches = match_entities(&entities_a, &entities_b, &config);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].entity_name, "users");
        assert!(matches!(matches[0].reason, MatchReason::ExactName));
    }

    #[test]
    fn test_normalized_entity_match() {
        let config = CompareConfig::default();
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
        let config = CompareConfig::all_features().with_similarity_threshold(0.8);
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
        let config = CompareConfig::default();
        let fields_a = vec![create_test_field("email")];
        let fields_b = vec![create_test_field("email")];

        let matches = match_fields("users", &fields_a, &fields_b, &config);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].field_name, Some("email".to_string()));
        assert!(matches!(matches[0].reason, MatchReason::ExactName));
    }

    #[test]
    fn test_normalized_field_match() {
        let config = CompareConfig::default();
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
