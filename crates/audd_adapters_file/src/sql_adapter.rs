//! SQL/DDL adapter implementation

use crate::adapter::SchemaAdapter;
use crate::error::{AdapterError, AdapterResult};
use audd_ir::{CanonicalType, Constraint, EntitySchema, FieldSchema, Key, SourceSchema};
use std::fs;
use std::path::Path;

/// Adapter for SQL DDL files
///
/// Supports a minimal subset of CREATE TABLE:
/// - Column definitions with types
/// - PRIMARY KEY constraint
/// - NOT NULL constraint
/// - UNIQUE constraint
///
/// Example:
/// ```sql
/// CREATE TABLE users (
///     id INT PRIMARY KEY,
///     name VARCHAR(255) NOT NULL,
///     email VARCHAR(255) UNIQUE
/// );
/// ```
pub struct SqlAdapter;

impl SqlAdapter {
    /// Create a new SQL adapter
    pub fn new() -> Self {
        SqlAdapter
    }

    /// Parse a simple CREATE TABLE statement
    fn parse_create_table(sql: &str) -> Result<(String, Vec<FieldSchema>, Vec<Key>), String> {
        let sql = sql.trim();

        // Extract table name
        let table_name = Self::extract_table_name(sql)?;

        // Extract column definitions
        let columns_section = Self::extract_columns_section(sql)?;
        let (fields, keys) = Self::parse_columns(&columns_section)?;

        Ok((table_name, fields, keys))
    }

    fn extract_table_name(sql: &str) -> Result<String, String> {
        let upper = sql.to_uppercase();
        if let Some(start) = upper.find("CREATE TABLE") {
            let after_create = &sql[start + 12..].trim_start();
            
            // Handle optional IF NOT EXISTS
            let after_if = if after_create.to_uppercase().starts_with("IF NOT EXISTS") {
                after_create[13..].trim_start()
            } else {
                after_create
            };
            
            // Find table name (before opening parenthesis)
            if let Some(paren_pos) = after_if.find('(') {
                let name = after_if[..paren_pos].trim();
                // Remove quotes if present
                let name = name.trim_matches('`').trim_matches('"').trim_matches('\'');
                return Ok(name.to_string());
            }
        }
        Err("Invalid CREATE TABLE syntax".to_string())
    }

    fn extract_columns_section(sql: &str) -> Result<String, String> {
        if let Some(start) = sql.find('(') {
            if let Some(end) = sql.rfind(')') {
                if end > start {
                    return Ok(sql[start + 1..end].to_string());
                }
            }
        }
        Err("Could not find column definitions".to_string())
    }

    fn parse_columns(columns: &str) -> Result<(Vec<FieldSchema>, Vec<Key>), String> {
        let mut fields = Vec::new();
        let mut primary_keys = Vec::new();

        // Split by comma, but be careful with nested parentheses
        let column_defs = Self::split_column_definitions(columns);

        for def in column_defs {
            let def = def.trim();
            
            // Check if this is a PRIMARY KEY constraint
            if def.to_uppercase().starts_with("PRIMARY KEY") {
                // Extract column names from PRIMARY KEY (col1, col2)
                if let Some(start) = def.find('(') {
                    if let Some(end) = def.find(')') {
                        let cols = def[start + 1..end].to_string();
                        primary_keys.extend(cols.split(',').map(|s| s.trim().to_string()));
                    }
                }
                continue;
            }

            // Skip CONSTRAINT clauses
            if def.to_uppercase().starts_with("CONSTRAINT") {
                continue;
            }

            // Parse column definition
            if let Some(field) = Self::parse_column_definition(def)? {
                if field.0 {
                    // This column is a primary key
                    primary_keys.push(field.1.field_name.clone());
                }
                fields.push(field.1);
            }
        }

        let mut keys = Vec::new();
        if !primary_keys.is_empty() {
            keys.push(Key::primary(primary_keys));
        }

        Ok((fields, keys))
    }

    fn split_column_definitions(columns: &str) -> Vec<String> {
        let mut defs = Vec::new();
        let mut current = String::new();
        let mut paren_depth = 0;

        for ch in columns.chars() {
            match ch {
                '(' => {
                    paren_depth += 1;
                    current.push(ch);
                }
                ')' => {
                    paren_depth -= 1;
                    current.push(ch);
                }
                ',' if paren_depth == 0 => {
                    if !current.trim().is_empty() {
                        defs.push(current.trim().to_string());
                    }
                    current.clear();
                }
                _ => current.push(ch),
            }
        }

        if !current.trim().is_empty() {
            defs.push(current.trim().to_string());
        }

        defs
    }

    fn parse_column_definition(def: &str) -> Result<Option<(bool, FieldSchema)>, String> {
        let parts: Vec<&str> = def.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(None);
        }

        let field_name = parts[0].trim_matches('`').trim_matches('"').to_string();
        
        if parts.len() < 2 {
            return Err(format!("Invalid column definition: {}", def));
        }

        let type_str = parts[1];
        let canonical_type = Self::map_sql_type(type_str);

        let def_upper = def.to_uppercase();
        let is_primary = def_upper.contains("PRIMARY KEY");
        let nullable = !def_upper.contains("NOT NULL") && !is_primary;

        let mut builder = FieldSchema::builder()
            .field_name(field_name)
            .canonical_type(canonical_type)
            .nullable(nullable);

