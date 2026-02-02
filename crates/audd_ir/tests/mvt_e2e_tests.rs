//! End-to-End MVT Tests
//!
//! This module contains comprehensive end-to-end tests for the AUDD MVT (Minimum Verification Tests).
//! These tests validate complete workflows from data loading through comparison to resolution.

use audd_ir::SourceSchema;
use std::fs;
use std::path::PathBuf;

/// Helper function to get test fixture path
fn fixture_path(relative_path: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../tests/fixtures");
    path.push(relative_path);
    path
}

mod ir_tests {
    use super::*;
    use audd_ir::{CanonicalType, EntitySchema, FieldSchema};

    /// IR-001: Build IR from minimal schema
    #[test]
    fn test_ir_001_build_from_minimal_schema() {
        // Create minimal schema: single entity with 2 fields
        let field1 = FieldSchema::builder()
            .field_name("id")
            .canonical_type(CanonicalType::Int32)
            .nullable(false)
            .build();

        let field2 = FieldSchema::builder()
            .field_name("name")
            .canonical_type(CanonicalType::String)
            .nullable(true)
            .build();

        let entity = EntitySchema::builder()
            .entity_name("users")
            .entity_type("table")
            .add_field(field1)
            .add_field(field2)
            .build();

        let schema = SourceSchema::builder()
            .source_name("minimal_db")
            .source_type("test")
            .add_entity(entity)
            .build();

        // Validate
        assert_eq!(schema.entities.len(), 1);
        assert_eq!(schema.entities[0].fields.len(), 2);
        assert_eq!(schema.source_name, "minimal_db");
    }

    /// IR-002: Build IR from complex schema with relationships
    #[test]
    fn test_ir_002_build_complex_schema_with_relationships() {
        use audd_ir::{Key, KeyType};

        // Create users entity
        let users = EntitySchema::builder()
            .entity_name("users")
            .entity_type("table")
            .add_field(
                FieldSchema::builder()
                    .field_name("id")
                    .canonical_type(CanonicalType::Int32)
                    .nullable(false)
                    .build(),
            )
            .add_field(
                FieldSchema::builder()
                    .field_name("username")
                    .canonical_type(CanonicalType::String)
                    .nullable(false)
                    .build(),
            )
            .add_key(Key::primary(vec!["id"]))
            .build();

        // Create posts entity with foreign key
        let posts = EntitySchema::builder()
            .entity_name("posts")
            .entity_type("table")
            .add_field(
                FieldSchema::builder()
                    .field_name("id")
                    .canonical_type(CanonicalType::Int32)
                    .nullable(false)
                    .build(),
            )
            .add_field(
                FieldSchema::builder()
                    .field_name("user_id")
                    .canonical_type(CanonicalType::Int32)
                    .nullable(false)
                    .build(),
            )
            .add_field(
                FieldSchema::builder()
                    .field_name("title")
                    .canonical_type(CanonicalType::String)
                    .nullable(false)
                    .build(),
            )
            .add_key(Key::primary(vec!["id"]))
            .add_key(
                Key::foreign(vec!["user_id"])
                    .with_metadata(
                        "referenced_entity".to_string(),
                        serde_json::json!("users"),
                    )
                    .with_metadata(
                        "referenced_fields".to_string(),
                        serde_json::json!(["id"]),
                    ),
            )
            .build();

        let schema = SourceSchema::builder()
            .source_name("complex_db")
            .source_type("test")
            .add_entity(users)
            .add_entity(posts)
            .build();

        // Validate
        assert_eq!(schema.entities.len(), 2);

        let posts_entity = schema
            .entities
            .iter()
            .find(|e| e.entity_name == "posts")
            .expect("posts entity not found");

        let foreign_keys: Vec<_> = posts_entity
            .keys
            .iter()
            .filter(|k| k.key_type == KeyType::Foreign)
            .collect();

        assert_eq!(foreign_keys.len(), 1);
        // Check metadata for referenced entity
        assert!(foreign_keys[0].metadata.contains_key("referenced_entity"));
    }

