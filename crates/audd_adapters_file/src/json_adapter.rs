//! JSON adapter implementation

use crate::adapter::SchemaAdapter;
use crate::error::{AdapterError, AdapterResult};
use audd_ir::{CanonicalType, EntitySchema, FieldSchema, SourceSchema};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Adapter for JSON files
///
/// Supports:
/// - Single flat object: keys → fields
/// - Array of flat objects: keys from first object → fields
///
/// MVP: Does not support deeply nested structures or heterogeneous arrays.
pub struct JsonAdapter;

impl JsonAdapter {
    /// Create a new JSON adapter
    pub fn new() -> Self {
        JsonAdapter
    }

    /// Infer canonical type from JSON value
    fn infer_type(value: &Value) -> CanonicalType {
        match value {
            Value::Null => CanonicalType::String, // Default for null
            Value::Bool(_) => CanonicalType::Boolean,
            Value::Number(n) => {
                if n.is_i64() {
                    CanonicalType::Int64
                } else if n.is_f64() {
                    CanonicalType::Float64
                } else {
                    CanonicalType::String
                }
            }
            Value::String(_) => CanonicalType::String,
            Value::Array(_) => CanonicalType::Json, // Nested arrays as JSON
            Value::Object(_) => CanonicalType::Json, // Nested objects as JSON
        }
    }

    /// Extract fields from a JSON object
    fn extract_fields(obj: &serde_json::Map<String, Value>) -> Vec<FieldSchema> {
        obj.iter()
            .map(|(key, value)| {
                FieldSchema::builder()
                    .field_name(key.clone())
                    .canonical_type(Self::infer_type(value))
                    .nullable(value.is_null())
                    .build()
            })
            .collect()
    }
}

impl Default for JsonAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaAdapter for JsonAdapter {
    fn load(&self, path: &Path) -> AdapterResult<SourceSchema> {
        let content = fs::read_to_string(path)?;
        let json: Value = serde_json::from_str(&content)?;

        let fields = match &json {
            Value::Object(obj) => {
                // Single object case
                Self::extract_fields(obj)
            }
            Value::Array(arr) => {
                // Array of objects case
                if arr.is_empty() {
                    return Err(AdapterError::EmptyData("JSON array is empty".to_string()));
                }

                // Use first object to determine schema
                if let Some(Value::Object(first_obj)) = arr.first() {
                    Self::extract_fields(first_obj)
                } else {
                    return Err(AdapterError::InvalidStructure(
                        "JSON array must contain objects".to_string(),
                    ));
                }
            }
            _ => {
                return Err(AdapterError::InvalidStructure(
                    "JSON must be an object or array of objects".to_string(),
                ));
            }
        };

        if fields.is_empty() {
            return Err(AdapterError::EmptyData("No fields found in JSON".to_string()));
        }

        // Derive entity name from filename
        let entity_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("table")
            .to_string();

        let entity = EntitySchema::builder()
            .entity_name(entity_name)
            .entity_type("collection")
            .fields(fields)
            .build();

        // Derive source name from filename
        let source_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("json_source")
            .to_string();

        Ok(SourceSchema::builder()
            .source_name(source_name)
            .source_type("json")
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
    fn test_json_single_object() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"{{"id": 1, "name": "Alice", "active": true, "score": 95.5}}"#
        )
        .unwrap();

        let adapter = JsonAdapter::new();
        let schema = adapter.load(file.path()).unwrap();

        assert_eq!(schema.source_type, "json");
        assert_eq!(schema.entities.len(), 1);

        let entity = &schema.entities[0];
        assert_eq!(entity.fields.len(), 4);

        // Check field types
        let id_field = entity.fields.iter().find(|f| f.field_name == "id").unwrap();
        assert_eq!(id_field.canonical_type, CanonicalType::Int64);

        let name_field = entity
            .fields
            .iter()
            .find(|f| f.field_name == "name")
            .unwrap();
        assert_eq!(name_field.canonical_type, CanonicalType::String);

        let active_field = entity
            .fields
            .iter()
            .find(|f| f.field_name == "active")
            .unwrap();
        assert_eq!(active_field.canonical_type, CanonicalType::Boolean);

        let score_field = entity
            .fields
            .iter()
            .find(|f| f.field_name == "score")
            .unwrap();
        assert_eq!(score_field.canonical_type, CanonicalType::Float64);
    }

    #[test]
    fn test_json_array_of_objects() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"[
            {{"id": 1, "name": "Alice"}},
            {{"id": 2, "name": "Bob"}}
        ]"#
        )
        .unwrap();

        let adapter = JsonAdapter::new();
        let schema = adapter.load(file.path()).unwrap();

        assert_eq!(schema.source_type, "json");
        assert_eq!(schema.entities.len(), 1);

        let entity = &schema.entities[0];
        assert_eq!(entity.fields.len(), 2);
    }

    #[test]
    fn test_json_empty_array() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "[]").unwrap();

        let adapter = JsonAdapter::new();
        let result = adapter.load(file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_json_invalid_structure() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, r#""just a string""#).unwrap();

        let adapter = JsonAdapter::new();
        let result = adapter.load(file.path());
        assert!(result.is_err());
    }
}
