//! Schema loading utilities

use audd_ir::SourceSchema;
use crate::error::{CliError, CliResult};

/// Load schema from either a file, database, or remote URL source
pub fn load_schema(source: &str, conn: Option<&str>) -> CliResult<SourceSchema> {
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
            .map_err(|e| CliError::SchemaLoad {
                source: source.to_string(),
                details: anyhow::anyhow!("Failed to create database connector: {}", e),
            })?;
        
        connector.load()
            .map_err(|e| CliError::SchemaLoad {
                source: source.to_string(),
                details: anyhow::anyhow!("Failed to load database schema: {}", e),
            })
    } else if source.starts_with("http://") || source.starts_with("https://") {
        // Remote URL source
        load_schema_from_url(source)
            .map_err(|e| CliError::SchemaLoad {
                source: source.to_string(),
                details: anyhow::anyhow!("Failed to load remote schema: {}", e),
            })
    } else {
        // File source
        let path = source.strip_prefix("file:").unwrap_or(source);
        load_schema_from_file(path)
            .map_err(|e| CliError::SchemaLoad {
                source: source.to_string(),
                details: anyhow::anyhow!("Failed to load file schema: {}", e),
            })
    }
}
