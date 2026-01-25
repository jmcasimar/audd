//! Integration tests for the comparison engine

use audd_compare::{compare, CompareConfig};
use audd_ir::{CanonicalType, Constraint, EntitySchema, FieldSchema, SourceSchema};

/// Helper to create a field
fn field(name: &str, typ: CanonicalType, nullable: bool) -> FieldSchema {
    FieldSchema::builder()
        .field_name(name)
        .canonical_type(typ)
        .nullable(nullable)
        .build()
}

/// Helper to create a field with constraints
fn field_with_constraints(
    name: &str,
    typ: CanonicalType,
    nullable: bool,
    constraints: Vec<Constraint>,
) -> FieldSchema {
    FieldSchema::builder()
        .field_name(name)
        .canonical_type(typ)
        .nullable(nullable)
        .constraints(constraints)
        .build()
}

#[test]
fn test_case_basic_match() {
    // Create schema A with users table
    let schema_a = SourceSchema::builder()
        .source_name("db_a")
        .source_type("mysql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("email", CanonicalType::String, false))
                .add_field(field("name", CanonicalType::String, true))
                .add_field(field("created_at", CanonicalType::Timestamp, false))
                .add_field(field("is_active", CanonicalType::Boolean, false))
                .build(),
        )
        .build();

    // Create schema B with users table (same structure)
    let schema_b = SourceSchema::builder()
        .source_name("db_b")
        .source_type("postgresql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("email", CanonicalType::String, false))
                .add_field(field("name", CanonicalType::String, true))
                .add_field(field("created_at", CanonicalType::Timestamp, false))
                .add_field(field("is_active", CanonicalType::Boolean, false))
                .build(),
        )
        .build();

    let config = CompareConfig::default();
    let result = compare(&schema_a, &schema_b, &config);

    // Should match 1 entity + 5 fields
    assert_eq!(result.summary.total_matches, 6);
    assert_eq!(result.summary.total_exclusives, 0);
    assert_eq!(result.summary.total_conflicts, 0);
}

#[test]
fn test_case_add_safe() {
    // Schema A has users with basic fields
    let schema_a = SourceSchema::builder()
        .source_name("db_a")
        .source_type("mysql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("email", CanonicalType::String, false))
                .build(),
        )
        .add_entity(
            EntitySchema::builder()
                .entity_name("posts")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("title", CanonicalType::String, false))
                .build(),
        )
        .build();

    // Schema B has users with additional fields and a comments table
    let schema_b = SourceSchema::builder()
        .source_name("db_b")
        .source_type("postgresql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("email", CanonicalType::String, false))
                .add_field(field("phone", CanonicalType::String, true)) // Exclusive to B
                .add_field(field("avatar_url", CanonicalType::String, true)) // Exclusive to B
                .build(),
        )
        .add_entity(
            EntitySchema::builder()
                .entity_name("comments") // Exclusive entity to B
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("content", CanonicalType::Text, false))
                .build(),
        )
        .build();

    let config = CompareConfig::default();
    let result = compare(&schema_a, &schema_b, &config);

    // Matches: users entity + id field + email field, posts entity
    assert_eq!(result.summary.total_matches, 3);

    // Exclusives: phone, avatar_url from B in users, posts exclusive to A, comments exclusive to B
    assert_eq!(result.summary.total_exclusives, 4);
    assert_eq!(result.summary.exclusives_a, 1); // posts entity
    assert_eq!(result.summary.exclusives_b, 3); // phone, avatar_url, comments
}

#[test]
fn test_case_conflict_types() {
    // Schema A with specific types
    let schema_a = SourceSchema::builder()
        .source_name("db_a")
        .source_type("mysql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("products")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("price", CanonicalType::Float64, false))
                .add_field(field("name", CanonicalType::String, false))
                .add_field(field("stock", CanonicalType::Int32, false))
                .add_field(field("is_available", CanonicalType::Boolean, false))
                .build(),
        )
        .build();

    // Schema B with incompatible types for some fields
    let schema_b = SourceSchema::builder()
        .source_name("db_b")
        .source_type("postgresql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("products")
                .add_field(field("id", CanonicalType::String, false)) // Incompatible!
                .add_field(field("price", CanonicalType::String, false)) // Incompatible!
                .add_field(field("name", CanonicalType::Text, false)) // Compatible (String<->Text)
                .add_field(field("stock", CanonicalType::Int64, false)) // Compatible (widening)
                .add_field(field("is_available", CanonicalType::String, false)) // Incompatible!
                .build(),
        )
        .build();

    let config = CompareConfig::default();
    let result = compare(&schema_a, &schema_b, &config);

    // Should match entity + all 5 fields by name
    assert_eq!(result.summary.total_matches, 6);

    // Should have 3 type conflicts: id, price, is_available
    assert_eq!(result.summary.total_conflicts, 3);

    // Verify the conflicts are type conflicts
    for conflict in &result.conflicts {
        use audd_compare::ConflictType;
        assert_eq!(conflict.conflict_type, ConflictType::TypeIncompatible);
    }
}

