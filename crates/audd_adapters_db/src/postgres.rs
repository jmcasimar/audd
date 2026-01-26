//! PostgreSQL database schema connector

#[cfg(feature = "postgres")]
use tokio_postgres::{Client, NoTls};

use audd_ir::{CanonicalType, EntitySchema, FieldSchema, Key, Index, IndexType, View, StoredProcedure, Trigger, SourceSchema};
use crate::connector::DbSchemaConnector;
use crate::error::{DbError, DbResult};
use serde_json::Value;

/// PostgreSQL schema connector
///
/// Extracts schema metadata from PostgreSQL databases using information_schema
/// and pg_catalog.
///
/// # Examples
///
/// ```no_run
/// # #[cfg(feature = "postgres")]
/// # {
/// use audd_adapters_db::postgres::PostgresConnector;
/// use audd_adapters_db::DbSchemaConnector;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let connector = PostgresConnector::new("user:password@localhost:5432/mydb").await?;
/// let schema = connector.load()?;
/// # Ok(())
/// # }
/// # }
/// ```
#[cfg(feature = "postgres")]
pub struct PostgresConnector {
    client: Client,
    database_name: String,
}

#[cfg(feature = "postgres")]
impl PostgresConnector {
    /// Create a new PostgreSQL connector
    ///
    /// # Arguments
    ///
    /// * `conn_str` - Connection string in format: `user:password@host:port/database`
    ///
    /// # Errors
    ///
    /// Returns an error if the connection cannot be established
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

        // Build PostgreSQL connection URL
        let postgres_url = format!("postgresql://{}/{}", credentials_host, database);

        // Connect to PostgreSQL
        let (client, connection) = tokio_postgres::connect(&postgres_url, NoTls)
            .await
            .map_err(|e| {
                DbError::ConnectionError(format!("Failed to connect to PostgreSQL: {}", e))
            })?;

