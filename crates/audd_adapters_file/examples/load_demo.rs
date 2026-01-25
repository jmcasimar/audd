//! Example: Loading schemas from different file formats

use audd_adapters_file::{load_schema_from_file, JsonAdapter, SchemaAdapter};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AUDD File Adapters Demo");
    println!("=======================\n");

    // Get workspace root (examples run from workspace root)
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    // Example 1: Auto-detect format from extension
    println!("1. Auto-detection from extension:");
    println!("   Loading CSV file...");
    let csv_path = workspace_root.join("fixtures/adapters/users.csv");
    let schema = load_schema_from_file(&csv_path)?;
    println!("   ✓ Loaded: {} ({})", schema.source_name, schema.source_type);
    println!("   Entities: {}", schema.entities.len());
    println!("   Fields in first entity: {}\n", schema.entities[0].fields.len());

    // Example 2: Explicit adapter - JSON
    println!("2. Using explicit JSON adapter:");
    let json_path = workspace_root.join("fixtures/adapters/users.json");
    let json_adapter = JsonAdapter::new();
    let schema = json_adapter.load(&json_path)?;
    println!("   ✓ Loaded: {} ({})", schema.source_name, schema.source_type);
    
    // Show type inference
    println!("   Type inference examples:");
    for field in &schema.entities[0].fields {
        println!("     - {}: {}", field.field_name, field.canonical_type.type_name());
    }
    println!();

    // Example 3: SQL with multiple tables
    println!("3. Loading SQL DDL with multiple tables:");
    let sql_path = workspace_root.join("fixtures/adapters/schema.sql");
    let schema = load_schema_from_file(&sql_path)?;
    println!("   ✓ Loaded: {} tables", schema.entities.len());
    for entity in &schema.entities {
        println!("     - {} ({} fields, {} keys)", 
            entity.entity_name, 
            entity.fields.len(),
            entity.keys.len()
        );
    }
    println!();

    // Example 4: Serialize to JSON
    println!("4. Serializing schema to JSON:");
    let csv_path = workspace_root.join("fixtures/adapters/users.csv");
    let schema = load_schema_from_file(&csv_path)?;
    let json_output = schema.to_json()?;
    println!("   JSON output ({} bytes)", json_output.len());
    println!("   {}", &json_output[..200.min(json_output.len())]);
    println!("   ...\n");

    println!("✓ All examples completed successfully!");

    Ok(())
}
