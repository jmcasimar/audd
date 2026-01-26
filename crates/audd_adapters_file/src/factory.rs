//! Factory for auto-detecting and loading schemas from files and URLs

use crate::adapter::SchemaAdapter;
use crate::csv_adapter::CsvAdapter;
use crate::error::{AdapterError, AdapterResult};
use crate::json_adapter::JsonAdapter;
use crate::remote_adapter::RemoteAdapter;
use crate::sql_adapter::SqlAdapter;
use crate::xml_adapter::XmlAdapter;
use audd_ir::SourceSchema;
use std::path::Path;

/// Load a schema from a file with automatic format detection
///
/// The file format is detected based on the file extension:
/// - `.csv` → CSV adapter
/// - `.json` → JSON adapter
/// - `.xml` → XML adapter
/// - `.sql`, `.ddl` → SQL adapter
///
/// # Arguments
///
/// * `path` - Path to the file to load
///
/// # Returns
///
/// A `SourceSchema` representing the file's structure
///
/// # Errors
///
/// Returns an error if:
/// - The file extension is not recognized
/// - The file cannot be read
/// - The file format is invalid
///
/// # Example
///
/// ```no_run
/// use audd_adapters_file::load_schema_from_file;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let schema = load_schema_from_file("data.csv")?;
/// println!("Loaded schema: {}", schema.source_name);
/// # Ok(())
/// # }
/// ```
pub fn load_schema_from_file<P: AsRef<Path>>(path: P) -> AdapterResult<SourceSchema> {
    let path = path.as_ref();
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "csv" => CsvAdapter::new().load(path),
        "json" => JsonAdapter::new().load(path),
        "xml" => XmlAdapter::new().load(path),
        "sql" | "ddl" => SqlAdapter::new().load(path),
        _ => Err(AdapterError::UnsupportedFormat(format!(
            "File extension '{}' is not supported. Supported formats: csv, json, xml, sql, ddl",
            extension
        ))),
    }
}

/// Get the appropriate adapter for a file based on its extension
///
/// # Arguments
///
/// * `path` - Path to the file
///
/// # Returns
///
/// A boxed `SchemaAdapter` that can handle the file format
///
/// # Errors
///
/// Returns an error if the file extension is not recognized
pub fn get_adapter_for_file<P: AsRef<Path>>(
    path: P,
) -> AdapterResult<Box<dyn SchemaAdapter>> {
    let path = path.as_ref();
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "csv" => Ok(Box::new(CsvAdapter::new())),
        "json" => Ok(Box::new(JsonAdapter::new())),
        "xml" => Ok(Box::new(XmlAdapter::new())),
        "sql" | "ddl" => Ok(Box::new(SqlAdapter::new())),
        _ => Err(AdapterError::UnsupportedFormat(format!(
            "File extension '{}' is not supported",
            extension
        ))),
    }
}

/// Load a schema from a URL (HTTP/HTTPS or Google Sheets)
///
/// Supports:
/// - HTTP/HTTPS URLs with file extensions (.csv, .json, .xml, .sql, .ddl)
/// - Google Sheets public URLs (automatically exported as CSV)
///
/// # Arguments
///
/// * `url` - The URL to fetch the schema from
///
/// # Returns
///
/// A `SourceSchema` representing the remote file's structure
///
/// # Errors
///
/// Returns an error if:
/// - The URL cannot be fetched
/// - The format cannot be detected
/// - The content is invalid
///
/// # Example
///
/// ```no_run
/// use audd_adapters_file::load_schema_from_url;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Load from HTTP URL
/// let schema = load_schema_from_url("https://example.com/data.csv")?;
/// println!("Loaded schema: {}", schema.source_name);
///
/// // Load from Google Sheets (public)
/// let sheet_url = "https://docs.google.com/spreadsheets/d/SHEET_ID/edit";
/// let schema = load_schema_from_url(sheet_url)?;
/// println!("Loaded schema: {}", schema.source_name);
/// # Ok(())
/// # }
/// ```
pub fn load_schema_from_url(url: &str) -> AdapterResult<SourceSchema> {
    RemoteAdapter::new(url).load_schema()
}

/// Load a schema from a URL with an explicit format hint
///
/// This is useful when the URL doesn't have a clear file extension
///
/// # Arguments
///
/// * `url` - The URL to fetch the schema from
/// * `format` - The format hint (csv, json, xml, sql)
///
/// # Returns
///
/// A `SourceSchema` representing the remote file's structure
///
/// # Example
///
/// ```no_run
/// use audd_adapters_file::load_schema_from_url_with_format;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let schema = load_schema_from_url_with_format(
///     "https://api.example.com/data",
///     "json"
/// )?;
/// # Ok(())
/// # }
/// ```
pub fn load_schema_from_url_with_format(url: &str, format: &str) -> AdapterResult<SourceSchema> {
    RemoteAdapter::with_format(url, format).load_schema()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_csv_detection() {
        let mut file = NamedTempFile::with_suffix(".csv").unwrap();
        writeln!(file, "id,name").unwrap();
        writeln!(file, "1,Alice").unwrap();

        let schema = load_schema_from_file(file.path()).unwrap();
        assert_eq!(schema.source_type, "csv");
    }

    #[test]
    fn test_json_detection() {
        let mut file = NamedTempFile::with_suffix(".json").unwrap();
        writeln!(file, r#"{{"id": 1, "name": "Alice"}}"#).unwrap();

        let schema = load_schema_from_file(file.path()).unwrap();
        assert_eq!(schema.source_type, "json");
    }

    #[test]
    fn test_xml_detection() {
        let mut file = NamedTempFile::with_suffix(".xml").unwrap();
        writeln!(
            file,
            r#"<?xml version="1.0"?>
<root><record><id>1</id></record></root>"#
        )
        .unwrap();

        let schema = load_schema_from_file(file.path()).unwrap();
        assert_eq!(schema.source_type, "xml");
    }

    #[test]
    fn test_sql_detection() {
        let mut file = NamedTempFile::with_suffix(".sql").unwrap();
        writeln!(file, "CREATE TABLE users (id INT);").unwrap();

        let schema = load_schema_from_file(file.path()).unwrap();
        assert_eq!(schema.source_type, "sql");
    }

    #[test]
    fn test_unsupported_format() {
        let result = load_schema_from_file("test.xyz");
        assert!(result.is_err());
        match result {
            Err(AdapterError::UnsupportedFormat(_)) => (),
            _ => panic!("Expected UnsupportedFormat error"),
        }
    }

    #[test]
    fn test_get_adapter_for_csv() {
        let adapter = get_adapter_for_file("test.csv");
        // Just verify we got an adapter successfully
        assert!(adapter.is_ok());
    }
}
