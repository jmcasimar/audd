//! Firebird database schema connector

#[cfg(feature = "firebird")]
use rsfbclient::{Connection, ConnectionBuilder, FbError};
#[cfg(feature = "firebird")]
use std::collections::HashMap;
#[cfg(feature = "firebird")]
use serde_json;

use audd_ir::{
    CanonicalType, EntitySchema, FieldSchema, Index, IndexType, Key, KeyType, SourceSchema,
    StoredProcedure, Trigger, View, Constraint,
};
use crate::connector::DbSchemaConnector;
use crate::error::{DbError, DbResult};

/// Firebird database schema connector
///
/// Extracts schema metadata from Firebird databases using system tables.
///
/// # Examples
///
/// ```no_run
/// # #[cfg(feature = "firebird")]
/// # {
/// use audd_adapters_db::firebird::FirebirdConnector;
/// use audd_adapters_db::DbSchemaConnector;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let connector = FirebirdConnector::new("localhost:/path/to/database.fdb", "SYSDBA", "masterkey")?;
/// let schema = connector.load()?;
/// # Ok(())
/// # }
/// # }
/// ```
#[cfg(feature = "firebird")]
pub struct FirebirdConnector {
    conn: Connection,
    database_name: String,
}

#[cfg(feature = "firebird")]
impl FirebirdConnector {
    /// Create a new Firebird connector
    ///
    /// # Arguments
    ///
    /// * `database_path` - Database path in format: `host:/path/to/database.fdb` or `/path/to/database.fdb`
    /// * `username` - Database username (typically "SYSDBA")
    /// * `password` - Database password
    ///
    /// # Errors
    ///
    /// Returns an error if the connection fails
    pub fn new(database_path: &str, username: &str, password: &str) -> DbResult<Self> {
        // Extract database name from path
        let database_name = database_path
            .rsplit('/')
            .next()
            .unwrap_or("unknown")
            .trim_end_matches(".fdb")
            .trim_end_matches(".FDB")
            .to_string();

        // Connect to Firebird
        let conn = ConnectionBuilder::linked()
            .map_err(|e| DbError::ConnectionError(format!("Failed to create Firebird connection builder: {}", e)))?
            .user(username)
            .pass(password)
            .connect(database_path)
            .map_err(|e| DbError::ConnectionError(format!("Failed to connect to Firebird: {}", e)))?;

        Ok(Self {
            conn,
            database_name,
        })
    }

    /// Extract tables from the database
    fn extract_tables(&self) -> DbResult<Vec<EntitySchema>> {
        let query = r#"
            SELECT TRIM(RDB$RELATION_NAME) as TABLE_NAME
            FROM RDB$RELATIONS
            WHERE RDB$SYSTEM_FLAG = 0
            AND RDB$VIEW_BLR IS NULL
            ORDER BY RDB$RELATION_NAME
        "#;

        let rows = self.conn.query_iter(query, ())
            .map_err(|e| DbError::QueryError(format!("Failed to query tables: {}", e)))?;

        let mut tables = Vec::new();
        for row in rows {
            let row = row.map_err(|e| DbError::QueryError(format!("Failed to fetch row: {}", e)))?;
            let table_name: String = row.get(0)
                .map_err(|e| DbError::QueryError(format!("Failed to get table name: {}", e)))?;

            let fields = self.extract_columns(&table_name)?;
            let keys = self.extract_keys(&table_name)?;
            let indexes = self.extract_indexes(&table_name)?;

            tables.push(EntitySchema {
                entity_name: table_name,
                entity_type: "TABLE".to_string(),
                fields,
                keys,
                indexes,
            });
        }

        Ok(tables)
    }

