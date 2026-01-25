//! Integration tests for resolution engine with comparison results

use audd_compare::{Conflict, ConflictType};
use audd_resolution::{DecisionLog, SuggestionEngine, Decision, DecisionSource};

#[test]
fn test_generate_suggestions_for_conflicts() {
    // Create sample conflicts
    let conflicts = vec![
        Conflict::type_incompatible(
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            0,
            1,
        ),
        Conflict::normalization_collision(
            "users".to_string(),
            "UserID".to_string(),
            "user_id".to_string(),
            "user_id".to_string(),
            2,
            3,
        ),
    ];

    // Generate suggestions for all conflicts
    let engine = SuggestionEngine::new();
    let all_suggestions: Vec<_> = conflicts
        .iter()
        .flat_map(|conflict| engine.suggest(conflict))
        .collect();

    // Should have at least one suggestion per conflict
    assert!(all_suggestions.len() >= conflicts.len());

    // Verify suggestions are generated for each conflict type
    let has_cast_suggestion = all_suggestions.iter().any(|s| {
        matches!(
            s.kind,
            audd_resolution::SuggestionKind::CastSafe { .. }
        )
    });
    let has_rename_suggestion = all_suggestions.iter().any(|s| {
        matches!(
            s.kind,
            audd_resolution::SuggestionKind::RenameField { .. }
        )
    });

    assert!(has_cast_suggestion);
    assert!(has_rename_suggestion);
}

#[test]
fn test_decision_log_workflow() {
    // Create a conflict
    let conflict = Conflict::type_incompatible(
        "users".to_string(),
        "id".to_string(),
        "Int32".to_string(),
        "Int64".to_string(),
        0,
        1,
    );

    // Generate suggestions
    let engine = SuggestionEngine::new();
    let suggestions = engine.suggest(&conflict);

    assert!(!suggestions.is_empty());

    // Create a decision log
    let mut log = DecisionLog::new()
        .with_schema_ids("schema_a".to_string(), "schema_b".to_string());

    // Make a decision on the first suggestion
    let decision = Decision::accept(
        "dec1".to_string(),
        suggestions[0].clone(),
        "Safe widening cast approved".to_string(),
        DecisionSource::User {
            username: "admin".to_string(),
        },
    )
    .mark_applied();

    log.add_decision(decision);

    // Verify the log
    assert_eq!(log.metadata.total_decisions, 1);
    assert_eq!(log.metadata.accepted_decisions, 1);
    assert_eq!(log.get_accepted_decisions().len(), 1);

    // Export to JSON
    let json = log.to_json().unwrap();
    assert!(json.contains("schema_a"));
    assert!(json.contains("schema_b"));
    assert!(json.contains("Safe widening cast approved"));
}

#[test]
fn test_coverage_all_conflict_types_have_suggestions() {
    // Test that all conflict types produce at least one suggestion
    let engine = SuggestionEngine::new();

    // Type incompatible
    let conflict = Conflict::type_incompatible(
        "users".to_string(),
        "id".to_string(),
        "Int32".to_string(),
        "Int64".to_string(),
        0,
        1,
    );
    let suggestions = engine.suggest(&conflict);
    assert!(!suggestions.is_empty(), "TypeIncompatible should have suggestions");

    // Normalization collision
    let conflict = Conflict::normalization_collision(
        "users".to_string(),
        "UserID".to_string(),
        "user_id".to_string(),
        "user_id".to_string(),
        0,
        1,
    );
    let suggestions = engine.suggest(&conflict);
    assert!(!suggestions.is_empty(), "NormalizationCollision should have suggestions");

    // Nullability mismatch
    let conflict = Conflict::nullability_mismatch(
        "users".to_string(),
        "email".to_string(),
        false,
        true,
        0,
        1,
    );
    let suggestions = engine.suggest(&conflict);
    assert!(!suggestions.is_empty(), "NullabilityMismatch should have suggestions");

    // Constraint mismatch
    let conflict = Conflict::constraint_mismatch(
        "users".to_string(),
        "email".to_string(),
        "UNIQUE".to_string(),
        "NOT UNIQUE".to_string(),
        0,
        1,
    );
    let suggestions = engine.suggest(&conflict);
    assert!(!suggestions.is_empty(), "ConstraintMismatch should have suggestions");

    // All conflict types covered
    println!("✓ All conflict types produce at least one suggestion");
}

#[test]
fn test_export_decision_log_to_files() {
    use std::path::PathBuf;

    let mut log = DecisionLog::new();

    // Add a sample decision
    let suggestion = audd_resolution::Suggestion::cast_safe(
        "sug1".to_string(),
        "users".to_string(),
        "id".to_string(),
        "Int32".to_string(),
        "Int64".to_string(),
        "Safe cast".to_string(),
    );

    let decision = Decision::accept(
        "dec1".to_string(),
        suggestion,
        "Approved".to_string(),
        DecisionSource::User {
            username: "admin".to_string(),
        },
    );

    log.add_decision(decision);

    // Export to temp directory
    let temp_dir = std::env::temp_dir();
    let json_path = temp_dir.join("test_decision_log.json");
    
    log.to_json_file(&json_path).unwrap();
    assert!(json_path.exists());

    // Verify we can read it back
    let loaded_log = DecisionLog::from_json_file(&json_path).unwrap();
    assert_eq!(loaded_log.metadata.total_decisions, 1);

    // Clean up
    std::fs::remove_file(json_path).ok();
}

#[test]
fn test_suggestion_coverage_percentage() {
    // Generate various conflicts and ensure high coverage
    let conflicts = vec![
        Conflict::type_incompatible(
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            0,
            1,
        ),
        Conflict::type_incompatible(
            "users".to_string(),
            "score".to_string(),
            "Int64".to_string(),
            "Int32".to_string(),
            0,
            1,
        ),
        Conflict::normalization_collision(
            "users".to_string(),
            "UserID".to_string(),
            "user_id".to_string(),
            "user_id".to_string(),
            0,
            1,
        ),
        Conflict::nullability_mismatch(
            "users".to_string(),
            "email".to_string(),
            false,
            true,
            0,
            1,
        ),
    ];

    let engine = SuggestionEngine::new();
    let mut conflicts_with_suggestions = 0;

    for conflict in &conflicts {
        let suggestions = engine.suggest(conflict);
        if !suggestions.is_empty() {
            conflicts_with_suggestions += 1;
        }
    }

    let coverage = (conflicts_with_suggestions as f64 / conflicts.len() as f64) * 100.0;
    
    // MVP goal is ≥90% coverage
    assert!(
        coverage >= 90.0,
        "Suggestion coverage is {}%, expected ≥90%",
        coverage
    );
    
    println!("✓ Suggestion coverage: {:.1}%", coverage);
}
