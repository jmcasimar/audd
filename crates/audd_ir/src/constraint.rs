//! Field constraints and validation rules

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Constraints that can be applied to fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "constraint_type", rename_all = "PascalCase")]
pub enum Constraint {
    /// Maximum string length
    MaxLength { value: usize },

    /// Minimum string length
    MinLength { value: usize },

    /// Numeric precision and scale for decimals
    Precision { precision: u16, scale: u16 },

    /// Field must have unique values
    Unique,

    /// Default value for the field
    DefaultValue { value: Value },

    /// Check constraint (stored as expression string for MVP)
    Check { expression: String },
}

impl Constraint {
    /// Create a MaxLength constraint
    pub fn max_length(value: usize) -> Self {
        Self::MaxLength { value }
    }

    /// Create a MinLength constraint
    pub fn min_length(value: usize) -> Self {
        Self::MinLength { value }
    }

    /// Create a Precision constraint
    pub fn precision(precision: u16, scale: u16) -> Self {
        Self::Precision { precision, scale }
    }

    /// Create a Unique constraint
    pub fn unique() -> Self {
        Self::Unique
    }

    /// Create a DefaultValue constraint
    pub fn default_value(value: Value) -> Self {
        Self::DefaultValue { value }
    }

    /// Create a Check constraint
    pub fn check(expression: String) -> Self {
        Self::Check { expression }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_creation() {
        let max_len = Constraint::max_length(255);
        assert!(matches!(max_len, Constraint::MaxLength { value: 255 }));

        let unique = Constraint::unique();
        assert!(matches!(unique, Constraint::Unique));
    }

    #[test]
    fn test_constraint_serialization() {
        let constraint = Constraint::MaxLength { value: 100 };
        let json = serde_json::to_string(&constraint).unwrap();
        assert!(json.contains("MaxLength"));
        assert!(json.contains("100"));

        let deserialized: Constraint = serde_json::from_str(&json).unwrap();
        assert_eq!(constraint, deserialized);
    }

    #[test]
    fn test_default_value_constraint() {
        let default = Constraint::default_value(Value::String("test".to_string()));
        let json = serde_json::to_string(&default).unwrap();
        let deserialized: Constraint = serde_json::from_str(&json).unwrap();
        assert_eq!(default, deserialized);
    }
}
