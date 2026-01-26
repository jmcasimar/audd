//! Comparison result structures

use serde::{Deserialize, Serialize};

/// Reason why two fields/entities were matched
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "snake_case")]
pub enum MatchReason {
    /// Exact name match
    ExactName,
    /// Normalized name match (case-insensitive, etc.)
    NormalizedName {
        original_a: String,
        original_b: String,
    },
    /// Similarity-based match
    Similarity { score: f64 },
    /// Semantic match with detailed scoring
    Semantic {
        score: f64,
        decision: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<serde_json::Value>,
    },
}

/// A matched field or entity between schemas A and B
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Match {
    /// Name of the matched entity
    pub entity_name: String,

    /// Name of the matched field (None for entity-level matches)
    pub field_name: Option<String>,

    /// Reason for the match
    pub reason: MatchReason,

    /// Match confidence score (0.0 to 1.0)
    pub score: f64,

    /// Index in schema A
    pub index_a: usize,

    /// Index in schema B
    pub index_b: usize,
}

impl Match {
    /// Create a new match
    pub fn new(
        entity_name: String,
        field_name: Option<String>,
        reason: MatchReason,
        score: f64,
        index_a: usize,
        index_b: usize,
    ) -> Self {
        Self {
            entity_name,
            field_name,
            reason,
            score,
            index_a,
            index_b,
        }
    }

    /// Create an exact name match
    pub fn exact(
        entity_name: String,
        field_name: Option<String>,
        index_a: usize,
        index_b: usize,
    ) -> Self {
        Self::new(
            entity_name,
            field_name,
            MatchReason::ExactName,
            1.0,
            index_a,
            index_b,
        )
    }

    /// Create a normalized name match
    pub fn normalized(
        entity_name: String,
        field_name: Option<String>,
        original_a: String,
        original_b: String,
        index_a: usize,
        index_b: usize,
    ) -> Self {
        Self::new(
            entity_name,
            field_name,
            MatchReason::NormalizedName {
                original_a,
                original_b,
            },
            0.9,
            index_a,
            index_b,
        )
    }

    /// Create a similarity match
    pub fn similarity(
        entity_name: String,
        field_name: Option<String>,
        score: f64,
        index_a: usize,
        index_b: usize,
    ) -> Self {
        Self::new(
            entity_name,
            field_name,
            MatchReason::Similarity { score },
            score,
            index_a,
            index_b,
        )
    }

    /// Create a semantic match
    pub fn semantic(
        entity_name: String,
        field_name: Option<String>,
        score: f64,
        decision: String,
        details: Option<serde_json::Value>,
        index_a: usize,
        index_b: usize,
    ) -> Self {
        Self::new(
            entity_name,
            field_name,
            MatchReason::Semantic {
                score,
                decision,
                details,
            },
            score,
            index_a,
            index_b,
        )
    }
}

/// Side indicator for exclusive items
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ExclusiveSide {
    /// Present only in schema A
    A,
    /// Present only in schema B
    B,
}

/// A field or entity present in only one schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Exclusive {
    /// Name of the entity
    pub entity_name: String,

    /// Name of the field (None for entity-level exclusives)
    pub field_name: Option<String>,

    /// Which schema contains this exclusive
    pub side: ExclusiveSide,

    /// Index in the source schema
    pub index: usize,

    /// Whether this exclusive can be safely added to the unified schema
    pub safe_to_add: bool,
}

impl Exclusive {
    /// Create a new exclusive
    pub fn new(
        entity_name: String,
        field_name: Option<String>,
        side: ExclusiveSide,
        index: usize,
    ) -> Self {
        Self {
            entity_name,
            field_name,
            side,
            index,
            safe_to_add: true, // Default to safe; can be changed based on analysis
        }
    }

    /// Create an exclusive from schema A
    pub fn from_a(entity_name: String, field_name: Option<String>, index: usize) -> Self {
        Self::new(entity_name, field_name, ExclusiveSide::A, index)
    }

