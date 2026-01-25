//! Decision log for tracking all decisions in a comparison session

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::decision::Decision;

/// Version of the decision log format
const DECISION_LOG_VERSION: &str = "1.0.0";

/// Metadata about a decision log
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionLogMetadata {
    /// Version of the log format
    pub version: String,

    /// Timestamp when the log was created
    pub created_at: DateTime<Utc>,

    /// Timestamp when the log was last updated
    pub updated_at: DateTime<Utc>,

    /// Total number of decisions
    pub total_decisions: usize,

    /// Number of accepted decisions
    pub accepted_decisions: usize,

    /// Number of rejected decisions
    pub rejected_decisions: usize,

    /// Source schema A identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_a_id: Option<String>,

    /// Source schema B identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_b_id: Option<String>,

    /// Additional custom metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<serde_json::Value>,
}

impl DecisionLogMetadata {
    /// Create new metadata
    fn new() -> Self {
        let now = Utc::now();
        Self {
            version: DECISION_LOG_VERSION.to_string(),
            created_at: now,
            updated_at: now,
            total_decisions: 0,
            accepted_decisions: 0,
            rejected_decisions: 0,
            schema_a_id: None,
            schema_b_id: None,
            custom: None,
        }
    }

    /// Update counts based on decisions
    fn update_counts(&mut self, decisions: &[Decision]) {
        self.total_decisions = decisions.len();
        self.accepted_decisions = decisions.iter().filter(|d| d.accepted).count();
        self.rejected_decisions = decisions.iter().filter(|d| !d.accepted).count();
        self.updated_at = Utc::now();
    }
}

/// A log of all decisions made during schema comparison and resolution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionLog {
    /// Metadata about this log
    pub metadata: DecisionLogMetadata,

    /// List of decisions, in chronological order
    pub decisions: Vec<Decision>,
}

impl DecisionLog {
    /// Create a new empty decision log
    pub fn new() -> Self {
        Self {
            metadata: DecisionLogMetadata::new(),
            decisions: Vec::new(),
        }
    }

    /// Add a decision to the log
    pub fn add_decision(&mut self, decision: Decision) {
        self.decisions.push(decision);
        self.metadata.update_counts(&self.decisions);
    }

    /// Add multiple decisions to the log
    pub fn add_decisions(&mut self, decisions: Vec<Decision>) {
        self.decisions.extend(decisions);
        self.metadata.update_counts(&self.decisions);
    }

    /// Get all decisions
    pub fn get_decisions(&self) -> &[Decision] {
        &self.decisions
    }

    /// Get decisions for a specific entity
    pub fn get_decisions_for_entity(&self, entity_name: &str) -> Vec<&Decision> {
        self.decisions
            .iter()
            .filter(|d| d.suggestion.entity_name == entity_name)
            .collect()
    }

    /// Get accepted decisions
    pub fn get_accepted_decisions(&self) -> Vec<&Decision> {
        self.decisions.iter().filter(|d| d.accepted).collect()
    }

    /// Get rejected decisions
    pub fn get_rejected_decisions(&self) -> Vec<&Decision> {
        self.decisions.iter().filter(|d| !d.accepted).collect()
    }

    /// Set schema identifiers
    pub fn with_schema_ids(mut self, schema_a_id: String, schema_b_id: String) -> Self {
        self.metadata.schema_a_id = Some(schema_a_id);
        self.metadata.schema_b_id = Some(schema_b_id);
        self
    }

    /// Add custom metadata
    pub fn with_custom_metadata(mut self, custom: serde_json::Value) -> Self {
        self.metadata.custom = Some(custom);
        self
    }

    /// Export the log to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Export the log to a JSON file
    pub fn to_json_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json()?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load a decision log from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Load a decision log from a JSON file
    pub fn from_json_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        Ok(Self::from_json(&json)?)
    }

    /// Generate a markdown summary of the decision log
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Decision Log\n\n");
        md.push_str(&format!("**Version:** {}\n", self.metadata.version));
        md.push_str(&format!("**Created:** {}\n", self.metadata.created_at));
        md.push_str(&format!("**Updated:** {}\n\n", self.metadata.updated_at));

        md.push_str("## Summary\n\n");
        md.push_str(&format!(
            "- Total Decisions: {}\n",
            self.metadata.total_decisions
        ));
        md.push_str(&format!(
            "- Accepted: {}\n",
            self.metadata.accepted_decisions
        ));
        md.push_str(&format!(
            "- Rejected: {}\n\n",
            self.metadata.rejected_decisions
        ));

        if !self.decisions.is_empty() {
            md.push_str("## Decisions\n\n");
            for (i, decision) in self.decisions.iter().enumerate() {
                md.push_str(&format!("### {}. Decision {}\n\n", i + 1, decision.id));
                md.push_str(&format!(
                    "- **Status:** {}\n",
                    if decision.accepted {
                        "✅ Accepted"
                    } else {
                        "❌ Rejected"
                    }
                ));
                md.push_str(&format!(
                    "- **Entity:** {}\n",
                    decision.suggestion.entity_name
                ));
                if let Some(ref field) = decision.suggestion.field_name {
                    md.push_str(&format!("- **Field:** {}\n", field));
                }
                md.push_str(&format!("- **Rationale:** {}\n", decision.rationale));
                md.push_str(&format!("- **Timestamp:** {}\n\n", decision.timestamp));
            }
        }

        md
    }
}

