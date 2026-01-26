//! Schema structures for the IR

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::{CanonicalType, Constraint, Key, Index, View, Trigger, StoredProcedure, IR_VERSION};

/// Complete schema for a data source
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceSchema {
    /// Normalized name of the source
    pub source_name: String,

    /// Type of source (e.g., "mysql", "postgresql", "csv")
    pub source_type: String,

    /// Entities (tables/collections) in this source
    pub entities: Vec<EntitySchema>,

    /// Views in this source
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub views: Vec<View>,

    /// Stored procedures and functions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stored_procedures: Vec<StoredProcedure>,

    /// Triggers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub triggers: Vec<Trigger>,

    /// Version of the IR specification
    pub ir_version: String,

    /// Source-specific metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, Value>,
}

impl SourceSchema {
    /// Create a builder for SourceSchema
    pub fn builder() -> SourceSchemaBuilder {
        SourceSchemaBuilder::default()
    }

    /// Export this schema to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Import a schema from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Find an entity by name
    pub fn find_entity(&self, name: &str) -> Option<&EntitySchema> {
        self.entities.iter().find(|e| e.entity_name == name)
    }
}

/// Schema for an entity (table/collection)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntitySchema {
    /// Normalized entity name
    pub entity_name: String,

    /// Type of entity ("table", "collection", "sheet", etc.)
    pub entity_type: String,

    /// Fields in this entity
    pub fields: Vec<FieldSchema>,

    /// Keys (primary, unique, foreign)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<Key>,

    /// Indexes on this entity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indexes: Vec<Index>,

    /// Entity-specific metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, Value>,
}

impl EntitySchema {
    /// Create a builder for EntitySchema
    pub fn builder() -> EntitySchemaBuilder {
        EntitySchemaBuilder::default()
    }

    /// Find a field by name
    pub fn find_field(&self, name: &str) -> Option<&FieldSchema> {
        self.fields.iter().find(|f| f.field_name == name)
    }
}

/// Schema for a field/column
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldSchema {
    /// Normalized field name
    pub field_name: String,

    /// Canonical data type
    pub canonical_type: CanonicalType,

    /// Whether the field can be null
    pub nullable: bool,

    /// Field constraints
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<Constraint>,

    /// Field-specific metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, Value>,
}

impl FieldSchema {
    /// Create a builder for FieldSchema
    pub fn builder() -> FieldSchemaBuilder {
        FieldSchemaBuilder::default()
    }
}

/// Builder for SourceSchema
#[derive(Default)]
pub struct SourceSchemaBuilder {
    source_name: Option<String>,
    source_type: Option<String>,
    entities: Vec<EntitySchema>,
    views: Vec<View>,
    stored_procedures: Vec<StoredProcedure>,
    triggers: Vec<Trigger>,
    metadata: HashMap<String, Value>,
}

impl SourceSchemaBuilder {
    /// Set the source name
    pub fn source_name<S: Into<String>>(mut self, name: S) -> Self {
        self.source_name = Some(name.into());
        self
    }

    /// Set the source type
    pub fn source_type<S: Into<String>>(mut self, source_type: S) -> Self {
        self.source_type = Some(source_type.into());
        self
    }

    /// Add an entity
    pub fn add_entity(mut self, entity: EntitySchema) -> Self {
        self.entities.push(entity);
        self
    }

    /// Add multiple entities
    pub fn entities(mut self, entities: Vec<EntitySchema>) -> Self {
        self.entities = entities;
        self
    }

    /// Add a view
    pub fn add_view(mut self, view: View) -> Self {
        self.views.push(view);
        self
    }

    /// Add multiple views
    pub fn views(mut self, views: Vec<View>) -> Self {
        self.views = views;
        self
    }

    /// Add a stored procedure
    pub fn add_stored_procedure(mut self, proc: StoredProcedure) -> Self {
        self.stored_procedures.push(proc);
        self
    }

    /// Add multiple stored procedures
    pub fn stored_procedures(mut self, procs: Vec<StoredProcedure>) -> Self {
        self.stored_procedures = procs;
        self
    }

    /// Add a trigger
    pub fn add_trigger(mut self, trigger: Trigger) -> Self {
        self.triggers.push(trigger);
        self
    }

    /// Add multiple triggers
    pub fn triggers(mut self, triggers: Vec<Trigger>) -> Self {
        self.triggers = triggers;
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: String, value: Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Build the SourceSchema
    pub fn build(self) -> SourceSchema {
        SourceSchema {
            source_name: self.source_name.expect("source_name is required"),
            source_type: self.source_type.expect("source_type is required"),
            entities: self.entities,
            views: self.views,
            stored_procedures: self.stored_procedures,
            triggers: self.triggers,
            ir_version: IR_VERSION.to_string(),
            metadata: self.metadata,
        }
    }
}

/// Builder for EntitySchema
#[derive(Default)]
pub struct EntitySchemaBuilder {
    entity_name: Option<String>,
    entity_type: Option<String>,
    fields: Vec<FieldSchema>,
    keys: Vec<Key>,
    indexes: Vec<Index>,
    metadata: HashMap<String, Value>,
}

impl EntitySchemaBuilder {
    /// Set the entity name
    pub fn entity_name<S: Into<String>>(mut self, name: S) -> Self {
        self.entity_name = Some(name.into());
        self
    }

