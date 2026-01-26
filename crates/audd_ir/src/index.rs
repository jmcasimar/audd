//! Index definitions for database schemas

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Type of index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IndexType {
    /// Regular non-unique index
    Regular,
    /// Unique index
    Unique,
    /// Full-text index
    FullText,
    /// Spatial index
    Spatial,
}

/// Index definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Index {
    /// Name of the index
    pub index_name: String,

    /// Type of index
    pub index_type: IndexType,

    /// Fields that comprise this index
    pub field_names: Vec<String>,

    /// Whether this is a partial/filtered index
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_condition: Option<String>,

    /// Additional index metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, Value>,
}

impl Index {
    /// Create a new index
    pub fn new(index_name: String, index_type: IndexType, field_names: Vec<String>) -> Self {
        Self {
            index_name,
            index_type,
            field_names,
            filter_condition: None,
            metadata: HashMap::new(),
        }
    }

    /// Create a regular index
    pub fn regular<S: Into<String>, I, F>(name: S, field_names: I) -> Self
    where
        I: IntoIterator<Item = F>,
        F: Into<String>,
    {
        Self::new(
            name.into(),
            IndexType::Regular,
            field_names.into_iter().map(|s| s.into()).collect(),
        )
    }

    /// Create a unique index
    pub fn unique<S: Into<String>, I, F>(name: S, field_names: I) -> Self
    where
        I: IntoIterator<Item = F>,
        F: Into<String>,
    {
        Self::new(
            name.into(),
            IndexType::Unique,
            field_names.into_iter().map(|s| s.into()).collect(),
        )
    }

    /// Add a filter condition to this index
    pub fn with_filter(mut self, condition: String) -> Self {
        self.filter_condition = Some(condition);
        self
    }

    /// Add metadata to this index
    pub fn with_metadata(mut self, key: String, value: Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regular_index_creation() {
        let idx = Index::regular("idx_email", vec!["email"]);
        assert_eq!(idx.index_name, "idx_email");
        assert_eq!(idx.index_type, IndexType::Regular);
        assert_eq!(idx.field_names, vec!["email"]);
    }

    #[test]
    fn test_composite_index() {
        let idx = Index::regular("idx_name", vec!["first_name", "last_name"]);
        assert_eq!(idx.field_names.len(), 2);
    }

    #[test]
    fn test_unique_index() {
        let idx = Index::unique("idx_username", vec!["username"]);
        assert_eq!(idx.index_type, IndexType::Unique);
    }

    #[test]
    fn test_filtered_index() {
        let idx = Index::regular("idx_active_users", vec!["created_at"])
            .with_filter("active = true".to_string());
        assert!(idx.filter_condition.is_some());
        assert_eq!(idx.filter_condition.unwrap(), "active = true");
    }

    #[test]
    fn test_index_serialization() {
        let idx = Index::regular("idx_test", vec!["field1", "field2"]);
        let json = serde_json::to_string(&idx).unwrap();
        let deserialized: Index = serde_json::from_str(&json).unwrap();
        assert_eq!(idx, deserialized);
    }
}
