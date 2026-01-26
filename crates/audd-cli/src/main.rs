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
    /// Load and display schema from a source (file, database, or remote URL)
    Load {
        /// Source identifier
        /// - File: path/to/file.csv or file:path/to/file.csv
        /// - Database: db:sqlite://path, db:mysql://user:pass@host/db, 
        ///   db:postgres://user:pass@host/db, db:mongodb://host/db,
        ///   db:sqlserver://user:pass@host/db, or db:firebird://user:pass@host:/db.fdb
        /// - Remote URL: https://example.com/data.csv
        /// - Google Sheets: https://docs.google.com/spreadsheets/d/SHEET_ID/edit
        #[arg(short, long)]
        source: String,

        /// Connection string (only for database sources)
        #[arg(short, long)]
        conn: Option<String>,

        /// Output format
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Compare and unify data from different sources
    Compare {
        /// First source identifier
        #[arg(long = "source-a")]
        source_a: String,

        /// Connection string for first source (only for database sources)
        #[arg(long = "conn-a")]
        conn_a: Option<String>,

        /// Second source identifier
        #[arg(long = "source-b")]
        source_b: String,

        /// Connection string for second source (only for database sources)
        #[arg(long = "conn-b")]
        conn_b: Option<String>,

        /// Output format
        #[arg(short, long, default_value = "json")]
        format: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Load { source, conn, format }) => {
            handle_load(source, conn.as_deref(), format);
        }
        Some(Commands::Compare {
            source_a,
            conn_a,
            source_b,
            conn_b,
            format,
        }) => {
            println!("🔍 AUDD Compare");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            
            // Load schema A
            println!("Loading schema A...");
            let schema_a = match load_schema(source_a, conn_a.as_deref()) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("❌ Error loading source A: {}", e);
                    std::process::exit(1);
                }
            };
            println!("✓ Schema A loaded: {} ({} entities)", schema_a.source_name, schema_a.entities.len());

            // Load schema B
            println!("Loading schema B...");
            let schema_b = match load_schema(source_b, conn_b.as_deref()) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("❌ Error loading source B: {}", e);
                    std::process::exit(1);
                }
            };
            println!("✓ Schema B loaded: {} ({} entities)", schema_b.source_name, schema_b.entities.len());

            println!();
            println!("Format:   {}", format);
            println!();
            println!("✓ Schemas loaded successfully!");
            println!();
            println!("Note: Full comparison logic will be implemented in upcoming sprints.");
        }
        None => {
            println!("AUDD - Dynamic Data Unification Algorithm");
            println!("Use --help to see available commands");
        }
    }
}

/// Load schema from either a file, database, or remote URL source
fn load_schema(source: &str, conn: Option<&str>) -> Result<audd_ir::SourceSchema, String> {
    use audd_adapters_file::{load_schema_from_file, load_schema_from_url};
    use audd_adapters_db::create_connector;

    // Determine source type
    if source.starts_with("db:") {
        // Database source
        let db_conn_str = if let Some(conn_str) = conn {
            // Legacy format: --source db:sqlite --conn /path/to/db
            let engine = source.strip_prefix("db:").unwrap();
            format!("{}://{}", engine, conn_str)
        } else {
            // New format: --source db:sqlite:///path/to/db
            source.strip_prefix("db:").unwrap().to_string()
        };

        let connector = create_connector(&db_conn_str)
            .map_err(|e| format!("Failed to create database connector: {}", e))?;
        
        connector.load()
            .map_err(|e| format!("Failed to load database schema: {}", e))
    } else if source.starts_with("http://") || source.starts_with("https://") {
        // Remote URL source
        load_schema_from_url(source)
            .map_err(|e| format!("Failed to load remote schema: {}", e))
    } else {
        // File source
        let path = source.strip_prefix("file:").unwrap_or(source);
        load_schema_from_file(path)
            .map_err(|e| format!("Failed to load file schema: {}", e))
    }
}

fn handle_load(source: &str, conn: Option<&str>, format: &str) {
    println!("📁 AUDD Load Schema");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Source: {}", source);
    if let Some(conn_str) = conn {
        println!("Connection: {}", conn_str);
    }
    println!();

    match load_schema(source, conn) {
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