impl Default for DecisionLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decision::{Decision, DecisionSource};
    use crate::suggestion::Suggestion;

    #[test]
    fn test_decision_log_creation() {
        let log = DecisionLog::new();
        assert_eq!(log.decisions.len(), 0);
        assert_eq!(log.metadata.total_decisions, 0);
        assert_eq!(log.metadata.version, DECISION_LOG_VERSION);
    }

    #[test]
    fn test_add_decision() {
        let mut log = DecisionLog::new();
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
            "Good choice".to_string(),
            DecisionSource::User {
                username: "admin".to_string(),
            },
        );

        log.add_decision(decision);

        assert_eq!(log.metadata.total_decisions, 1);
        assert_eq!(log.metadata.accepted_decisions, 1);
        assert_eq!(log.metadata.rejected_decisions, 0);
    }

    #[test]
    fn test_add_multiple_decisions() {
        let mut log = DecisionLog::new();

        let suggestion1 = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe cast".to_string(),
        );

        let suggestion2 = Suggestion::cast_risky(
            "sug2".to_string(),
            "users".to_string(),
            "age".to_string(),
            "Int64".to_string(),
            "Int32".to_string(),
            "Potential overflow".to_string(),
            "Risky cast".to_string(),
        );

        let decisions = vec![
            Decision::accept(
                "dec1".to_string(),
                suggestion1,
                "Good".to_string(),
                DecisionSource::User {
                    username: "admin".to_string(),
                },
            ),
            Decision::reject(
                "dec2".to_string(),
                suggestion2,
                "Too risky".to_string(),
                DecisionSource::User {
                    username: "admin".to_string(),
                },
            ),
        ];

        log.add_decisions(decisions);

        assert_eq!(log.metadata.total_decisions, 2);
        assert_eq!(log.metadata.accepted_decisions, 1);
        assert_eq!(log.metadata.rejected_decisions, 1);
    }

    #[test]
    fn test_get_decisions_for_entity() {
        let mut log = DecisionLog::new();

        let suggestion1 = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe cast".to_string(),
        );

        let suggestion2 = Suggestion::cast_safe(
            "sug2".to_string(),
            "posts".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe cast".to_string(),
        );

        log.add_decision(Decision::accept(
            "dec1".to_string(),
            suggestion1,
            "Good".to_string(),
            DecisionSource::User {
                username: "admin".to_string(),
            },
        ));

        log.add_decision(Decision::accept(
            "dec2".to_string(),
            suggestion2,
            "Good".to_string(),
            DecisionSource::User {
                username: "admin".to_string(),
            },
        ));

        let users_decisions = log.get_decisions_for_entity("users");
        assert_eq!(users_decisions.len(), 1);
        assert_eq!(users_decisions[0].suggestion.entity_name, "users");
    }

    #[test]
    fn test_get_accepted_and_rejected() {
        let mut log = DecisionLog::new();

        let suggestion1 = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe cast".to_string(),
        );

        let suggestion2 = Suggestion::cast_risky(
            "sug2".to_string(),
            "users".to_string(),
            "age".to_string(),
            "Int64".to_string(),
            "Int32".to_string(),
            "Potential overflow".to_string(),
            "Risky cast".to_string(),
        );

        log.add_decision(Decision::accept(
            "dec1".to_string(),
            suggestion1,
            "Good".to_string(),
            DecisionSource::User {
                username: "admin".to_string(),
            },
        ));

        log.add_decision(Decision::reject(
            "dec2".to_string(),
            suggestion2,
            "Too risky".to_string(),
            DecisionSource::User {
                username: "admin".to_string(),
            },
        ));

        assert_eq!(log.get_accepted_decisions().len(), 1);
        assert_eq!(log.get_rejected_decisions().len(), 1);
    }

    #[test]
    fn test_with_schema_ids() {
        let log = DecisionLog::new().with_schema_ids("schema_a".to_string(), "schema_b".to_string());

        assert_eq!(log.metadata.schema_a_id, Some("schema_a".to_string()));
        assert_eq!(log.metadata.schema_b_id, Some("schema_b".to_string()));
    }

    #[test]
    fn test_json_serialization() {
        let mut log = DecisionLog::new();
        let suggestion = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe cast".to_string(),
        );

        log.add_decision(Decision::accept(
            "dec1".to_string(),
            suggestion,
            "Good".to_string(),
            DecisionSource::User {
                username: "admin".to_string(),
            },
        ));

        let json = log.to_json().unwrap();
        let deserialized = DecisionLog::from_json(&json).unwrap();

        assert_eq!(log.metadata.total_decisions, deserialized.metadata.total_decisions);
        assert_eq!(log.decisions.len(), deserialized.decisions.len());
    }

    #[test]
    fn test_markdown_generation() {
        let mut log = DecisionLog::new();
        let suggestion = Suggestion::cast_safe(
            "sug1".to_string(),
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            "Safe cast".to_string(),
        );

        log.add_decision(Decision::accept(
            "dec1".to_string(),
            suggestion,
            "Good choice".to_string(),
            DecisionSource::User {
                username: "admin".to_string(),
            },
        ));

        let md = log.to_markdown();
        assert!(md.contains("# Decision Log"));
        assert!(md.contains("Total Decisions: 1"));
        assert!(md.contains("✅ Accepted"));
        assert!(md.contains("users"));
    }
}
