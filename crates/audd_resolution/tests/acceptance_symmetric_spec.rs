//! Acceptance tests for the symmetric unification spec (E1, E2, E3, E5).
//!
//! AUDD builds a unified schema C from A and B, so every difference that forces a
//! decision on C is a conflict; severity separates safe from lossy changes. As a
//! consequence, swapping A and B must yield the same conflict types (RNF2 symmetry).

use std::collections::BTreeMap;

use audd_compare::{compare, CompareConfig, Conflict, ConflictSeverity, ConflictType};
use audd_ir::{CanonicalType, Constraint, EntitySchema, FieldSchema, Key, SourceSchema};
use audd_resolution::{SuggestionEngine, SuggestionKind};

fn field(name: &str, ty: CanonicalType, nullable: bool) -> FieldSchema {
    FieldSchema::builder()
        .field_name(name)
        .canonical_type(ty)
        .nullable(nullable)
        .build()
}

fn schema(name: &str, entity: EntitySchema) -> SourceSchema {
    SourceSchema::builder()
        .source_name(name)
        .source_type("test")
        .entities(vec![entity])
        .build()
}

fn single_field_pair(a: FieldSchema, b: FieldSchema) -> (SourceSchema, SourceSchema) {
    let ea = EntitySchema::builder()
        .entity_name("items")
        .add_field(a)
        .build();
    let eb = EntitySchema::builder()
        .entity_name("items")
        .add_field(b)
        .build();
    (schema("db_a", ea), schema("db_b", eb))
}

fn of_type(conflicts: &[Conflict], ty: ConflictType) -> Vec<&Conflict> {
    conflicts.iter().filter(|c| c.conflict_type == ty).collect()
}

#[test]
fn acceptance_rf3_type_widening_is_low_severity_conflict() {
    // E1: Int32 -> Int64 is a safe widening, reported with low severity
    let (a, b) = single_field_pair(
        field("qty", CanonicalType::Int32, false),
        field("qty", CanonicalType::Int64, false),
    );
    let result = compare(&a, &b, &CompareConfig::default());
    let conflicts = of_type(&result.conflicts, ConflictType::TypeIncompatible);
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].severity, ConflictSeverity::Low);

    let suggestions = SuggestionEngine::new().suggest(conflicts[0]);
    assert!(
        suggestions
            .iter()
            .any(|s| matches!(s.kind, SuggestionKind::CastSafe { .. })),
        "Expected CastSafe for a widening conversion"
    );
}

#[test]
fn acceptance_rf3_type_requires_cast_is_medium_severity_conflict() {
    // E1: DateTime -> Date loses the time part, reported with medium severity
    let (a, b) = single_field_pair(
        field("created", CanonicalType::DateTime, false),
        field("created", CanonicalType::Date, false),
    );
    let result = compare(&a, &b, &CompareConfig::default());
    let conflicts = of_type(&result.conflicts, ConflictType::TypeIncompatible);
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].severity, ConflictSeverity::Medium);
}

#[test]
fn acceptance_rf3_equivalent_types_are_not_conflicts() {
    // String/Text and DateTime/Timestamp are equivalent: no conflict
    for (ta, tb) in [
        (CanonicalType::String, CanonicalType::Text),
        (CanonicalType::DateTime, CanonicalType::Timestamp),
    ] {
        let (a, b) = single_field_pair(field("v", ta, true), field("v", tb, true));
        let result = compare(&a, &b, &CompareConfig::default());
        assert!(result.conflicts.is_empty(), "{:?}", result.conflicts);
    }
}

#[test]
fn acceptance_rf3_nullability_mismatch_in_both_directions() {
    // E2: nullable in A and NOT NULL in B is also a conflict
    let (a, b) = single_field_pair(
        field("email", CanonicalType::String, true),
        field("email", CanonicalType::String, false),
    );
    let result = compare(&a, &b, &CompareConfig::default());
    let conflicts = of_type(&result.conflicts, ConflictType::NullabilityMismatch);
    assert_eq!(conflicts.len(), 1);

    let suggestions = SuggestionEngine::new().suggest(conflicts[0]);
    assert!(suggestions
        .iter()
        .any(|s| !matches!(s.kind, SuggestionKind::NoSuggestion { .. })));
}

#[test]
fn acceptance_rf3_constraint_mismatch_from_unique_key() {
    // E3: a single-column unique key counts as UNIQUE, like a field constraint
    let ea = EntitySchema::builder()
        .entity_name("users")
        .add_field(field("email", CanonicalType::String, false))
        .add_key(Key::unique(vec!["email"]))
        .build();
    let eb = EntitySchema::builder()
        .entity_name("users")
        .add_field(field("email", CanonicalType::String, false))
        .build();
    let result = compare(
        &schema("db_a", ea),
        &schema("db_b", eb),
        &CompareConfig::default(),
    );
    assert_eq!(
        of_type(&result.conflicts, ConflictType::ConstraintMismatch).len(),
        1
    );
}

