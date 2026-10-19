//! Acceptance tests for one-to-one matching of entities and fields
//!
//! Each element of A is matched to at most one element of B (and vice versa),
//! with priority exact > normalized > semantic > similarity.

use audd_compare::{compare, CompareConfig, ConflictType, ExclusiveSide};
use audd_ir::{CanonicalType, EntitySchema, FieldSchema, SourceSchema};
use std::collections::BTreeSet;

fn field(name: &str, nullable: bool) -> FieldSchema {
    FieldSchema::builder()
        .field_name(name)
        .canonical_type(CanonicalType::String)
        .nullable(nullable)
        .build()
}

fn entity(name: &str, fields: Vec<FieldSchema>) -> EntitySchema {
    let mut builder = EntitySchema::builder().entity_name(name);
    for f in fields {
        builder = builder.add_field(f);
    }
    builder.build()
}

fn schema(name: &str, entities: Vec<EntitySchema>) -> SourceSchema {
    let mut builder = SourceSchema::builder()
        .source_name(name)
        .source_type("sqlite");
    for e in entities {
        builder = builder.add_entity(e);
    }
    builder.build()
}

/// Schema with near-duplicate names, as in growventa (`Product` / `Product_Product`)
fn near_duplicates_schema() -> SourceSchema {
    schema(
        "growventa_like",
        vec![
            entity(
                "Product",
                vec![
                    field("id", false),
                    field("name", false),
                    field("price", true),
                ],
            ),
            entity(
                "Product_Product",
                vec![field("id", false), field("product_id", false)],
            ),
            entity(
                "Price_history",
                vec![
                    field("id", false),
                    field("to_date", true),
                    field("ToDate", false),
                ],
            ),
        ],
    )
}

/// Exclusives keyed by (side, entity, field), with the side relative to `a_first`
fn exclusive_keys(
    schema_a: &SourceSchema,
    schema_b: &SourceSchema,
    a_first: bool,
) -> BTreeSet<(char, String, Option<String>)> {
    let result = compare(schema_a, schema_b, &CompareConfig::default());
    result
        .exclusives
        .iter()
        .map(|e| {
            let side = match (&e.side, a_first) {
                (ExclusiveSide::A, true) | (ExclusiveSide::B, false) => 'A',
                _ => 'B',
            };
            (side, e.entity_name.clone(), e.field_name.clone())
        })
        .collect()
}

#[test]
fn test_identity_has_no_exclusives_or_conflicts() {
    let s = near_duplicates_schema();
    let result = compare(&s, &s, &CompareConfig::default());

    assert!(
        result.exclusives.is_empty(),
        "identity produced exclusives: {:?}",
        result.exclusives
    );
    assert!(
        result.conflicts.is_empty(),
        "identity produced conflicts: {:?}",
        result.conflicts
    );

    let entity_matches: Vec<_> = result
        .matches
        .iter()
        .filter(|m| m.field_name.is_none())
        .collect();
    assert_eq!(entity_matches.len(), 3);
    for m in entity_matches {
        assert_eq!(
            m.index_a, m.index_b,
            "entity {} paired off-diagonal",
            m.entity_name
        );
    }
}

#[test]
fn test_field_with_exact_name_matches_exactly_one_field() {
    let schema_a = schema(
        "a",
        vec![entity(
            "Price_history",
            vec![field("id", false), field("to_date", true)],
        )],
    );
    let schema_b = schema(
        "b",
        vec![entity(
            "Price_history",
            vec![
                field("id", false),
                field("to_date", true),
                field("ToDate", false),
            ],
        )],
    );

    let result = compare(&schema_a, &schema_b, &CompareConfig::default());

    let to_date_matches: Vec<_> = result
        .matches
        .iter()
        .filter(|m| m.field_name.is_some() && m.index_a == 1)
        .collect();
    assert_eq!(to_date_matches.len(), 1, "matches: {:?}", to_date_matches);
    assert_eq!(
        to_date_matches[0].index_b, 1,
        "to_date must pair with to_date"
    );

    // ToDate (non-nullable) stays exclusive instead of raising a false nullability conflict
    assert!(!result
        .conflicts
        .iter()
        .any(|c| c.conflict_type == ConflictType::NullabilityMismatch));
    assert!(result
        .exclusives
        .iter()
        .any(|e| e.side == ExclusiveSide::B && e.field_name.as_deref() == Some("ToDate")));
}

#[test]
fn test_each_element_matched_at_most_once() {
    let s = near_duplicates_schema();
    let result = compare(&s, &s, &CompareConfig::default());

    let mut seen_a = BTreeSet::new();
    let mut seen_b = BTreeSet::new();
    let mut entity_a = 0;
    let mut entity_b = 0;
    for m in &result.matches {
        if m.field_name.is_none() {
            entity_a = m.index_a;
            entity_b = m.index_b;
            assert!(
                seen_a.insert((None, m.index_a)),
                "entity A {} reused",
                m.index_a
            );
            assert!(
                seen_b.insert((None, m.index_b)),
                "entity B {} reused",
                m.index_b
            );
        } else {
            assert!(
                seen_a.insert((Some(entity_a), m.index_a)),
                "field A reused: {:?}",
                m
            );
            assert!(
                seen_b.insert((Some(entity_b), m.index_b)),
                "field B reused: {:?}",
                m
            );
        }
    }
}

#[test]
fn test_exclusives_are_symmetric() {
    let schema_a = schema(
        "a",
        vec![
            entity("Product", vec![field("id", false), field("name", false)]),
            entity(
                "Price_history",
                vec![field("id", false), field("to_date", true)],
            ),
        ],
    );
    let schema_b = near_duplicates_schema();

    let forward = exclusive_keys(&schema_a, &schema_b, true);
    let backward = exclusive_keys(&schema_b, &schema_a, false);

    assert_eq!(forward, backward);
    assert!(forward.contains(&('B', "Product_Product".to_string(), None)));
    assert!(forward.contains(&('B', "Price_history".to_string(), Some("ToDate".to_string()))));
}