        // Spawn connection handler
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("PostgreSQL connection error: {}", e);
            }
        });

        Ok(Self {
            client,
            database_name,
        })
    }

    /// Get list of all tables in the database
    async fn get_table_names(&self) -> DbResult<Vec<String>> {
        let query = "
            SELECT table_name 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
              AND table_type = 'BASE TABLE'
            ORDER BY table_name
        ";

        let rows = self
            .client
            .query(query, &[])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query tables: {}", e)))?;

        let tables: Vec<String> = rows
            .iter()
            .map(|row| row.get(0))
            .collect();

        Ok(tables)
    }

    /// Extract schema for a specific table
    async fn extract_table_schema(&self, table_name: &str) -> DbResult<EntitySchema> {
        let fields = self.extract_fields(table_name).await?;
        let keys = self.extract_keys(table_name).await?;

        Ok(EntitySchema::builder()
            .entity_name(table_name.to_string())
            .entity_type("table")
            .fields(fields)
            .keys(keys)
            .build())
    }

    /// Extract field schemas for a table
    async fn extract_fields(&self, table_name: &str) -> DbResult<Vec<FieldSchema>> {
        let query = "
            SELECT 
                column_name,
                data_type,
                is_nullable,
                udt_name,
                character_maximum_length,
                numeric_precision,
                numeric_scale,
                column_default
            FROM information_schema.columns
            WHERE table_schema = 'public' 
              AND table_name = $1
            ORDER BY ordinal_position
        ";

        let rows = self
            .client
            .query(query, &[&table_name])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query columns: {}", e)))?;

        let mut fields = Vec::new();

        for row in rows {
            let col_name: String = row.get(0);
            let data_type: String = row.get(1);
            let is_nullable: String = row.get(2);
            let udt_name: String = row.get(3);
            let char_max_length: Option<i32> = row.get(4);
            let numeric_precision: Option<i32> = row.get(5);
            let numeric_scale: Option<i32> = row.get(6);
            let _column_default: Option<String> = row.get(7);

            let canonical_type = map_postgres_type(
                &data_type,
                &udt_name,
                char_max_length,
                numeric_precision,
                numeric_scale,
            );
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
    async fn extract_keys(&self, table_name: &str) -> DbResult<Vec<Key>> {
        let mut keys = Vec::new();

        // Extract primary key
        let pk_fields = self.extract_primary_key(table_name).await?;
        if !pk_fields.is_empty() {
            keys.push(Key::primary(pk_fields));
        }

        // Extract unique constraints
        let unique_keys = self.extract_unique_constraints(table_name).await?;
        keys.extend(unique_keys);

        Ok(keys)
    }

    /// Extract primary key fields
    async fn extract_primary_key(&self, table_name: &str) -> DbResult<Vec<String>> {
        let query = "
            SELECT a.attname
            FROM pg_index i
            JOIN pg_attribute a ON a.attrelid = i.indrelid AND a.attnum = ANY(i.indkey)
            WHERE i.indrelid = $1::regclass
              AND i.indisprimary
            ORDER BY array_position(i.indkey, a.attnum)
        ";

        let rows = self
            .client
            .query(query, &[&table_name])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query primary key: {}", e)))?;

        let pk_fields: Vec<String> = rows
            .iter()
            .map(|row| row.get(0))
            .collect();

        Ok(pk_fields)
    }

    /// Extract unique constraints
    async fn extract_unique_constraints(&self, table_name: &str) -> DbResult<Vec<Key>> {
        let query = "
            SELECT 
                tc.constraint_name,
                array_agg(kcu.column_name ORDER BY kcu.ordinal_position)
            FROM information_schema.table_constraints tc
            JOIN information_schema.key_column_usage kcu 
                ON tc.constraint_name = kcu.constraint_name
                AND tc.table_schema = kcu.table_schema
            WHERE tc.constraint_type = 'UNIQUE'
              AND tc.table_schema = 'public'
              AND tc.table_name = $1
            GROUP BY tc.constraint_name
        ";

        let rows = self
            .client
            .query(query, &[&table_name])
            .await
            .map_err(|e| {
                DbError::QueryError(format!("Failed to query unique constraints: {}", e))
            })?;

        let mut unique_keys = Vec::new();

        for row in rows {
            let columns: Vec<String> = row.get(1);
            if !columns.is_empty() {
                unique_keys.push(Key::unique(columns));
            }
        }

        Ok(unique_keys)
    }
}

#[cfg(feature = "postgres")]
impl DbSchemaConnector for PostgresConnector {
    fn load(&self) -> DbResult<SourceSchema> {
        // Use tokio runtime to execute async operations
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| DbError::Other(format!("Failed to create tokio runtime: {}", e)))?;

        runtime.block_on(async {
            let table_names = self.get_table_names().await?;
            let mut entities = Vec::new();

            for table_name in table_names {
                let entity = self.extract_table_schema(&table_name).await?;
                entities.push(entity);
            }

            Ok(SourceSchema::builder()
                .source_name(self.database_name.clone())
                .source_type("postgres")
                .entities(entities)
                .build())
        })
    }
}

