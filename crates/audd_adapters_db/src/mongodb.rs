//! MongoDB database schema connector

#[cfg(feature = "mongodb")]
use mongodb::{Client, options::ClientOptions};
#[cfg(feature = "mongodb")]
use mongodb::bson::{Bson, Document, doc};

use audd_ir::{CanonicalType, EntitySchema, FieldSchema, Key, SourceSchema, Index, IndexType, View};
use crate::connector::DbSchemaConnector;
use crate::error::{DbError, DbResult};

#[cfg(feature = "mongodb")]
use std::collections::{HashMap, HashSet};

/// MongoDB schema connector
///
/// Extracts schema metadata from MongoDB databases through schema inference
/// by sampling documents from collections.
///
/// # Examples
///
/// ```no_run
/// # #[cfg(feature = "mongodb")]
/// # {
/// use audd_adapters_db::mongodb::MongoDbConnector;
/// use audd_adapters_db::DbSchemaConnector;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let connector = MongoDbConnector::new("mongodb://localhost:27017/mydb").await?;
/// let schema = connector.load()?;
/// # Ok(())
/// # }
/// # }
/// ```
#[cfg(feature = "mongodb")]
pub struct MongoDbConnector {
    client: Client,
    database_name: String,
    sample_size: usize,
}

#[cfg(feature = "mongodb")]
impl MongoDbConnector {
    /// Create a new MongoDB connector
    ///
    /// # Arguments
    ///
    /// * `conn_str` - Full MongoDB connection string (e.g., `mongodb://localhost:27017/mydb`)
    ///
    /// # Errors
    ///
    /// Returns an error if the connection cannot be established
    pub async fn new(conn_str: &str) -> DbResult<Self> {
        Self::new_with_sample_size(conn_str, 100).await
    }

    /// Create a new MongoDB connector with custom sample size
    ///
    /// # Arguments
    ///
    /// * `conn_str` - Full MongoDB connection string
    /// * `sample_size` - Number of documents to sample for schema inference
    pub async fn new_with_sample_size(conn_str: &str, sample_size: usize) -> DbResult<Self> {
        // P1 Fix (H4): Validate sample size to prevent OOM and invalid inputs
        const MIN_SAMPLE_SIZE: usize = 1;
        const MAX_SAMPLE_SIZE: usize = 10000;
        
        if sample_size < MIN_SAMPLE_SIZE || sample_size > MAX_SAMPLE_SIZE {
            return Err(DbError::InvalidConnectionString(format!(
                "Sample size must be between {} and {} (got {})",
                MIN_SAMPLE_SIZE, MAX_SAMPLE_SIZE, sample_size
            )));
        }
        
        // Parse the connection string to get the database name
        let full_uri = if conn_str.starts_with("mongodb://") || conn_str.starts_with("mongodb+srv://") {
            conn_str.to_string()
        } else {
            format!("mongodb://{}", conn_str)
        };

        // Extract database name from connection string
        let database_name = extract_database_name(&full_uri)?;

        // Parse MongoDB connection options
        let client_options = ClientOptions::parse(&full_uri)
            .await
            .map_err(|e| {
                DbError::InvalidConnectionString(format!("Failed to parse MongoDB URI: {}", e))
            })?;

        // Create MongoDB client
        let client = Client::with_options(client_options)
            .map_err(|e| {
                DbError::ConnectionError(format!("Failed to create MongoDB client: {}", e))
            })?;

        Ok(Self {
            client,
            database_name,
            sample_size,
        })
    }

    /// Get list of all collections in the database
    async fn get_collection_names(&self) -> DbResult<Vec<String>> {
        let db = self.client.database(&self.database_name);
        
        let collections = db
            .list_collection_names()
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to list collections: {}", e)))?;

        Ok(collections)
    }

