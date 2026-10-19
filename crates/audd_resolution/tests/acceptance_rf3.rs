//! Acceptance tests for RF3: detect and classify the five conflict types and associate suggestions.
//!
//! Each test builds a synthetic schema pair that should produce exactly one conflict of the
//! target type and verifies that the resolution engine returns at least one suggestion.
//!
//! Note: ConstraintMismatch currently returns `NoSuggestion` because the resolution catalog
//! (R01 / section 6) is not yet integrated. The corresponding test documents this partial
//! state and will be updated once the catalog provides concrete constraint rules.

use audd_compare::{compare, CompareConfig, ConflictType};
use audd_ir::{CanonicalType, Constraint, EntitySchema, FieldSchema, SourceSchema};
use audd_resolution::SuggestionEngine;

fn field(name: &str, ty: CanonicalType, nullable: bool) -> FieldSchema {
    FieldSchema::builder()
        .field_name(name)
        .canonical_type(ty)
        .nullable(nullable)
        .build()
}

fn field_with_constraints(
    name: &str,
    ty: CanonicalType,
    nullable: bool,
    constraints: Vec<Constraint>,
) -> FieldSchema {
    let mut builder = FieldSchema::builder()
        .field_name(name)
        .canonical_type(ty)
        .nullable(nullable);
    for c in constraints {
        builder = builder.add_constraint(c);
    }
    builder.build()
}

fn entity(name: &str, fields: Vec<FieldSchema>) -> EntitySchema {
    EntitySchema::builder()
        .entity_name(name)
        .fields(fields)
        .build()
}

fn schema(name: &str, entities: Vec<EntitySchema>) -> SourceSchema {
    SourceSchema::builder()
        .source_name(name)
        .source_type("test")
        .entities(entities)
        .build()
}

#[test]
fn acceptance_rf3_type_incompatible() {
    let a = schema(
        "db_a",
        vec![entity(
            "users",
            vec![field("id", CanonicalType::Int32, false)],
        )],
    );
    let b = schema(
        "db_b",
        vec![entity(
            "users",
            vec![field("id", CanonicalType::String, false)],
        )],
    );

    let result = compare(&a, &b, &CompareConfig::default());
    let conflicts: Vec<_> = result
        .conflicts
        .iter()
        .filter(|c| c.conflict_type == ConflictType::TypeIncompatible)
        .collect();
    assert_eq!(
        conflicts.len(),
        1,
        "Expected exactly one TypeIncompatible conflict"
    );
    let suggestions = SuggestionEngine::new().suggest(conflicts[0]);
    assert!(
        suggestions
            .iter()
            .any(|s| !matches!(s.kind, audd_resolution::SuggestionKind::NoSuggestion { .. })),
        "Expected at least one concrete suggestion for TypeIncompatible"
    );
}

#[test]
fn acceptance_rf3_nullability_mismatch() {
    let a = schema(
        "db_a",
        vec![entity(
            "users",
            vec![field("email", CanonicalType::String, false)],
        )],
    );
    let b = schema(
        "db_b",
        vec![entity(
            "users",
            vec![field("email", CanonicalType::String, true)],
        )],
    );

    let result = compare(&a, &b, &CompareConfig::default());
    let conflicts: Vec<_> = result
        .conflicts
        .iter()
        .filter(|c| c.conflict_type == ConflictType::NullabilityMismatch)
        .collect();
    assert_eq!(
        conflicts.len(),
        1,
        "Expected exactly one NullabilityMismatch conflict"
    );
    let suggestions = SuggestionEngine::new().suggest(conflicts[0]);
    assert!(
        suggestions
            .iter()
            .any(|s| !matches!(s.kind, audd_resolution::SuggestionKind::NoSuggestion { .. })),
        "Expected at least one concrete suggestion for NullabilityMismatch"
    );
}

