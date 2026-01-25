//! SQLite database schema connector

#[cfg(feature = "sqlite")]
use rusqlite::{Connection, Result as SqliteResult};

use audd_ir::{CanonicalType, EntitySchema, FieldSchema, Key, SourceSchema};
use crate::connector::DbSchemaConnector;
use crate::error::{DbError, DbResult};

/// SQLite schema connector
///
/// Extracts schema metadata from SQLite databases using PRAGMA commands
/// and sqlite_master system table.
///
/// # Examples
///
/// ```no_run
/// # #[cfg(feature = "sqlite")]
/// # {
/// use audd_adapters_db::sqlite::SqliteConnector;
/// use audd_adapters_db::DbSchemaConnector;
///
/// let connector = SqliteConnector::new("/path/to/database.db").unwrap();
/// let schema = connector.load().unwrap();
/// # }
/// ```
#[cfg(feature = "sqlite")]
pub struct SqliteConnector {
    connection: Connection,
    db_name: String,
}

#[cfg(feature = "sqlite")]
impl SqliteConnector {
    /// Create a new SQLite connector
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the SQLite database file
    ///
    /// # Errors
    ///
    /// Returns an error if the database cannot be opened
    pub fn new(path: &str) -> DbResult<Self> {
        let connection = Connection::open(path).map_err(|e| {
            DbError::ConnectionError(format!("Failed to open SQLite database: {}", e))
        })?;

        // Extract database name from path
        let db_name = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("sqlite_db")
            .to_string();

        Ok(Self {
            connection,
            db_name,
        })
    }

    /// Get list of all user tables (excluding system tables)
    fn get_table_names(&self) -> DbResult<Vec<String>> {
        let mut stmt = self
            .connection
            .prepare(
                "SELECT name FROM sqlite_master 
                 WHERE type='table' AND name NOT LIKE 'sqlite_%'
                 ORDER BY name",
            )
            .map_err(|e| DbError::QueryError(format!("Failed to query tables: {}", e)))?;

        let tables = stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| DbError::QueryError(format!("Failed to fetch tables: {}", e)))?
            .collect::<SqliteResult<Vec<String>>>()
            .map_err(|e| DbError::ExtractionError(format!("Failed to extract tables: {}", e)))?;

