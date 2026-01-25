//! Example demonstrating the AUDD comparison engine
//!
//! This example shows how to compare two schemas and analyze the results.

use audd_compare::{compare, CompareConfig};
use audd_ir::{CanonicalType, Constraint, EntitySchema, FieldSchema, SourceSchema};

fn main() {
    println!("=== AUDD Comparison Engine Demo ===\n");

    // Create Schema A (Legacy MySQL database)
    let schema_a = create_schema_a();
    println!("Schema A (MySQL - legacy_db):");
    print_schema_summary(&schema_a);

    // Create Schema B (New PostgreSQL database)
    let schema_b = create_schema_b();
    println!("\nSchema B (PostgreSQL - new_db):");
    print_schema_summary(&schema_b);

    // Compare the schemas
    println!("\n=== Running Comparison ===\n");
    let config = CompareConfig::default();
    let result = compare(&schema_a, &schema_b, &config);

    // Display results
    println!("Comparison Summary:");
    println!("  Total Matches:    {}", result.summary.total_matches);
    println!("  Total Exclusives: {}", result.summary.total_exclusives);
    println!("    - From A:       {}", result.summary.exclusives_a);
    println!("    - From B:       {}", result.summary.exclusives_b);
    println!("  Total Conflicts:  {}", result.summary.total_conflicts);

    // Show details
    println!("\n=== Matches ===");
    for m in &result.matches {
        match &m.field_name {
            Some(field) => println!(
                "  ✓ {}.{} (score: {:.2}, reason: {:?})",
                m.entity_name, field, m.score, m.reason
            ),
            None => println!("  ✓ {} (entity) (score: {:.2})", m.entity_name, m.score),
        }
    }

    println!("\n=== Exclusives ===");
    for e in &result.exclusives {
        let side = match e.side {
            audd_compare::ExclusiveSide::A => "A",
            audd_compare::ExclusiveSide::B => "B",
        };
        match &e.field_name {
            Some(field) => println!(
                "  • {}.{} (from {}, safe_to_add: {})",
                e.entity_name, field, side, e.safe_to_add
            ),
            None => println!("  • {} (entity from {})", e.entity_name, side),
        }
    }

    if !result.conflicts.is_empty() {
        println!("\n=== Conflicts ===");
        for c in &result.conflicts {
            match &c.field_name {
                Some(field) => println!(
                    "  ⚠ {}.{} ({:?}, severity: {:?})",
                    c.entity_name, field, c.conflict_type, c.severity
                ),
                None => println!("  ⚠ {} (entity conflict)", c.entity_name),
            }
            println!("     From A: {}", c.evidence.from_a);
            println!("     From B: {}", c.evidence.from_b);
            println!("     Rule: {}", c.evidence.rule);
        }
    }

    // Generate unified schema
    use audd_compare::UnifiedSchema;
    println!("\n=== Generating Unified Schema ===");
    let unified = UnifiedSchema::from_comparison(&schema_a, &schema_b, &result);

    println!("Unified Schema: {}", unified.schema_name);
    println!("Entities: {}", unified.entities.len());

    for entity in &unified.entities {
        println!("\n  Entity: {} (origin: {:?})", entity.entity_name, entity.origin);
        for field in &entity.fields {
            println!(
                "    - {} [{:?}] (state: {:?}, origin: {:?})",
                field.field.field_name,
                field.field.canonical_type.type_name(),
                field.state,
                field.origin
            );
        }
    }

    // Demonstrate similarity matching
    println!("\n=== Testing Similarity Matching ===");
    let config_with_similarity = CompareConfig::all_features().with_similarity_threshold(0.7);
    let result_similarity = compare(&schema_a, &schema_b, &config_with_similarity);

    println!(
        "With similarity matching: {} total matches",
        result_similarity.summary.total_matches
    );

    println!("\n=== Demo Complete ===");
}

fn create_schema_a() -> SourceSchema {
    SourceSchema::builder()
        .source_name("legacy_db")
        .source_type("mysql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(
                    FieldSchema::builder()
                        .field_name("id")
                        .canonical_type(CanonicalType::Int32)
                        .nullable(false)
                        .build(),
                )
                .add_field(
                    FieldSchema::builder()
                        .field_name("email")
                        .canonical_type(CanonicalType::String)
                        .nullable(false)
                        .add_constraint(Constraint::unique())
                        .add_constraint(Constraint::max_length(255))
                        .build(),
                )
                .add_field(
                    FieldSchema::builder()
                        .field_name("password_hash")
                        .canonical_type(CanonicalType::String)
                        .nullable(false)
                        .build(),
                )
                .add_field(
                    FieldSchema::builder()
                        .field_name("created_at")
                        .canonical_type(CanonicalType::DateTime)
                        .nullable(false)
                        .build(),
                )
                .build(),
        )
        .add_entity(
            EntitySchema::builder()
                .entity_name("sessions")
                .add_field(
                    FieldSchema::builder()
                        .field_name("id")
                        .canonical_type(CanonicalType::Int32)
                        .nullable(false)
                        .build(),
                )
                .add_field(
                    FieldSchema::builder()
                        .field_name("token")
                        .canonical_type(CanonicalType::String)
                        .nullable(false)
                        .build(),
                )
                .build(),
        )
        .build()
}

fn create_schema_b() -> SourceSchema {
    SourceSchema::builder()
        .source_name("new_db")
        .source_type("postgresql")
        .add_entity(
            EntitySchema::builder()
                .entity_name("users")
                .add_field(
                    FieldSchema::builder()
                        .field_name("id")
                        .canonical_type(CanonicalType::Int64)
                        .nullable(false)
                        .build(),
                )
                .add_field(
                    FieldSchema::builder()
                        .field_name("email")
                        .canonical_type(CanonicalType::Text)
                        .nullable(false)
                        .add_constraint(Constraint::unique())
                        .build(),
                )
                .add_field(
                    FieldSchema::builder()
                        .field_name("uuid")
                        .canonical_type(CanonicalType::Uuid)
                        .nullable(false)
                        .add_constraint(Constraint::unique())
                        .build(),
                )
                .add_field(
                    FieldSchema::builder()
                        .field_name("created_at")
                        .canonical_type(CanonicalType::Timestamp)
                        .nullable(false)
                        .build(),
                )
                .build(),
        )
        .add_entity(
            EntitySchema::builder()
                .entity_name("profiles")
                .add_field(
                    FieldSchema::builder()
                        .field_name("id")
                        .canonical_type(CanonicalType::Int32)
                        .nullable(false)
                        .build(),
                )
                .add_field(
                    FieldSchema::builder()
                        .field_name("bio")
                        .canonical_type(CanonicalType::Text)
                        .nullable(true)
                        .build(),
                )
                .build(),
        )
        .build()
}

fn print_schema_summary(schema: &SourceSchema) {
    println!("  Source: {} ({})", schema.source_name, schema.source_type);
    println!("  Entities: {}", schema.entities.len());
    for entity in &schema.entities {
        println!("    - {} ({} fields)", entity.entity_name, entity.fields.len());
    }
}