    /// Set the entity type
    pub fn entity_type<S: Into<String>>(mut self, entity_type: S) -> Self {
        self.entity_type = Some(entity_type.into());
        self
    }

    /// Add a field
    pub fn add_field(mut self, field: FieldSchema) -> Self {
        self.fields.push(field);
        self
    }

    /// Add multiple fields
    pub fn fields(mut self, fields: Vec<FieldSchema>) -> Self {
        self.fields = fields;
        self
    }

    /// Add a key
    pub fn add_key(mut self, key: Key) -> Self {
        self.keys.push(key);
        self
    }

    /// Add multiple keys
    pub fn keys(mut self, keys: Vec<Key>) -> Self {
        self.keys = keys;
        self
    }

    /// Add an index
    pub fn add_index(mut self, index: Index) -> Self {
        self.indexes.push(index);
        self
    }

    /// Add multiple indexes
    pub fn indexes(mut self, indexes: Vec<Index>) -> Self {
        self.indexes = indexes;
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: String, value: Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Build the EntitySchema
    pub fn build(self) -> EntitySchema {
        EntitySchema {
            entity_name: self.entity_name.expect("entity_name is required"),
            entity_type: self.entity_type.unwrap_or_else(|| "table".to_string()),
            fields: self.fields,
            keys: self.keys,
            indexes: self.indexes,
            metadata: self.metadata,
        }
    }
}

/// Builder for FieldSchema
#[derive(Default)]
pub struct FieldSchemaBuilder {
    field_name: Option<String>,
    canonical_type: Option<CanonicalType>,
    nullable: bool,
    constraints: Vec<Constraint>,
    metadata: HashMap<String, Value>,
}

impl FieldSchemaBuilder {
    /// Set the field name
    pub fn field_name<S: Into<String>>(mut self, name: S) -> Self {
        self.field_name = Some(name.into());
        self
    }

    /// Set the canonical type
    pub fn canonical_type(mut self, canonical_type: CanonicalType) -> Self {
        self.canonical_type = Some(canonical_type);
        self
    }

    /// Set nullable
    pub fn nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }

    /// Add a constraint
    pub fn add_constraint(mut self, constraint: Constraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// Add multiple constraints
    pub fn constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.constraints = constraints;
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: String, value: Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Build the FieldSchema
    pub fn build(self) -> FieldSchema {
        FieldSchema {
            field_name: self.field_name.expect("field_name is required"),
            canonical_type: self.canonical_type.expect("canonical_type is required"),
            nullable: self.nullable,
            constraints: self.constraints,
            metadata: self.metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_builder() {
        let field = FieldSchema::builder()
            .field_name("user_id")
            .canonical_type(CanonicalType::Int32)
            .nullable(false)
            .build();

        assert_eq!(field.field_name, "user_id");
        assert_eq!(field.canonical_type, CanonicalType::Int32);
        assert!(!field.nullable);
    }

    #[test]
    fn test_entity_builder() {
        let field = FieldSchema::builder()
            .field_name("id")
            .canonical_type(CanonicalType::Int32)
            .nullable(false)
            .build();

        let entity = EntitySchema::builder()
            .entity_name("users")
            .entity_type("table")
            .add_field(field)
            .add_key(Key::primary(vec!["id"]))
            .build();

        assert_eq!(entity.entity_name, "users");
        assert_eq!(entity.fields.len(), 1);
        assert_eq!(entity.keys.len(), 1);
    }

    #[test]
    fn test_source_builder() {
        let field = FieldSchema::builder()
            .field_name("id")
            .canonical_type(CanonicalType::Int32)
            .nullable(false)
            .build();

        let entity = EntitySchema::builder()
            .entity_name("users")
            .add_field(field)
            .build();

        let source = SourceSchema::builder()
            .source_name("myapp_db")
            .source_type("mysql")
            .add_entity(entity)
            .build();

        assert_eq!(source.source_name, "myapp_db");
        assert_eq!(source.source_type, "mysql");
        assert_eq!(source.ir_version, IR_VERSION);
        assert_eq!(source.entities.len(), 1);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let field = FieldSchema::builder()
            .field_name("email")
            .canonical_type(CanonicalType::String)
            .nullable(false)
            .add_constraint(Constraint::max_length(255))
            .build();

        let entity = EntitySchema::builder()
            .entity_name("users")
            .add_field(field)
            .build();

        let source = SourceSchema::builder()
            .source_name("test_db")
            .source_type("postgresql")
            .add_entity(entity)
            .build();

        let json = source.to_json().unwrap();
        let deserialized = SourceSchema::from_json(&json).unwrap();

        assert_eq!(source, deserialized);
    }

    #[test]
    fn test_find_entity() {
        let entity = EntitySchema::builder().entity_name("users").build();

        let source = SourceSchema::builder()
            .source_name("test_db")
            .source_type("mysql")
            .add_entity(entity)
            .build();

        assert!(source.find_entity("users").is_some());
        assert!(source.find_entity("posts").is_none());
    }

    #[test]
    fn test_find_field() {
        let field = FieldSchema::builder()
            .field_name("email")
            .canonical_type(CanonicalType::String)
            .nullable(false)
            .build();

        let entity = EntitySchema::builder()
            .entity_name("users")
            .add_field(field)
            .build();

        assert!(entity.find_field("email").is_some());
        assert!(entity.find_field("password").is_none());
    }
}