        Ok(tables)
    }

    /// Extract schema for a specific table
    fn extract_table_schema(&self, table_name: &str) -> DbResult<EntitySchema> {
        let fields = self.extract_fields(table_name)?;
        let keys = self.extract_keys(table_name)?;

        Ok(EntitySchema::builder()
            .entity_name(table_name.to_string())
            .entity_type("table")
            .fields(fields)
            .keys(keys)
            .build())
    }

    /// Extract field schemas for a table
    fn extract_fields(&self, table_name: &str) -> DbResult<Vec<FieldSchema>> {
        let query = format!("PRAGMA table_info('{}')", table_name);
        let mut stmt = self
            .connection
            .prepare(&query)
            .map_err(|e| DbError::QueryError(format!("Failed to get table info: {}", e)))?;

        let mut fields = Vec::new();

        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,      // cid
                    row.get::<_, String>(1)?,   // name
                    row.get::<_, String>(2)?,   // type
                    row.get::<_, i32>(3)?,      // notnull
                    row.get::<_, Option<String>>(4)?, // dflt_value
                    row.get::<_, i32>(5)?,      // pk
                ))
            })
            .map_err(|e| DbError::QueryError(format!("Failed to query table info: {}", e)))?;

        for row_result in rows {
            let (_cid, name, type_str, notnull, _dflt_value, pk) = row_result
                .map_err(|e| DbError::ExtractionError(format!("Failed to extract field: {}", e)))?;

            let canonical_type = map_sqlite_type(&type_str);
            // In SQLite, PRIMARY KEY columns are implicitly NOT NULL
            let nullable = notnull == 0 && pk == 0;

            fields.push(
                FieldSchema::builder()
                    .field_name(name)
                    .canonical_type(canonical_type)
                    .nullable(nullable)
                    .build(),
            );
        }

        Ok(fields)
    }

    /// Extract keys (primary and unique) for a table
    fn extract_keys(&self, table_name: &str) -> DbResult<Vec<Key>> {
        let mut keys = Vec::new();

        // Extract primary key
        let pk_fields = self.extract_primary_key(table_name)?;
        if !pk_fields.is_empty() {
            keys.push(Key::primary(pk_fields));
        }

        // Extract unique indexes
        let unique_keys = self.extract_unique_indexes(table_name)?;
        keys.extend(unique_keys);

        Ok(keys)
    }

    /// Extract primary key fields
    fn extract_primary_key(&self, table_name: &str) -> DbResult<Vec<String>> {
        let query = format!("PRAGMA table_info('{}')", table_name);
        let mut stmt = self
            .connection
            .prepare(&query)
            .map_err(|e| DbError::QueryError(format!("Failed to get table info: {}", e)))?;

        let mut pk_fields: Vec<(i32, String)> = Vec::new();

        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(1)?,   // name
                    row.get::<_, i32>(5)?,      // pk (position in pk, 0 if not pk)
                ))
            })
            .map_err(|e| DbError::QueryError(format!("Failed to query primary key: {}", e)))?;

        for row_result in rows {
            let (name, pk_pos) = row_result.map_err(|e| {
                DbError::ExtractionError(format!("Failed to extract primary key: {}", e))
            })?;

            if pk_pos > 0 {
                pk_fields.push((pk_pos, name));
            }
        }

        // Sort by pk position to maintain order
        pk_fields.sort_by_key(|(pos, _)| *pos);
        Ok(pk_fields.into_iter().map(|(_, name)| name).collect())
    }

    /// Extract unique indexes
    fn extract_unique_indexes(&self, table_name: &str) -> DbResult<Vec<Key>> {
        let query = format!("PRAGMA index_list('{}')", table_name);
        let mut stmt = self
            .connection
            .prepare(&query)
            .map_err(|e| DbError::QueryError(format!("Failed to get index list: {}", e)))?;

        let mut unique_keys = Vec::new();

        let indexes = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(1)?,   // name
                    row.get::<_, i32>(2)?,      // unique
                ))
            })
            .map_err(|e| DbError::QueryError(format!("Failed to query indexes: {}", e)))?;

        for index_result in indexes {
            let (index_name, is_unique) = index_result.map_err(|e| {
                DbError::ExtractionError(format!("Failed to extract index: {}", e))
            })?;

            // Only process unique indexes (excluding primary key auto-indexes)
            if is_unique == 1 && !index_name.starts_with("sqlite_autoindex_") {
                let index_fields = self.extract_index_columns(table_name, &index_name)?;
                if !index_fields.is_empty() {
                    unique_keys.push(Key::unique(index_fields));
                }
            }
        }

        Ok(unique_keys)
    }

    /// Extract columns for a specific index
    fn extract_index_columns(&self, _table_name: &str, index_name: &str) -> DbResult<Vec<String>> {
        let query = format!("PRAGMA index_info('{}')", index_name);
        let mut stmt = self
            .connection
            .prepare(&query)
            .map_err(|e| DbError::QueryError(format!("Failed to get index info: {}", e)))?;

        let columns = stmt
            .query_map([], |row| row.get::<_, String>(2)) // column name is at index 2
            .map_err(|e| DbError::QueryError(format!("Failed to query index columns: {}", e)))?
            .collect::<SqliteResult<Vec<String>>>()
            .map_err(|e| {
                DbError::ExtractionError(format!("Failed to extract index columns: {}", e))
            })?;

        Ok(columns)
    }
}

#[cfg(feature = "sqlite")]
impl DbSchemaConnector for SqliteConnector {
    fn load(&self) -> DbResult<SourceSchema> {
        let table_names = self.get_table_names()?;
        let mut entities = Vec::new();

        for table_name in table_names {
            let entity = self.extract_table_schema(&table_name)?;
            entities.push(entity);
        }

        Ok(SourceSchema::builder()
            .source_name(self.db_name.clone())
            .source_type("sqlite")
            .entities(entities)
            .build())
    }
}

/// Map SQLite type to canonical type
///
/// SQLite has a flexible type system with type affinity.
/// This function maps declared types to canonical types.
fn map_sqlite_type(sqlite_type: &str) -> CanonicalType {
    let type_upper = sqlite_type.to_uppercase();

    // SQLite type affinity rules
    if type_upper.contains("INT") {
        CanonicalType::Int64
    } else if type_upper.contains("TEXT") || type_upper.contains("CLOB") {
        CanonicalType::Text
    } else if type_upper.contains("CHAR") {
        CanonicalType::String
    } else if type_upper.contains("BLOB") {
        CanonicalType::Binary
    } else if type_upper.contains("REAL") 
        || type_upper.contains("FLOA") 
        || type_upper.contains("DOUB") {
        CanonicalType::Float64
    } else if type_upper.contains("NUMERIC") 
        || type_upper.contains("DECIMAL") {
        // Default decimal precision for SQLite
        CanonicalType::Decimal {
            precision: 10,
            scale: 2,
        }
    } else if type_upper == "DATE" {
        CanonicalType::Date
    } else if type_upper.contains("TIME") {
        CanonicalType::DateTime
    } else if type_upper.contains("BOOL") {
        CanonicalType::Boolean
    } else if sqlite_type.is_empty() {
        // SQLite allows columns without type
        CanonicalType::Unknown {
            original_type: "NONE".to_string(),
        }
    } else {
        CanonicalType::Unknown {
            original_type: sqlite_type.to_string(),
        }
    }
}