    /// Extract columns for a specific table
    fn extract_columns(&self, table_name: &str) -> DbResult<Vec<FieldSchema>> {
        let query = r#"
            SELECT 
                TRIM(RF.RDB$FIELD_NAME) as COLUMN_NAME,
                TRIM(F.RDB$FIELD_TYPE) as FIELD_TYPE,
                F.RDB$FIELD_SUB_TYPE as FIELD_SUB_TYPE,
                F.RDB$FIELD_LENGTH as FIELD_LENGTH,
                F.RDB$FIELD_PRECISION as FIELD_PRECISION,
                F.RDB$FIELD_SCALE as FIELD_SCALE,
                RF.RDB$NULL_FLAG as NULL_FLAG,
                RF.RDB$DEFAULT_SOURCE as DEFAULT_VALUE,
                RF.RDB$FIELD_POSITION as ORDINAL_POSITION
            FROM RDB$RELATION_FIELDS RF
            JOIN RDB$FIELDS F ON RF.RDB$FIELD_SOURCE = F.RDB$FIELD_NAME
            WHERE TRIM(RF.RDB$RELATION_NAME) = ?
            ORDER BY RF.RDB$FIELD_POSITION
        "#;

        let rows = self.conn.query_iter(query, (table_name,))
            .map_err(|e| DbError::QueryError(format!("Failed to query columns: {}", e)))?;

        let mut fields = Vec::new();
        for row in rows {
            let row = row.map_err(|e| DbError::QueryError(format!("Failed to fetch row: {}", e)))?;
            
            let column_name: String = row.get(0)
                .map_err(|e| DbError::QueryError(format!("Failed to get column name: {}", e)))?;
            let field_type: i16 = row.get(1)
                .map_err(|e| DbError::QueryError(format!("Failed to get field type: {}", e)))?;
            let field_sub_type: Option<i16> = row.get(2).ok();
            let field_length: Option<i16> = row.get(3).ok();
            let field_precision: Option<i16> = row.get(4).ok();
            let field_scale: Option<i16> = row.get(5).ok();
            let null_flag: Option<i16> = row.get(6).ok();
            let default_value: Option<String> = row.get(7).ok();

            let canonical_type = Self::map_firebird_type(
                field_type,
                field_sub_type,
                field_length,
                field_precision,
                field_scale,
            );

            let nullable = null_flag.is_none();

            fields.push(FieldSchema {
                field_name: column_name,
                canonical_type,
                nullable,
                constraints: Vec::new(),
                metadata: HashMap::new(),
            });
        }

        Ok(fields)
    }

    /// Map Firebird data types to canonical types
    fn map_firebird_type(
        field_type: i16,
        field_sub_type: Option<i16>,
        field_length: Option<i16>,
        field_precision: Option<i16>,
        field_scale: Option<i16>,
    ) -> CanonicalType {
        // Firebird data type codes from ibase.h
        match field_type {
            7 => {
                // SMALLINT
                if field_scale.unwrap_or(0) < 0 {
                    // NUMERIC/DECIMAL
                    CanonicalType::Decimal {
                        precision: field_precision.map(|p| p as u8),
                        scale: field_scale.map(|s| (-s) as u8),
                    }
                } else {
                    CanonicalType::Int32
                }
            }
            8 => {
                // INTEGER
                if field_scale.unwrap_or(0) < 0 {
                    // NUMERIC/DECIMAL
                    CanonicalType::Decimal {
                        precision: field_precision.map(|p| p as u8),
                        scale: field_scale.map(|s| (-s) as u8),
                    }
                } else {
                    CanonicalType::Int32
                }
            }
            9 => CanonicalType::Float64,  // QUAD (deprecated, treated as float)
            10 => CanonicalType::Float32, // FLOAT
            11 => CanonicalType::Float64, // D_FLOAT (deprecated)
            12 => CanonicalType::Date,    // DATE (legacy)
            13 => CanonicalType::Time,    // TIME
            14 => {
                // TEXT (CHAR)
                CanonicalType::String
            }
            16 => {
                // INT64/BIGINT
                if field_scale.unwrap_or(0) < 0 {
                    // NUMERIC/DECIMAL
                    CanonicalType::Decimal {
                        precision: field_precision.map(|p| p as u8),
                        scale: field_scale.map(|s| (-s) as u8),
                    }
                } else {
                    CanonicalType::Int64
                }
            }
            23 => CanonicalType::Boolean, // BOOLEAN (Firebird 3.0+)
            27 => CanonicalType::Float64, // DOUBLE PRECISION
            35 => CanonicalType::DateTime, // TIMESTAMP
            37 => {
                // VARCHAR
                CanonicalType::String
            }
            40 => {
                // CSTRING (null-terminated string)
                CanonicalType::String
            }
            45 => {
                // BLOB_ID
                match field_sub_type {
                    Some(1) => CanonicalType::Text, // TEXT BLOB
                    _ => CanonicalType::Binary,     // BINARY BLOB
                }
            }
            261 => {
                // BLOB
                match field_sub_type {
                    Some(1) => CanonicalType::Text, // TEXT BLOB
                    _ => CanonicalType::Binary,     // BINARY BLOB
                }
            }
            _ => CanonicalType::Unknown {
                type_info: format!("Firebird type code: {}", field_type),
            },
        }
    }

