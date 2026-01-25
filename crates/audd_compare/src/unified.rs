//! Unified schema construction

use audd_ir::{EntitySchema, FieldSchema, SourceSchema};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::result::{ComparisonResult, ExclusiveSide};

/// Origin of a field in the unified schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FieldOrigin {
    /// From schema A only
    A,
    /// From schema B only
    B,
    /// Matched between A and B
    Both,
}

/// State of a field in the unified schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldState {
    /// Successfully matched and compatible
    Matched,
    /// Exclusive to one schema, safe to add
    Exclusive,
    /// Has a conflict that needs resolution
    Conflicted,
}

/// A field in the unified schema with metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnifiedField {
    /// The field schema (from A, B, or merged)
    pub field: FieldSchema,

    /// Origin of this field
    pub origin: FieldOrigin,

    /// State of this field
    pub state: FieldState,

    /// Index in source schema A (if applicable)
    pub index_a: Option<usize>,

    /// Index in source schema B (if applicable)
    pub index_b: Option<usize>,
}

/// An entity in the unified schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnifiedEntity {
    /// Entity name
    pub entity_name: String,

    /// Fields in this entity
    pub fields: Vec<UnifiedField>,

    /// Origin of this entity
    pub origin: FieldOrigin,
}

/// Unified schema combining A and B
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnifiedSchema {
    /// Name of the unified schema
    pub schema_name: String,

    /// Entities in the unified schema
    pub entities: Vec<UnifiedEntity>,

    /// Metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl UnifiedSchema {
    /// Build a unified schema from a comparison result
    pub fn from_comparison(
        schema_a: &SourceSchema,
        schema_b: &SourceSchema,
        result: &ComparisonResult,
    ) -> Self {
        let mut entities = Vec::new();

        // Track which entities have been processed
        let mut processed_a = vec![false; schema_a.entities.len()];
        let mut processed_b = vec![false; schema_b.entities.len()];

        // Process matched entities
        for m in &result.matches {
            if m.field_name.is_none() {
                // Entity-level match
                let entity_a = &schema_a.entities[m.index_a];
                let entity_b = &schema_b.entities[m.index_b];

                let unified_entity =
                    Self::merge_entities(entity_a, entity_b, result, m.index_a, m.index_b);
                entities.push(unified_entity);

                processed_a[m.index_a] = true;
                processed_b[m.index_b] = true;
            }
        }

        // Process exclusive entities from A
        for (idx, entity) in schema_a.entities.iter().enumerate() {
            if !processed_a[idx] {
                let unified_entity = Self::entity_from_a(entity, idx, result);
                entities.push(unified_entity);
            }
        }

        // Process exclusive entities from B
        for (idx, entity) in schema_b.entities.iter().enumerate() {
            if !processed_b[idx] {
                let unified_entity = Self::entity_from_b(entity, idx, result);
                entities.push(unified_entity);
            }
        }

        Self {
            schema_name: format!("{}_{}_unified", schema_a.source_name, schema_b.source_name),
            entities,
            metadata: HashMap::new(),
        }
    }

    /// Merge two matched entities
    fn merge_entities(
        entity_a: &EntitySchema,
        entity_b: &EntitySchema,
        result: &ComparisonResult,
        entity_idx_a: usize,
        entity_idx_b: usize,
    ) -> UnifiedEntity {
        let mut fields = Vec::new();
        let entity_name = &entity_a.entity_name;

        // Track processed fields
        let mut processed_a = vec![false; entity_a.fields.len()];
        let mut processed_b = vec![false; entity_b.fields.len()];

        // Add matched fields
        for m in &result.matches {
            if let Some(field_name) = &m.field_name {
                if m.entity_name == *entity_name {
                    let field_a = &entity_a.fields[m.index_a];
                    let has_conflict = result.conflicts.iter().any(|c| {
                        c.entity_name == *entity_name
                            && c.field_name.as_ref() == Some(field_name)
                            && c.index_a == m.index_a
                            && c.index_b == m.index_b
                    });

                    let state = if has_conflict {
                        FieldState::Conflicted
                    } else {
                        FieldState::Matched
                    };

                    fields.push(UnifiedField {
                        field: field_a.clone(),
                        origin: FieldOrigin::Both,
                        state,
                        index_a: Some(m.index_a),
                        index_b: Some(m.index_b),
                    });

                    processed_a[m.index_a] = true;
                    processed_b[m.index_b] = true;
                }
            }
        }

        // Add exclusive fields from A
        for (idx, field) in entity_a.fields.iter().enumerate() {
            if !processed_a[idx] {
                // Check if it's in exclusives
                let is_exclusive = result.exclusives.iter().any(|e| {
                    e.entity_name == *entity_name
                        && e.field_name.as_ref() == Some(&field.field_name)
                        && e.side == ExclusiveSide::A
                });

                if is_exclusive {
                    fields.push(UnifiedField {
                        field: field.clone(),
                        origin: FieldOrigin::A,
                        state: FieldState::Exclusive,
                        index_a: Some(idx),
                        index_b: None,
                    });
                }
            }
        }

        // Add exclusive fields from B
        for (idx, field) in entity_b.fields.iter().enumerate() {
            if !processed_b[idx] {
                // Check if it's in exclusives
                let is_exclusive = result.exclusives.iter().any(|e| {
                    e.entity_name == *entity_name
                        && e.field_name.as_ref() == Some(&field.field_name)
                        && e.side == ExclusiveSide::B
                });

                if is_exclusive {
                    fields.push(UnifiedField {
                        field: field.clone(),
                        origin: FieldOrigin::B,
                        state: FieldState::Exclusive,
                        index_a: None,
                        index_b: Some(idx),
                    });
                }
            }
        }

        UnifiedEntity {
            entity_name: entity_name.clone(),
            fields,
            origin: FieldOrigin::Both,
        }
    }

    /// Create entity from schema A (exclusive)
    fn entity_from_a(entity: &EntitySchema, idx: usize, _result: &ComparisonResult) -> UnifiedEntity {
        let fields = entity
            .fields
            .iter()
            .enumerate()
            .map(|(field_idx, field)| UnifiedField {
                field: field.clone(),
                origin: FieldOrigin::A,
                state: FieldState::Exclusive,
                index_a: Some(field_idx),
                index_b: None,
            })
            .collect();

        UnifiedEntity {
            entity_name: entity.entity_name.clone(),
            fields,
            origin: FieldOrigin::A,
        }
    }

    /// Create entity from schema B (exclusive)
    fn entity_from_b(entity: &EntitySchema, idx: usize, _result: &ComparisonResult) -> UnifiedEntity {
        let fields = entity
            .fields
            .iter()
            .enumerate()
            .map(|(field_idx, field)| UnifiedField {
                field: field.clone(),
                origin: FieldOrigin::B,
                state: FieldState::Exclusive,
                index_a: None,
                index_b: Some(field_idx),
            })
            .collect();

        UnifiedEntity {
            entity_name: entity.entity_name.clone(),
            fields,
            origin: FieldOrigin::B,
        }
    }

    /// Export to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::{Exclusive, Match};
    use audd_ir::CanonicalType;

    #[test]
    fn test_unified_field_creation() {
        let field = FieldSchema::builder()
            .field_name("id")
            .canonical_type(CanonicalType::Int32)
            .nullable(false)
            .build();

        let unified = UnifiedField {
            field,
            origin: FieldOrigin::Both,
            state: FieldState::Matched,
            index_a: Some(0),
            index_b: Some(0),
        };

        assert_eq!(unified.origin, FieldOrigin::Both);
        assert_eq!(unified.state, FieldState::Matched);
    }

    #[test]
    fn test_unified_schema_basic() {
        let entity_a = EntitySchema::builder()
            .entity_name("users")
            .add_field(
                FieldSchema::builder()
                    .field_name("id")
                    .canonical_type(CanonicalType::Int32)
                    .nullable(false)
                    .build(),
            )
            .build();

        let schema_a = SourceSchema::builder()
            .source_name("db_a")
            .source_type("mysql")
            .add_entity(entity_a)
            .build();

        let entity_b = EntitySchema::builder()
            .entity_name("users")
            .add_field(
                FieldSchema::builder()
                    .field_name("id")
                    .canonical_type(CanonicalType::Int32)
                    .nullable(false)
                    .build(),
            )
            .build();

        let schema_b = SourceSchema::builder()
            .source_name("db_b")
            .source_type("postgresql")
            .add_entity(entity_b)
            .build();

        let matches = vec![
            Match::exact("users".to_string(), None, 0, 0),
            Match::exact("users".to_string(), Some("id".to_string()), 0, 0),
        ];

        let result = ComparisonResult::new(matches, vec![], vec![]);
        let unified = UnifiedSchema::from_comparison(&schema_a, &schema_b, &result);

        assert_eq!(unified.entities.len(), 1);
        assert_eq!(unified.entities[0].entity_name, "users");
        assert_eq!(unified.entities[0].fields.len(), 1);
        assert_eq!(unified.entities[0].fields[0].state, FieldState::Matched);
    }

    #[test]
    fn test_unified_schema_with_exclusives() {
        let entity_a = EntitySchema::builder()
            .entity_name("users")
            .add_field(
                FieldSchema::builder()
                    .field_name("id")
                    .canonical_type(CanonicalType::Int32)
                    .nullable(false)
                    .build(),
            )
            .add_field(
                FieldSchema::builder()
                    .field_name("password")
                    .canonical_type(CanonicalType::String)
                    .nullable(false)
                    .build(),
            )
            .build();

        let schema_a = SourceSchema::builder()
            .source_name("db_a")
            .source_type("mysql")
            .add_entity(entity_a)
            .build();

        let entity_b = EntitySchema::builder()
            .entity_name("users")
            .add_field(
                FieldSchema::builder()
                    .field_name("id")
                    .canonical_type(CanonicalType::Int32)
                    .nullable(false)
                    .build(),
            )
            .build();

        let schema_b = SourceSchema::builder()
            .source_name("db_b")
            .source_type("postgresql")
            .add_entity(entity_b)
            .build();

        let matches = vec![
            Match::exact("users".to_string(), None, 0, 0),
            Match::exact("users".to_string(), Some("id".to_string()), 0, 0),
        ];

        let exclusives = vec![Exclusive::from_a(
            "users".to_string(),
            Some("password".to_string()),
            1,
        )];

        let result = ComparisonResult::new(matches, exclusives, vec![]);
        let unified = UnifiedSchema::from_comparison(&schema_a, &schema_b, &result);

        assert_eq!(unified.entities[0].fields.len(), 2);

        // Find password field
        let password_field = unified.entities[0]
            .fields
            .iter()
            .find(|f| f.field.field_name == "password")
            .unwrap();

        assert_eq!(password_field.state, FieldState::Exclusive);
        assert_eq!(password_field.origin, FieldOrigin::A);
    }
}
