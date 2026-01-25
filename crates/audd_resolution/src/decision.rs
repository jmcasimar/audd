//! Decision structures for tracking resolution choices

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::suggestion::Suggestion;

/// Source of a decision (human or automated)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionSource {
    /// Decision made by a human user
    User {
        /// Username or identifier
        username: String,
    },
    /// Decision made by automated system
    System {
        /// Name of the automated rule or system
        rule: String,
    },
}

/// Status of a decision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DecisionStatus {
    /// Decision is pending
    Pending,
    /// Decision has been applied
    Applied,
    /// Decision was rejected
    Rejected,
    /// Decision was superseded by another
    Superseded,
}

/// A decision about a suggestion
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Decision {
    /// Unique identifier for this decision
    pub id: String,

    /// The suggestion that was selected (or rejected)
    pub suggestion: Suggestion,

    /// Whether this suggestion was accepted
    pub accepted: bool,

    /// Rationale for the decision
    pub rationale: String,

    /// Source of the decision
    pub source: DecisionSource,

    /// Status of the decision
    pub status: DecisionStatus,

    /// Timestamp when decision was made
    pub timestamp: DateTime<Utc>,

    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Decision {
    /// Create a new decision
    pub fn new(
        id: String,
        suggestion: Suggestion,
        accepted: bool,
        rationale: String,
        source: DecisionSource,
    ) -> Self {
        Self {
            id,
            suggestion,
            accepted,
            rationale,
            source,
            status: DecisionStatus::Pending,
            timestamp: Utc::now(),
            metadata: None,
        }
    }

    /// Create a decision accepting a suggestion
    pub fn accept(
        id: String,
        suggestion: Suggestion,
        rationale: String,
        source: DecisionSource,
    ) -> Self {
        Self::new(id, suggestion, true, rationale, source)
    }

    /// Create a decision rejecting a suggestion
    pub fn reject(
        id: String,
        suggestion: Suggestion,
        rationale: String,
        source: DecisionSource,
    ) -> Self {
        Self::new(id, suggestion, false, rationale, source)
    }

    /// Create an automated decision by system
    pub fn by_system(
        id: String,
        suggestion: Suggestion,
        accepted: bool,
        rule: String,
    ) -> Self {
        Self::new(
            id,
            suggestion,
            accepted,
            format!("Automated by rule: {}", rule),
            DecisionSource::System {
                rule: rule.clone(),
            },
        )
    }

    /// Create a user decision
    pub fn by_user(
        id: String,
        suggestion: Suggestion,
        accepted: bool,
        rationale: String,
        username: String,
    ) -> Self {
        Self::new(
            id,
            suggestion,
            accepted,
            rationale,
            DecisionSource::User { username },
        )
    }

    /// Mark decision as applied
    pub fn mark_applied(mut self) -> Self {
        self.status = DecisionStatus::Applied;
        self
    }

    /// Mark decision as rejected
    pub fn mark_rejected(mut self) -> Self {
        self.status = DecisionStatus::Rejected;
        self
    }

    /// Mark decision as superseded
    pub fn mark_superseded(mut self) -> Self {
        self.status = DecisionStatus::Superseded;
        self
    }

    /// Add metadata to the decision
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::suggestion::Suggestion;

    #[test]
    fn test_decision_creation() {
        let suggestion = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe cast".to_string(),
        );

        let decision = Decision::accept(
            "dec1".to_string(),
            suggestion.clone(),
            "Looks good".to_string(),
            DecisionSource::User {
                username: "admin".to_string(),
            },
        );

        assert!(decision.accepted);
        assert_eq!(decision.rationale, "Looks good");
        assert_eq!(decision.status, DecisionStatus::Pending);
    }

    #[test]
    fn test_decision_by_system() {
        let suggestion = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe cast".to_string(),
        );

        let decision = Decision::by_system(
            "dec1".to_string(),
            suggestion,
            true,
            "auto_accept_safe_casts".to_string(),
        );

        assert!(decision.accepted);
        match decision.source {
            DecisionSource::System { rule } => {
                assert_eq!(rule, "auto_accept_safe_casts");
            }
            _ => panic!("Expected System source"),
        }
    }

    #[test]
    fn test_decision_by_user() {
        let suggestion = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe cast".to_string(),
        );

        let decision = Decision::by_user(
            "dec1".to_string(),
            suggestion,
            false,
            "Not appropriate for this case".to_string(),
            "john.doe".to_string(),
        );

        assert!(!decision.accepted);
        match decision.source {
            DecisionSource::User { username } => {
                assert_eq!(username, "john.doe");
            }
            _ => panic!("Expected User source"),
        }
    }

    #[test]
    fn test_decision_status_transitions() {
        let suggestion = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe cast".to_string(),
        );

        let decision = Decision::accept(
            "dec1".to_string(),
            suggestion,
            "Approved".to_string(),
            DecisionSource::User {
                username: "admin".to_string(),
            },
        );

        let applied = decision.mark_applied();
        assert_eq!(applied.status, DecisionStatus::Applied);
    }

    #[test]
    fn test_decision_with_metadata() {
        let suggestion = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe cast".to_string(),
        );

        let metadata = serde_json::json!({
            "ticket": "JIRA-123",
            "reviewer": "jane.smith"
        });

        let decision = Decision::accept(
            "dec1".to_string(),
            suggestion,
            "Approved".to_string(),
            DecisionSource::User {
                username: "admin".to_string(),
            },
        )
        .with_metadata(metadata.clone());

        assert_eq!(decision.metadata, Some(metadata));
    }

    #[test]
    fn test_serialization() {
        let suggestion = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe cast".to_string(),
        );

        let decision = Decision::accept(
            "dec1".to_string(),
            suggestion,
            "Approved".to_string(),
            DecisionSource::User {
                username: "admin".to_string(),
            },
        );

        let json = serde_json::to_string(&decision).unwrap();
        let deserialized: Decision = serde_json::from_str(&json).unwrap();
        
        // Compare fields individually since DateTime might have slight differences
        assert_eq!(decision.id, deserialized.id);
        assert_eq!(decision.accepted, deserialized.accepted);
        assert_eq!(decision.rationale, deserialized.rationale);
        assert_eq!(decision.source, deserialized.source);
        assert_eq!(decision.status, deserialized.status);
    }
}