    /// Extract primary and foreign keys for a specific table
    fn extract_keys(&self, table_name: &str) -> DbResult<Vec<Key>> {
        let mut keys = Vec::new();

        // Extract primary key
        let pk_query = r#"
            SELECT 
                TRIM(RC.RDB$CONSTRAINT_NAME) as CONSTRAINT_NAME,
                TRIM(ISE.RDB$FIELD_NAME) as COLUMN_NAME,
                ISE.RDB$FIELD_POSITION as ORDINAL_POSITION
            FROM RDB$RELATION_CONSTRAINTS RC
            JOIN RDB$INDEX_SEGMENTS ISE ON RC.RDB$INDEX_NAME = ISE.RDB$INDEX_NAME
            WHERE TRIM(RC.RDB$RELATION_NAME) = ?
            AND RC.RDB$CONSTRAINT_TYPE = 'PRIMARY KEY'
            ORDER BY ISE.RDB$FIELD_POSITION
        "#;

        let pk_rows = self.conn.query_iter(pk_query, (table_name,))
            .map_err(|e| DbError::QueryError(format!("Failed to query primary keys: {}", e)))?;

        let mut pk_columns = Vec::new();
        let mut pk_name = String::new();
        
        for row in pk_rows {
            let row = row.map_err(|e| DbError::QueryError(format!("Failed to fetch row: {}", e)))?;
            if pk_name.is_empty() {
                pk_name = row.get(0)
                    .map_err(|e| DbError::QueryError(format!("Failed to get constraint name: {}", e)))?;
            }
            let column_name: String = row.get(1)
                .map_err(|e| DbError::QueryError(format!("Failed to get column name: {}", e)))?;
            pk_columns.push(column_name);
        }

        if !pk_columns.is_empty() {
            keys.push(Key {
                key_type: KeyType::Primary,
                key_name: Some(pk_name),
                columns: pk_columns,
                metadata: HashMap::new(),
            });
        }

        // Extract foreign keys
        let fk_query = r#"
            SELECT 
                TRIM(RC.RDB$CONSTRAINT_NAME) as CONSTRAINT_NAME,
                TRIM(ISE.RDB$FIELD_NAME) as COLUMN_NAME,
                TRIM(REF.RDB$CONST_NAME_UQ) as REFERENCED_CONSTRAINT,
                TRIM(RC2.RDB$RELATION_NAME) as REFERENCED_TABLE,
                TRIM(ISE2.RDB$FIELD_NAME) as REFERENCED_COLUMN,
                ISE.RDB$FIELD_POSITION as ORDINAL_POSITION
            FROM RDB$RELATION_CONSTRAINTS RC
            JOIN RDB$INDEX_SEGMENTS ISE ON RC.RDB$INDEX_NAME = ISE.RDB$INDEX_NAME
            JOIN RDB$REF_CONSTRAINTS REF ON RC.RDB$CONSTRAINT_NAME = REF.RDB$CONSTRAINT_NAME
            JOIN RDB$RELATION_CONSTRAINTS RC2 ON REF.RDB$CONST_NAME_UQ = RC2.RDB$CONSTRAINT_NAME
            JOIN RDB$INDEX_SEGMENTS ISE2 ON RC2.RDB$INDEX_NAME = ISE2.RDB$INDEX_NAME
                AND ISE.RDB$FIELD_POSITION = ISE2.RDB$FIELD_POSITION
            WHERE TRIM(RC.RDB$RELATION_NAME) = ?
            AND RC.RDB$CONSTRAINT_TYPE = 'FOREIGN KEY'
            ORDER BY RC.RDB$CONSTRAINT_NAME, ISE.RDB$FIELD_POSITION
        "#;

        let fk_rows = self.conn.query_iter(fk_query, (table_name,))
            .map_err(|e| DbError::QueryError(format!("Failed to query foreign keys: {}", e)))?;

        let mut current_fk: Option<(String, Vec<String>, String, Vec<String>)> = None;

        for row in fk_rows {
            let row = row.map_err(|e| DbError::QueryError(format!("Failed to fetch row: {}", e)))?;
            
            let constraint_name: String = row.get(0)
                .map_err(|e| DbError::QueryError(format!("Failed to get constraint name: {}", e)))?;
            let column_name: String = row.get(1)
                .map_err(|e| DbError::QueryError(format!("Failed to get column name: {}", e)))?;
            let referenced_table: String = row.get(3)
                .map_err(|e| DbError::QueryError(format!("Failed to get referenced table: {}", e)))?;
            let referenced_column: String = row.get(4)
                .map_err(|e| DbError::QueryError(format!("Failed to get referenced column: {}", e)))?;

            match &mut current_fk {
                Some((name, cols, _ref_table, ref_cols)) if name == &constraint_name => {
                    cols.push(column_name);
                    ref_cols.push(referenced_column);
                }
                _ => {
                    // Save previous FK if exists
                    if let Some((name, cols, ref_table, ref_cols)) = current_fk.take() {
                        let mut metadata = HashMap::new();
                        metadata.insert("referenced_table".to_string(), serde_json::Value::String(ref_table));
                        metadata.insert("referenced_columns".to_string(), serde_json::Value::String(ref_cols.join(",")));

                        keys.push(Key {
                            key_type: KeyType::Foreign,
                            key_name: Some(name),
                            columns: cols,
                            metadata,
                        });
                    }
                    
                    current_fk = Some((
                        constraint_name,
                        vec![column_name],
                        referenced_table,
                        vec![referenced_column],
                    ));
                }
            }
        }

        // Save last FK
        if let Some((name, cols, ref_table, ref_cols)) = current_fk {
            let mut metadata = HashMap::new();
            metadata.insert("referenced_table".to_string(), serde_json::Value::String(ref_table));
            metadata.insert("referenced_columns".to_string(), serde_json::Value::String(ref_cols.join(",")));

            keys.push(Key {
                key_type: KeyType::Foreign,
                key_name: Some(name),
                columns: cols,
                metadata,
            });
        }

        // Extract unique constraints
        let uq_query = r#"
            SELECT 
                TRIM(RC.RDB$CONSTRAINT_NAME) as CONSTRAINT_NAME,
                TRIM(ISE.RDB$FIELD_NAME) as COLUMN_NAME,
                ISE.RDB$FIELD_POSITION as ORDINAL_POSITION
            FROM RDB$RELATION_CONSTRAINTS RC
            JOIN RDB$INDEX_SEGMENTS ISE ON RC.RDB$INDEX_NAME = ISE.RDB$INDEX_NAME
            WHERE TRIM(RC.RDB$RELATION_NAME) = ?
            AND RC.RDB$CONSTRAINT_TYPE = 'UNIQUE'
            ORDER BY RC.RDB$CONSTRAINT_NAME, ISE.RDB$FIELD_POSITION
        "#;

        let uq_rows = self.conn.query_iter(uq_query, (table_name,))
            .map_err(|e| DbError::QueryError(format!("Failed to query unique constraints: {}", e)))?;

        let mut current_uq: Option<(String, Vec<String>)> = None;

        for row in uq_rows {
            let row = row.map_err(|e| DbError::QueryError(format!("Failed to fetch row: {}", e)))?;
            
            let constraint_name: String = row.get(0)
                .map_err(|e| DbError::QueryError(format!("Failed to get constraint name: {}", e)))?;
            let column_name: String = row.get(1)
                .map_err(|e| DbError::QueryError(format!("Failed to get column name: {}", e)))?;

            match &mut current_uq {
                Some((name, cols)) if name == &constraint_name => {
                    cols.push(column_name);
                }
                _ => {
                    // Save previous unique constraint if exists
                    if let Some((name, cols)) = current_uq.take() {
                        keys.push(Key {
                            key_type: KeyType::Unique,
                            key_name: Some(name),
                            columns: cols,
                            metadata: HashMap::new(),
                        });
                    }
                    
                    current_uq = Some((constraint_name, vec![column_name]));
                }
            }
        }

        // Save last unique constraint
        if let Some((name, cols)) = current_uq {
            keys.push(Key {
                key_type: KeyType::Unique,
                key_name: Some(name),
                columns: cols,
                metadata: HashMap::new(),
            });
        }

        Ok(keys)
    }