    /// IR-020: No entity loss invariant
    #[test]
    fn test_ir_020_no_entity_loss() {
        let entities_count = 3;
        let mut builder = SourceSchema::builder()
            .source_name("test")
            .source_type("test");

        // Add multiple entities
        for i in 0..entities_count {
            let entity = EntitySchema::builder()
                .entity_name(format!("entity_{}", i))
                .entity_type("table")
                .add_field(
                    FieldSchema::builder()
                        .field_name("id")
                        .canonical_type(CanonicalType::Int32)
                        .nullable(false)
                        .build(),
                )
                .build();
            builder = builder.add_entity(entity);
        }

        let schema = builder.build();

        // Validate: no entities lost
        assert_eq!(schema.entities.len(), entities_count);
    }

    /// IR-021: No field loss invariant
    #[test]
    fn test_ir_021_no_field_loss() {
        let fields_count = 5;
        let mut entity_builder = EntitySchema::builder()
            .entity_name("test_entity")
            .entity_type("table");

        // Add multiple fields
        for i in 0..fields_count {
            let field = FieldSchema::builder()
                .field_name(format!("field_{}", i))
                .canonical_type(CanonicalType::String)
                .nullable(true)
                .build();
            entity_builder = entity_builder.add_field(field);
        }

        let entity = entity_builder.build();

        // Validate: no fields lost
        assert_eq!(entity.fields.len(), fields_count);
    }

    /// IR-022: ID consistency and uniqueness
    #[test]
    fn test_ir_022_id_consistency() {
        // Build schema twice with same data
        let build_schema = || {
            SourceSchema::builder()
                .source_name("test")
                .source_type("test")
                .add_entity(
                    EntitySchema::builder()
                        .entity_name("users")
                        .entity_type("table")
                        .add_field(
                            FieldSchema::builder()
                                .field_name("id")
                                .canonical_type(CanonicalType::Int32)
                                .nullable(false)
                                .build(),
                        )
                        .build(),
                )
                .build()
        };

        let schema1 = build_schema();
        let schema2 = build_schema();

        // IDs should be stable (same input = same IDs)
        assert_eq!(schema1.entities[0].entity_name, schema2.entities[0].entity_name);

        // Serialize and compare for determinism
        let json1 = schema1.to_json().unwrap();
        let json2 = schema2.to_json().unwrap();
        assert_eq!(json1, json2);
    }

    /// IR-005: Normalize identifier variants
    #[test]
    fn test_ir_005_normalize_identifiers() {
        use audd_ir::normalize_identifier;

        // Test various naming conventions
        let variations = vec![
            ("User", "user"),
            ("user", "user"),
            ("USERS", "users"),
            ("user_profile", "user_profile"),
            ("UserProfile", "user_profile"),  // CamelCase -> snake_case
            ("firstName", "first_name"),
        ];

        for (input, expected) in variations {
            let normalized = normalize_identifier(input);
            assert_eq!(
                normalized,
                expected,
                "Failed to normalize '{}'",
                input
            );
        }
    }

    /// IR-010: Map SQL types to canonical
    #[test]
    fn test_ir_010_map_sql_types_to_canonical() {
        use audd_ir::map_type_to_canonical;

        let type_mappings = vec![
            ("sqlite", "INTEGER", CanonicalType::Int64),  // SQLite uses 64-bit
            ("sqlite", "REAL", CanonicalType::Float64),
            ("sqlite", "TEXT", CanonicalType::Text),
            ("sqlite", "BOOLEAN", CanonicalType::Boolean),
            ("mysql", "INT", CanonicalType::Int32),
            ("mysql", "BIGINT", CanonicalType::Int64),
            ("mysql", "VARCHAR", CanonicalType::String),
            ("mysql", "TEXT", CanonicalType::Text),
            ("postgresql", "INTEGER", CanonicalType::Int32),
            ("postgresql", "BIGINT", CanonicalType::Int64),
        ];

        for (source_type, sql_type, expected_canonical) in type_mappings {
            let result = map_type_to_canonical(source_type, sql_type);
            assert_eq!(
                result, expected_canonical,
                "Failed to map type '{}' from {}",
                sql_type,
                source_type
            );
        }
    }