    /// Extract schema for a specific collection by sampling documents
    async fn extract_collection_schema(&self, collection_name: &str) -> DbResult<EntitySchema> {
        let db = self.client.database(&self.database_name);
        let collection = db.collection::<Document>(collection_name);

        // Sample documents from the collection
        let mut cursor = collection
            .find(doc! {})
            .limit(self.sample_size as i64)
            .await
            .map_err(|e| {
                DbError::QueryError(format!("Failed to query collection {}: {}", collection_name, e))
            })?;

        // Collect schema information from sampled documents
        let mut field_schemas: HashMap<String, FieldTypeInfo> = HashMap::new();
        let mut has_id = false;

        // Process documents
        loop {
            match cursor.advance().await {
                Ok(true) => {
                    // We have a document
                    let doc = cursor.deserialize_current()
                        .map_err(|e| DbError::ExtractionError(format!("Failed to deserialize document: {}", e)))?;
                    
                    // Check if _id field exists
                    if doc.contains_key("_id") {
                        has_id = true;
                    }

                    for (field_name, value) in doc {
                        let entry = field_schemas
                            .entry(field_name.to_string())
                            .or_insert_with(|| FieldTypeInfo {
                                types_seen: HashSet::new(),
                                null_count: 0,
                                total_count: 0,
                            });

                        entry.total_count += 1;

                        if value.as_null().is_some() {
                            entry.null_count += 1;
                        } else {
                            entry.types_seen.insert(infer_bson_type(&value));
                        }
                    }
                }
                Ok(false) => {
                    // No more documents
                    break;
                }
                Err(e) => {
                    return Err(DbError::ExtractionError(format!("Failed to read document: {}", e)));
                }
            }
        }

        // Convert field type info to field schemas
        let fields: Vec<FieldSchema> = field_schemas
            .into_iter()
            .map(|(field_name, type_info)| {
                let canonical_type = type_info.get_canonical_type();
                let nullable = type_info.null_count > 0 || type_info.total_count < self.sample_size;

                FieldSchema::builder()
                    .field_name(field_name)
                    .canonical_type(canonical_type)
                    .nullable(nullable)
                    .build()
            })
            .collect();

        // MongoDB collections typically have _id as primary key
        let keys = if has_id {
            vec![Key::primary(vec!["_id"])]
        } else {
            vec![]
        };

        // Extract indexes for this collection
        let indexes = self.extract_collection_indexes(collection_name).await?;

        Ok(EntitySchema::builder()
            .entity_name(collection_name.to_string())
            .entity_type("collection")
            .fields(fields)
            .keys(keys)
            .indexes(indexes)
            .build())
    }

    /// Extract indexes for a specific collection
    async fn extract_collection_indexes(&self, collection_name: &str) -> DbResult<Vec<Index>> {
        use mongodb::IndexModel;
        use futures::stream::TryStreamExt;

        let db = self.client.database(&self.database_name);
        let collection = db.collection::<Document>(collection_name);

        // List indexes using the listIndexes command
        let cursor = collection
            .list_indexes()
            .await
            .map_err(|e| {
                DbError::QueryError(format!("Failed to list indexes for collection {}: {}", collection_name, e))
            })?;

        let mut indexes = Vec::new();

        // Collect all index models
        let index_models: Vec<IndexModel> = cursor
            .try_collect()
            .await
            .map_err(|e| DbError::ExtractionError(format!("Failed to collect indexes: {}", e)))?;

        // Process each index
        for index_model in index_models {
            // Get the index options as BSON Document
            let index_doc = mongodb::bson::to_document(&index_model)
                .map_err(|e| DbError::ExtractionError(format!("Failed to convert index to document: {}", e)))?;

            // Skip the _id index (it's the primary key)
            if let Some(name) = index_doc.get_str("name").ok() {
                if name == "_id_" {
                    continue;
                }
            }

            // Parse index information
            if let Some(index) = self.parse_index_document(&index_doc)? {
                indexes.push(index);
            }
        }

        Ok(indexes)
    }

    /// Parse a MongoDB index document into an Index structure
    fn parse_index_document(&self, index_doc: &Document) -> DbResult<Option<Index>> {
        let index_name = index_doc
            .get_str("name")
            .map_err(|_| DbError::ExtractionError("Index missing name field".to_string()))?
            .to_string();

        // Get the key specification
        let key_doc = index_doc
            .get_document("key")
            .map_err(|_| DbError::ExtractionError(format!("Index {} missing key field", index_name)))?;

        // Extract column names from key
        let field_names: Vec<String> = key_doc.keys().map(|k| k.to_string()).collect();

        if field_names.is_empty() {
            return Ok(None);
        }

        // Determine index type
        let index_type = if let Ok(text_index_version) = index_doc.get_i32("textIndexVersion") {
            // Text index
            if text_index_version > 0 {
                IndexType::FullText
            } else {
                IndexType::Regular
            }
        } else if let Ok(sphere_version) = index_doc.get_i32("2dsphereIndexVersion") {
            // 2dsphere (spatial) index
            if sphere_version > 0 {
                IndexType::Spatial
            } else {
                IndexType::Regular
            }
        } else {
            // Check if it's a unique index
            let is_unique = index_doc.get_bool("unique").unwrap_or(false);
            if is_unique {
                IndexType::Unique
            } else {
                // Check for hashed index
                if let Some(Bson::String(ref val)) = key_doc.values().next() {
                    if val == "hashed" {
                        IndexType::Regular // Hashed indexes are just regular indexes
                    } else {
                        IndexType::Regular
                    }
                } else {
                    IndexType::Regular
                }
            }
        };

        // Check for partial filter expression
        let filter_condition = index_doc
            .get_document("partialFilterExpression")
            .ok()
            .map(|doc| format!("{:?}", doc));

        Ok(Some(Index {
            index_name,
            index_type,
            field_names,
            filter_condition,
            metadata: std::collections::HashMap::new(),
        }))
    }

