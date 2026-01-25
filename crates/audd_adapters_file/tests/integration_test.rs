//! Integration tests for file adapters

use audd_adapters_file::load_schema_from_file;
use std::path::PathBuf;

fn fixture_path(name: &str) -> PathBuf {
    // Tests run from workspace root
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("fixtures/adapters")
        .join(name)
}

#[test]
fn test_csv_fixture_deterministic() {
    let path = fixture_path("users.csv");
    let schema = load_schema_from_file(&path).expect("Failed to load CSV");

    assert_eq!(schema.source_name, "users");
    assert_eq!(schema.source_type, "csv");
    assert_eq!(schema.entities.len(), 1);

    let entity = &schema.entities[0];
    assert_eq!(entity.entity_name, "users");
    assert_eq!(entity.fields.len(), 5);

    // Verify deterministic field names
    let field_names: Vec<&str> = entity.fields.iter().map(|f| f.field_name.as_str()).collect();
    assert_eq!(field_names, vec!["id", "name", "email", "age", "active"]);
}

#[test]
fn test_json_fixture_deterministic() {
    let path = fixture_path("users.json");
    let schema = load_schema_from_file(&path).expect("Failed to load JSON");

    assert_eq!(schema.source_name, "users");
    assert_eq!(schema.source_type, "json");
    assert_eq!(schema.entities.len(), 1);

    let entity = &schema.entities[0];
    assert_eq!(entity.entity_name, "users");
    assert_eq!(entity.fields.len(), 6);

    // JSON can infer types
    use audd_ir::CanonicalType;
    let id_field = entity.fields.iter().find(|f| f.field_name == "id").unwrap();
    assert_eq!(id_field.canonical_type, CanonicalType::Int64);

    let active_field = entity
        .fields
        .iter()
        .find(|f| f.field_name == "active")
        .unwrap();
    assert_eq!(active_field.canonical_type, CanonicalType::Boolean);
}

#[test]
fn test_xml_fixture_deterministic() {
    let path = fixture_path("users.xml");
    let schema = load_schema_from_file(&path).expect("Failed to load XML");

    assert_eq!(schema.source_name, "users");
    assert_eq!(schema.source_type, "xml");
    assert_eq!(schema.entities.len(), 1);

    let entity = &schema.entities[0];
    assert_eq!(entity.entity_name, "users");
    assert_eq!(entity.fields.len(), 4);

    // XML fields should include id, name, email, age (order may vary)
    let field_names: Vec<&str> = entity.fields.iter().map(|f| f.field_name.as_str()).collect();
    assert!(field_names.contains(&"id"));
    assert!(field_names.contains(&"name"));
    assert!(field_names.contains(&"email"));
    assert!(field_names.contains(&"age"));
}

#[test]
fn test_sql_fixture_deterministic() {
    let path = fixture_path("schema.sql");
    let schema = load_schema_from_file(&path).expect("Failed to load SQL");

    assert_eq!(schema.source_name, "schema");
    assert_eq!(schema.source_type, "sql");
    assert_eq!(schema.entities.len(), 2);

    // First table: users
    let users = &schema.entities[0];
    assert_eq!(users.entity_name, "users");
    assert_eq!(users.fields.len(), 5);
    assert_eq!(users.keys.len(), 1);

    // Primary key on id
    let pk = &users.keys[0];
    assert_eq!(pk.field_names, vec!["id"]);

    // Second table: posts
    let posts = &schema.entities[1];
    assert_eq!(posts.entity_name, "posts");
    assert_eq!(posts.fields.len(), 5);
}

#[test]
fn test_unsupported_extension() {
    let result = load_schema_from_file("test.xyz");
    assert!(result.is_err());
}

#[test]
fn test_nonexistent_file() {
    let result = load_schema_from_file("nonexistent.csv");
    assert!(result.is_err());
}

#[test]
fn test_json_serialization_roundtrip() {
    let path = fixture_path("users.csv");
    let schema = load_schema_from_file(&path).expect("Failed to load CSV");

    // Serialize to JSON
    let json = schema.to_json().expect("Failed to serialize");

    // Deserialize back
    let deserialized =
        audd_ir::SourceSchema::from_json(&json).expect("Failed to deserialize");

    // Should be identical
    assert_eq!(schema, deserialized);
}

#[test]
fn test_all_formats_have_ir_version() {
    let fixtures = vec!["users.csv", "users.json", "users.xml", "schema.sql"];

    for fixture in fixtures {
        let path = fixture_path(fixture);
        let schema = load_schema_from_file(&path)
            .unwrap_or_else(|_| panic!("Failed to load {}", fixture));
        assert_eq!(schema.ir_version, "1.0.0");
    }
}
