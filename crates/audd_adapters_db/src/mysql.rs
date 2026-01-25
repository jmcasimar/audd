//! MySQL/MariaDB database schema connector

#[cfg(feature = "mysql")]
use mysql::{Pool, PooledConn, prelude::Queryable};

use audd_ir::{CanonicalType, EntitySchema, FieldSchema, Key, SourceSchema};
use crate::connector::DbSchemaConnector;
use crate::error::{DbError, DbResult};

/// MySQL/MariaDB schema connector
///
/// Extracts schema metadata from MySQL/MariaDB databases using INFORMATION_SCHEMA.
///
/// # Examples
///
/// ```no_run
/// # #[cfg(feature = "mysql")]
/// # {
/// use audd_adapters_db::mysql::MysqlConnector;
/// use audd_adapters_db::DbSchemaConnector;
///
/// let connector = MysqlConnector::new("user:password@localhost:3306/mydb").unwrap();
/// let schema = connector.load().unwrap();
/// # }
/// ```
#[cfg(feature = "mysql")]
pub struct MysqlConnector {
    pool: Pool,
    database_name: String,
}

#[cfg(feature = "mysql")]
impl MysqlConnector {
    /// Create a new MySQL connector
    ///
    /// # Arguments
    ///
    /// * `conn_str` - Connection string in format: `user:password@host:port/database`
    ///   Port is optional and defaults to 3306
    ///
    /// # Errors
    ///
    /// Returns an error if the connection cannot be established
    pub fn new(conn_str: &str) -> DbResult<Self> {
        // Parse connection string: user:password@host:port/database
        let (credentials_host, database) = conn_str
            .rsplit_once('/')
            .ok_or_else(|| {
                DbError::InvalidConnectionString(
                    "Missing database name (format: user:pass@host/database)".to_string(),
                )
            })?;

        let database_name = database.to_string();

        // Build MySQL connection URL
        let mysql_url = format!("mysql://{}/{}", credentials_host, database);

        let pool = Pool::new(mysql_url.as_str()).map_err(|e| {
            DbError::ConnectionError(format!("Failed to connect to MySQL: {}", e))
        })?;

        Ok(Self {
            pool,
            database_name,
        })
    }

    /// Get a connection from the pool
    fn get_conn(&self) -> DbResult<PooledConn> {
        self.pool.get_conn().map_err(|e| {
            DbError::ConnectionError(format!("Failed to get connection from pool: {}", e))
        })
    }