#[test]
fn acceptance_rf3_unique_key_and_field_constraint_are_equivalent() {
    // UNIQUE declared as a key in A and as a field constraint in B: no conflict
    let ea = EntitySchema::builder()
        .entity_name("users")
        .add_field(field("email", CanonicalType::String, false))
        .add_key(Key::unique(vec!["email"]))
        .build();
    let eb = EntitySchema::builder()
        .entity_name("users")
        .add_field(
            FieldSchema::builder()
                .field_name("email")
                .canonical_type(CanonicalType::String)
                .nullable(false)
                .add_constraint(Constraint::unique())
                .build(),
        )
        .build();
    let result = compare(
        &schema("db_a", ea),
        &schema("db_b", eb),
        &CompareConfig::default(),
    );
    assert!(result.conflicts.is_empty(), "{:?}", result.conflicts);
}

#[test]
fn acceptance_rf3_composite_unique_key_is_not_field_unique() {
    // A multi-column unique key does not make each column unique on its own
    let fields = || {
        vec![
            field("first", CanonicalType::String, false),
            field("last", CanonicalType::String, false),
        ]
    };
    let ea = EntitySchema::builder()
        .entity_name("people")
        .fields(fields())
        .add_key(Key::unique(vec!["first", "last"]))
        .build();
    let eb = EntitySchema::builder()
        .entity_name("people")
        .fields(fields())
        .build();
    let result = compare(
        &schema("db_a", ea),
        &schema("db_b", eb),
        &CompareConfig::default(),
    );
    assert!(of_type(&result.conflicts, ConflictType::ConstraintMismatch).is_empty());
}

/// Conflict types per (entity, field), independent of comparison direction.
fn conflict_signature(conflicts: &[Conflict]) -> BTreeMap<(String, String), Vec<String>> {
    let mut sig: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for c in conflicts {
        sig.entry((
            c.entity_name.to_lowercase(),
            c.field_name.clone().unwrap_or_default().to_lowercase(),
        ))
        .or_default()
        .push(format!("{:?}", c.conflict_type));
    }
    for types in sig.values_mut() {
        types.sort();
    }
    sig
}

#[test]
fn acceptance_rnf2_symmetry() {
    // E5: A -> B and B -> A must report the same conflict types per field
    let ea = EntitySchema::builder()
        .entity_name("products")
        .add_field(field("id", CanonicalType::Int32, false))
        .add_field(field("qty", CanonicalType::Int32, true))
        .add_field(field("created", CanonicalType::DateTime, false))
        .add_field(field("price", CanonicalType::Float64, false))
        .add_field(field("code", CanonicalType::String, false))
        .add_field(field("name", CanonicalType::String, true))
        .add_field(
            FieldSchema::builder()
                .field_name("sku")
                .canonical_type(CanonicalType::String)
                .nullable(false)
                .add_constraint(Constraint::max_length(20))
                .build(),
        )
        .add_key(Key::primary(vec!["id"]))
        .add_key(Key::unique(vec!["code"]))
        .build();
    let eb = EntitySchema::builder()
        .entity_name("products")
        .add_field(field("id", CanonicalType::Int64, false))
        .add_field(field("qty", CanonicalType::Int32, false))
        .add_field(field("created", CanonicalType::Date, false))
        .add_field(field(
            "price",
            CanonicalType::Decimal {
                precision: 10,
                scale: 2,
            },
            true,
        ))
        .add_field(field("code", CanonicalType::String, false))
        .add_field(field("name", CanonicalType::Text, true))
        .add_field(
            FieldSchema::builder()
                .field_name("sku")
                .canonical_type(CanonicalType::String)
                .nullable(false)
                .add_constraint(Constraint::max_length(40))
                .build(),
        )
        .add_key(Key::primary(vec!["id"]))
        .build();

    let a = schema("db_a", ea);
    let b = schema("db_b", eb);
    let config = CompareConfig::default();
    let ab = compare(&a, &b, &config);
    let ba = compare(&b, &a, &config);

    // Sanity check: every kind of difference above produced a conflict
    assert!(ab.conflicts.len() >= 7, "{:?}", ab.conflicts);
    assert_eq!(
        conflict_signature(&ab.conflicts),
        conflict_signature(&ba.conflicts)
    );
    assert_eq!(ab.summary.total_matches, ba.summary.total_matches);
    assert_eq!(ab.summary.exclusives_a, ba.summary.exclusives_b);
    assert_eq!(ab.summary.exclusives_b, ba.summary.exclusives_a);
}