#[test]
fn test_case_nullability_conflicts() {
    let schema_a = SourceSchema::builder()
        .source_name("db_a")
        .source_type("mysql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("email", CanonicalType::String, false)) // NOT NULL in A
                .add_field(field("name", CanonicalType::String, false)) // NOT NULL in A
                .build(),
        )
        .build();

    let schema_b = SourceSchema::builder()
        .source_name("db_b")
        .source_type("postgresql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("email", CanonicalType::String, true)) // NULLABLE in B
                .add_field(field("name", CanonicalType::String, true)) // NULLABLE in B
                .build(),
        )
        .build();

    let config = CompareConfig::default();
    let result = compare(&schema_a, &schema_b, &config);

    // Should have 2 nullability conflicts (email, name)
    let nullability_conflicts: Vec<_> = result
        .conflicts
        .iter()
        .filter(|c| {
            use audd_compare::ConflictType;
            c.conflict_type == ConflictType::NullabilityMismatch
        })
        .collect();

    assert_eq!(nullability_conflicts.len(), 2);
}

#[test]
fn test_case_constraint_conflicts() {
    let schema_a = SourceSchema::builder()
        .source_name("db_a")
        .source_type("mysql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field_with_constraints(
                    "email",
                    CanonicalType::String,
                    false,
                    vec![Constraint::unique(), Constraint::max_length(255)],
                ))
                .add_field(field_with_constraints(
                    "username",
                    CanonicalType::String,
                    false,
                    vec![Constraint::unique()],
                ))
                .build(),
        )
        .build();

    let schema_b = SourceSchema::builder()
        .source_name("db_b")
        .source_type("postgresql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field_with_constraints(
                    "email",
                    CanonicalType::String,
                    false,
                    vec![Constraint::max_length(100)], // Different length!
                ))
                .add_field(field("username", CanonicalType::String, false)) // No unique constraint!
                .build(),
        )
        .build();

    let config = CompareConfig::default();
    let result = compare(&schema_a, &schema_b, &config);

    // Should have constraint conflicts
    let constraint_conflicts: Vec<_> = result
        .conflicts
        .iter()
        .filter(|c| {
            use audd_compare::ConflictType;
            c.conflict_type == ConflictType::ConstraintMismatch
        })
        .collect();

    // Should have at least 2 conflicts: unique on email, max_length on email, unique on username
    assert!(constraint_conflicts.len() >= 2);
}

#[test]
fn test_case_collision_normalization() {
    // This tests the case where two UNMATCHED fields from different schemas normalize to the same identifier
    // creating ambiguity about which should be merged
    let schema_a = SourceSchema::builder()
        .source_name("db_a")
        .source_type("mysql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("FirstName", CanonicalType::String, false)) // Normalizes to first_name
                .build(),
        )
        .build();

    let schema_b = SourceSchema::builder()
        .source_name("db_b")
        .source_type("postgresql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("first_name", CanonicalType::Int32, false)) // Also normalizes to first_name BUT different type!
                .build(),
        )
        .build();

    let config = CompareConfig::default();
    let result = compare(&schema_a, &schema_b, &config);

    // These should match by normalized name, creating a type conflict
    // NOT a normalization collision (since they successfully matched)
    assert_eq!(result.summary.total_matches, 3); // entity + id + firstName/first_name
    
    // Should have a type conflict
    let type_conflicts: Vec<_> = result
        .conflicts
        .iter()
        .filter(|c| {
            use audd_compare::ConflictType;
            c.conflict_type == ConflictType::TypeIncompatible
        })
        .collect();
    assert_eq!(type_conflicts.len(), 1);
    
    // No normalization collisions (they matched)
    let collision_conflicts: Vec<_> = result
        .conflicts
        .iter()
        .filter(|c| {
            use audd_compare::ConflictType;
            c.conflict_type == ConflictType::NormalizationCollision
        })
        .collect();
    assert_eq!(collision_conflicts.len(), 0);
}

