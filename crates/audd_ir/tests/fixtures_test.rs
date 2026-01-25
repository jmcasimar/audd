//! Integration tests for IR fixtures

use audd_ir::{CanonicalType, SourceSchema};
use std::fs;

#[test]
fn test_load_simple_a_fixture() {
    let fixture_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ir/simple_a.json"
    );

    let json = fs::read_to_string(fixture_path).expect("Failed to read simple_a.json");

    let schema: SourceSchema = serde_json::from_str(&json).expect("Failed to parse simple_a.json");

    // Validate schema structure
    assert_eq!(schema.source_name, "users_db");
    assert_eq!(schema.source_type, "mysql");
    assert_eq!(schema.ir_version, "1.0.0");
    assert_eq!(schema.entities.len(), 1);

    // Validate entity
    let entity = &schema.entities[0];
    assert_eq!(entity.entity_name, "users");
    assert_eq!(entity.entity_type, "table");
    assert_eq!(entity.fields.len(), 5);
    assert_eq!(entity.keys.len(), 1);

    // Validate fields
    let id_field = entity.find_field("id").expect("id field not found");
    assert_eq!(id_field.canonical_type, CanonicalType::Int32);
    assert!(!id_field.nullable);

    let email_field = entity.find_field("email").expect("email field not found");
    assert_eq!(email_field.canonical_type, CanonicalType::String);
    assert!(!email_field.nullable);
    assert_eq!(email_field.constraints.len(), 2);

    let is_active_field = entity
        .find_field("is_active")
        .expect("is_active field not found");
    assert_eq!(is_active_field.canonical_type, CanonicalType::Boolean);
}

#[test]
fn test_load_simple_b_fixture() {
    let fixture_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ir/simple_b.json"
    );

    let json = fs::read_to_string(fixture_path).expect("Failed to read simple_b.json");

    let schema: SourceSchema = serde_json::from_str(&json).expect("Failed to parse simple_b.json");

    // Validate schema structure
    assert_eq!(schema.source_name, "customers_db");
    assert_eq!(schema.source_type, "postgresql");
    assert_eq!(schema.ir_version, "1.0.0");
    assert_eq!(schema.entities.len(), 1);

    // Validate entity
    let entity = &schema.entities[0];
    assert_eq!(entity.entity_name, "users");
    assert_eq!(entity.entity_type, "table");
    assert_eq!(entity.fields.len(), 6);

    // Validate PostgreSQL-specific fields
    let id_field = entity.find_field("id").expect("id field not found");
    assert_eq!(id_field.canonical_type, CanonicalType::Int64);

    let uuid_field = entity
        .find_field("user_uuid")
        .expect("user_uuid field not found");
    assert_eq!(uuid_field.canonical_type, CanonicalType::Uuid);
}

#[test]
fn test_fixture_roundtrip() {
    let fixture_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ir/simple_a.json"
    );

    let json = fs::read_to_string(fixture_path).expect("Failed to read simple_a.json");

    let schema: SourceSchema = serde_json::from_str(&json).expect("Failed to parse simple_a.json");

    // Serialize back to JSON
    let serialized = schema.to_json().expect("Failed to serialize schema");

    // Deserialize again
    let roundtrip_schema: SourceSchema =
        serde_json::from_str(&serialized).expect("Failed to parse roundtrip JSON");

    // Verify they're equal
    assert_eq!(schema, roundtrip_schema);
}

#[test]
fn test_compare_fixtures() {
    // Load both fixtures
    let fixture_a_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ir/simple_a.json"
    );
    let fixture_b_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ir/simple_b.json"
    );

    let json_a = fs::read_to_string(fixture_a_path).expect("Failed to read simple_a.json");
    let json_b = fs::read_to_string(fixture_b_path).expect("Failed to read simple_b.json");

    let schema_a: SourceSchema =
        serde_json::from_str(&json_a).expect("Failed to parse simple_a.json");
    let schema_b: SourceSchema =
        serde_json::from_str(&json_b).expect("Failed to parse simple_b.json");

    // Both should have a "users" entity
    let entity_a = schema_a
        .find_entity("users")
        .expect("users entity not found in schema_a");
    let entity_b = schema_b
        .find_entity("users")
        .expect("users entity not found in schema_b");

    // Both should have common fields
    assert!(entity_a.find_field("id").is_some());
    assert!(entity_b.find_field("id").is_some());

    assert!(entity_a.find_field("email").is_some());
    assert!(entity_b.find_field("email").is_some());

    assert!(entity_a.find_field("is_active").is_some());
    assert!(entity_b.find_field("is_active").is_some());

    // Schema B has an additional UUID field
    assert!(entity_a.find_field("user_uuid").is_none());
    assert!(entity_b.find_field("user_uuid").is_some());
}