    /// Extract indexes for a specific table
    fn extract_indexes(&self, table_name: &str) -> DbResult<Vec<Index>> {
        let query = r#"
            SELECT 
                TRIM(I.RDB$INDEX_NAME) as INDEX_NAME,
                I.RDB$UNIQUE_FLAG as UNIQUE_FLAG,
                TRIM(ISE.RDB$FIELD_NAME) as COLUMN_NAME,
                ISE.RDB$FIELD_POSITION as ORDINAL_POSITION,
                I.RDB$EXPRESSION_SOURCE as EXPRESSION,
                I.RDB$INDEX_TYPE as INDEX_TYPE
            FROM RDB$INDICES I
            JOIN RDB$INDEX_SEGMENTS ISE ON I.RDB$INDEX_NAME = ISE.RDB$INDEX_NAME
            LEFT JOIN RDB$RELATION_CONSTRAINTS RC ON I.RDB$INDEX_NAME = RC.RDB$INDEX_NAME
            WHERE TRIM(I.RDB$RELATION_NAME) = ?
            AND RC.RDB$CONSTRAINT_TYPE IS NULL
            ORDER BY I.RDB$INDEX_NAME, ISE.RDB$FIELD_POSITION
        "#;

        let rows = self.conn.query_iter(query, (table_name,))
            .map_err(|e| DbError::QueryError(format!("Failed to query indexes: {}", e)))?;

        let mut indexes_map: HashMap<String, (bool, Vec<String>, Option<String>)> = HashMap::new();

        for row in rows {
            let row = row.map_err(|e| DbError::QueryError(format!("Failed to fetch row: {}", e)))?;
            
            let index_name: String = row.get(0)
                .map_err(|e| DbError::QueryError(format!("Failed to get index name: {}", e)))?;
            let unique_flag: Option<i16> = row.get(1).ok();
            let column_name: String = row.get(2)
                .map_err(|e| DbError::QueryError(format!("Failed to get column name: {}", e)))?;
            let expression: Option<String> = row.get(4).ok();

            let is_unique = unique_flag == Some(1);

            indexes_map
                .entry(index_name.clone())
                .or_insert((is_unique, Vec::new(), expression))
                .1
                .push(column_name);
        }

        let mut indexes = Vec::new();
        for (name, (is_unique, columns, expression)) in indexes_map {
            let index_type = if is_unique {
                IndexType::Unique
            } else {
                IndexType::Regular
            };

            indexes.push(Index {
                index_name: name,
                index_type,
                columns,
                filter: expression,
            });
        }

        Ok(indexes)
    }

