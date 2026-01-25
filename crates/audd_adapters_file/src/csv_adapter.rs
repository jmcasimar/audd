//! CSV adapter implementation

use crate::adapter::SchemaAdapter;
use crate::error::{AdapterError, AdapterResult};
use audd_ir::{CanonicalType, EntitySchema, FieldSchema, SourceSchema};
use csv::ReaderBuilder;
use std::fs::File;
use std::path::Path;

/// Adapter for CSV files
///
/// Converts CSV headers to IR fields. By default, all fields are typed as `String`.
/// The entity name is derived from the filename.
pub struct CsvAdapter;

impl CsvAdapter {
    /// Create a new CSV adapter
    pub fn new() -> Self {
        CsvAdapter
    }
}

impl Default for CsvAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaAdapter for CsvAdapter {
    fn load(&self, path: &Path) -> AdapterResult<SourceSchema> {
        let file = File::open(path)?;
        let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

        // Extract headers
        let headers = reader.headers()?;
        if headers.is_empty() {
            return Err(AdapterError::EmptyData("CSV has no headers".to_string()));
        }

        // Build fields from headers
        let fields: Vec<FieldSchema> = headers
            .iter()
            .map(|header| {
                FieldSchema::builder()
                    .field_name(header.to_string())
                    .canonical_type(CanonicalType::String) // Default type
                    .nullable(true) // CSV fields are nullable by default
                    .build()
            })
            .collect();

        // Derive entity name from filename
        let entity_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("table")
            .to_string();

        let entity = EntitySchema::builder()
            .entity_name(entity_name)
            .entity_type("table")
            .fields(fields)
            .build();

        // Derive source name from filename
        let source_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("csv_source")
            .to_string();

        Ok(SourceSchema::builder()
            .source_name(source_name)
            .source_type("csv")
            .add_entity(entity)
            .build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_csv_basic() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "id,name,email").unwrap();
        writeln!(file, "1,Alice,alice@example.com").unwrap();
        writeln!(file, "2,Bob,bob@example.com").unwrap();

        let adapter = CsvAdapter::new();
        let schema = adapter.load(file.path()).unwrap();

        assert_eq!(schema.source_type, "csv");
        assert_eq!(schema.entities.len(), 1);

        let entity = &schema.entities[0];
        assert_eq!(entity.fields.len(), 3);
        assert_eq!(entity.fields[0].field_name, "id");
        assert_eq!(entity.fields[1].field_name, "name");
        assert_eq!(entity.fields[2].field_name, "email");

        // All fields should be String type by default
        for field in &entity.fields {
            assert_eq!(field.canonical_type, CanonicalType::String);
            assert!(field.nullable);
        }
    }

    #[test]
    fn test_csv_empty_headers() {
        let mut file = NamedTempFile::new().unwrap();
        // Empty file - no headers

        let adapter = CsvAdapter::new();
        let result = adapter.load(file.path());
        assert!(result.is_err());
    }
}