#[test]
fn acceptance_rf3_length_mismatch() {
    let a = schema(
        "db_a",
        vec![entity(
            "users",
            vec![field_with_constraints(
                "email",
                CanonicalType::String,
                false,
                vec![Constraint::max_length(255)],
            )],
        )],
    );
    let b = schema(
        "db_b",
        vec![entity(
            "users",
            vec![field_with_constraints(
                "email",
                CanonicalType::String,
                false,
                vec![Constraint::max_length(100)],
            )],
        )],
    );

    let result = compare(&a, &b, &CompareConfig::default());
    let conflicts: Vec<_> = result
        .conflicts
        .iter()
        .filter(|c| c.conflict_type == ConflictType::LengthMismatch)
        .collect();
    assert_eq!(
        conflicts.len(),
        1,
        "Expected exactly one LengthMismatch conflict"
    );
    let suggestions = SuggestionEngine::new().suggest(conflicts[0]);
    assert!(
        suggestions
            .iter()
            .any(|s| !matches!(s.kind, audd_resolution::SuggestionKind::NoSuggestion { .. })),
        "Expected at least one concrete suggestion for LengthMismatch"
    );
}

#[test]
fn acceptance_rf3_normalization_collision() {
    // Two different original names that normalize to the same identifier, neither matched.
    // Disable normalized and semantic matching so the fields remain unmatched and trigger
    // the collision detector.
    let mut config = CompareConfig::default();
    config.normalized_matching = false;
    config.semantic_match.enabled = false;

    let a = schema(
        "db_a",
        vec![entity(
            "users",
            vec![
                field("id", CanonicalType::Int32, false),
                field("UserID", CanonicalType::Int64, false),
            ],
        )],
    );
    let b = schema(
        "db_b",
        vec![entity(
            "users",
            vec![
                field("id", CanonicalType::Int32, false),
                field("user_id", CanonicalType::Int64, false),
            ],
        )],
    );

    let result = compare(&a, &b, &config);
    let collisions: Vec<_> = result
        .conflicts
        .iter()
        .filter(|c| c.conflict_type == ConflictType::NormalizationCollision)
        .collect();
    assert!(
        collisions.len() >= 1,
        "Expected at least one NormalizationCollision conflict"
    );
    let suggestions = SuggestionEngine::new().suggest(collisions[0]);
    assert!(
        suggestions
            .iter()
            .any(|s| !matches!(s.kind, audd_resolution::SuggestionKind::NoSuggestion { .. })),
        "Expected at least one concrete suggestion for NormalizationCollision"
    );
}

#[test]
fn acceptance_rf3_constraint_mismatch_with_catalog() {
    // With the default resolution catalog, ConstraintMismatch on UNIQUE should
    // produce a concrete PreferType suggestion instead of NoSuggestion.
    let a = schema(
        "db_a",
        vec![entity(
            "users",
            vec![field_with_constraints(
                "email",
                CanonicalType::String,
                false,
                vec![Constraint::unique()],
            )],
        )],
    );
    let b = schema(
        "db_b",
        vec![entity(
            "users",
            vec![field("email", CanonicalType::String, false)],
        )],
    );

    let result = compare(&a, &b, &CompareConfig::default());
    let conflicts: Vec<_> = result
        .conflicts
        .iter()
        .filter(|c| c.conflict_type == ConflictType::ConstraintMismatch)
        .collect();
    assert_eq!(
        conflicts.len(),
        1,
        "Expected exactly one ConstraintMismatch conflict"
    );

    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let catalog_dir = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("catalog");
    let catalog = audd_resolution::ResolutionCatalog::load_from_dir(&catalog_dir)
        .expect("Default catalog should be loadable from workspace root");
    let engine = audd_resolution::SuggestionEngine::new().with_catalog(catalog);
    let suggestions = engine.suggest(conflicts[0]);
    assert!(
        !suggestions.is_empty(),
        "Expected at least one suggestion entry"
    );
    assert!(
        suggestions
            .iter()
            .any(|s| !matches!(s.kind, audd_resolution::SuggestionKind::NoSuggestion { .. })),
        "Expected a concrete suggestion for UNIQUE ConstraintMismatch when catalog is loaded"
    );
    assert!(
        suggestions
            .iter()
            .any(|s| s.evidence.iter().any(|e| e.starts_with("catalog:"))),
        "Expected suggestion to trace back to a catalog rule"
    );
}
