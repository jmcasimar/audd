use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "audd")]
#[command(about = "AUDD - Dynamic Data Unification Algorithm", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Load and display schema from a file source
    Load {
        /// Path to the file source (e.g., data.csv, schema.sql)
        /// Supports: .csv, .json, .xml, .sql, .ddl
        #[arg(short, long)]
        source: String,

        /// Output format
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Compare and unify data from different sources (stub implementation)
    Compare {
        /// Path to the first data source
        #[arg(short = '1', long)]
        source1: String,

        /// Path to the second data source
        #[arg(short = '2', long)]
        source2: String,

        /// Output format
        #[arg(short, long, default_value = "json")]
        format: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Load { source, format }) => {
            handle_load(source, format);
        }
        Some(Commands::Compare {
            source1,
            source2,
            format,
        }) => {
            println!("🔍 AUDD Compare (Stub Implementation)");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("Source 1: {}", source1);
            println!("Source 2: {}", source2);
            println!("Format:   {}", format);
            println!();
            println!("✓ Comparison completed successfully!");
            println!();
            println!("Note: This is a stub implementation.");
            println!("Full comparison logic will be implemented in upcoming sprints.");
        }
        None => {
            println!("AUDD - Dynamic Data Unification Algorithm");
            println!("Use --help to see available commands");
        }
    }
}

fn handle_load(source: &str, format: &str) {
    use audd_adapters_file::load_schema_from_file;

    // Support both "file:path" and "path" formats
    let path = source.strip_prefix("file:").unwrap_or(source);

    println!("📁 AUDD Load Schema");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Source: {}", path);
    println!();

    match load_schema_from_file(path) {
        Ok(schema) => {
            println!("✓ Schema loaded successfully!");
            println!();
            println!("Source Name: {}", schema.source_name);
            println!("Source Type: {}", schema.source_type);
            println!("Entities: {}", schema.entities.len());
            println!();

            for entity in &schema.entities {
                println!("Entity: {} ({})", entity.entity_name, entity.entity_type);
                println!("  Fields: {}", entity.fields.len());
                for field in &entity.fields {
                    let nullable = if field.nullable { "NULL" } else { "NOT NULL" };
                    println!(
                        "    - {}: {} {}",
                        field.field_name,
                        field.canonical_type.type_name(),
                        nullable
                    );
                }
                if !entity.keys.is_empty() {
                    println!("  Keys: {}", entity.keys.len());
                    for key in &entity.keys {
                        println!("    - {:?}: {:?}", key.key_type, key.field_names);
                    }
                }
                println!();
            }

            if format == "json" {
                println!("JSON Output:");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                match schema.to_json() {
                    Ok(json) => println!("{}", json),
                    Err(e) => eprintln!("Error serializing to JSON: {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error loading schema: {}", e);
            std::process::exit(1);
        }
    }
}