    /// Create an exclusive from schema B
    pub fn from_b(entity_name: String, field_name: Option<String>, index: usize) -> Self {
        Self::new(entity_name, field_name, ExclusiveSide::B, index)
    }
}

/// Complete result of comparing two schemas
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComparisonResult {
    /// Successfully matched fields/entities
    pub matches: Vec<Match>,

    /// Fields/entities present in only one schema
    pub exclusives: Vec<Exclusive>,

    /// Conflicts that require resolution
    pub conflicts: Vec<crate::Conflict>,

    /// Summary statistics
    pub summary: ComparisonSummary,
}

impl ComparisonResult {
    /// Create a new comparison result
    pub fn new(
        matches: Vec<Match>,
        exclusives: Vec<Exclusive>,
        conflicts: Vec<crate::Conflict>,
    ) -> Self {
        let summary = ComparisonSummary {
            total_matches: matches.len(),
            total_exclusives: exclusives.len(),
            total_conflicts: conflicts.len(),
            exclusives_a: exclusives
                .iter()
                .filter(|e| e.side == ExclusiveSide::A)
                .count(),
            exclusives_b: exclusives
                .iter()
                .filter(|e| e.side == ExclusiveSide::B)
                .count(),
        };

        Self {
            matches,
            exclusives,
            conflicts,
            summary,
        }
    }
}

/// Summary statistics for a comparison
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComparisonSummary {
    /// Total number of matches
    pub total_matches: usize,

    /// Total number of exclusives
    pub total_exclusives: usize,

    /// Total number of conflicts
    pub total_conflicts: usize,

    /// Exclusives from schema A
    pub exclusives_a: usize,

    /// Exclusives from schema B
    pub exclusives_b: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_creation() {
        let m = Match::exact("users".to_string(), Some("id".to_string()), 0, 0);
        assert_eq!(m.entity_name, "users");
        assert_eq!(m.field_name, Some("id".to_string()));
        assert_eq!(m.score, 1.0);
        assert!(matches!(m.reason, MatchReason::ExactName));
    }

    #[test]
    fn test_normalized_match() {
        let m = Match::normalized(
            "users".to_string(),
            Some("email".to_string()),
            "Email".to_string(),
            "email".to_string(),
            0,
            1,
        );
        assert_eq!(m.score, 0.9);
        match m.reason {
            MatchReason::NormalizedName {
                original_a,
                original_b,
            } => {
                assert_eq!(original_a, "Email");
                assert_eq!(original_b, "email");
            }
            _ => panic!("Expected NormalizedName"),
        }
    }

    #[test]
    fn test_exclusive_creation() {
        let e = Exclusive::from_a("users".to_string(), Some("password".to_string()), 5);
        assert_eq!(e.entity_name, "users");
        assert_eq!(e.field_name, Some("password".to_string()));
        assert_eq!(e.side, ExclusiveSide::A);
        assert!(e.safe_to_add);
    }

    #[test]
    fn test_comparison_result() {
        let matches = vec![Match::exact("users".to_string(), None, 0, 0)];
        let exclusives = vec![
            Exclusive::from_a("posts".to_string(), None, 1),
            Exclusive::from_b("comments".to_string(), None, 2),
        ];
        let conflicts = vec![];

        let result = ComparisonResult::new(matches, exclusives, conflicts);

        assert_eq!(result.summary.total_matches, 1);
        assert_eq!(result.summary.total_exclusives, 2);
        assert_eq!(result.summary.total_conflicts, 0);
        assert_eq!(result.summary.exclusives_a, 1);
        assert_eq!(result.summary.exclusives_b, 1);
    }

    #[test]
    fn test_serialization() {
        let m = Match::exact("users".to_string(), Some("id".to_string()), 0, 0);
        let json = serde_json::to_string(&m).unwrap();
        let deserialized: Match = serde_json::from_str(&json).unwrap();
        assert_eq!(m, deserialized);
    }
}
