mod config;
mod error;
mod loader;
mod output;

use clap::{Parser, Subcommand};
use config::Config;
use error::CliResult;
use std::path::PathBuf;

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
        ///   db:postgresql://user:pass@host/db, db:mongodb://host/db,
        ///   db:sqlserver://user:pass@host/db, or db:firebird://user:pass@host:/db.fdb
        /// - Remote URL (requires --features remote): https://example.com/data.csv
        /// - Google Sheets (requires --features remote): https://docs.google.com/spreadsheets/d/SHEET_ID/edit
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

        /// Use a fixed timestamp to make outputs reproducible byte-for-byte
        #[arg(long = "deterministic")]
        deterministic: bool,

        /// Path to a resolution catalog directory (default: ./catalog)
        #[arg(long = "catalog")]
        catalog: Option<PathBuf>,

        /// Activate a domain catalog (e.g., pos)
        #[arg(long = "domain")]
        domain: Option<String>,

        /// Continue without catalog if it cannot be loaded (prints a warning)
        #[arg(long = "catalog-optional")]
        catalog_optional: bool,

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

    /// Work with the resolution catalog
    Catalog {
        #[command(subcommand)]
        action: CatalogCommand,
    },

    /// Generate a sample configuration file
    GenerateConfig {
        /// Output path for the configuration file
        #[arg(short, long, default_value = "audd.toml")]
        out: PathBuf,
    },
}

