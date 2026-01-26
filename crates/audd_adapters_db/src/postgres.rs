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

        // P1 Fix (H7): Proper error handling for connection task
        // Spawn connection handler with error logging
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                // Log to stderr as fallback - caller should handle connection errors through queries
                eprintln!("PostgreSQL connection handler encountered error: {}", e);
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
        let indexes = self.extract_indexes(table_name).await?;

        Ok(EntitySchema::builder()
            .entity_name(table_name.to_string())
            .entity_type("table")
            .fields(fields)
            .keys(keys)
            .indexes(indexes)
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

    /// Extract keys (primary, unique, and foreign) for a table
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

        // Extract foreign keys
        let foreign_keys = self.extract_foreign_keys(table_name).await?;
        keys.extend(foreign_keys);

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

    /// Extract foreign keys for a table
    async fn extract_foreign_keys(&self, table_name: &str) -> DbResult<Vec<Key>> {
        let query = "
            SELECT
                tc.constraint_name,
                kcu.column_name,
                ccu.table_name AS foreign_table_name,
                ccu.column_name AS foreign_column_name
            FROM information_schema.table_constraints AS tc
            JOIN information_schema.key_column_usage AS kcu
                ON tc.constraint_name = kcu.constraint_name
                AND tc.table_schema = kcu.table_schema
            JOIN information_schema.constraint_column_usage AS ccu
                ON ccu.constraint_name = tc.constraint_name
                AND ccu.table_schema = tc.table_schema
            WHERE tc.constraint_type = 'FOREIGN KEY'
                AND tc.table_schema = 'public'
                AND tc.table_name = $1
        ";

        let rows = self
            .client
            .query(query, &[&table_name])
            .await
            .map_err(|e| {
                DbError::QueryError(format!("Failed to query foreign keys: {}", e))
            })?;

        let mut foreign_keys = Vec::new();

        for row in rows {
            let _constraint_name: String = row.get(0);
            let column_name: String = row.get(1);
            let foreign_table_name: String = row.get(2);
            let foreign_column_name: String = row.get(3);

            let fk = Key::foreign(vec![column_name.clone()])
                .with_metadata("referenced_table".to_string(), Value::String(foreign_table_name))
                .with_metadata("referenced_column".to_string(), Value::String(foreign_column_name))
                .with_metadata("from_column".to_string(), Value::String(column_name));

            foreign_keys.push(fk);
        }

        Ok(foreign_keys)
    }

    /// Extract indexes for a table
    async fn extract_indexes(&self, table_name: &str) -> DbResult<Vec<Index>> {
        let query = "
            SELECT DISTINCT
                i.relname AS index_name,
                ix.indisunique AS is_unique,
                pg_get_expr(ix.indpred, ix.indrelid) AS filter_condition,
                am.amname AS index_type
            FROM pg_class t
            JOIN pg_index ix ON t.oid = ix.indrelid
            JOIN pg_class i ON i.oid = ix.indexrelid
            JOIN pg_am am ON i.relam = am.oid
            WHERE t.relkind = 'r'
                AND t.relname = $1
                AND NOT ix.indisprimary
            ORDER BY i.relname
        ";

        let rows = self
            .client
            .query(query, &[&table_name])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query indexes: {}", e)))?;

        let mut indexes = Vec::new();

        for row in rows {
            let index_name: String = row.get(0);
            let is_unique: bool = row.get(1);
            let filter_condition: Option<String> = row.get(2);
            let index_type_str: String = row.get(3);

            // Get columns for this index
            let columns = self.extract_index_columns(table_name, &index_name).await?;

            if !columns.is_empty() {
                let index_type = if is_unique {
                    IndexType::Unique
                } else {
                    match index_type_str.as_str() {
                        "gin" | "gist" => IndexType::FullText,
                        "brin" => IndexType::Regular,
                        _ => IndexType::Regular,
                    }
                };

                let mut index = Index::new(index_name, index_type, columns);
                if let Some(filter) = filter_condition {
                    index = index.with_filter(filter);
                }

                indexes.push(index);
            }
        }

        Ok(indexes)
    }

    /// Extract column names for an index
    async fn extract_index_columns(&self, table_name: &str, index_name: &str) -> DbResult<Vec<String>> {
        let query = "
            SELECT a.attname
            FROM pg_class t
            JOIN pg_index ix ON t.oid = ix.indrelid
            JOIN pg_class i ON i.oid = ix.indexrelid
            JOIN pg_attribute a ON a.attrelid = t.oid AND a.attnum = ANY(ix.indkey)
            WHERE t.relname = $1
                AND i.relname = $2
            ORDER BY array_position(ix.indkey, a.attnum)
        ";

        let rows = self
            .client
            .query(query, &[&table_name, &index_name])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query index columns: {}", e)))?;

        let columns: Vec<String> = rows.iter().map(|row| row.get(0)).collect();

        Ok(columns)
    }

    /// Extract views from the database
    async fn extract_views(&self) -> DbResult<Vec<View>> {
        // Extract regular views
        let regular_views_query = "
            SELECT
                table_name AS view_name,
                view_definition
            FROM information_schema.views
            WHERE table_schema = 'public'
        ";

        let rows = self
            .client
            .query(regular_views_query, &[])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query views: {}", e)))?;

        let mut views = Vec::new();

        for row in rows {
            let view_name: String = row.get(0);
            let definition: String = row.get(1);

            let view = View::new(view_name).with_definition(definition);
            views.push(view);
        }

        // Extract materialized views
        let mat_views_query = "
            SELECT
                matviewname,
                definition
            FROM pg_matviews
            WHERE schemaname = 'public'
        ";

        let mat_rows = self
            .client
            .query(mat_views_query, &[])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query materialized views: {}", e)))?;

        for row in mat_rows {
            let view_name: String = row.get(0);
            let definition: String = row.get(1);

            let view = View::new(view_name)
                .with_definition(definition)
                .materialized();
            views.push(view);
        }

        Ok(views)
    }

    /// Extract stored procedures and functions
    async fn extract_stored_procedures(&self) -> DbResult<Vec<StoredProcedure>> {
        let query = "
            SELECT
                routine_name,
                routine_type,
                data_type AS return_type,
                routine_definition
            FROM information_schema.routines
            WHERE routine_schema = 'public'
            ORDER BY routine_name
        ";

        let rows = self
            .client
            .query(query, &[])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query routines: {}", e)))?;

        let mut procedures = Vec::new();

        for row in rows {
            let name: String = row.get(0);
            let routine_type: String = row.get(1);
            let return_type: Option<String> = row.get(2);
            let definition: Option<String> = row.get(3);

            let mut proc = StoredProcedure::new(name, routine_type);
            
            if let Some(ret_type) = return_type {
                proc = proc.with_return_type(ret_type);
            }
            
            if let Some(def) = definition {
                proc = proc.with_definition(def);
            }

            procedures.push(proc);
        }

        Ok(procedures)
    }

    /// Extract triggers for a table
    async fn extract_table_triggers(&self, table_name: &str) -> DbResult<Vec<Trigger>> {
        let query = "
            SELECT
                trigger_name,
                event_manipulation AS event,
                action_timing AS timing,
                action_statement AS definition
            FROM information_schema.triggers
            WHERE event_object_table = $1
                AND event_object_schema = 'public'
        ";

        let rows = self
            .client
            .query(query, &[&table_name])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query triggers: {}", e)))?;

        let mut triggers = Vec::new();

        for row in rows {
            let trigger_name: String = row.get(0);
            let event: String = row.get(1);
            let timing: String = row.get(2);
            let definition: String = row.get(3);

            let trigger = Trigger::new(trigger_name, table_name.to_string(), timing, event)
                .with_definition(definition);

            triggers.push(trigger);
        }

        Ok(triggers)
    }

    /// Extract all triggers from the database
    async fn extract_all_triggers(&self) -> DbResult<Vec<Trigger>> {
        let query = "
            SELECT DISTINCT
                trigger_name,
                event_object_table,
                event_manipulation AS event,
                action_timing AS timing,
                action_statement AS definition
            FROM information_schema.triggers
            WHERE event_object_schema = 'public'
            ORDER BY trigger_name
        ";

        let rows = self
            .client
            .query(query, &[])
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to query all triggers: {}", e)))?;

        let mut triggers = Vec::new();

        for row in rows {
            let trigger_name: String = row.get(0);
            let table_name: String = row.get(1);
            let event: String = row.get(2);
            let timing: String = row.get(3);
            let definition: String = row.get(4);

            let trigger = Trigger::new(trigger_name, table_name, timing, event)
                .with_definition(definition);

            triggers.push(trigger);
        }

        Ok(triggers)
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

            // Extract views (regular and materialized)
            let views = self.extract_views().await?;

            // Extract stored procedures and functions
            let stored_procedures = self.extract_stored_procedures().await?;

            // Extract all triggers
            let triggers = self.extract_all_triggers().await?;

            Ok(SourceSchema::builder()
                .source_name(self.database_name.clone())
                .source_type("postgres")
                .entities(entities)
                .views(views)
                .stored_procedures(stored_procedures)
                .triggers(triggers)
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
