//! Conflict detection and representation

use serde::{Deserialize, Serialize};

/// Type of conflict between schemas
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictType {
    /// Type incompatibility
    TypeIncompatible,
    /// Nullability mismatch
    NullabilityMismatch,
    /// Constraint mismatch
    ConstraintMismatch,
    /// Length/precision mismatch
    LengthMismatch,
    /// Normalization collision (two different names normalize to the same)
    NormalizationCollision,
}

/// Severity of a conflict
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConflictSeverity {
    /// Low severity - might be resolvable with simple casting
    Low,
    /// Medium severity - requires careful consideration
    Medium,
    /// High severity - likely to cause data loss or errors
    High,
    /// Critical severity - cannot be automatically resolved
    Critical,
}

/// Evidence for a conflict
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConflictEvidence {
    /// Description of what was found in schema A
    pub from_a: String,

    /// Description of what was found in schema B
    pub from_b: String,

    /// The rule that was violated
    pub rule: String,
}

impl ConflictEvidence {
    /// Create new conflict evidence
    pub fn new(from_a: String, from_b: String, rule: String) -> Self {
        Self {
            from_a,
            from_b,
            rule,
        }
    }
}

/// A conflict between schemas
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Conflict {
    /// Name of the entity where conflict occurs
    pub entity_name: String,

    /// Name of the field where conflict occurs (None for entity-level conflicts)
    pub field_name: Option<String>,

    /// Type of conflict
    pub conflict_type: ConflictType,

    /// Severity of the conflict
    pub severity: ConflictSeverity,

    /// Evidence supporting the conflict
    pub evidence: ConflictEvidence,

    /// Index in schema A
    pub index_a: usize,

    /// Index in schema B
    pub index_b: usize,
}

impl Conflict {
    /// Create a new conflict
    pub fn new(
        entity_name: String,
        field_name: Option<String>,
        conflict_type: ConflictType,
        severity: ConflictSeverity,
        evidence: ConflictEvidence,
        index_a: usize,
        index_b: usize,
    ) -> Self {
        Self {
            entity_name,
            field_name,
            conflict_type,
            severity,
            evidence,
            index_a,
            index_b,
        }
    }

    /// Create a type incompatibility conflict
    pub fn type_incompatible(
        entity_name: String,
        field_name: String,
        type_a: String,
        type_b: String,
        index_a: usize,
        index_b: usize,
    ) -> Self {
        Self::new(
            entity_name,
            Some(field_name),
            ConflictType::TypeIncompatible,
            ConflictSeverity::High,
            ConflictEvidence::new(
                format!("Type: {}", type_a),
                format!("Type: {}", type_b),
                "Types must be compatible".to_string(),
            ),
            index_a,
            index_b,
        )
    }

    /// Create a nullability mismatch conflict
    pub fn nullability_mismatch(
        entity_name: String,
        field_name: String,
        nullable_a: bool,
        nullable_b: bool,
        index_a: usize,
        index_b: usize,
    ) -> Self {
        Self::new(
            entity_name,
            Some(field_name),
            ConflictType::NullabilityMismatch,
            ConflictSeverity::Medium,
            ConflictEvidence::new(
                format!("Nullable: {}", nullable_a),
                format!("Nullable: {}", nullable_b),
                "Nullability must match or be compatible".to_string(),
            ),
            index_a,
            index_b,
        )
    }

    /// Create a constraint mismatch conflict
    pub fn constraint_mismatch(
        entity_name: String,
        field_name: String,
        constraint_a: String,
        constraint_b: String,
        index_a: usize,
        index_b: usize,
    ) -> Self {
        Self::new(
            entity_name,
            Some(field_name),
            ConflictType::ConstraintMismatch,
            ConflictSeverity::Medium,
            ConflictEvidence::new(
                constraint_a,
                constraint_b,
                "Constraints must be compatible".to_string(),
            ),
            index_a,
            index_b,
        )
    }

    /// Create a normalization collision conflict
    pub fn normalization_collision(
        entity_name: String,
        name_a: String,
        name_b: String,
        normalized: String,
        index_a: usize,
        index_b: usize,
    ) -> Self {
        Self::new(
            entity_name,
            None,
            ConflictType::NormalizationCollision,
            ConflictSeverity::Critical,
            ConflictEvidence::new(
                format!("Field '{}' normalizes to '{}'", name_a, normalized),
                format!("Field '{}' normalizes to '{}'", name_b, normalized),
                "Different fields cannot normalize to the same identifier".to_string(),
            ),
            index_a,
            index_b,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflict_creation() {
        let c = Conflict::type_incompatible(
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "String".to_string(),
            0,
            1,
        );

        assert_eq!(c.entity_name, "users");
        assert_eq!(c.field_name, Some("id".to_string()));
        assert_eq!(c.conflict_type, ConflictType::TypeIncompatible);
        assert_eq!(c.severity, ConflictSeverity::High);
    }

    #[test]
    fn test_nullability_conflict() {
        let c = Conflict::nullability_mismatch(
            "users".to_string(),
            "email".to_string(),
            false,
            true,
            0,
            1,
        );

        assert_eq!(c.conflict_type, ConflictType::NullabilityMismatch);
        assert_eq!(c.severity, ConflictSeverity::Medium);
    }

    #[test]
    fn test_collision_conflict() {
        let c = Conflict::normalization_collision(
            "users".to_string(),
            "UserID".to_string(),
            "user_id".to_string(),
            "user_id".to_string(),
            0,
            1,
        );

        assert_eq!(c.conflict_type, ConflictType::NormalizationCollision);
        assert_eq!(c.severity, ConflictSeverity::Critical);
    }

    #[test]
    fn test_serialization() {
        let c = Conflict::type_incompatible(
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "String".to_string(),
            0,
            1,
        );

        let json = serde_json::to_string(&c).unwrap();
        let deserialized: Conflict = serde_json::from_str(&json).unwrap();
        assert_eq!(c, deserialized);
    }

    #[test]
    fn test_severity_ordering() {
        assert!(ConflictSeverity::Low < ConflictSeverity::Medium);
        assert!(ConflictSeverity::Medium < ConflictSeverity::High);
        assert!(ConflictSeverity::High < ConflictSeverity::Critical);
    }
}
