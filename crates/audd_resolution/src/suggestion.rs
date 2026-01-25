//! Suggestion types for conflict resolution

use serde::{Deserialize, Serialize};

/// Kind of suggestion for resolving a conflict
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionKind {
    /// Safe type cast (no data loss expected)
    CastSafe {
        /// Source type
        from_type: String,
        /// Target type
        to_type: String,
    },
    /// Risky type cast (potential data loss)
    CastRisky {
        /// Source type
        from_type: String,
        /// Target type
        to_type: String,
        /// Warning message about potential risks
        warning: String,
    },
    /// Rename a field to avoid collision or ambiguity
    RenameField {
        /// Original name
        original_name: String,
        /// Suggested new name
        suggested_name: String,
        /// Reason for rename
        reason: String,
    },
    /// Choose preferred type based on policy
    PreferType {
        /// The preferred type
        preferred_type: String,
        /// Alternative type
        alternative_type: String,
        /// Rule that determined preference
        rule: String,
    },
    /// Split a field into multiple fields
    SplitField {
        /// Original field name
        original_name: String,
        /// Suggested new field names
        new_fields: Vec<String>,
    },
    /// Merge multiple fields into one
    MergeFields {
        /// Original field names
        original_names: Vec<String>,
        /// Suggested merged field name
        merged_name: String,
    },
    /// No suggestion available - conflict cannot be auto-resolved
    NoSuggestion {
        /// Reason why no suggestion is available
        reason: String,
    },
}

/// Confidence level for a suggestion
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Confidence(f64);

impl Confidence {
    /// Create a new confidence level (clamped to 0.0-1.0)
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// High confidence (0.9-1.0)
    pub fn high() -> Self {
        Self(0.9)
    }

    /// Medium confidence (0.6-0.89)
    pub fn medium() -> Self {
        Self(0.75)
    }

    /// Low confidence (0.0-0.59)
    pub fn low() -> Self {
        Self(0.4)
    }

    /// Get the confidence value
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Impact level of applying a suggestion
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Impact {
    /// Minimal impact - safe to apply
    Minimal,
    /// Low impact - generally safe
    Low,
    /// Medium impact - requires validation
    Medium,
    /// High impact - requires careful review
    High,
    /// Critical impact - manual intervention recommended
    Critical,
}

/// A suggestion for resolving a conflict
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Suggestion {
    /// Unique identifier for this suggestion
    pub id: String,

    /// Entity name this suggestion applies to
    pub entity_name: String,

    /// Field name this suggestion applies to (None for entity-level)
    pub field_name: Option<String>,

    /// Kind of suggestion
    pub kind: SuggestionKind,

    /// Confidence level (0.0 to 1.0)
    pub confidence: Confidence,

    /// Human-readable explanation
    pub explanation: String,

    /// Evidence supporting this suggestion
    pub evidence: Vec<String>,

    /// Impact of applying this suggestion
    pub impact: Impact,

    /// Index of conflict this suggestion resolves
    pub conflict_index: Option<usize>,
}

impl Suggestion {
    /// Create a new suggestion
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        entity_name: String,
        field_name: Option<String>,
        kind: SuggestionKind,
        confidence: Confidence,
        explanation: String,
        evidence: Vec<String>,
        impact: Impact,
    ) -> Self {
        Self {
            id,
            entity_name,
            field_name,
            kind,
            confidence,
            explanation,
            evidence,
            impact,
            conflict_index: None,
        }
    }

    /// Set the conflict index this suggestion resolves
    pub fn with_conflict_index(mut self, index: usize) -> Self {
        self.conflict_index = Some(index);
        self
    }

    /// Create a safe cast suggestion
    pub fn cast_safe(
        id: String,
        entity_name: String,
        field_name: String,
        from_type: String,
        to_type: String,
        explanation: String,
    ) -> Self {
        Self::new(
            id,
            entity_name,
            Some(field_name),
            SuggestionKind::CastSafe {
                from_type: from_type.clone(),
                to_type: to_type.clone(),
            },
            Confidence::high(),
            explanation,
            vec![
                format!("Source type: {}", from_type),
                format!("Target type: {}", to_type),
                "No data loss expected".to_string(),
            ],
            Impact::Low,
        )
    }

    /// Create a risky cast suggestion
    pub fn cast_risky(
        id: String,
        entity_name: String,
        field_name: String,
        from_type: String,
        to_type: String,
        warning: String,
        explanation: String,
    ) -> Self {
        Self::new(
            id,
            entity_name,
            Some(field_name),
            SuggestionKind::CastRisky {
                from_type: from_type.clone(),
                to_type: to_type.clone(),
                warning: warning.clone(),
            },
            Confidence::medium(),
            explanation,
            vec![
                format!("Source type: {}", from_type),
                format!("Target type: {}", to_type),
                format!("Warning: {}", warning),
            ],
            Impact::High,
        )
    }

