//! Key definitions (primary, unique, foreign)

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Type of key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeyType {
    /// Primary key
    Primary,
    /// Unique key
    Unique,
    /// Foreign key (limited support in MVP)
    Foreign,
}

/// Key definition (primary, unique, foreign)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Key {
    /// Type of key
    pub key_type: KeyType,

    /// Fields that comprise this key
    pub field_names: Vec<String>,

    /// Additional key metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, Value>,
}

impl Key {
    /// Create a new key
    pub fn new(key_type: KeyType, field_names: Vec<String>) -> Self {
        Self {
            key_type,
            field_names,
            metadata: HashMap::new(),
        }
    }

    /// Create a primary key
    pub fn primary<I, S>(field_names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::new(
            KeyType::Primary,
            field_names.into_iter().map(|s| s.into()).collect(),
        )
    }

    /// Create a unique key
    pub fn unique<I, S>(field_names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::new(
            KeyType::Unique,
            field_names.into_iter().map(|s| s.into()).collect(),
        )
    }

    /// Create a foreign key
    pub fn foreign<I, S>(field_names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::new(
            KeyType::Foreign,
            field_names.into_iter().map(|s| s.into()).collect(),
        )
    }

    /// Add metadata to this key
    pub fn with_metadata(mut self, key: String, value: Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_key_creation() {
        let key = Key::primary(vec!["id"]);
        assert_eq!(key.key_type, KeyType::Primary);
        assert_eq!(key.field_names, vec!["id"]);
    }

    #[test]
    fn test_composite_key() {
        let key = Key::unique(vec!["first_name", "last_name"]);
        assert_eq!(key.key_type, KeyType::Unique);
        assert_eq!(key.field_names.len(), 2);
    }

    #[test]
    fn test_key_serialization() {
        let key = Key::primary(vec!["user_id"]);
        let json = serde_json::to_string(&key).unwrap();
        let deserialized: Key = serde_json::from_str(&json).unwrap();
        assert_eq!(key, deserialized);
    }

    #[test]
    fn test_key_with_metadata() {
        let key = Key::primary(vec!["id"]).with_metadata(
            "index_name".to_string(),
            Value::String("pk_users".to_string()),
        );
        assert!(key.metadata.contains_key("index_name"));
    }
}