    /// Extract views from the database
    async fn extract_views(&self) -> DbResult<Vec<View>> {
        use futures::stream::TryStreamExt;

        let db = self.client.database(&self.database_name);

        // List collections and filter for views
        let cursor = db
            .list_collections()
            .await
            .map_err(|e| DbError::QueryError(format!("Failed to list collections: {}", e)))?;

        let mut views = Vec::new();

        // Collect all collection specifications
        let coll_specs: Vec<mongodb::results::CollectionSpecification> = cursor
            .try_collect()
            .await
            .map_err(|e| DbError::ExtractionError(format!("Failed to collect collections: {}", e)))?;

        // Process each collection specification
        for coll_spec in coll_specs {
            // Check if this is a view based on collection_type field
            // CollectionType::View is the enum variant for views
            if matches!(coll_spec.collection_type, mongodb::results::CollectionType::View) {
                let view_name = coll_spec.name.clone();

                // Extract view options if available
                // options is a CreateCollectionOptions struct, not an Option
                let definition = {
                    // Convert options to document to access pipeline
                    mongodb::bson::to_document(&coll_spec.options)
                        .ok()
                        .and_then(|doc| {
                            doc.get_array("pipeline")
                                .ok()
                                .map(|pipeline| format!("{:?}", pipeline))
                        })
                };

                views.push(View {
                    view_name,
                    definition,
                    field_names: Vec::new(), // We don't infer fields for views
                    is_materialized: false, // MongoDB doesn't have materialized views
                    metadata: std::collections::HashMap::new(),
                });
            }
        }

        Ok(views)
    }
}

#[cfg(feature = "mongodb")]
impl DbSchemaConnector for MongoDbConnector {
    fn load(&self) -> DbResult<SourceSchema> {
        // Use tokio runtime to execute async operations
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| DbError::Other(format!("Failed to create tokio runtime: {}", e)))?;

        runtime.block_on(async {
            let collection_names = self.get_collection_names().await?;
            let mut entities = Vec::new();

            for collection_name in collection_names {
                let entity = self.extract_collection_schema(&collection_name).await?;
                entities.push(entity);
            }

            // Extract views
            let views = self.extract_views().await?;

            Ok(SourceSchema::builder()
                .source_name(self.database_name.clone())
                .source_type("mongodb")
                .entities(entities)
                .views(views)
                .build())
        })
    }
}

/// Helper struct to track field type information during sampling
#[cfg(feature = "mongodb")]
struct FieldTypeInfo {
    types_seen: HashSet<String>,
    null_count: usize,
    total_count: usize,
}

#[cfg(feature = "mongodb")]
impl FieldTypeInfo {
    /// Determine the canonical type based on sampled values
    fn get_canonical_type(&self) -> CanonicalType {
        // If multiple types seen, return Unknown
        if self.types_seen.len() > 1 {
            return CanonicalType::Unknown {
                original_type: format!("mixed: {}", self.types_seen.iter().cloned().collect::<Vec<_>>().join(", ")),
            };
        }

        // Get the single type seen
        if let Some(type_name) = self.types_seen.iter().next() {
            map_bson_type_to_canonical(type_name)
        } else {
            // All values were null
            CanonicalType::Unknown {
                original_type: "null".to_string(),
            }
        }
    }
}

/// Infer BSON type from a value
#[cfg(feature = "mongodb")]
fn infer_bson_type(value: &Bson) -> String {
    match value {
        Bson::Double(_) => "double".to_string(),
        Bson::String(_) => "string".to_string(),
        Bson::Array(_) => "array".to_string(),
        Bson::Document(_) => "object".to_string(),
        Bson::Boolean(_) => "boolean".to_string(),
        Bson::Null => "null".to_string(),
        Bson::RegularExpression(_) => "regex".to_string(),
        Bson::JavaScriptCode(_) => "javascript".to_string(),
        Bson::JavaScriptCodeWithScope(_) => "javascript_with_scope".to_string(),
        Bson::Int32(_) => "int32".to_string(),
        Bson::Int64(_) => "int64".to_string(),
        Bson::Timestamp(_) => "timestamp".to_string(),
        Bson::Binary(_) => "binary".to_string(),
        Bson::ObjectId(_) => "objectid".to_string(),
        Bson::DateTime(_) => "datetime".to_string(),
        Bson::Symbol(_) => "symbol".to_string(),
        Bson::Decimal128(_) => "decimal128".to_string(),
        Bson::Undefined => "undefined".to_string(),
        Bson::MaxKey => "maxkey".to_string(),
        Bson::MinKey => "minkey".to_string(),
        Bson::DbPointer(_) => "dbpointer".to_string(),
    }
}

