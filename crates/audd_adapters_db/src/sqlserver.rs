//! Microsoft SQL Server database schema connector

#[cfg(feature = "sqlserver")]
use tiberius::{Client, Config, AuthMethod};
#[cfg(feature = "sqlserver")]
use tokio::net::TcpStream;
#[cfg(feature = "sqlserver")]
use tokio_util::compat::{TokioAsyncWriteCompatExt, Compat};
#[cfg(feature = "sqlserver")]
use std::collections::HashMap;

use audd_ir::{
    CanonicalType, EntitySchema, FieldSchema, Index, IndexType, Key, KeyType, SourceSchema,
    StoredProcedure, Trigger, View, Constraint,
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
    client: Client<Compat<TcpStream>>,
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
    /// Returns an error if the connection string cannot be parsed or connection fails
    pub async fn new(conn_str: &str) -> DbResult<Self> {
        // Parse connection string: user:password@host:port/database
        let (credentials_host, database) = conn_str
            .rsplit_once('/')
            .ok_or_else(|| {
                DbError::InvalidConnectionString(
                    "Missing database name (format: user:pass@host:port/database)".to_string(),
                )
            })?;

        let database_name = database.to_string();

        // Parse credentials and host
        let (credentials, host_port) = credentials_host
            .split_once('@')
            .ok_or_else(|| {
                DbError::InvalidConnectionString(
                    "Missing credentials (format: user:pass@host:port/database)".to_string(),
                )
            })?;

        let (username, password) = credentials
            .split_once(':')
            .ok_or_else(|| {
                DbError::InvalidConnectionString(
                    "Missing password (format: user:pass@host:port/database)".to_string(),
                )
            })?;

        // Parse host and port
        let (host, port) = if let Some((h, p)) = host_port.split_once(':') {
            // P1 Fix (H1): Proper port validation instead of silent fallback
            let port = p.parse::<u16>().map_err(|_|
                DbError::InvalidConnectionString(format!("Invalid port number: '{}'. Port must be a number between 1 and 65535.", p))
            )?;
            (h.to_string(), port)
        } else {
            (host_port.to_string(), 1433)
        };

        // Build SQL Server config
        let mut config = Config::new();
        config.host(&host);
        config.port(port);
        config.database(&database_name);
        config.authentication(AuthMethod::sql_server(username, password));
        // Enable TLS encryption for production use
        config.encryption(tiberius::EncryptionLevel::Required);

        // Connect to SQL Server
        let tcp = TcpStream::connect(config.get_addr())
            .await
            .map_err(|e| {
                DbError::ConnectionError(format!("Failed to connect to SQL Server: {}", e))
            })?;

        let client = Client::connect(config, tcp.compat_write())
            .await
            .map_err(|e| {
                DbError::ConnectionError(format!("Failed to authenticate with SQL Server: {}", e))
            })?;

        Ok(Self {
            client,
            database_name,
        })
    }

    /// Get list of all tables in the database
    async fn get_table_names(&mut self) -> DbResult<Vec<String>> {
        let query = "
            SELECT TABLE_NAME
            FROM INFORMATION_SCHEMA.TABLES
            WHERE TABLE_TYPE = 'BASE TABLE'
              AND TABLE_SCHEMA = 'dbo'
            ORDER BY TABLE_NAME
        ";

        let stream = self
            .client
            .query(query, &[])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query tables: {}", e)))?;

        let rows = stream
            .into_first_result()
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to fetch tables: {}", e)))?;

        let tables: Vec<String> = rows
            .iter()
            .filter_map(|row| row.get::<&str, _>(0).map(|s| s.to_string()))
            .collect();

        Ok(tables)
    }

    /// Get columns for a specific table
    async fn get_table_columns(&mut self, table_name: &str) -> DbResult<Vec<FieldSchema>> {
        let query = "
            SELECT 
                c.COLUMN_NAME,
                c.DATA_TYPE,
                c.IS_NULLABLE,
                c.CHARACTER_MAXIMUM_LENGTH,
                c.NUMERIC_PRECISION,
                c.NUMERIC_SCALE,
                c.COLUMN_DEFAULT
            FROM INFORMATION_SCHEMA.COLUMNS c
            WHERE c.TABLE_NAME = @P1
              AND c.TABLE_SCHEMA = 'dbo'
            ORDER BY c.ORDINAL_POSITION
        ";

        let stream = self
            .client
            .query(query, &[&table_name])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query columns: {}", e)))?;

        let rows = stream
            .into_first_result()
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to fetch columns: {}", e)))?;

        let mut fields = Vec::new();
        for row in rows.iter() {
            // P1 Fix (H3): Proper error handling instead of silent data corruption
            let column_name: &str = row.try_get(0)
                .map_err(|e| DbError::QueryError(format!("Failed to read column name: {}", e)))?;
            let data_type: &str = row.try_get(1)
                .map_err(|e| DbError::QueryError(format!("Failed to read data type for column {}: {}", column_name, e)))?;
            let is_nullable: &str = row.try_get(2)
                .map_err(|e| DbError::QueryError(format!("Failed to read nullable flag for column {}: {}", column_name, e)))?;
            let max_length: Option<i32> = row.get(3);
            let precision: Option<u8> = row.get(4);
            let scale: Option<u8> = row.get(5);
            let default_value: Option<&str> = row.get(6);

            let canonical_type = Self::map_type(data_type, max_length, precision, scale);
            let nullable = is_nullable == "YES";

            let mut constraints = Vec::new();
            if let Some(def) = default_value {
                constraints.push(Constraint::DefaultValue {
                    value: serde_json::Value::String(def.to_string()),
                });
            }

            fields.push(FieldSchema {
                field_name: column_name.to_string(),
                canonical_type,
                nullable,
                constraints,
                metadata: HashMap::new(),
            });
        }

        Ok(fields)
    }

    /// Get primary keys for a table
    async fn get_primary_keys(&mut self, table_name: &str) -> DbResult<Vec<Key>> {
        let query = "
            SELECT 
                kc.COLUMN_NAME,
                kc.ORDINAL_POSITION
            FROM INFORMATION_SCHEMA.TABLE_CONSTRAINTS tc
            JOIN INFORMATION_SCHEMA.KEY_COLUMN_USAGE kc
                ON tc.CONSTRAINT_NAME = kc.CONSTRAINT_NAME
                AND tc.TABLE_SCHEMA = kc.TABLE_SCHEMA
            WHERE tc.TABLE_NAME = @P1
              AND tc.TABLE_SCHEMA = 'dbo'
              AND tc.CONSTRAINT_TYPE = 'PRIMARY KEY'
            ORDER BY kc.ORDINAL_POSITION
        ";

        let stream = self
            .client
            .query(query, &[&table_name])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query primary keys: {}", e)))?;

        let rows = stream
            .into_first_result()
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to fetch primary keys: {}", e)))?;

        if rows.is_empty() {
            return Ok(Vec::new());
        }

        let field_names: Vec<String> = rows
            .iter()
            .filter_map(|row| row.get::<&str, _>(0).map(|s| s.to_string()))
            .collect();

        if !field_names.is_empty() {
            Ok(vec![Key {
                key_type: KeyType::Primary,
                field_names,
                metadata: HashMap::new(),
            }])
        } else {
            Ok(Vec::new())
        }
    }

    /// Get foreign keys for a table
    async fn get_foreign_keys(&mut self, table_name: &str) -> DbResult<Vec<Key>> {
        let query = "
            SELECT 
                fk.name AS FK_NAME,
                COL_NAME(fkc.parent_object_id, fkc.parent_column_id) AS COLUMN_NAME,
                OBJECT_NAME(fkc.referenced_object_id) AS REFERENCED_TABLE,
                COL_NAME(fkc.referenced_object_id, fkc.referenced_column_id) AS REFERENCED_COLUMN
            FROM sys.foreign_keys fk
            JOIN sys.foreign_key_columns fkc
                ON fk.object_id = fkc.constraint_object_id
            WHERE OBJECT_NAME(fk.parent_object_id) = @P1
            ORDER BY fk.name, fkc.constraint_column_id
        ";

        let stream = self
            .client
            .query(query, &[&table_name])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query foreign keys: {}", e)))?;

        let rows = stream
            .into_first_result()
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to fetch foreign keys: {}", e)))?;

        let mut fk_map: HashMap<String, (Vec<String>, String, String)> = HashMap::new();

        for row in rows.iter() {
            // P1 Fix (H3): Proper error handling instead of silent data corruption
            let fk_name: &str = row.try_get(0)
                .map_err(|e| DbError::QueryError(format!("Failed to read foreign key name for table {}: {}", table_name, e)))?;
            let column_name: &str = row.try_get(1)
                .map_err(|e| DbError::QueryError(format!("Failed to read column name for FK {}: {}", fk_name, e)))?;
            let ref_table: &str = row.try_get(2)
                .map_err(|e| DbError::QueryError(format!("Failed to read referenced table for FK {}: {}", fk_name, e)))?;
            let ref_column: &str = row.try_get(3)
                .map_err(|e| DbError::QueryError(format!("Failed to read referenced column for FK {}: {}", fk_name, e)))?;

            fk_map
                .entry(fk_name.to_string())
                .or_insert_with(|| (Vec::new(), ref_table.to_string(), ref_column.to_string()))
                .0
                .push(column_name.to_string());
        }

        let mut foreign_keys = Vec::new();
        for (_fk_name, (field_names, ref_table, ref_column)) in fk_map {
            let mut metadata = HashMap::new();
            metadata.insert("referenced_table".to_string(), serde_json::Value::String(ref_table));
            metadata.insert("referenced_column".to_string(), serde_json::Value::String(ref_column));

            foreign_keys.push(Key {
                key_type: KeyType::Foreign,
                field_names,
                metadata,
            });
        }

        Ok(foreign_keys)
    }

    /// Get indexes for a table
    async fn get_indexes(&mut self, table_name: &str) -> DbResult<Vec<Index>> {
        let query = "
            SELECT 
                i.name AS INDEX_NAME,
                i.is_unique,
                i.type_desc,
                i.filter_definition,
                COL_NAME(ic.object_id, ic.column_id) AS COLUMN_NAME
            FROM sys.indexes i
            JOIN sys.index_columns ic
                ON i.object_id = ic.object_id
                AND i.index_id = ic.index_id
            WHERE OBJECT_NAME(i.object_id) = @P1
              AND i.is_primary_key = 0
              AND i.is_unique_constraint = 0
            ORDER BY i.name, ic.key_ordinal
        ";

        let stream = self
            .client
            .query(query, &[&table_name])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query indexes: {}", e)))?;

        let rows = stream
            .into_first_result()
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to fetch indexes: {}", e)))?;

        let mut index_map: HashMap<String, (bool, String, Option<String>, Vec<String>)> = HashMap::new();

        for row in rows.iter() {
            // P1 Fix (H3): Proper error handling instead of silent data corruption
            let index_name: &str = row.try_get(0)
                .map_err(|e| DbError::QueryError(format!("Failed to read index name for table {}: {}", table_name, e)))?;
            let is_unique: bool = row.try_get(1)
                .map_err(|e| DbError::QueryError(format!("Failed to read unique flag for index {}: {}", index_name, e)))?;
            let type_desc: &str = row.try_get(2)
                .map_err(|e| DbError::QueryError(format!("Failed to read type for index {}: {}", index_name, e)))?;
            let filter_def: Option<&str> = row.get(3);
            let column_name: &str = row.try_get(4)
                .map_err(|e| DbError::QueryError(format!("Failed to read column name for index {}: {}", index_name, e)))?;

            index_map
                .entry(index_name.to_string())
                .or_insert_with(|| {
                    (
                        is_unique,
                        type_desc.to_string(),
                        filter_def.map(|s| s.to_string()),
                        Vec::new(),
                    )
                })
                .3
                .push(column_name.to_string());
        }

        let mut indexes = Vec::new();
        for (index_name, (is_unique, type_desc, filter_def, field_names)) in index_map {
            let index_type = if is_unique {
                IndexType::Unique
            } else if type_desc.contains("FULLTEXT") {
                IndexType::FullText
            } else if type_desc.contains("SPATIAL") {
                IndexType::Spatial
            } else {
                IndexType::Regular
            };

            indexes.push(Index {
                index_name,
                index_type,
                field_names,
                filter_condition: filter_def,
                metadata: HashMap::new(),
            });
        }

        Ok(indexes)
    }

    /// Get views from the database
    async fn get_views(&mut self) -> DbResult<Vec<View>> {
        let query = "
            SELECT 
                TABLE_NAME,
                VIEW_DEFINITION
            FROM INFORMATION_SCHEMA.VIEWS
            WHERE TABLE_SCHEMA = 'dbo'
            ORDER BY TABLE_NAME
        ";

        let stream = self
            .client
            .query(query, &[])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query views: {}", e)))?;

        let rows = stream
            .into_first_result()
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to fetch views: {}", e)))?;

        let mut views = Vec::new();
        for row in rows.iter() {
            let view_name: &str = row.get(0).unwrap_or("");
            let definition: Option<&str> = row.get(1);

            views.push(View {
                view_name: view_name.to_string(),
                definition: definition.map(|s| s.to_string()),
                is_materialized: false, // SQL Server doesn't have materialized views like PostgreSQL
                field_names: Vec::new(),
                metadata: HashMap::new(),
            });
        }

        Ok(views)
    }

    /// Get stored procedures from the database
    async fn get_stored_procedures(&mut self) -> DbResult<Vec<StoredProcedure>> {
        let query = "
            SELECT 
                ROUTINE_NAME,
                ROUTINE_TYPE,
                DATA_TYPE,
                ROUTINE_DEFINITION
            FROM INFORMATION_SCHEMA.ROUTINES
            WHERE ROUTINE_SCHEMA = 'dbo'
            ORDER BY ROUTINE_NAME
        ";

        let stream = self
            .client
            .query(query, &[])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query procedures: {}", e)))?;

        let rows = stream
            .into_first_result()
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to fetch procedures: {}", e)))?;

        let mut procedures = Vec::new();
        for row in rows.iter() {
            let name: &str = row.get(0).unwrap_or("");
            let routine_type: &str = row.get(1).unwrap_or("");
            let return_type: Option<&str> = row.get(2);
            let definition: Option<&str> = row.get(3);

            procedures.push(StoredProcedure {
                name: name.to_string(),
                procedure_type: routine_type.to_string(),
                parameters: Vec::new(), // TODO: Extract parameters from sys.parameters
                return_type: return_type.map(|s| s.to_string()),
                definition: definition.map(|s| s.to_string()),
                metadata: HashMap::new(),
            });
        }

        Ok(procedures)
    }

    /// Get triggers from the database
    async fn get_triggers(&mut self) -> DbResult<Vec<Trigger>> {
        let query = "
            SELECT 
                t.name AS TRIGGER_NAME,
                OBJECT_NAME(t.parent_id) AS TABLE_NAME,
                CASE 
                    WHEN OBJECTPROPERTY(t.object_id, 'ExecIsInsertTrigger') = 1 THEN 'INSERT'
                    WHEN OBJECTPROPERTY(t.object_id, 'ExecIsUpdateTrigger') = 1 THEN 'UPDATE'
                    WHEN OBJECTPROPERTY(t.object_id, 'ExecIsDeleteTrigger') = 1 THEN 'DELETE'
                END AS EVENT,
                CASE 
                    WHEN OBJECTPROPERTY(t.object_id, 'ExecIsAfterTrigger') = 1 THEN 'AFTER'
                    WHEN OBJECTPROPERTY(t.object_id, 'ExecIsInsteadOfTrigger') = 1 THEN 'INSTEAD OF'
                    ELSE 'AFTER'
                END AS TIMING,
                OBJECT_DEFINITION(t.object_id) AS DEFINITION
            FROM sys.triggers t
            WHERE t.is_ms_shipped = 0
            ORDER BY t.name
        ";

        let stream = self
            .client
            .query(query, &[])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query triggers: {}", e)))?;

        let rows = stream
            .into_first_result()
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to fetch triggers: {}", e)))?;

        let mut triggers = Vec::new();
        for row in rows.iter() {
            let trigger_name: &str = row.get(0).unwrap_or("");
            let table_name: &str = row.get(1).unwrap_or("");
            let event: &str = row.get(2).unwrap_or("");
            let timing: &str = row.get(3).unwrap_or("");
            let definition: Option<&str> = row.get(4);

            triggers.push(Trigger {
                trigger_name: trigger_name.to_string(),
                table_name: table_name.to_string(),
                timing: timing.to_string(),
                event: event.to_string(),
                definition: definition.map(|s| s.to_string()),
                metadata: HashMap::new(),
            });
        }

        Ok(triggers)
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
        crate::runtime::block_on(async {
            // We need mutable access to client for queries
            // Since load() takes &self, we need to create a new connection
            // This is a limitation of the current design
            // In a real implementation, you might want to redesign the trait to support async
            
            Err(DbError::Other(
                "SQL Server connector requires async support. Please use the async methods directly or redesign the trait.".to_string()
            ))
        })
    }
}