#[test]
fn test_normalized_matching() {
    // Test that normalized matching works correctly
    let schema_a = SourceSchema::builder()
        .source_name("db_a")
        .source_type("mysql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("UserTable")
                .add_field(field("userId", CanonicalType::Int32, false))
                .add_field(field("firstName", CanonicalType::String, false))
                .add_field(field("lastName", CanonicalType::String, false))
                .build(),
        )
        .build();

    let schema_b = SourceSchema::builder()
        .source_name("db_b")
        .source_type("postgresql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("user_table")
                .add_field(field("user_id", CanonicalType::Int32, false))
                .add_field(field("first_name", CanonicalType::String, false))
                .add_field(field("last_name", CanonicalType::String, false))
                .build(),
        )
        .build();

    let config = CompareConfig::default();
    let result = compare(&schema_a, &schema_b, &config);

    // Should match via normalization: 1 entity + 3 fields
    assert_eq!(result.summary.total_matches, 4);
    assert_eq!(result.summary.total_conflicts, 0);
    assert_eq!(result.summary.total_exclusives, 0);
}

#[test]
fn test_similarity_matching() {
    let schema_a = SourceSchema::builder()
        .source_name("db_a")
        .source_type("mysql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("email", CanonicalType::String, false))
                .add_field(field("telephone", CanonicalType::String, true))
                .build(),
        )
        .build();

    let schema_b = SourceSchema::builder()
        .source_name("db_b")
        .source_type("postgresql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("user") // Similar to "users"
                .add_field(field("e_mail", CanonicalType::String, false)) // Similar to "email"
                .add_field(field("phone", CanonicalType::String, true)) // Not similar to "telephone"
                .build(),
        )
        .build();

    let config = CompareConfig::all_features().with_similarity_threshold(0.7);
    let result = compare(&schema_a, &schema_b, &config);

    // With similarity matching, should find matches
    // Note: Results may vary based on similarity algorithm
    assert!(result.summary.total_matches > 0);
}

#[test]
fn test_comprehensive_scenario() {
    // A comprehensive test with matches, exclusives, and conflicts
    let schema_a = SourceSchema::builder()
        .source_name("legacy_db")
        .source_type("mysql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("email", CanonicalType::String, false))
                .add_field(field("password_hash", CanonicalType::String, false)) // Exclusive to A
                .add_field(field("created", CanonicalType::DateTime, false))
                .build(),
        )
        .add_entity(
            EntitySchema::builder()
                .entity_name("sessions") // Exclusive entity to A
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("token", CanonicalType::String, false))
                .build(),
        )
        .build();

    let schema_b = SourceSchema::builder()
        .source_name("new_db")
        .source_type("postgresql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int64, false)) // Different type (but compatible)
                .add_field(field("email", CanonicalType::Text, false)) // Different type (but compatible)
                .add_field(field("uuid", CanonicalType::Uuid, false)) // Exclusive to B
                .add_field(field("created", CanonicalType::Timestamp, false)) // Compatible
                .build(),
        )
        .add_entity(
            EntitySchema::builder()
                .entity_name("profiles") // Exclusive entity to B
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("bio", CanonicalType::Text, true))
                .build(),
        )
        .build();

    let config = CompareConfig::default();
    let result = compare(&schema_a, &schema_b, &config);

    // Should have matches for users entity and some fields
    assert!(result.summary.total_matches > 0);

    // Should have exclusives from both sides
    assert!(result.summary.exclusives_a > 0);
    assert!(result.summary.exclusives_b > 0);

    // May or may not have conflicts depending on type compatibility rules
    println!("Matches: {}", result.summary.total_matches);
    println!("Exclusives A: {}", result.summary.exclusives_a);
    println!("Exclusives B: {}", result.summary.exclusives_b);
    println!("Conflicts: {}", result.summary.total_conflicts);
}

#[test]
fn test_unified_schema_generation() {
    use audd_compare::UnifiedSchema;

    let schema_a = SourceSchema::builder()
        .source_name("db_a")
        .source_type("mysql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("email", CanonicalType::String, false))
                .add_field(field("password", CanonicalType::String, false))
                .build(),
        )
        .build();

    let schema_b = SourceSchema::builder()
        .source_name("db_b")
        .source_type("postgresql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(field("id", CanonicalType::Int32, false))
                .add_field(field("email", CanonicalType::String, false))
                .add_field(field("phone", CanonicalType::String, true))
                .build(),
        )
        .build();

    let config = CompareConfig::default();
    let result = compare(&schema_a, &schema_b, &config);

    let unified = UnifiedSchema::from_comparison(&schema_a, &schema_b, &result);

    // Should have one entity (users)
    assert_eq!(unified.entities.len(), 1);

    // Should have all fields (id, email matched; password from A, phone from B)
    assert_eq!(unified.entities[0].fields.len(), 4);

    // Verify JSON serialization works
    let json = unified.to_json().expect("Failed to serialize unified schema");
    assert!(json.contains("users"));
}