    /// Get list of all tables in the database
    fn get_table_names(&self) -> DbResult<Vec<String>> {
        let mut conn = self.get_conn()?;

        let query = format!(
            "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES 
             WHERE TABLE_SCHEMA = '{}' AND TABLE_TYPE = 'BASE TABLE'
             ORDER BY TABLE_NAME",
            self.database_name
        );

        let tables: Vec<String> = conn
            .query_map(query, |table_name: String| table_name)
            .map_err(|e| DbError::QueryError(format!("Failed to query tables: {}", e)))?;

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
        let mut conn = self.get_conn()?;

        let query = format!(
            "SELECT COLUMN_NAME, DATA_TYPE, IS_NULLABLE, COLUMN_TYPE, COLUMN_DEFAULT
             FROM INFORMATION_SCHEMA.COLUMNS
             WHERE TABLE_SCHEMA = '{}' AND TABLE_NAME = '{}'
             ORDER BY ORDINAL_POSITION",
            self.database_name, table_name
        );

        let rows: Vec<(String, String, String, String, Option<String>)> = conn
            .query_map(
                query,
                |(col_name, data_type, is_nullable, column_type, default_value)| {
                    (col_name, data_type, is_nullable, column_type, default_value)
                },
            )
            .map_err(|e| DbError::QueryError(format!("Failed to query columns: {}", e)))?;

        let mut fields = Vec::new();

        for (col_name, data_type, is_nullable, column_type, _default_value) in rows {
            let canonical_type = map_mysql_type(&data_type, &column_type);
            let nullable = is_nullable.to_uppercase() == "YES";

            fields.push(
                FieldSchema::builder()
                    .field_name(col_name)
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
        let mut conn = self.get_conn()?;

        let query = format!(
            "SELECT COLUMN_NAME
             FROM INFORMATION_SCHEMA.KEY_COLUMN_USAGE
             WHERE TABLE_SCHEMA = '{}' 
               AND TABLE_NAME = '{}'
               AND CONSTRAINT_NAME = 'PRIMARY'
             ORDER BY ORDINAL_POSITION",
            self.database_name, table_name
        );

        let pk_fields: Vec<String> = conn
            .query_map(query, |col_name: String| col_name)
            .map_err(|e| DbError::QueryError(format!("Failed to query primary key: {}", e)))?;

        Ok(pk_fields)
    }

    /// Extract unique indexes
    fn extract_unique_indexes(&self, table_name: &str) -> DbResult<Vec<Key>> {
        let mut conn = self.get_conn()?;

        // Get unique index names (excluding primary key)
        let query = format!(
            "SELECT DISTINCT INDEX_NAME
             FROM INFORMATION_SCHEMA.STATISTICS
             WHERE TABLE_SCHEMA = '{}' 
               AND TABLE_NAME = '{}'
               AND NON_UNIQUE = 0
               AND INDEX_NAME != 'PRIMARY'
             ORDER BY INDEX_NAME",
            self.database_name, table_name
        );

        let index_names: Vec<String> = conn
            .query_map(query, |index_name: String| index_name)
            .map_err(|e| DbError::QueryError(format!("Failed to query unique indexes: {}", e)))?;

        let mut unique_keys = Vec::new();

        for index_name in index_names {
            let columns = self.extract_index_columns(table_name, &index_name)?;
            if !columns.is_empty() {
                unique_keys.push(Key::unique(columns));
            }
        }

        Ok(unique_keys)
    }

    /// Extract columns for a specific index
    fn extract_index_columns(&self, table_name: &str, index_name: &str) -> DbResult<Vec<String>> {
        let mut conn = self.get_conn()?;

        let query = format!(
            "SELECT COLUMN_NAME
             FROM INFORMATION_SCHEMA.STATISTICS
             WHERE TABLE_SCHEMA = '{}' 
               AND TABLE_NAME = '{}'
               AND INDEX_NAME = '{}'
             ORDER BY SEQ_IN_INDEX",
            self.database_name, table_name, index_name
        );

        let columns: Vec<String> = conn
            .query_map(query, |col_name: String| col_name)
            .map_err(|e| {
                DbError::QueryError(format!("Failed to query index columns: {}", e))
            })?;

        Ok(columns)
    }
}

#[cfg(feature = "mysql")]
impl DbSchemaConnector for MysqlConnector {
    fn load(&self) -> DbResult<SourceSchema> {
        let table_names = self.get_table_names()?;
        let mut entities = Vec::new();

        for table_name in table_names {
            let entity = self.extract_table_schema(&table_name)?;
            entities.push(entity);
        }

        Ok(SourceSchema::builder()
            .source_name(self.database_name.clone())
            .source_type("mysql")
            .entities(entities)
            .build())
    }
}

/// Map MySQL type to canonical type
///
/// # Arguments
///
/// * `data_type` - The base data type (e.g., "int", "varchar")
/// * `column_type` - The full column type with details (e.g., "int(11)", "varchar(255)")
fn map_mysql_type(data_type: &str, column_type: &str) -> CanonicalType {
    let data_type_lower = data_type.to_lowercase();

    match data_type_lower.as_str() {
        // Integer types
        "tinyint" => {
            // TINYINT(1) is often used for boolean
            if column_type.contains("tinyint(1)") {
                CanonicalType::Boolean
            } else {
                CanonicalType::Int32
            }
        }
        "smallint" => CanonicalType::Int32,
        "mediumint" => CanonicalType::Int32,
        "int" | "integer" => CanonicalType::Int32,
        "bigint" => CanonicalType::Int64,

        // Floating point types
        "float" => CanonicalType::Float32,
        "double" | "real" => CanonicalType::Float64,

        // Decimal/Numeric types
        "decimal" | "numeric" => {
            // Try to extract precision and scale from column_type
            // Format: decimal(10,2)
            if let Some(params) = extract_decimal_params(column_type) {
                params
            } else {
                // Default MySQL decimal
                CanonicalType::Decimal {
                    precision: 10,
                    scale: 0,
                }
            }
        }

        // String types
        "char" | "varchar" => CanonicalType::String,
        "text" | "tinytext" | "mediumtext" | "longtext" => CanonicalType::Text,

        // Binary types
        "binary" | "varbinary" | "blob" | "tinyblob" | "mediumblob" | "longblob" => {
            CanonicalType::Binary
        }

        // Date/Time types
        "date" => CanonicalType::Date,
        "time" => CanonicalType::Time,
        "datetime" => CanonicalType::DateTime,
        "timestamp" => CanonicalType::Timestamp,
        "year" => CanonicalType::Int32,

        // JSON type
        "json" => CanonicalType::Json,

        // Enum and Set (treat as string)
        "enum" | "set" => CanonicalType::String,

        // Spatial types (not fully supported, map to unknown)
        "geometry" | "point" | "linestring" | "polygon" | "multipoint" | "multilinestring"
        | "multipolygon" | "geometrycollection" => CanonicalType::Unknown {
            original_type: data_type.to_string(),
        },

        // Unknown types
        _ => CanonicalType::Unknown {
            original_type: data_type.to_string(),
        },
    }
}

/// Extract precision and scale from decimal/numeric column type
fn extract_decimal_params(column_type: &str) -> Option<CanonicalType> {
    // Expected format: decimal(precision,scale) or decimal(precision)
    let start = column_type.find('(')?;
    let end = column_type.find(')')?;
    let params = &column_type[start + 1..end];

    let parts: Vec<&str> = params.split(',').collect();

    match parts.len() {
        1 => {
            // Only precision specified
            let precision = parts[0].trim().parse::<u16>().ok()?;
            Some(CanonicalType::Decimal {
                precision,
                scale: 0,
            })
        }
        2 => {
            // Both precision and scale specified
            let precision = parts[0].trim().parse::<u16>().ok()?;
            let scale = parts[1].trim().parse::<u16>().ok()?;
            Some(CanonicalType::Decimal { precision, scale })
        }
        _ => None,
    }
}

// Stub implementation when feature is not enabled
#[cfg(not(feature = "mysql"))]
pub struct MysqlConnector;

#[cfg(not(feature = "mysql"))]
impl MysqlConnector {
    pub fn new(_conn_str: &str) -> DbResult<Self> {
        Err(DbError::FeatureNotEnabled("mysql".to_string()))
    }
}

#[cfg(all(test, feature = "mysql"))]
mod tests {
    use super::*;

    #[test]
    fn test_mysql_type_mapping() {
        assert_eq!(map_mysql_type("int", "int(11)"), CanonicalType::Int32);
        assert_eq!(map_mysql_type("bigint", "bigint(20)"), CanonicalType::Int64);
        assert_eq!(map_mysql_type("varchar", "varchar(255)"), CanonicalType::String);
        assert_eq!(map_mysql_type("text", "text"), CanonicalType::Text);
        assert_eq!(map_mysql_type("float", "float"), CanonicalType::Float32);
        assert_eq!(map_mysql_type("double", "double"), CanonicalType::Float64);
        assert_eq!(map_mysql_type("date", "date"), CanonicalType::Date);
        assert_eq!(map_mysql_type("datetime", "datetime"), CanonicalType::DateTime);
        assert_eq!(map_mysql_type("timestamp", "timestamp"), CanonicalType::Timestamp);
        assert_eq!(map_mysql_type("json", "json"), CanonicalType::Json);
        assert_eq!(map_mysql_type("blob", "blob"), CanonicalType::Binary);

        // TINYINT(1) as boolean
        assert_eq!(map_mysql_type("tinyint", "tinyint(1)"), CanonicalType::Boolean);
        // Other TINYINT as Int32
        assert_eq!(map_mysql_type("tinyint", "tinyint(4)"), CanonicalType::Int32);
    }

    #[test]
    fn test_decimal_params_extraction() {
        let result = map_mysql_type("decimal", "decimal(10,2)");
        match result {
            CanonicalType::Decimal { precision, scale } => {
                assert_eq!(precision, 10);
                assert_eq!(scale, 2);
            }
            _ => panic!("Expected Decimal type"),
        }

        let result = map_mysql_type("decimal", "decimal(8)");
        match result {
            CanonicalType::Decimal { precision, scale } => {
                assert_eq!(precision, 8);
                assert_eq!(scale, 0);
            }
            _ => panic!("Expected Decimal type"),
        }
    }

    #[test]
    fn test_unknown_type() {
        match map_mysql_type("geometry", "geometry") {
            CanonicalType::Unknown { original_type } => {
                assert_eq!(original_type, "geometry");
            }
            _ => panic!("Expected Unknown type"),
        }
    }
}
