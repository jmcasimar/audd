//! XML adapter implementation (MVP)

use crate::adapter::SchemaAdapter;
use crate::error::{AdapterError, AdapterResult};
use audd_ir::{CanonicalType, EntitySchema, FieldSchema, SourceSchema};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Adapter for XML files
///
/// MVP approach:
/// - Treats first-level tags as potential field names
/// - Assumes homogeneous structure (all records have same tags)
/// - Attributes are treated as fields with "_attr" suffix
/// - Text content is treated as a field
///
/// Example:
/// ```xml
/// <root>
///   <record>
///     <id>1</id>
///     <name>Alice</name>
///   </record>
/// </root>
/// ```
/// Results in fields: id, name
pub struct XmlAdapter;

impl XmlAdapter {
    /// Create a new XML adapter
    pub fn new() -> Self {
        XmlAdapter
    }

    /// Extract field names from XML
    fn extract_fields_from_xml(content: &str) -> Result<Vec<String>, String> {
        let mut reader = Reader::from_str(content);
        reader.config_mut().trim_text(true);

        let mut field_names = HashSet::new();
        let mut depth = 0;
        let mut in_record = false;
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    depth += 1;
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                    // Detect record level (depth 2 or specific tag)
                    if depth == 2 || name == "record" || name == "item" || name == "row" {
                        in_record = true;
                    } else if in_record && depth == 3 {
                        // Field names are at depth 3
                        field_names.insert(name);
                    }

                    // Also capture attributes as fields
                    for attr in e.attributes().flatten() {
                        let attr_name = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                        field_names.insert(format!("{}_attr", attr_name));
                    }
                }
                Ok(Event::End(_)) => {
                    depth -= 1;
                    if depth < 2 {
                        in_record = false;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(format!("XML parse error at position {}: {}", reader.buffer_position(), e)),
                _ => {}
            }
            buf.clear();
        }

        Ok(field_names.into_iter().collect())
    }
}

impl Default for XmlAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaAdapter for XmlAdapter {
    fn load(&self, path: &Path) -> AdapterResult<SourceSchema> {
        let content = fs::read_to_string(path)?;

        let field_names = Self::extract_fields_from_xml(&content)
            .map_err(|e| AdapterError::XmlError(e))?;

        if field_names.is_empty() {
            return Err(AdapterError::EmptyData(
                "No fields found in XML".to_string(),
            ));
        }

        // Create fields (all String type for MVP)
        let fields: Vec<FieldSchema> = field_names
            .into_iter()
            .map(|name| {
                FieldSchema::builder()
                    .field_name(name)
                    .canonical_type(CanonicalType::String)
                    .nullable(true)
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
            .entity_type("collection")
            .fields(fields)
            .build();

        // Derive source name from filename
        let source_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("xml_source")
            .to_string();

        Ok(SourceSchema::builder()
            .source_name(source_name)
            .source_type("xml")
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
    fn test_xml_basic() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"<?xml version="1.0"?>
<root>
  <record>
    <id>1</id>
    <name>Alice</name>
    <email>alice@example.com</email>
  </record>
  <record>
    <id>2</id>
    <name>Bob</name>
    <email>bob@example.com</email>
  </record>
</root>"#
        )
        .unwrap();

        let adapter = XmlAdapter::new();
        let schema = adapter.load(file.path()).unwrap();

        assert_eq!(schema.source_type, "xml");
        assert_eq!(schema.entities.len(), 1);

        let entity = &schema.entities[0];
        assert_eq!(entity.fields.len(), 3);

        // Verify field names (order may vary due to HashSet)
        let field_names: Vec<_> = entity.fields.iter().map(|f| &f.field_name).collect();
        assert!(field_names.contains(&&"id".to_string()));
        assert!(field_names.contains(&&"name".to_string()));
        assert!(field_names.contains(&&"email".to_string()));
    }

    #[test]
    fn test_xml_with_attributes() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"<?xml version="1.0"?>
<root>
  <record id="1">
    <name>Alice</name>
  </record>
</root>"#
        )
        .unwrap();

        let adapter = XmlAdapter::new();
        let schema = adapter.load(file.path()).unwrap();

        let entity = &schema.entities[0];
        let field_names: Vec<_> = entity.fields.iter().map(|f| &f.field_name).collect();
        
        // Should have both the name tag and the id attribute
        assert!(field_names.contains(&&"name".to_string()));
        assert!(field_names.contains(&&"id_attr".to_string()));
    }

    #[test]
    fn test_xml_empty() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, r#"<?xml version="1.0"?><root></root>"#).unwrap();

        let adapter = XmlAdapter::new();
        let result = adapter.load(file.path());
        assert!(result.is_err());
    }
}
