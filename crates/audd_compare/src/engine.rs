//! Main comparison engine

use audd_ir::{normalize_identifier, SourceSchema};
use std::collections::HashMap;

use crate::config::CompareConfig;
use crate::conflict::Conflict;
use crate::matcher::{match_entities, match_fields};
use crate::result::{ComparisonResult, Exclusive, ExclusiveSide};
use crate::types::{compare_types, TypeCompatibility};

/// Compare two schemas and produce a comparison result
///
/// # Examples
///
/// ```no_run
/// use audd_compare::{compare, CompareConfig};
/// use audd_ir::SourceSchema;
///
/// # fn example(schema_a: SourceSchema, schema_b: SourceSchema) {
/// let config = CompareConfig::default();
/// let result = compare(&schema_a, &schema_b, &config);
///
/// println!("Found {} matches", result.matches.len());
/// println!("Found {} exclusives", result.exclusives.len());
/// println!("Found {} conflicts", result.conflicts.len());
/// # }
/// ```
pub fn compare(
    schema_a: &SourceSchema,
    schema_b: &SourceSchema,
    config: &CompareConfig,
) -> ComparisonResult {
    let mut matches = Vec::new();
    let mut exclusives = Vec::new();
    let mut conflicts = Vec::new();

    // Step 1: Match entities
    let entity_matches = match_entities(&schema_a.entities, &schema_b.entities, config);

    // Track which entities and fields have been matched
    let mut matched_entities_a = vec![false; schema_a.entities.len()];
    let mut matched_entities_b = vec![false; schema_b.entities.len()];

    // Step 2: For each matched entity, match fields and detect conflicts
    for entity_match in &entity_matches {
        matches.push(entity_match.clone());
        matched_entities_a[entity_match.index_a] = true;
        matched_entities_b[entity_match.index_b] = true;

        let entity_a = &schema_a.entities[entity_match.index_a];
        let entity_b = &schema_b.entities[entity_match.index_b];

        // Match fields within the entity
        let field_matches = match_fields(
            &entity_match.entity_name,
            &entity_a.fields,
            &entity_b.fields,
            config,
        );

        let mut matched_fields_a = vec![false; entity_a.fields.len()];
        let mut matched_fields_b = vec![false; entity_b.fields.len()];

        // Detect conflicts for matched fields
        for field_match in &field_matches {
            matches.push(field_match.clone());
            matched_fields_a[field_match.index_a] = true;
            matched_fields_b[field_match.index_b] = true;

            let field_a = &entity_a.fields[field_match.index_a];
            let field_b = &entity_b.fields[field_match.index_b];

            // Check for conflicts
            let field_conflicts = detect_field_conflicts(
                &entity_match.entity_name,
                field_a,
                field_b,
                field_match.index_a,
                field_match.index_b,
                config,
            );

            conflicts.extend(field_conflicts);
        }

        // Find exclusive fields in entity A
        for (idx, field) in entity_a.fields.iter().enumerate() {
            if !matched_fields_a[idx] {
                exclusives.push(Exclusive::from_a(
                    entity_match.entity_name.clone(),
                    Some(field.field_name.clone()),
                    idx,
                ));
            }
        }

        // Find exclusive fields in entity B
        for (idx, field) in entity_b.fields.iter().enumerate() {
            if !matched_fields_b[idx] {
                exclusives.push(Exclusive::from_b(
                    entity_match.entity_name.clone(),
                    Some(field.field_name.clone()),
                    idx,
                ));
            }
        }

        // Detect normalization collisions within the entity
        if config.detect_collisions {
            let collision_conflicts = detect_normalization_collisions(
                entity_a,
                entity_b,
                &entity_match.entity_name,
                &matched_fields_a,
                &matched_fields_b,
            );
            conflicts.extend(collision_conflicts);
        }
    }

    // Step 3: Find exclusive entities
    for (idx, entity) in schema_a.entities.iter().enumerate() {
        if !matched_entities_a[idx] {
            exclusives.push(Exclusive::from_a(
                entity.entity_name.clone(),
                None,
                idx,
            ));
        }
    }

    for (idx, entity) in schema_b.entities.iter().enumerate() {
        if !matched_entities_b[idx] {
            exclusives.push(Exclusive::from_b(
                entity.entity_name.clone(),
                None,
                idx,
            ));
        }
    }

    ComparisonResult::new(matches, exclusives, conflicts)
}

