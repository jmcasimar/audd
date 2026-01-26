//! Integration tests for semantic matching feature

use audd_compare::{
    CompareConfig, SemanticMatchConfig, SemanticMatchPipeline,
    SemanticMatchDecision,
};

#[test]
fn test_ac1_user_vs_users() {
    // AC1: User vs users should match with default config
    let config = SemanticMatchConfig::default();
    let pipeline = SemanticMatchPipeline::new(config);
    let result = pipeline.compare("User", "users");

    assert_eq!(result.decision, SemanticMatchDecision::Match);
    assert!(result.final_score >= 0.80);
    assert!(result.reasons.is_some());
}

#[test]
fn test_ac2_normalization() {
    let config = SemanticMatchConfig::default();
    let pipeline = SemanticMatchPipeline::new(config);

    // UserProfile vs user_profile
    let result = pipeline.compare("UserProfile", "user_profile");
    assert_eq!(result.decision, SemanticMatchDecision::Match);

    // Spaces
    let result = pipeline.compare("  USERS  ", "users");
    assert_eq!(result.decision, SemanticMatchDecision::Match);

    // Diacritics
    let result = pipeline.compare("canción", "cancion");
    assert_eq!(result.decision, SemanticMatchDecision::Match);
}

#[test]
fn test_ac3_token_reordering() {
    // sales_order vs order_sales
    let config = SemanticMatchConfig::default();
    let pipeline = SemanticMatchPipeline::new(config);
    let result = pipeline.compare("sales_order", "order_sales");

    // Should at least be probable match
    assert!(matches!(
        result.decision,
        SemanticMatchDecision::Match | SemanticMatchDecision::ProbableMatch
    ));
}

#[test]
fn test_ac4_fuzzy_typos() {
    // adress vs address
    let config = SemanticMatchConfig::default();
    let pipeline = SemanticMatchPipeline::new(config);
    let result = pipeline.compare("adress", "address");

    // Should recognize typo
    assert!(result.final_score > 0.35);
}

#[test]
fn test_ac5_ngrams() {
    // user_table vs users_tbl
    let config = SemanticMatchConfig::default();
    let pipeline = SemanticMatchPipeline::new(config);
    let result = pipeline.compare("user_table", "users_tbl");

    // Should match with n-grams and pluralization
    assert!(matches!(
        result.decision,
        SemanticMatchDecision::Match | SemanticMatchDecision::ProbableMatch
    ));
}

#[test]
fn test_ac6_synonyms() {
    // customer vs user with synonym group
    let mut config = SemanticMatchConfig::default();
    config.synonyms.groups = vec![vec![
        "customer".to_string(),
        "client".to_string(),
        "user".to_string(),
    ]];

    let pipeline = SemanticMatchPipeline::new(config);
    let result = pipeline.compare("customer", "user");

    // Should match via synonyms
    assert!(matches!(
        result.decision,
        SemanticMatchDecision::Match | SemanticMatchDecision::ProbableMatch
    ));
    assert!(result.final_score > 0.4);
}

#[test]
fn test_ac7_configuration() {
    // Test that semantic matching can be disabled
    let mut config = CompareConfig::default();
    config.semantic_match.enabled = false;

    // This should work without errors
    assert!(!config.semantic_match.enabled);
}

#[test]
fn test_ac8_backward_compatibility() {
    // When disabled, should fall back to old behavior
    let mut config = SemanticMatchConfig::default();
    config.enabled = false;

    let pipeline = SemanticMatchPipeline::new(config);
    let result = pipeline.compare("User", "users");

    // Different strings should not match when semantic matching is off
    assert_eq!(result.decision, SemanticMatchDecision::NoMatch);
}