    /// Extract views from the database
    fn extract_views(&self) -> DbResult<Vec<View>> {
        let query = r#"
            SELECT 
                TRIM(RDB$RELATION_NAME) as VIEW_NAME,
                RDB$VIEW_SOURCE as VIEW_DEFINITION
            FROM RDB$RELATIONS
            WHERE RDB$SYSTEM_FLAG = 0
            AND RDB$VIEW_BLR IS NOT NULL
            ORDER BY RDB$RELATION_NAME
        "#;

        let rows = self.conn.query_iter(query, ())
            .map_err(|e| DbError::QueryError(format!("Failed to query views: {}", e)))?;

        let mut views = Vec::new();
        for row in rows {
            let row = row.map_err(|e| DbError::QueryError(format!("Failed to fetch row: {}", e)))?;
            
            let view_name: String = row.get(0)
                .map_err(|e| DbError::QueryError(format!("Failed to get view name: {}", e)))?;
            let definition: Option<String> = row.get(1).ok();

            views.push(View {
                view_name,
                definition,
                is_materialized: false, // Firebird doesn't have materialized views
                field_names: Vec::new(),
                metadata: HashMap::new(),
            });
        }

        Ok(views)
    }

    /// Extract stored procedures from the database
    fn extract_stored_procedures(&self) -> DbResult<Vec<StoredProcedure>> {
        let query = r#"
            SELECT 
                TRIM(RDB$PROCEDURE_NAME) as PROCEDURE_NAME,
                RDB$PROCEDURE_SOURCE as PROCEDURE_SOURCE
            FROM RDB$PROCEDURES
            WHERE RDB$SYSTEM_FLAG = 0
            ORDER BY RDB$PROCEDURE_NAME
        "#;

        let rows = self.conn.query_iter(query, ())
            .map_err(|e| DbError::QueryError(format!("Failed to query stored procedures: {}", e)))?;

        let mut procedures = Vec::new();
        for row in rows {
            let row = row.map_err(|e| DbError::QueryError(format!("Failed to fetch row: {}", e)))?;
            
            let procedure_name: String = row.get(0)
                .map_err(|e| DbError::QueryError(format!("Failed to get procedure name: {}", e)))?;
            let source: Option<String> = row.get(1).ok();

            procedures.push(StoredProcedure {
                name: procedure_name,
                procedure_type: "PROCEDURE".to_string(),
                return_type: None,
                parameters: Vec::new(),
                definition: source,
                metadata: HashMap::new(),
            });
        }

        Ok(procedures)
    }