/// Detect conflicts between two matched fields
fn detect_field_conflicts(
    entity_name: &str,
    field_a: &audd_ir::FieldSchema,
    field_b: &audd_ir::FieldSchema,
    index_a: usize,
    index_b: usize,
    config: &CompareConfig,
) -> Vec<Conflict> {
    let mut conflicts = Vec::new();

    // Type compatibility check
    if config.check_type_compatibility {
        let type_compat = compare_types(&field_a.canonical_type, &field_b.canonical_type);

        if type_compat == TypeCompatibility::Incompatible {
            conflicts.push(Conflict::type_incompatible(
                entity_name.to_string(),
                field_a.field_name.clone(),
                field_a.canonical_type.type_name(),
                field_b.canonical_type.type_name(),
                index_a,
                index_b,
            ));
        }
    }

    // Nullability check
    if config.check_nullability {
        // Only flag if A is NOT nullable but B is nullable (potential data loss)
        if !field_a.nullable && field_b.nullable {
            conflicts.push(Conflict::nullability_mismatch(
                entity_name.to_string(),
                field_a.field_name.clone(),
                field_a.nullable,
                field_b.nullable,
                index_a,
                index_b,
            ));
        }
    }

    // Constraint checks
    if config.check_constraints {
        // Check for unique constraint mismatches
        let has_unique_a = field_a
            .constraints
            .iter()
            .any(|c| matches!(c, audd_ir::Constraint::Unique));
        let has_unique_b = field_b
            .constraints
            .iter()
            .any(|c| matches!(c, audd_ir::Constraint::Unique));

        if has_unique_a != has_unique_b {
            conflicts.push(Conflict::constraint_mismatch(
                entity_name.to_string(),
                field_a.field_name.clone(),
                format!("Unique: {}", has_unique_a),
                format!("Unique: {}", has_unique_b),
                index_a,
                index_b,
            ));
        }

        // Check for max length mismatches
        let max_len_a = field_a.constraints.iter().find_map(|c| {
            if let audd_ir::Constraint::MaxLength { value } = c {
                Some(*value)
            } else {
                None
            }
        });

        let max_len_b = field_b.constraints.iter().find_map(|c| {
            if let audd_ir::Constraint::MaxLength { value } = c {
                Some(*value)
            } else {
                None
            }
        });

        if let (Some(len_a), Some(len_b)) = (max_len_a, max_len_b) {
            if len_a != len_b {
                conflicts.push(Conflict::constraint_mismatch(
                    entity_name.to_string(),
                    field_a.field_name.clone(),
                    format!("MaxLength: {}", len_a),
                    format!("MaxLength: {}", len_b),
                    index_a,
                    index_b,
                ));
            }
        }
    }

    conflicts
}

/// Detect normalization collisions (different names that normalize to the same identifier)
/// Only detects collisions for fields that are NOT already matched
fn detect_normalization_collisions(
    entity_a: &audd_ir::EntitySchema,
    entity_b: &audd_ir::EntitySchema,
    entity_name: &str,
    matched_fields_a: &[bool],
    matched_fields_b: &[bool],
) -> Vec<Conflict> {
    let mut conflicts = Vec::new();
    let mut normalized_map: HashMap<String, Vec<(String, usize, ExclusiveSide)>> = HashMap::new();

    // Only collect UNMATCHED fields from A
    for (idx, field) in entity_a.fields.iter().enumerate() {
        if !matched_fields_a[idx] {
            let normalized = normalize_identifier(&field.field_name);
            normalized_map
                .entry(normalized)
                .or_default()
                .push((field.field_name.clone(), idx, ExclusiveSide::A));
        }
    }

    // Only collect UNMATCHED fields from B
    for (idx, field) in entity_b.fields.iter().enumerate() {
        if !matched_fields_b[idx] {
            let normalized = normalize_identifier(&field.field_name);
            normalized_map
                .entry(normalized)
                .or_default()
                .push((field.field_name.clone(), idx, ExclusiveSide::B));
        }
    }

    // Find collisions (normalized names with multiple distinct original names)
    for (normalized, entries) in normalized_map {
        if entries.len() > 1 {
            // Check if they're actually different original names
            let unique_names: Vec<_> = entries
                .iter()
                .map(|(name, _, _)| name.as_str())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();

            if unique_names.len() > 1 {
                // We have a collision - pick the first two for the conflict
                let (name_a, idx_a, side_a) = &entries[0];
                let (name_b, idx_b, side_b) = &entries[1];

                // Only report if they're from different sides
                if side_a != side_b {
                    conflicts.push(Conflict::normalization_collision(
                        entity_name.to_string(),
                        name_a.clone(),
                        name_b.clone(),
                        normalized,
                        *idx_a,
                        *idx_b,
                    ));
                }
            }
        }
    }

    conflicts
}

#[cfg(test)]
mod tests {
    use super::*;
    use audd_ir::{CanonicalType, Constraint, EntitySchema, FieldSchema, SourceSchema};

    fn create_test_schema(name: &str, entity_name: &str, fields: Vec<FieldSchema>) -> SourceSchema {
        let entity = EntitySchema::builder()
            .entity_name(entity_name)
            .fields(fields)
            .build();

        SourceSchema::builder()
            .source_name(name)
            .source_type("test")
            .add_entity(entity)
            .build()
    }

