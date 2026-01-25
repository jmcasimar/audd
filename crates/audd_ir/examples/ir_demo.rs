//! Example demonstrating the AUDD IR (Intermediate Representation)
//!
//! This example shows how to:
//! 1. Build schemas using the builder pattern
//! 2. Normalize identifiers
//! 3. Map database types to canonical types
//! 4. Serialize/deserialize schemas to/from JSON

use audd_ir::{
    normalize_identifier, map_type_to_canonical,
    SourceSchema, EntitySchema, FieldSchema,
    CanonicalType, Constraint, Key,
};

fn main() {
    println!("=== AUDD IR Example ===\n");

    // Example 1: Identifier Normalization
    println!("1. Identifier Normalization:");
    let examples = vec![
        ("UserEmail", normalize_identifier("UserEmail")),
        ("Product Name", normalize_identifier("Product Name")),
        ("firstName", normalize_identifier("firstName")),
        ("Customer ID", normalize_identifier("Customer ID")),
    ];
    for (original, normalized) in examples {
        println!("  '{}' -> '{}'", original, normalized);
    }

    // Example 2: Type Mapping
    println!("\n2. Type Mapping:");
    let type_examples = vec![
        ("mysql", "VARCHAR(255)"),
        ("mysql", "INT"),
        ("mysql", "DECIMAL(10,2)"),
        ("postgresql", "BIGINT"),
        ("postgresql", "UUID"),
        ("sqlite", "INTEGER"),
    ];
    for (db, type_name) in type_examples {
        let canonical = map_type_to_canonical(db, type_name);
        println!("  {} {} -> {}", db, type_name, canonical.type_name());
    }

    // Example 3: Building a schema
    println!("\n3. Building a Schema:");
    
    // Create fields
    let id_field = FieldSchema::builder()
        .field_name("id")
        .canonical_type(CanonicalType::Int32)
        .nullable(false)
        .build();

    let email_field = FieldSchema::builder()
        .field_name("email")
        .canonical_type(CanonicalType::String)
        .nullable(false)
        .add_constraint(Constraint::max_length(255))
        .add_constraint(Constraint::unique())
        .build();

    let created_at_field = FieldSchema::builder()
        .field_name("created_at")
        .canonical_type(CanonicalType::Timestamp)
        .nullable(false)
        .build();

    // Create entity
    let users_entity = EntitySchema::builder()
        .entity_name("users")
        .entity_type("table")
        .add_field(id_field)
        .add_field(email_field)
        .add_field(created_at_field)
        .add_key(Key::primary(vec!["id"]))
        .build();

    // Create source
    let source = SourceSchema::builder()
        .source_name("my_database")
        .source_type("mysql")
        .add_entity(users_entity)
        .build();

    println!("  Source: {}", source.source_name);
    println!("  Type: {}", source.source_type);
    println!("  IR Version: {}", source.ir_version);
    println!("  Entities: {}", source.entities.len());

    if let Some(entity) = source.find_entity("users") {
        println!("\n  Entity: {}", entity.entity_name);
        println!("    Fields:");
        for field in &entity.fields {
            println!("      - {}: {} (nullable: {})",
                field.field_name,
                field.canonical_type.type_name(),
                field.nullable
            );
        }
        println!("    Keys: {}", entity.keys.len());
    }

    // Example 4: JSON Serialization
    println!("\n4. JSON Serialization:");
    match source.to_json() {
        Ok(json) => {
            println!("  Serialized to JSON ({} bytes)", json.len());
            
            // Example 5: JSON Deserialization
            println!("\n5. JSON Deserialization:");
            match SourceSchema::from_json(&json) {
                Ok(loaded) => {
                    println!("  Loaded from JSON successfully");
                    println!("  Roundtrip verification: {}",
                        if source == loaded { "✓ PASSED" } else { "✗ FAILED" }
                    );
                }
                Err(e) => println!("  Error: {}", e),
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // Example 6: Type Compatibility
    println!("\n6. Type Compatibility:");
    let compatibility_examples = vec![
        (CanonicalType::Int32, CanonicalType::Int32, true),
        (CanonicalType::Int32, CanonicalType::Int64, true),
        (CanonicalType::Int64, CanonicalType::Int32, false),
        (CanonicalType::String, CanonicalType::Text, true),
        (CanonicalType::DateTime, CanonicalType::Timestamp, true),
    ];
    
    for (t1, t2, expected) in compatibility_examples {
        let compatible = t1.is_compatible_with(&t2);
        let symbol = if compatible == expected { "✓" } else { "✗" };
        println!("  {} {} ↔ {}: {}",
            symbol,
            t1.type_name(),
            t2.type_name(),
            if compatible { "compatible" } else { "incompatible" }
        );
    }

    println!("\n=== Example completed successfully! ===");
}