    /// Extract triggers from the database
    fn extract_triggers(&self) -> DbResult<Vec<Trigger>> {
        let query = r#"
            SELECT 
                TRIM(T.RDB$TRIGGER_NAME) as TRIGGER_NAME,
                TRIM(T.RDB$RELATION_NAME) as TABLE_NAME,
                T.RDB$TRIGGER_TYPE as TRIGGER_TYPE,
                T.RDB$TRIGGER_SOURCE as TRIGGER_SOURCE
            FROM RDB$TRIGGERS T
            WHERE T.RDB$SYSTEM_FLAG = 0
            AND T.RDB$RELATION_NAME IS NOT NULL
            ORDER BY T.RDB$TRIGGER_NAME
        "#;

        let rows = self.conn.query_iter(query, ())
            .map_err(|e| DbError::QueryError(format!("Failed to query triggers: {}", e)))?;

        let mut triggers = Vec::new();
        for row in rows {
            let row = row.map_err(|e| DbError::QueryError(format!("Failed to fetch row: {}", e)))?;
            
            let trigger_name: String = row.get(0)
                .map_err(|e| DbError::QueryError(format!("Failed to get trigger name: {}", e)))?;
            let table_name: String = row.get(1)
                .map_err(|e| DbError::QueryError(format!("Failed to get table name: {}", e)))?;
            let trigger_type: i16 = row.get(2)
                .map_err(|e| DbError::QueryError(format!("Failed to get trigger type: {}", e)))?;
            let source: Option<String> = row.get(3).ok();

            // Decode trigger type (combination of timing and event)
            // Firebird trigger type codes:
            // 1 = BEFORE INSERT, 2 = AFTER INSERT
            // 3 = BEFORE UPDATE, 4 = AFTER UPDATE
            // 5 = BEFORE DELETE, 6 = AFTER DELETE
            // 17 = BEFORE INSERT OR UPDATE, etc.
            let (timing, event) = match trigger_type {
                1 | 3 | 5 | 17 | 25 | 27 | 113 => ("BEFORE", Self::decode_trigger_event(trigger_type)),
                2 | 4 | 6 | 18 | 26 | 28 | 114 => ("AFTER", Self::decode_trigger_event(trigger_type)),
                _ => ("UNKNOWN", "UNKNOWN"),
            };

            triggers.push(Trigger {
                trigger_name,
                table_name,
                timing: timing.to_string(),
                event: event.to_string(),
                definition: source,
                metadata: HashMap::new(),
            });
        }

        Ok(triggers)
    }

    /// Decode Firebird trigger event from trigger type code
    fn decode_trigger_event(trigger_type: i16) -> &'static str {
        match trigger_type {
            1 | 2 => "INSERT",
            3 | 4 => "UPDATE",
            5 | 6 => "DELETE",
            17 | 18 => "INSERT OR UPDATE",
            25 | 26 => "INSERT OR DELETE",
            27 | 28 => "UPDATE OR DELETE",
            113 | 114 => "INSERT OR UPDATE OR DELETE",
            _ => "UNKNOWN",
        }
    }
}