#[derive(Subcommand)]
enum CatalogCommand {
    /// Turn accepted decisions into candidate rules (source = "learned") for review
    Learn {
        /// decision_log.json produced by `audd compare`
        #[arg(long = "from")]
        from: PathBuf,

        /// Output TOML file with the candidate rules (never merged automatically)
        #[arg(long = "out")]
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
        Some(Commands::Load {
            source,
            conn,
            format,
        }) => handle_load(source, conn.as_deref(), format),
        Some(Commands::Compare {
            source_a,
            conn_a,
            source_b,
            conn_b,
            out,
            confidence_threshold,
            deterministic,
            catalog,
            domain,
            catalog_optional,
            format: _,
        }) => {
            let out_dir = out
                .clone()
                .unwrap_or_else(|| PathBuf::from(config.get_default_output_dir()));
            handle_compare(
                source_a,
                conn_a.as_deref(),
                source_b,
                conn_b.as_deref(),
                &out_dir,
                &config,
                *confidence_threshold,
                *deterministic,
                catalog.as_deref(),
                domain.as_deref(),
                *catalog_optional,
            )
        }
        Some(Commands::Catalog {
            action: CatalogCommand::Learn { from, out },
        }) => handle_catalog_learn(from, out),
        Some(Commands::Inspect { source, conn, out }) => {
            handle_inspect(source, conn.as_deref(), out.as_ref())
        }
        Some(Commands::GenerateConfig { out }) => handle_generate_config(out),
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

/// Replace entity and field names with their canonical form from an active domain.
fn apply_domain_aliases(
    mut schema: audd_ir::SourceSchema,
    aliases: &std::collections::HashMap<String, String>,
) -> audd_ir::SourceSchema {
    for entity in &mut schema.entities {
        if let Some(canonical) = aliases.get(&entity.entity_name.to_lowercase()) {
            entity.entity_name.clone_from(canonical);
        }
        for field in &mut entity.fields {
            if let Some(canonical) = aliases.get(&field.field_name.to_lowercase()) {
                field.field_name.clone_from(canonical);
            }
        }
    }
    schema
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

#[allow(clippy::too_many_arguments)]
fn handle_compare(
    source_a: &str,
    conn_a: Option<&str>,
    source_b: &str,
    conn_b: Option<&str>,
    out_dir: &std::path::Path,
    config: &Config,
    confidence_threshold_override: Option<f64>,
    deterministic: bool,
    catalog_dir: Option<&std::path::Path>,
    domain: Option<&str>,
    catalog_optional: bool,
) -> CliResult<()> {
    if deterministic {
        audd_resolution::time::enable_deterministic_mode();
    }

    println!("🔍 AUDD Compare");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Load schema A
    println!("Loading schema A from {}...", source_a);
    let schema_a = loader::load_schema(source_a, conn_a)?;
    println!(
        "✓ Schema A loaded: {} ({} entities)",
        schema_a.source_name,
        schema_a.entities.len()
    );

    // Load schema B
    println!("Loading schema B from {}...", source_b);
    let schema_b = loader::load_schema(source_b, conn_b)?;
    println!(
        "✓ Schema B loaded: {} ({} entities)",
        schema_b.source_name,
        schema_b.entities.len()
    );

    // Load resolution catalog: --catalog <dir>, or ./catalog if it exists (K01)
    let default_dir = std::path::Path::new("catalog");
    let dir = catalog_dir.or_else(|| default_dir.is_dir().then_some(default_dir));
    let catalog = match dir.map(audd_resolution::ResolutionCatalog::load_from_dir) {
        None => audd_resolution::ResolutionCatalog::new(),
        Some(Ok(catalog)) => catalog,
        Some(Err(e)) if catalog_optional => {
            eprintln!(
                "⚠️  Could not load the resolution catalog: {} (continuing without it)",
                e
            );
            audd_resolution::ResolutionCatalog::new()
        }
        Some(Err(e)) => {
            return Err(error::CliError::CatalogError(format!(
                "{} (use --catalog-optional to continue without it)",
                e
            )))
        }
    };

    if !catalog.rules.is_empty() || !catalog.domains.is_empty() {
        println!(
            "✓ Loaded resolution catalog ({} rules, {} domains)",
            catalog.rules.len(),
            catalog.domains.len()
        );
    }

    // Apply domain aliases if requested
    let (schema_a, schema_b) = if let Some(domain_name) = domain {
        let aliases = catalog.domain_aliases(domain_name);
        if aliases.is_empty() {
            eprintln!("⚠️  Domain '{}' not found in catalog", domain_name);
        } else {
            println!(
                "✓ Applied domain '{}' ({} aliases)",
                domain_name,
                aliases.len()
            );
        }
        (
            apply_domain_aliases(schema_a, &aliases),
            apply_domain_aliases(schema_b, &aliases),
        )
    } else {
        (schema_a, schema_b)
    };

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
    // Which catalog produced the suggestions (K06)
    let catalog_summary = catalog.summary();
    let suggestion_engine = audd_resolution::SuggestionEngine::new().with_catalog(catalog);
    let mut decision_log = audd_resolution::DecisionLog::new()
        .with_schema_ids(schema_a.source_name.clone(), schema_b.source_name.clone())
        .with_custom_metadata(serde_json::json!({
            "catalog": catalog_summary,
            "domain": domain,
        }));

    // Get confidence threshold from precedence: CLI flag > config > default
    let confidence_threshold = config.get_confidence_threshold(confidence_threshold_override);
    let decision_id_prefix = config.get_decision_id_prefix(None);

    let mut decision_counter = 0;
    let mut all_suggestions: Vec<output::SuggestionEntry> = Vec::new();
    for (conflict_index, conflict) in comparison_result.conflicts.iter().enumerate() {
        let suggestions = suggestion_engine.suggest(conflict);
        // Record every suggestion (gap B01); only auto-accept high-confidence ones
        for mut suggestion in suggestions {
            // Trace the origin in every output (K03)
            let origin = audd_resolution::suggestion_origin(&suggestion);
            if !suggestion.evidence.contains(&origin) {
                suggestion.evidence.push(origin.clone());
            }
            let accepted_auto = suggestion.confidence.value() >= confidence_threshold;
            all_suggestions.push(output::SuggestionEntry::new(
                conflict_index,
                conflict,
                &suggestion,
                accepted_auto,
            ));
            if accepted_auto {
                decision_counter += 1;
                let decision = audd_resolution::Decision::by_system(
                    format!("{}_{}", decision_id_prefix, decision_counter),
                    suggestion,
                    true,
                    "high_confidence_auto_accept".to_string(),
                )
                .with_metadata(serde_json::json!({
                    "conflict_index": conflict_index,
                    "conflict_type": format!("{:?}", conflict.conflict_type),
                    "origin": origin,
                }));
                decision_log.add_decision(decision);
            }
        }
    }

    println!(
        "✓ Generated {} suggestions",
        decision_log.get_decisions().len()
    );

    // Generate unified schema
    println!();
    println!("Building unified schema...");
    let unified_schema =
        audd_compare::UnifiedSchema::from_comparison(&schema_a, &schema_b, &comparison_result);
    println!(
        "✓ Unified schema created with {} entities",
        unified_schema.entities.len()
    );

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
        let json_report_path =
            output::write_json_report(out_dir, &decision_log, &comparison_result)?;
        println!("✓ Wrote {}", json_report_path.display());
    }

    if config.should_generate_suggestions() {
        let suggestions_path = output::write_suggestions(out_dir, &all_suggestions)?;
        println!("✓ Wrote {}", suggestions_path.display());
    }

    println!();
    println!("✅ Comparison completed successfully!");
    println!("Output files written to: {}", out_dir.display());

    Ok(())
}

fn handle_catalog_learn(from: &std::path::Path, out: &std::path::Path) -> CliResult<()> {
    let log = audd_resolution::DecisionLog::from_json_file(from)
        .map_err(|e| error::CliError::CatalogError(format!("{}: {}", from.display(), e)))?;
    let rules = audd_resolution::learn_rules(&log);
    let toml = audd_resolution::rules_to_toml(&rules)
        .map_err(|e| error::CliError::CatalogError(e.to_string()))?;
    let header = format!(
        "# Candidate rules learned from {} (source = \"learned\").\n\
         # Review them before copying any into catalog/resolutions.toml.\n\n",
        from.display()
    );
    std::fs::write(out, header + &toml).map_err(|e| error::CliError::OutputWrite {
        path: out.display().to_string(),
        details: e,
    })?;
    println!(
        "✓ {} candidate rules from {} accepted decisions written to {}",
        rules.len(),
        log.get_decisions().iter().filter(|d| d.accepted).count(),
        out.display()
    );
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
    println!(
        "✓ Schema loaded: {} ({} entities)",
        schema.source_name,
        schema.entities.len()
    );
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
