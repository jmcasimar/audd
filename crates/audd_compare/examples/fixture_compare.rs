//! Example comparing the IR fixtures (simple_a.json and simple_b.json)

use audd_compare::{compare, CompareConfig};
use audd_ir::SourceSchema;
use std::fs;

fn main() {
    println!("=== Comparing IR Fixtures ===\n");

    // Load fixtures
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

    println!("Schema A: {} ({})", schema_a.source_name, schema_a.source_type);
    println!("Schema B: {} ({})", schema_b.source_name, schema_b.source_type);

    // Compare with default configuration
    let config = CompareConfig::default();
    let result = compare(&schema_a, &schema_b, &config);

    println!("\n=== Comparison Results ===\n");
    println!("Total Matches:    {}", result.summary.total_matches);
    println!("Total Exclusives: {}", result.summary.total_exclusives);
    println!("Total Conflicts:  {}", result.summary.total_conflicts);

    // Show matches
    println!("\n--- Matches ---");
    for m in &result.matches {
        if let Some(field) = &m.field_name {
            println!("  ✓ {}.{}", m.entity_name, field);
        } else {
            println!("  ✓ {} (entity)", m.entity_name);
        }
    }

    // Show exclusives
    if !result.exclusives.is_empty() {
        println!("\n--- Exclusives ---");
        for e in &result.exclusives {
            let side = match e.side {
                audd_compare::ExclusiveSide::A => "A",
                audd_compare::ExclusiveSide::B => "B",
            };
            if let Some(field) = &e.field_name {
                println!("  • {}.{} (from {})", e.entity_name, field, side);
            } else {
                println!("  • {} (from {})", e.entity_name, side);
            }
        }
    }

    // Show conflicts
    if !result.conflicts.is_empty() {
        println!("\n--- Conflicts ---");
        for c in &result.conflicts {
            if let Some(field) = &c.field_name {
                println!(
                    "  ⚠ {}.{} ({:?})",
                    c.entity_name, field, c.conflict_type
                );
            } else {
                println!("  ⚠ {} ({:?})", c.entity_name, c.conflict_type);
            }
            println!("     {}", c.evidence.rule);
        }
    }

    // Generate unified schema
    use audd_compare::{FieldState, UnifiedSchema};
    let unified = UnifiedSchema::from_comparison(&schema_a, &schema_b, &result);

    println!("\n=== Unified Schema ===\n");
    println!("Name: {}", unified.schema_name);
    for entity in &unified.entities {
        println!("\nEntity: {}", entity.entity_name);
        for field in &entity.fields {
            let status = match field.state {
                FieldState::Matched => "✓",
                FieldState::Exclusive => "•",
                FieldState::Conflicted => "⚠",
            };
            println!(
                "  {} {} [{:?}]",
                status,
                field.field.field_name,
                field.field.canonical_type.type_name()
            );
        }
    }

    println!("\n=== Done ===");
}