#[cfg(feature = "firebird")]
impl DbSchemaConnector for FirebirdConnector {
    fn load(&self) -> DbResult<SourceSchema> {
        let tables = self.extract_tables()?;
        let views = self.extract_views()?;
        let stored_procedures = self.extract_stored_procedures()?;
        let triggers = self.extract_triggers()?;

        Ok(SourceSchema {
            source_name: self.database_name.clone(),
            source_type: "firebird".to_string(),
            ir_version: audd_ir::IR_VERSION.to_string(),
            entities: tables,
            views,
            stored_procedures,
            triggers,
            metadata: HashMap::new(),
        })
    }
}

#[cfg(not(feature = "firebird"))]
pub struct FirebirdConnector;

#[cfg(not(feature = "firebird"))]
impl FirebirdConnector {
    pub fn new(_database_path: &str, _username: &str, _password: &str) -> DbResult<Self> {
        Err(DbError::ConnectionError(
            "Firebird support not enabled. Enable the 'firebird' feature flag.".to_string(),
        ))
    }
}

#[cfg(not(feature = "firebird"))]
impl DbSchemaConnector for FirebirdConnector {
    fn load(&self) -> DbResult<SourceSchema> {
        Err(DbError::ConnectionError(
            "Firebird support not enabled.".to_string(),
        ))
    }
}

#[cfg(all(test, feature = "firebird"))]
mod tests {
    use super::*;

    #[test]
    fn test_firebird_type_mapping() {
        // Test SMALLINT
        assert!(matches!(
            FirebirdConnector::map_firebird_type(7, None, None, None, Some(0)),
            CanonicalType::Int32
        ));

        // Test INTEGER
        assert!(matches!(
            FirebirdConnector::map_firebird_type(8, None, None, None, Some(0)),
            CanonicalType::Int32
        ));

        // Test BIGINT
        assert!(matches!(
            FirebirdConnector::map_firebird_type(16, None, None, None, Some(0)),
            CanonicalType::Int64
        ));

        // Test DECIMAL
        assert!(matches!(
            FirebirdConnector::map_firebird_type(16, None, None, Some(18), Some(-2)),
            CanonicalType::Decimal { .. }
        ));

        // Test BOOLEAN
        assert!(matches!(
            FirebirdConnector::map_firebird_type(23, None, None, None, None),
            CanonicalType::Boolean
        ));

        // Test FLOAT
        assert!(matches!(
            FirebirdConnector::map_firebird_type(10, None, None, None, None),
            CanonicalType::Float32
        ));

        // Test DOUBLE PRECISION
        assert!(matches!(
            FirebirdConnector::map_firebird_type(27, None, None, None, None),
            CanonicalType::Float64
        ));

        // Test VARCHAR
        assert!(matches!(
            FirebirdConnector::map_firebird_type(37, None, Some(255), None, None),
            CanonicalType::String
        ));

        // Test TEXT BLOB
        assert!(matches!(
            FirebirdConnector::map_firebird_type(261, Some(1), None, None, None),
            CanonicalType::Text
        ));

        // Test BINARY BLOB
        assert!(matches!(
            FirebirdConnector::map_firebird_type(261, Some(0), None, None, None),
            CanonicalType::Binary
        ));

        // Test DATE
        assert!(matches!(
            FirebirdConnector::map_firebird_type(12, None, None, None, None),
            CanonicalType::Date
        ));

        // Test TIME
        assert!(matches!(
            FirebirdConnector::map_firebird_type(13, None, None, None, None),
            CanonicalType::Time
        ));

        // Test TIMESTAMP
        assert!(matches!(
            FirebirdConnector::map_firebird_type(35, None, None, None, None),
            CanonicalType::DateTime
        ));
    }

    #[test]
    fn test_decode_trigger_event() {
        assert_eq!(FirebirdConnector::decode_trigger_event(1), "INSERT");
        assert_eq!(FirebirdConnector::decode_trigger_event(3), "UPDATE");
        assert_eq!(FirebirdConnector::decode_trigger_event(5), "DELETE");
        assert_eq!(FirebirdConnector::decode_trigger_event(17), "INSERT OR UPDATE");
        assert_eq!(FirebirdConnector::decode_trigger_event(113), "INSERT OR UPDATE OR DELETE");
    }
}