        if def_upper.contains("UNIQUE") {
            builder = builder.add_constraint(Constraint::unique());
        }

        Ok(Some((is_primary, builder.build())))
    }

    fn map_sql_type(sql_type: &str) -> CanonicalType {
        let sql_type = sql_type.to_uppercase();
        let base_type = sql_type.split('(').next().unwrap_or(&sql_type);

        match base_type {
            "INT" | "INTEGER" | "SMALLINT" | "MEDIUMINT" => CanonicalType::Int32,
            "BIGINT" | "LONG" => CanonicalType::Int64,
            "FLOAT" | "REAL" => CanonicalType::Float32,
            "DOUBLE" | "DOUBLE PRECISION" => CanonicalType::Float64,
            "DECIMAL" | "NUMERIC" => CanonicalType::Decimal {
                precision: 10,
                scale: 2,
            },
            "BOOLEAN" | "BOOL" => CanonicalType::Boolean,
            "CHAR" | "VARCHAR" | "TEXT" | "NVARCHAR" | "NCHAR" => CanonicalType::String,
            "CLOB" | "LONGTEXT" | "MEDIUMTEXT" => CanonicalType::Text,
            "BLOB" | "BINARY" | "VARBINARY" => CanonicalType::Binary,
            "DATE" => CanonicalType::Date,
            "TIME" => CanonicalType::Time,
            "DATETIME" | "TIMESTAMP" => CanonicalType::Timestamp,
            "JSON" => CanonicalType::Json,
            "UUID" => CanonicalType::Uuid,
            _ => CanonicalType::Unknown {
                original_type: sql_type.clone(),
            },
        }
    }
}

impl Default for SqlAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaAdapter for SqlAdapter {
    fn load(&self, path: &Path) -> AdapterResult<SourceSchema> {
        let content = fs::read_to_string(path)?;

        // Support multiple CREATE TABLE statements
        let mut entities = Vec::new();

        for statement in content.split(';') {
            let statement = statement.trim();
            if statement.to_uppercase().contains("CREATE TABLE") {
                match Self::parse_create_table(statement) {
                    Ok((table_name, fields, keys)) => {
                        let entity = EntitySchema::builder()
                            .entity_name(table_name)
                            .entity_type("table")
                            .fields(fields)
                            .keys(keys)
                            .build();
                        entities.push(entity);
                    }
                    Err(e) => return Err(AdapterError::SqlError(e)),
                }
            }
        }

        if entities.is_empty() {
            return Err(AdapterError::EmptyData(
                "No CREATE TABLE statements found".to_string(),
            ));
        }

        // Derive source name from filename
        let source_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("sql_source")
            .to_string();

        Ok(SourceSchema::builder()
            .source_name(source_name)
            .source_type("sql")
            .entities(entities)
            .build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_sql_basic() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
CREATE TABLE users (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255)
);"#
        )
        .unwrap();

        let adapter = SqlAdapter::new();
        let schema = adapter.load(file.path()).unwrap();

        assert_eq!(schema.source_type, "sql");
        assert_eq!(schema.entities.len(), 1);

        let entity = &schema.entities[0];
        assert_eq!(entity.entity_name, "users");
        assert_eq!(entity.fields.len(), 3);

        let id_field = entity.fields.iter().find(|f| f.field_name == "id").unwrap();
        assert_eq!(id_field.canonical_type, CanonicalType::Int32);
        assert!(!id_field.nullable);

        let name_field = entity
            .fields
            .iter()
            .find(|f| f.field_name == "name")
            .unwrap();
        assert!(!name_field.nullable);

        assert_eq!(entity.keys.len(), 1);
    }

    #[test]
    fn test_sql_multiple_tables() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
CREATE TABLE users (
    id INT PRIMARY KEY,
    name VARCHAR(255)
);

CREATE TABLE posts (
    id INT PRIMARY KEY,
    title TEXT
);"#
        )
        .unwrap();

        let adapter = SqlAdapter::new();
        let schema = adapter.load(file.path()).unwrap();

        assert_eq!(schema.entities.len(), 2);
        assert_eq!(schema.entities[0].entity_name, "users");
        assert_eq!(schema.entities[1].entity_name, "posts");
    }

    #[test]
    fn test_sql_type_mapping() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
CREATE TABLE types_test (
    int_col INT,
    bigint_col BIGINT,
    float_col FLOAT,
    double_col DOUBLE,
    bool_col BOOLEAN,
    text_col TEXT,
    date_col DATE,
    timestamp_col TIMESTAMP
);"#
        )
        .unwrap();

        let adapter = SqlAdapter::new();
        let schema = adapter.load(file.path()).unwrap();
        let entity = &schema.entities[0];

        assert_eq!(
            entity
                .fields
                .iter()
                .find(|f| f.field_name == "int_col")
                .unwrap()
                .canonical_type,
            CanonicalType::Int32
        );
        assert_eq!(
            entity
                .fields
                .iter()
                .find(|f| f.field_name == "bigint_col")
                .unwrap()
                .canonical_type,
            CanonicalType::Int64
        );
        assert_eq!(
            entity
                .fields
                .iter()
                .find(|f| f.field_name == "bool_col")
                .unwrap()
                .canonical_type,
            CanonicalType::Boolean
        );
    }
}
