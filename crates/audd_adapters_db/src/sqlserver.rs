//! Microsoft SQL Server database schema connector

#[cfg(feature = "sqlserver")]
use std::sync::Arc;
#[cfg(feature = "sqlserver")]
use std::sync::Mutex;

use audd_ir::{
    CanonicalType, EntitySchema, FieldSchema, Index, IndexType, Key, KeyType, SourceSchema,
    StoredProcedure, Trigger, View,
};
use crate::connector::DbSchemaConnector;
use crate::error::{DbError, DbResult};

/// Microsoft SQL Server schema connector
///
/// Extracts schema metadata from SQL Server databases using system views.
///
/// # Examples
///
/// ```no_run
/// # #[cfg(feature = "sqlserver")]
/// # {
/// use audd_adapters_db::sqlserver::SqlServerConnector;
/// use audd_adapters_db::DbSchemaConnector;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let connector = SqlServerConnector::new("user:password@localhost:1433/mydb").await?;
/// let schema = connector.load()?;
/// # Ok(())
/// # }
/// # }
/// ```
#[cfg(feature = "sqlserver")]
pub struct SqlServerConnector {
    connection_string: String,
    database_name: String,
}

#[cfg(feature = "sqlserver")]
impl SqlServerConnector {
    /// Create a new SQL Server connector
    ///
    /// # Arguments
    ///
    /// * `conn_str` - Connection string in format: `user:password@host:port/database`
    ///   Port is optional and defaults to 1433
    ///
    /// # Errors
    ///
    /// Returns an error if the connection string cannot be parsed
    pub async fn new(conn_str: &str) -> DbResult<Self> {
        // Parse connection string: user:password@host:port/database
        let (_credentials_host, database) = conn_str
            .rsplit_once('/')
            .ok_or_else(|| {
                DbError::InvalidConnectionString(
                    "Missing database name (format: user:pass@host/database)".to_string(),
                )
            })?;

        let database_name = database.to_string();
        let connection_string = conn_str.to_string();

        Ok(Self {
            connection_string,
            database_name,
        })
    }

    /// Map SQL Server types to canonical types
    fn map_type(sql_type: &str, max_length: Option<i32>, precision: Option<u8>, scale: Option<u8>) -> CanonicalType {
        let sql_type_lower = sql_type.to_lowercase();
        
        match sql_type_lower.as_str() {
            // Integer types
            "bit" => CanonicalType::Boolean,
            "tinyint" => CanonicalType::Int32,
            "smallint" => CanonicalType::Int32,
            "int" => CanonicalType::Int32,
            "bigint" => CanonicalType::Int64,
            
            // Decimal/numeric types
            "decimal" | "numeric" => {
                CanonicalType::Decimal {
                    precision: precision.unwrap_or(18) as u16,
                    scale: scale.unwrap_or(0) as u16,
                }
            }
            "money" | "smallmoney" => CanonicalType::Decimal {
                precision: 19,
                scale: 4,
            },
            
            // Floating point types
            "real" => CanonicalType::Float32,
            "float" => CanonicalType::Float64,
            
            // String types
            "char" | "varchar" | "nchar" | "nvarchar" => {
                if let Some(len) = max_length {
                    if len == -1 { // MAX
                        CanonicalType::Text
                    } else {
                        CanonicalType::String
                    }
                } else {
                    CanonicalType::String
                }
            }
            "text" | "ntext" => CanonicalType::Text,
            
            // Binary types
            "binary" | "varbinary" | "image" => CanonicalType::Binary,
            
            // Date/time types
            "date" => CanonicalType::Date,
            "time" => CanonicalType::Time,
            "datetime" | "datetime2" | "smalldatetime" | "datetimeoffset" => CanonicalType::DateTime,
            
            // UUID type
            "uniqueidentifier" => CanonicalType::Uuid,
            
            // JSON type (SQL Server 2016+)
            "json" => CanonicalType::Json,
            
            // XML type
            "xml" => CanonicalType::Json, // Map XML to JSON for now
            
            // Spatial types
            "geography" | "geometry" => CanonicalType::Unknown { 
                original_type: format!("SQL Server spatial type: {}", sql_type)
            },
            
            // Other types
            _ => CanonicalType::Unknown {
                original_type: format!("SQL Server type: {}", sql_type),
            },
        }
    }
}

#[cfg(feature = "sqlserver")]
impl DbSchemaConnector for SqlServerConnector {
    fn load(&self) -> DbResult<SourceSchema> {
        // For now, return a basic schema structure
        // Full implementation requires working with tiberius async API
        // This provides the foundation for future implementation
        
        let _schema = SourceSchema::builder()
            .source_name(&self.database_name)
            .source_type("sqlserver")
            .build();
        
        // Note: Full implementation would extract:
        // - Tables and columns via sys.tables, sys.columns
        // - Foreign keys via sys.foreign_keys, sys.foreign_key_columns
        // - Indexes via sys.indexes, sys.index_columns
        // - Views via sys.views
        // - Stored procedures via sys.procedures
        // - Triggers via sys.triggers
        
        // For now, return an empty schema to allow compilation
        // Users can implement the full extraction logic using tiberius
        
        Err(DbError::Other(
            "SQL Server connector is a work in progress. Full implementation requires async database queries with tiberius.".to_string()
        ))
    }
}

#[cfg(test)]
#[cfg(feature = "sqlserver")]
mod tests {
    use super::*;

    #[test]
    fn test_type_mapping() {
        // Integer types
        assert_eq!(
            SqlServerConnector::map_type("int", None, None, None),
            CanonicalType::Int32
        );
        assert_eq!(
            SqlServerConnector::map_type("bigint", None, None, None),
            CanonicalType::Int64
        );
        assert_eq!(
            SqlServerConnector::map_type("bit", None, None, None),
            CanonicalType::Boolean
        );

        // Decimal types
        assert_eq!(
            SqlServerConnector::map_type("decimal", None, Some(10), Some(2)),
            CanonicalType::Decimal { precision: 10, scale: 2 }
        );

        // String types
        assert_eq!(
            SqlServerConnector::map_type("varchar", Some(50), None, None),
            CanonicalType::String
        );
        assert_eq!(
            SqlServerConnector::map_type("varchar", Some(-1), None, None),
            CanonicalType::Text
        );
        assert_eq!(
            SqlServerConnector::map_type("ntext", None, None, None),
            CanonicalType::Text
        );

        // Date/time types
        assert_eq!(
            SqlServerConnector::map_type("datetime", None, None, None),
            CanonicalType::DateTime
        );
        assert_eq!(
            SqlServerConnector::map_type("date", None, None, None),
            CanonicalType::Date
        );

        // UUID
        assert_eq!(
            SqlServerConnector::map_type("uniqueidentifier", None, None, None),
            CanonicalType::Uuid
        );

        // Binary
        assert_eq!(
            SqlServerConnector::map_type("varbinary", Some(100), None, None),
            CanonicalType::Binary
        );
    }
    
    #[tokio::test]
    async fn test_connector_creation() {
        let result = SqlServerConnector::new("user:pass@localhost:1433/testdb").await;
        assert!(result.is_ok());
        
        let connector = result.unwrap();
        assert_eq!(connector.database_name, "testdb");
    }
    
    #[tokio::test]
    async fn test_invalid_connection_string() {
        let result = SqlServerConnector::new("invalid").await;
        assert!(result.is_err());
    }
}