/// Async implementation for SQL Server
#[cfg(feature = "sqlserver")]
impl SqlServerConnector {
    /// Load schema asynchronously
    pub async fn load_async(&mut self) -> DbResult<SourceSchema> {
        let table_names = self.get_table_names().await?;
        let mut entities = Vec::new();

        for table_name in &table_names {
            let fields = self.get_table_columns(table_name).await?;
            let mut keys = self.get_primary_keys(table_name).await?;
            keys.extend(self.get_foreign_keys(table_name).await?);
            let indexes = self.get_indexes(table_name).await?;

            entities.push(EntitySchema {
                entity_name: table_name.clone(),
                entity_type: "table".to_string(),
                fields,
                keys,
                indexes,
                metadata: HashMap::new(),
            });
        }

        let views = self.get_views().await?;
        let stored_procedures = self.get_stored_procedures().await?;
        let triggers = self.get_triggers().await?;

        Ok(SourceSchema::builder()
            .source_name(&self.database_name)
            .source_type("sqlserver")
            .entities(entities)
            .views(views)
            .stored_procedures(stored_procedures)
            .triggers(triggers)
            .build())
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
    
    // Note: Connection tests would require an actual SQL Server instance
    // These are commented out but show the expected usage
    
    // #[tokio::test]
    // async fn test_connector_creation() {
    //     let result = SqlServerConnector::new("sa:YourPassword@localhost:1433/testdb").await;
    //     assert!(result.is_ok());
    //     
    //     let connector = result.unwrap();
    //     assert_eq!(connector.database_name, "testdb");
    // }
    
    // #[tokio::test]
    // async fn test_load_async() {
    //     let mut connector = SqlServerConnector::new("sa:YourPassword@localhost:1433/testdb").await.unwrap();
    //     let schema = connector.load_async().await.unwrap();
    //     assert_eq!(schema.source_type, "sqlserver");
    //     assert_eq!(schema.source_name, "testdb");
    // }
    
    #[test]
    fn test_invalid_connection_string() {
        // Test with tokio runtime
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let result = SqlServerConnector::new("invalid").await;
            assert!(result.is_err());
        });
    }
}