    /// Create a rename suggestion
    pub fn rename_field(
        id: String,
        entity_name: String,
        original_name: String,
        suggested_name: String,
        reason: String,
        explanation: String,
    ) -> Self {
        Self::new(
            id,
            entity_name,
            Some(original_name.clone()),
            SuggestionKind::RenameField {
                original_name: original_name.clone(),
                suggested_name: suggested_name.clone(),
                reason: reason.clone(),
            },
            Confidence::high(),
            explanation,
            vec![
                format!("Original name: {}", original_name),
                format!("Suggested name: {}", suggested_name),
                format!("Reason: {}", reason),
            ],
            Impact::Medium,
        )
    }

    /// Create a no-suggestion entry
    pub fn no_suggestion(
        id: String,
        entity_name: String,
        field_name: Option<String>,
        reason: String,
    ) -> Self {
        Self::new(
            id,
            entity_name,
            field_name,
            SuggestionKind::NoSuggestion {
                reason: reason.clone(),
            },
            Confidence::high(),
            "No automatic suggestion available - manual resolution required".to_string(),
            vec![format!("Reason: {}", reason)],
            Impact::Critical,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_clamping() {
        assert_eq!(Confidence::new(1.5).value(), 1.0);
        assert_eq!(Confidence::new(-0.5).value(), 0.0);
        assert_eq!(Confidence::new(0.5).value(), 0.5);
    }

    #[test]
    fn test_confidence_levels() {
        assert!(Confidence::high().value() >= 0.9);
        assert!(Confidence::medium().value() >= 0.6);
        assert!(Confidence::medium().value() < 0.9);
        assert!(Confidence::low().value() < 0.6);
    }

    #[test]
    fn test_impact_ordering() {
        assert!(Impact::Minimal < Impact::Low);
        assert!(Impact::Low < Impact::Medium);
        assert!(Impact::Medium < Impact::High);
        assert!(Impact::High < Impact::Critical);
    }

    #[test]
    fn test_cast_safe_suggestion() {
        let s = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Widening cast from Int32 to Int64".to_string(),
        );

        assert_eq!(s.entity_name, "users");
        assert_eq!(s.field_name, Some("id".to_string()));
        assert_eq!(s.confidence.value(), Confidence::high().value());
        assert_eq!(s.impact, Impact::Low);
        assert!(matches!(s.kind, SuggestionKind::CastSafe { .. }));
    }

    #[test]
    fn test_cast_risky_suggestion() {
        let s = Suggestion::cast_risky(
            "sug2".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int64".to_string(),
            "Int32".to_string(),
            "Potential overflow for large values".to_string(),
            "Narrowing cast from Int64 to Int32".to_string(),
        );

        assert_eq!(s.impact, Impact::High);
        assert!(matches!(s.kind, SuggestionKind::CastRisky { .. }));
    }

    #[test]
    fn test_rename_suggestion() {
        let s = Suggestion::rename_field(
            "sug3".to_string(),
            "users".to_string(),
            "UserID".to_string(),
            "user_id_a".to_string(),
            "Normalization collision".to_string(),
            "Rename to avoid collision with user_id".to_string(),
        );

        assert_eq!(s.impact, Impact::Medium);
        match &s.kind {
            SuggestionKind::RenameField {
                original_name,
                suggested_name,
                ..
            } => {
                assert_eq!(original_name, "UserID");
                assert_eq!(suggested_name, "user_id_a");
            }
            _ => panic!("Expected RenameField"),
        }
    }

    #[test]
    fn test_no_suggestion() {
        let s = Suggestion::no_suggestion(
            "sug4".to_string(),
            "users".to_string(),
            Some("complex_field".to_string()),
            "Complex type incompatibility".to_string(),
        );

        assert_eq!(s.impact, Impact::Critical);
        assert!(matches!(s.kind, SuggestionKind::NoSuggestion { .. }));
    }

    #[test]
    fn test_serialization() {
        let s = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe widening cast".to_string(),
        );

        let json = serde_json::to_string(&s).unwrap();
        let deserialized: Suggestion = serde_json::from_str(&json).unwrap();
        assert_eq!(s, deserialized);
    }

    #[test]
    fn test_with_conflict_index() {
        let s = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe widening cast".to_string(),
        )
        .with_conflict_index(5);

        assert_eq!(s.conflict_index, Some(5));
    }
}