// Stub implementation when feature is not enabled
#[cfg(not(feature = "sqlite"))]
pub struct SqliteConnector;

#[cfg(not(feature = "sqlite"))]
impl SqliteConnector {
    pub fn new(_path: &str) -> DbResult<Self> {
        Err(DbError::FeatureNotEnabled("sqlite".to_string()))
    }
}

#[cfg(all(test, feature = "sqlite"))]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_type_mapping() {
        assert_eq!(map_sqlite_type("INTEGER"), CanonicalType::Int64);
        assert_eq!(map_sqlite_type("INT"), CanonicalType::Int64);
        assert_eq!(map_sqlite_type("TEXT"), CanonicalType::Text);
        assert_eq!(map_sqlite_type("VARCHAR(255)"), CanonicalType::String);
        assert_eq!(map_sqlite_type("REAL"), CanonicalType::Float64);
        assert_eq!(map_sqlite_type("BLOB"), CanonicalType::Binary);
        assert_eq!(map_sqlite_type("BOOLEAN"), CanonicalType::Boolean);
        assert_eq!(map_sqlite_type("DATE"), CanonicalType::Date);
        assert_eq!(map_sqlite_type("DATETIME"), CanonicalType::DateTime);
        
        match map_sqlite_type("CUSTOM_TYPE") {
            CanonicalType::Unknown { original_type } => {
                assert_eq!(original_type, "CUSTOM_TYPE");
            }
            _ => panic!("Expected Unknown type"),
        }
    }

    #[test]
    fn test_sqlite_connector_with_test_db() {
        use tempfile::NamedTempFile;

        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();

        // Create a test database
        let conn = Connection::open(db_path).unwrap();
        conn.execute(
            "CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                email VARCHAR(255),
                age INTEGER
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE UNIQUE INDEX idx_username ON users(username)",
            [],
        )
        .unwrap();

        // Test connector
        let connector = SqliteConnector::new(db_path).unwrap();
        let schema = connector.load().unwrap();

        assert_eq!(schema.source_type, "sqlite");
        assert_eq!(schema.entities.len(), 1);

        let users_table = &schema.entities[0];
        assert_eq!(users_table.entity_name, "users");
        assert_eq!(users_table.fields.len(), 4);

        // Check fields
        let id_field = users_table.fields.iter().find(|f| f.field_name == "id").unwrap();
        assert_eq!(id_field.canonical_type, CanonicalType::Int64);
        assert!(!id_field.nullable);

        let username_field = users_table.fields.iter().find(|f| f.field_name == "username").unwrap();
        assert_eq!(username_field.canonical_type, CanonicalType::Text);
        assert!(!username_field.nullable);

        let email_field = users_table.fields.iter().find(|f| f.field_name == "email").unwrap();
        assert_eq!(email_field.canonical_type, CanonicalType::String);
        assert!(email_field.nullable);

        // Check primary key
        assert!(!users_table.keys.is_empty());
        use audd_ir::KeyType;
        let pk = users_table.keys.iter().find(|k| matches!(k.key_type, KeyType::Primary)).unwrap();
        assert_eq!(pk.field_names, vec!["id"]);

        // Check unique index
        let unique_keys: Vec<_> = users_table.keys.iter()
            .filter(|k| matches!(k.key_type, KeyType::Unique))
            .collect();
        assert!(!unique_keys.is_empty());
        assert!(unique_keys.iter().any(|k| k.field_names.contains(&"username".to_string())));
    }

    #[test]
    fn test_sqlite_connector_multiple_tables() {
        use tempfile::NamedTempFile;

        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();

        let conn = Connection::open(db_path).unwrap();
        conn.execute("CREATE TABLE table1 (id INTEGER PRIMARY KEY)", [])
            .unwrap();
        conn.execute("CREATE TABLE table2 (id INTEGER PRIMARY KEY)", [])
            .unwrap();

        let connector = SqliteConnector::new(db_path).unwrap();
        let schema = connector.load().unwrap();

        assert_eq!(schema.entities.len(), 2);
        assert!(schema.entities.iter().any(|e| e.entity_name == "table1"));
        assert!(schema.entities.iter().any(|e| e.entity_name == "table2"));
    }
}