/// Map PostgreSQL type to canonical type
fn map_postgres_type(
    data_type: &str,
    udt_name: &str,
    _char_max_length: Option<i32>,
    numeric_precision: Option<i32>,
    numeric_scale: Option<i32>,
) -> CanonicalType {
    let data_type_lower = data_type.to_lowercase();
    let udt_name_lower = udt_name.to_lowercase();

    match data_type_lower.as_str() {
        // Integer types
        "smallint" => CanonicalType::Int32,
        "integer" => CanonicalType::Int32,
        "bigint" => CanonicalType::Int64,

        // Serial types (auto-increment)
        "smallserial" => CanonicalType::Int32,
        "serial" => CanonicalType::Int32,
        "bigserial" => CanonicalType::Int64,

        // Floating point types
        "real" => CanonicalType::Float32,
        "double precision" => CanonicalType::Float64,

        // Decimal/Numeric types
        "numeric" | "decimal" => {
            let precision = numeric_precision.unwrap_or(10) as u16;
            let scale = numeric_scale.unwrap_or(0) as u16;
            CanonicalType::Decimal { precision, scale }
        }

        // Money type (map to decimal)
        "money" => CanonicalType::Decimal {
            precision: 19,
            scale: 2,
        },

        // String types
        "character varying" | "varchar" => CanonicalType::String,
        "character" | "char" => CanonicalType::String,
        "text" => CanonicalType::Text,

        // Binary types
        "bytea" => CanonicalType::Binary,

        // Boolean type
        "boolean" => CanonicalType::Boolean,

        // Date/Time types
        "date" => CanonicalType::Date,
        "time" | "time without time zone" => CanonicalType::Time,
        "timestamp" | "timestamp without time zone" => CanonicalType::DateTime,
        "timestamp with time zone" | "timestamptz" => CanonicalType::Timestamp,

        // JSON types
        "json" | "jsonb" => CanonicalType::Json,

        // UUID type
        "uuid" => CanonicalType::Uuid,

        // Array types (map to unknown with original type)
        "array" => CanonicalType::Unknown {
            original_type: format!("{}[]", udt_name_lower.trim_start_matches('_')),
        },

        // User-defined types - check udt_name
        "user-defined" => match udt_name_lower.as_str() {
            "citext" => CanonicalType::String,
            _ => CanonicalType::Unknown {
                original_type: udt_name_lower,
            },
        },

        // Geometric, network, and other special types
        "point" | "line" | "lseg" | "box" | "path" | "polygon" | "circle" | "inet" | "cidr"
        | "macaddr" | "macaddr8" | "bit" | "bit varying" | "tsvector" | "tsquery" | "xml"
        | "interval" => CanonicalType::Unknown {
            original_type: data_type_lower,
        },

        // Unknown types
        _ => CanonicalType::Unknown {
            original_type: data_type_lower,
        },
    }
}

// Stub implementation when feature is not enabled
#[cfg(not(feature = "postgres"))]
pub struct PostgresConnector;

#[cfg(not(feature = "postgres"))]
impl PostgresConnector {
    pub async fn new(_conn_str: &str) -> DbResult<Self> {
        Err(DbError::FeatureNotEnabled("postgres".to_string()))
    }
}

#[cfg(all(test, feature = "postgres"))]
mod tests {
    use super::*;

    #[test]
    fn test_postgres_type_mapping() {
        assert_eq!(
            map_postgres_type("integer", "int4", None, None, None),
            CanonicalType::Int32
        );
        assert_eq!(
            map_postgres_type("bigint", "int8", None, None, None),
            CanonicalType::Int64
        );
        assert_eq!(
            map_postgres_type("character varying", "varchar", Some(255), None, None),
            CanonicalType::String
        );
        assert_eq!(
            map_postgres_type("text", "text", None, None, None),
            CanonicalType::Text
        );
        assert_eq!(
            map_postgres_type("boolean", "bool", None, None, None),
            CanonicalType::Boolean
        );
        assert_eq!(
            map_postgres_type("date", "date", None, None, None),
            CanonicalType::Date
        );
        assert_eq!(
            map_postgres_type("timestamp without time zone", "timestamp", None, None, None),
            CanonicalType::DateTime
        );
        assert_eq!(
            map_postgres_type("timestamp with time zone", "timestamptz", None, None, None),
            CanonicalType::Timestamp
        );
        assert_eq!(
            map_postgres_type("jsonb", "jsonb", None, None, None),
            CanonicalType::Json
        );
        assert_eq!(
            map_postgres_type("uuid", "uuid", None, None, None),
            CanonicalType::Uuid
        );
        assert_eq!(
            map_postgres_type("bytea", "bytea", None, None, None),
            CanonicalType::Binary
        );
    }

    #[test]
    fn test_decimal_mapping() {
        let result = map_postgres_type("numeric", "numeric", None, Some(10), Some(2));
        match result {
            CanonicalType::Decimal { precision, scale } => {
                assert_eq!(precision, 10);
                assert_eq!(scale, 2);
            }
            _ => panic!("Expected Decimal type"),
        }
    }

    #[test]
    fn test_array_type() {
        match map_postgres_type("array", "_int4", None, None, None) {
            CanonicalType::Unknown { original_type } => {
                assert_eq!(original_type, "int4[]");
            }
            _ => panic!("Expected Unknown type for array"),
        }
    }
}
