//! Configuration for resolution preferences and policies

use serde::{Deserialize, Serialize};

/// Policy for choosing between types when there's a conflict
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypePreferencePolicy {
    /// Prefer larger/wider types (e.g., Int64 over Int32)
    PreferLarger,
    /// Prefer smaller/narrower types (e.g., Int32 over Int64)
    PreferSmaller,
    /// Prefer higher precision types (e.g., Double over Float)
    PreferHigherPrecision,
    /// Prefer lower precision types
    PreferLowerPrecision,
    /// Prefer types from schema A
    PreferSchemaA,
    /// Prefer types from schema B
    PreferSchemaB,
    /// Custom preference defined by user
    Custom { rule: String },
}

/// Policy for nullability conflicts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NullabilityPolicy {
    /// Always prefer nullable (more permissive)
    AlwaysNullable,
    /// Always prefer not null (more restrictive)
    AlwaysNotNull,
    /// Prefer nullability from schema A
    PreferSchemaA,
    /// Prefer nullability from schema B
    PreferSchemaB,
}

/// Policy for string/text length conflicts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LengthPolicy {
    /// Use maximum length from either schema
    UseMaximum,
    /// Use minimum length from either schema
    UseMinimum,
    /// Prefer length from schema A
    PreferSchemaA,
    /// Prefer length from schema B
    PreferSchemaB,
}

/// Resolution configuration and policies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolutionConfig {
    /// Policy for type preference
    pub type_preference: TypePreferencePolicy,

    /// Policy for nullability
    pub nullability_policy: NullabilityPolicy,

    /// Policy for length/precision
    pub length_policy: LengthPolicy,

    /// Whether to allow risky suggestions
    pub allow_risky_suggestions: bool,

    /// Minimum confidence threshold for suggestions (0.0-1.0)
    pub min_confidence: f64,

    /// Whether to generate multiple alternative suggestions
    pub generate_alternatives: bool,
}

impl ResolutionConfig {
    /// Create a new configuration with default policies
    pub fn new() -> Self {
        Self {
            type_preference: TypePreferencePolicy::PreferLarger,
            nullability_policy: NullabilityPolicy::AlwaysNullable,
            length_policy: LengthPolicy::UseMaximum,
            allow_risky_suggestions: true,
            min_confidence: 0.0,
            generate_alternatives: true,
        }
    }

    /// Create a conservative configuration (safe suggestions only)
    pub fn conservative() -> Self {
        Self {
            type_preference: TypePreferencePolicy::PreferLarger,
            nullability_policy: NullabilityPolicy::AlwaysNullable,
            length_policy: LengthPolicy::UseMaximum,
            allow_risky_suggestions: false,
            min_confidence: 0.7,
            generate_alternatives: false,
        }
    }

    /// Create configuration that prefers schema A
    pub fn prefer_schema_a() -> Self {
        Self {
            type_preference: TypePreferencePolicy::PreferSchemaA,
            nullability_policy: NullabilityPolicy::PreferSchemaA,
            length_policy: LengthPolicy::PreferSchemaA,
            allow_risky_suggestions: true,
            min_confidence: 0.0,
            generate_alternatives: true,
        }
    }

    /// Create configuration that prefers schema B
    pub fn prefer_schema_b() -> Self {
        Self {
            type_preference: TypePreferencePolicy::PreferSchemaB,
            nullability_policy: NullabilityPolicy::PreferSchemaB,
            length_policy: LengthPolicy::PreferSchemaB,
            allow_risky_suggestions: true,
            min_confidence: 0.0,
            generate_alternatives: true,
        }
    }

    /// Load configuration from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Load configuration from JSON file
    pub fn from_json_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        Ok(Self::from_json(&json)?)
    }

    /// Export configuration to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Export configuration to JSON file
    pub fn to_json_file(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json()?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

impl Default for ResolutionConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ResolutionConfig::default();
        assert_eq!(config.type_preference, TypePreferencePolicy::PreferLarger);
        assert_eq!(
            config.nullability_policy,
            NullabilityPolicy::AlwaysNullable
        );
        assert!(config.allow_risky_suggestions);
    }

    #[test]
    fn test_conservative_config() {
        let config = ResolutionConfig::conservative();
        assert!(!config.allow_risky_suggestions);
        assert_eq!(config.min_confidence, 0.7);
        assert!(!config.generate_alternatives);
    }

    #[test]
    fn test_prefer_schema_a_config() {
        let config = ResolutionConfig::prefer_schema_a();
        assert_eq!(
            config.type_preference,
            TypePreferencePolicy::PreferSchemaA
        );
        assert_eq!(config.nullability_policy, NullabilityPolicy::PreferSchemaA);
        assert_eq!(config.length_policy, LengthPolicy::PreferSchemaA);
    }

    #[test]
    fn test_prefer_schema_b_config() {
        let config = ResolutionConfig::prefer_schema_b();
        assert_eq!(
            config.type_preference,
            TypePreferencePolicy::PreferSchemaB
        );
        assert_eq!(config.nullability_policy, NullabilityPolicy::PreferSchemaB);
        assert_eq!(config.length_policy, LengthPolicy::PreferSchemaB);
    }

    #[test]
    fn test_json_serialization() {
        let config = ResolutionConfig::default();
        let json = config.to_json().unwrap();
        let deserialized = ResolutionConfig::from_json(&json).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_custom_policy() {
        let policy = TypePreferencePolicy::Custom {
            rule: "prefer_most_recent".to_string(),
        };

        assert!(matches!(policy, TypePreferencePolicy::Custom { .. }));
    }
}