    /// IR-013: Type compatibility check
    #[test]
    fn test_ir_013_type_compatibility() {
        use audd_ir::CanonicalType;

        // Compatible types (smaller to larger)
        assert!(CanonicalType::Int32.is_compatible_with(&CanonicalType::Int64));
        assert!(CanonicalType::Float32.is_compatible_with(&CanonicalType::Float64));

        // Same type is always compatible
        assert!(CanonicalType::String.is_compatible_with(&CanonicalType::String));
        assert!(CanonicalType::Int32.is_compatible_with(&CanonicalType::Int32));
    }

    /// IR-014: Type incompatibility check
    #[test]
    fn test_ir_014_type_incompatibility() {
        use audd_ir::CanonicalType;

        // Incompatible types
        assert!(!CanonicalType::String.is_compatible_with(&CanonicalType::Boolean));
        assert!(!CanonicalType::Int32.is_compatible_with(&CanonicalType::String));
        assert!(!CanonicalType::Boolean.is_compatible_with(&CanonicalType::Float64));
    }
}

mod fixture_roundtrip_tests {
    use super::*;

    /// Test that IR fixtures can be loaded and round-tripped
    /// Note: These tests require fixtures/ir/simple_a.json and simple_b.json to exist
    #[test]
    #[ignore] // Ignore until fixtures are in place
    fn test_roundtrip_simple_a() {
        let path = fixture_path("ir/simple_a.json");
        if !path.exists() {
            eprintln!("Skipping test: fixture not found at {:?}", path);
            return;
        }

        let json = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read {:?}", path));

        let schema: SourceSchema = serde_json::from_str(&json)
            .unwrap_or_else(|e| panic!("Failed to parse {:?}: {}", path, e));

        // Round-trip
        let serialized = schema.to_json().expect("Failed to serialize");
        let roundtrip: SourceSchema = serde_json::from_str(&serialized)
            .expect("Failed to parse round-tripped JSON");

        assert_eq!(schema, roundtrip);
    }

    /// Test that IR fixtures can be loaded and round-tripped
    #[test]
    #[ignore] // Ignore until fixtures are in place
    fn test_roundtrip_simple_b() {
        let path = fixture_path("ir/simple_b.json");
        if !path.exists() {
            eprintln!("Skipping test: fixture not found at {:?}", path);
            return;
        }

        let json = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read {:?}", path));

        let schema: SourceSchema = serde_json::from_str(&json)
            .unwrap_or_else(|e| panic!("Failed to parse {:?}: {}", path, e));

        // Round-trip
        let serialized = schema.to_json().expect("Failed to serialize");
        let roundtrip: SourceSchema = serde_json::from_str(&serialized)
            .expect("Failed to parse round-tripped JSON");

        assert_eq!(schema, roundtrip);
    }
}

mod determinism_tests {
    use super::*;

    /// Test that schema serialization is deterministic
    #[test]
    fn test_serialization_is_deterministic() {
        use audd_ir::{CanonicalType, EntitySchema, FieldSchema};

        let build_schema = || {
            SourceSchema::builder()
                .source_name("test")
                .source_type("test")
                .add_entity(
                    EntitySchema::builder()
                        .entity_name("users")
                        .entity_type("table")
                        .add_field(
                            FieldSchema::builder()
                                .field_name("id")
                                .canonical_type(CanonicalType::Int32)
                                .nullable(false)
                                .build(),
                        )
                        .add_field(
                            FieldSchema::builder()
                                .field_name("name")
                                .canonical_type(CanonicalType::String)
                                .nullable(true)
                                .build(),
                        )
                        .build(),
                )
                .build()
        };

        // Build and serialize 3 times
        let json1 = build_schema().to_json().unwrap();
        let json2 = build_schema().to_json().unwrap();
        let json3 = build_schema().to_json().unwrap();

        // All should be identical
        assert_eq!(json1, json2);
        assert_eq!(json2, json3);
    }
}