/// Map BSON type name to canonical type
#[cfg(feature = "mongodb")]
fn map_bson_type_to_canonical(type_name: &str) -> CanonicalType {
    match type_name {
        "boolean" => CanonicalType::Boolean,
        "int32" => CanonicalType::Int32,
        "int64" => CanonicalType::Int64,
        "double" => CanonicalType::Float64,
        "decimal128" => CanonicalType::Decimal {
            precision: 34,
            scale: 0,
        },
        "string" => CanonicalType::String,
        "binary" => CanonicalType::Binary,
        "datetime" => CanonicalType::DateTime,
        "timestamp" => CanonicalType::Timestamp,
        "objectid" => CanonicalType::String, // ObjectId as string
        "object" | "array" => CanonicalType::Json, // Nested documents/arrays as JSON
        _ => CanonicalType::Unknown {
            original_type: type_name.to_string(),
        },
    }
}

/// Extract database name from MongoDB connection string
#[cfg(feature = "mongodb")]
fn extract_database_name(conn_str: &str) -> DbResult<String> {
    // Connection string format: mongodb://[user:pass@]host[:port]/database[?options]
    // or mongodb+srv://[user:pass@]host/database[?options]
    
    // Remove the protocol prefix
    let without_protocol = conn_str
        .strip_prefix("mongodb+srv://")
        .or_else(|| conn_str.strip_prefix("mongodb://"))
        .ok_or_else(|| {
            DbError::InvalidConnectionString("Invalid MongoDB connection string".to_string())
        })?;

    // Find the database name (between first / and optional ?)
    let parts: Vec<&str> = without_protocol.split('/').collect();
    if parts.len() < 2 {
        return Err(DbError::InvalidConnectionString(
            "Missing database name in connection string".to_string(),
        ));
    }

    let db_with_options = parts[1];
    let db_name = db_with_options
        .split('?')
        .next()
        .unwrap_or(db_with_options);

    if db_name.is_empty() {
        return Err(DbError::InvalidConnectionString(
            "Database name cannot be empty".to_string(),
        ));
    }

    Ok(db_name.to_string())
}

// Stub implementation when feature is not enabled
#[cfg(not(feature = "mongodb"))]
pub struct MongoDbConnector;

#[cfg(not(feature = "mongodb"))]
impl MongoDbConnector {
    pub async fn new(_conn_str: &str) -> DbResult<Self> {
        Err(DbError::FeatureNotEnabled("mongodb".to_string()))
    }

    pub async fn new_with_sample_size(_conn_str: &str, _sample_size: usize) -> DbResult<Self> {
        Err(DbError::FeatureNotEnabled("mongodb".to_string()))
    }
}

#[cfg(all(test, feature = "mongodb"))]
mod tests {
    use super::*;

    #[test]
    fn test_bson_type_mapping() {
        assert_eq!(
            map_bson_type_to_canonical("boolean"),
            CanonicalType::Boolean
        );
        assert_eq!(map_bson_type_to_canonical("int32"), CanonicalType::Int32);
        assert_eq!(map_bson_type_to_canonical("int64"), CanonicalType::Int64);
        assert_eq!(
            map_bson_type_to_canonical("double"),
            CanonicalType::Float64
        );
        assert_eq!(
            map_bson_type_to_canonical("string"),
            CanonicalType::String
        );
        assert_eq!(
            map_bson_type_to_canonical("datetime"),
            CanonicalType::DateTime
        );
        assert_eq!(map_bson_type_to_canonical("binary"), CanonicalType::Binary);
        assert_eq!(map_bson_type_to_canonical("object"), CanonicalType::Json);
        assert_eq!(map_bson_type_to_canonical("array"), CanonicalType::Json);
    }

    #[test]
    fn test_extract_database_name() {
        assert_eq!(
            extract_database_name("mongodb://localhost:27017/mydb").unwrap(),
            "mydb"
        );
        assert_eq!(
            extract_database_name("mongodb://user:pass@localhost/testdb").unwrap(),
            "testdb"
        );
        assert_eq!(
            extract_database_name("mongodb+srv://cluster.mongodb.net/production").unwrap(),
            "production"
        );
        assert_eq!(
            extract_database_name("mongodb://localhost/mydb?retryWrites=true").unwrap(),
            "mydb"
        );
    }

    #[test]
    fn test_extract_database_name_invalid() {
        assert!(extract_database_name("mongodb://localhost").is_err());
        assert!(extract_database_name("mongodb://localhost/").is_err());
        assert!(extract_database_name("invalid://localhost/db").is_err());
    }

    #[test]
    fn test_decimal128_mapping() {
        match map_bson_type_to_canonical("decimal128") {
            CanonicalType::Decimal { precision, scale } => {
                assert_eq!(precision, 34);
                assert_eq!(scale, 0);
            }
            _ => panic!("Expected Decimal type"),
        }
    }
}