    #[test]
    fn test_basic_match() {
        let fields_a = vec![FieldSchema::builder()
            .field_name("id")
            .canonical_type(CanonicalType::Int32)
            .nullable(false)
            .build()];

        let fields_b = vec![FieldSchema::builder()
            .field_name("id")
            .canonical_type(CanonicalType::Int32)
            .nullable(false)
            .build()];

        let schema_a = create_test_schema("db_a", "users", fields_a);
        let schema_b = create_test_schema("db_b", "users", fields_b);

        let config = CompareConfig::default();
        let result = compare(&schema_a, &schema_b, &config);

        assert_eq!(result.summary.total_matches, 2); // 1 entity + 1 field
        assert_eq!(result.summary.total_exclusives, 0);
        assert_eq!(result.summary.total_conflicts, 0);
    }

    #[test]
    fn test_exclusive_fields() {
        let fields_a = vec![
            FieldSchema::builder()
                .field_name("id")
                .canonical_type(CanonicalType::Int32)
                .nullable(false)
                .build(),
            FieldSchema::builder()
                .field_name("password")
                .canonical_type(CanonicalType::String)
                .nullable(false)
                .build(),
        ];

        let fields_b = vec![FieldSchema::builder()
            .field_name("id")
            .canonical_type(CanonicalType::Int32)
            .nullable(false)
            .build()];

        let schema_a = create_test_schema("db_a", "users", fields_a);
        let schema_b = create_test_schema("db_b", "users", fields_b);

        let config = CompareConfig::default();
        let result = compare(&schema_a, &schema_b, &config);

        assert_eq!(result.summary.total_matches, 2); // 1 entity + 1 field (id)
        assert_eq!(result.summary.total_exclusives, 1); // password
        assert_eq!(result.summary.exclusives_a, 1);
    }

    #[test]
    fn test_type_conflict() {
        let fields_a = vec![FieldSchema::builder()
            .field_name("id")
            .canonical_type(CanonicalType::Int32)
            .nullable(false)
            .build()];

        let fields_b = vec![FieldSchema::builder()
            .field_name("id")
            .canonical_type(CanonicalType::String)
            .nullable(false)
            .build()];

        let schema_a = create_test_schema("db_a", "users", fields_a);
        let schema_b = create_test_schema("db_b", "users", fields_b);

        let config = CompareConfig::default();
        let result = compare(&schema_a, &schema_b, &config);

        assert_eq!(result.summary.total_matches, 2); // Still matched by name
        assert_eq!(result.summary.total_conflicts, 1); // Type conflict
    }

    #[test]
    fn test_nullability_conflict() {
        let fields_a = vec![FieldSchema::builder()
            .field_name("email")
            .canonical_type(CanonicalType::String)
            .nullable(false)
            .build()];

        let fields_b = vec![FieldSchema::builder()
            .field_name("email")
            .canonical_type(CanonicalType::String)
            .nullable(true)
            .build()];

        let schema_a = create_test_schema("db_a", "users", fields_a);
        let schema_b = create_test_schema("db_b", "users", fields_b);

        let config = CompareConfig::default();
        let result = compare(&schema_a, &schema_b, &config);

        assert_eq!(result.summary.total_conflicts, 1); // Nullability conflict
    }

    #[test]
    fn test_constraint_conflict() {
        let fields_a = vec![FieldSchema::builder()
            .field_name("email")
            .canonical_type(CanonicalType::String)
            .nullable(false)
            .add_constraint(Constraint::unique())
            .build()];

        let fields_b = vec![FieldSchema::builder()
            .field_name("email")
            .canonical_type(CanonicalType::String)
            .nullable(false)
            .build()];

        let schema_a = create_test_schema("db_a", "users", fields_a);
        let schema_b = create_test_schema("db_b", "users", fields_b);

        let config = CompareConfig::default();
        let result = compare(&schema_a, &schema_b, &config);

        assert_eq!(result.summary.total_conflicts, 1); // Constraint conflict
    }

    #[test]
    fn test_exclusive_entities() {
        let schema_a = SourceSchema::builder()
            .source_name("db_a")
            .source_type("test")
            .add_entity(EntitySchema::builder().entity_name("users").build())
            .add_entity(EntitySchema::builder().entity_name("posts").build())
            .build();

        let schema_b = SourceSchema::builder()
            .source_name("db_b")
            .source_type("test")
            .add_entity(EntitySchema::builder().entity_name("users").build())
            .build();

        let config = CompareConfig::default();
        let result = compare(&schema_a, &schema_b, &config);

        assert_eq!(result.summary.total_matches, 1); // users entity
        assert_eq!(result.summary.total_exclusives, 1); // posts entity
    }
}
