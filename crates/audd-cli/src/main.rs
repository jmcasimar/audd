mod config;
mod error;
mod loader;
mod output;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use error::CliResult;
use config::Config;

#[derive(Parser)]
#[command(name = "audd")]
#[command(about = "AUDD - Dynamic Data Unification Algorithm", long_about = None)]
#[command(version)]
struct Cli {
    /// Path to configuration file (optional)
    #[arg(long = "config", global = true)]
    config: Option<PathBuf>,

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

        /// Output directory for generated files (unified_schema.json, diff.json, report.md, decision_log.json)
        #[arg(short, long)]
        out: Option<PathBuf>,

        /// Confidence threshold for auto-accepting suggestions (0.0 to 1.0)
        #[arg(long = "confidence-threshold")]
        confidence_threshold: Option<f64>,

        /// Output format (legacy, not used with --out)
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Inspect and export intermediate representation (IR) of a schema
    Inspect {
        /// Source identifier (same format as load/compare)
        #[arg(short, long)]
        source: String,

        /// Connection string (only for database sources)
        #[arg(short, long)]
        conn: Option<String>,

        /// Output file path (optional, prints to stdout if not provided)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    /// Generate a sample configuration file
    GenerateConfig {
        /// Output path for the configuration file
        #[arg(short, long, default_value = "audd.toml")]
        out: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    // Load configuration
    let config = if let Some(config_path) = &cli.config {
        // Load from specified path
        match Config::from_file(config_path) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("❌ Error loading config file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Try to load from default locations
        Config::load_default()
    };

    let result = match &cli.command {
        Some(Commands::Load { source, conn, format }) => {
            handle_load(source, conn.as_deref(), format)
        }
        Some(Commands::Compare {
            source_a,
            conn_a,
            source_b,
            conn_b,
            out,
            confidence_threshold,
            format: _,
        }) => {
            let out_dir = out.clone().unwrap_or_else(|| PathBuf::from(config.get_default_output_dir()));
            handle_compare(source_a, conn_a.as_deref(), source_b, conn_b.as_deref(), &out_dir, &config, *confidence_threshold)
        }
        Some(Commands::Inspect { source, conn, out }) => {
            handle_inspect(source, conn.as_deref(), out.as_ref())
        }
        Some(Commands::GenerateConfig { out }) => {
            handle_generate_config(out)
        }
        None => {
            println!("AUDD - Dynamic Data Unification Algorithm");
            println!("Use --help to see available commands");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}

fn handle_load(source: &str, conn: Option<&str>, format: &str) -> CliResult<()> {
    println!("📁 AUDD Load Schema");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Source: {}", source);
    if let Some(conn_str) = conn {
        println!("Connection: {}", conn_str);
    }
    println!();

    let schema = loader::load_schema(source, conn)?;
    
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
        let json = schema.to_json()?;
        println!("{}", json);
    }

    Ok(())
}

fn handle_compare(
    source_a: &str,
    conn_a: Option<&str>,
    source_b: &str,
    conn_b: Option<&str>,
    out_dir: &PathBuf,
    config: &Config,
    confidence_threshold_override: Option<f64>,
) -> CliResult<()> {
    println!("🔍 AUDD Compare");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Load schema A
    println!("Loading schema A from {}...", source_a);
    let schema_a = loader::load_schema(source_a, conn_a)?;
    println!("✓ Schema A loaded: {} ({} entities)", schema_a.source_name, schema_a.entities.len());

    // Load schema B
    println!("Loading schema B from {}...", source_b);
    let schema_b = loader::load_schema(source_b, conn_b)?;
    println!("✓ Schema B loaded: {} ({} entities)", schema_b.source_name, schema_b.entities.len());

    // Compare schemas
    println!();
    println!("Comparing schemas...");
    let compare_config = audd_compare::CompareConfig::default();
    let comparison_result = audd_compare::compare(&schema_a, &schema_b, &compare_config);
    
    println!("✓ Comparison complete!");
    println!("  - Matches: {}", comparison_result.matches.len());
    println!("  - Exclusives: {}", comparison_result.exclusives.len());
    println!("  - Conflicts: {}", comparison_result.conflicts.len());

    // Generate suggestions for conflicts
    println!();
    println!("Generating resolution suggestions...");
    let suggestion_engine = audd_resolution::SuggestionEngine::new();
    let mut decision_log = audd_resolution::DecisionLog::new()
        .with_schema_ids(schema_a.source_name.clone(), schema_b.source_name.clone());

    // Get confidence threshold from precedence: CLI flag > config > default
    let confidence_threshold = config.get_confidence_threshold(confidence_threshold_override);
    let decision_id_prefix = config.get_decision_id_prefix(None);

    let mut decision_counter = 0;
    for conflict in &comparison_result.conflicts {
        let suggestions = suggestion_engine.suggest(conflict);
        // Auto-accept high-confidence suggestions (>= threshold)
        for suggestion in suggestions {
            if suggestion.confidence.value() >= confidence_threshold {
                decision_counter += 1;
                let decision = audd_resolution::Decision::by_system(
                    format!("{}_{}", decision_id_prefix, decision_counter),
                    suggestion,
                    true,
                    "high_confidence_auto_accept".to_string(),
                );
                decision_log.add_decision(decision);
            }
        }
    }

    println!("✓ Generated {} suggestions", decision_log.get_decisions().len());

    // Generate unified schema
    println!();
    println!("Building unified schema...");
    let unified_schema = audd_compare::UnifiedSchema::from_comparison(&schema_a, &schema_b, &comparison_result);
    println!("✓ Unified schema created with {} entities", unified_schema.entities.len());

    // Write outputs
    println!();
    println!("Writing output files to {}...", out_dir.display());
    output::ensure_output_dir(out_dir)?;

    if config.should_generate_unified_schema() {
        let unified_path = output::write_unified_schema(out_dir, &unified_schema)?;
        println!("✓ Wrote {}", unified_path.display());
    }

    if config.should_generate_diff() {
        let diff_path = output::write_diff(out_dir, &comparison_result)?;
        println!("✓ Wrote {}", diff_path.display());
    }

    if config.should_generate_decision_log() {
        let log_path = output::write_decision_log(out_dir, &decision_log)?;
        println!("✓ Wrote {}", log_path.display());
    }

    if config.should_generate_report() {
        let report_path = output::write_report(out_dir, &decision_log, &comparison_result)?;
        println!("✓ Wrote {}", report_path.display());
    }

    if config.should_generate_json_report() {
        let json_report_path = output::write_json_report(out_dir, &decision_log, &comparison_result)?;
        println!("✓ Wrote {}", json_report_path.display());
    }

    println!();
    println!("✅ Comparison completed successfully!");
    println!("Output files written to: {}", out_dir.display());

    Ok(())
}

fn handle_inspect(source: &str, conn: Option<&str>, out_path: Option<&PathBuf>) -> CliResult<()> {
    println!("🔍 AUDD Inspect");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Source: {}", source);
    if let Some(conn_str) = conn {
        println!("Connection: {}", conn_str);
    }
    println!();

    println!("Loading schema...");
    let schema = loader::load_schema(source, conn)?;
    println!("✓ Schema loaded: {} ({} entities)", schema.source_name, schema.entities.len());
    println!();

    if let Some(path) = out_path {
        // Write to file
        let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
        output::ensure_output_dir(parent)?;
        
        let json = schema.to_json()?;
        std::fs::write(path, json).map_err(|e| error::CliError::OutputWrite {
            path: path.display().to_string(),
            details: e,
        })?;
        
        println!("✓ IR exported to: {}", path.display());
    } else {
        // Print to stdout
        println!("IR (Intermediate Representation):");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        let json = schema.to_json()?;
        println!("{}", json);
    }

    Ok(())
}

fn handle_generate_config(out_path: &PathBuf) -> CliResult<()> {
    println!("📝 Generating sample configuration file...");
    
    let sample = Config::sample();
    
    std::fs::write(out_path, sample).map_err(|e| error::CliError::OutputWrite {
        path: out_path.display().to_string(),
        details: e,
    })?;
    
    println!("✓ Sample configuration written to: {}", out_path.display());
    println!();
    println!("You can customize this file and use it with:");
    println!("  audd --config {} compare ...", out_path.display());
    println!();
    println!("Or place it in one of these locations to be loaded automatically:");
    println!("  - ./audd.toml");
    println!("  - ~/.audd.toml");
    println!("  - ~/.config/audd/config.toml");
    
    Ok(())
}

