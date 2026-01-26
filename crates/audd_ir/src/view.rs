//! View and stored object definitions

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Database view definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct View {
    /// Name of the view
    pub view_name: String,

    /// SQL definition of the view (if available)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,

    /// Fields in the view (inferred or extracted)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub field_names: Vec<String>,

    /// Whether this is a materialized view
    #[serde(default)]
    pub is_materialized: bool,

    /// Additional view metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, Value>,
}

impl View {
    /// Create a new view
    pub fn new(view_name: String) -> Self {
        Self {
            view_name,
            definition: None,
            field_names: Vec::new(),
            is_materialized: false,
            metadata: HashMap::new(),
        }
    }

    /// Set the view definition
    pub fn with_definition(mut self, definition: String) -> Self {
        self.definition = Some(definition);
        self
    }

    /// Set the field names
    pub fn with_fields<I, S>(mut self, field_names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.field_names = field_names.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Mark as materialized view
    pub fn materialized(mut self) -> Self {
        self.is_materialized = true;
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Stored procedure or function definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoredProcedure {
    /// Name of the procedure/function
    pub name: String,

    /// Type (procedure, function, trigger function, etc.)
    pub procedure_type: String,

    /// Definition/body (if available)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,

    /// Parameter names and types (simplified)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<String>,

    /// Return type (for functions)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_type: Option<String>,

    /// Additional metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, Value>,
}

impl StoredProcedure {
    /// Create a new stored procedure
    pub fn new(name: String, procedure_type: String) -> Self {
        Self {
            name,
            procedure_type,
            definition: None,
            parameters: Vec::new(),
            return_type: None,
            metadata: HashMap::new(),
        }
    }

    /// Set the procedure definition
    pub fn with_definition(mut self, definition: String) -> Self {
        self.definition = Some(definition);
        self
    }

    /// Set parameters
    pub fn with_parameters<I, S>(mut self, parameters: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.parameters = parameters.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Set return type
    pub fn with_return_type(mut self, return_type: String) -> Self {
        self.return_type = Some(return_type);
        self
    }
}

/// Trigger definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trigger {
    /// Name of the trigger
    pub trigger_name: String,

    /// Table/entity this trigger is attached to
    pub table_name: String,

    /// Timing (BEFORE, AFTER, INSTEAD OF)
    pub timing: String,

    /// Event (INSERT, UPDATE, DELETE)
    pub event: String,

    /// Trigger definition/body (if available)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,

    /// Additional metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, Value>,
}

impl Trigger {
    /// Create a new trigger
    pub fn new(trigger_name: String, table_name: String, timing: String, event: String) -> Self {
        Self {
            trigger_name,
            table_name,
            timing,
            event,
            definition: None,
            metadata: HashMap::new(),
        }
    }

    /// Set the trigger definition
    pub fn with_definition(mut self, definition: String) -> Self {
        self.definition = Some(definition);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_creation() {
        let view = View::new("vw_active_users".to_string())
            .with_definition("SELECT * FROM users WHERE active = true".to_string())
            .with_fields(vec!["id", "name", "email"]);
        
        assert_eq!(view.view_name, "vw_active_users");
        assert!(view.definition.is_some());
        assert_eq!(view.field_names.len(), 3);
    }

    #[test]
    fn test_materialized_view() {
        let view = View::new("mv_stats".to_string()).materialized();
        assert!(view.is_materialized);
    }

    #[test]
    fn test_stored_procedure() {
        let proc = StoredProcedure::new("sp_get_user".to_string(), "function".to_string())
            .with_parameters(vec!["user_id INT"])
            .with_return_type("TABLE".to_string());
        
        assert_eq!(proc.name, "sp_get_user");
        assert_eq!(proc.parameters.len(), 1);
        assert!(proc.return_type.is_some());
    }

    #[test]
    fn test_trigger() {
        let trigger = Trigger::new(
            "tr_update_timestamp".to_string(),
            "users".to_string(),
            "BEFORE".to_string(),
            "UPDATE".to_string(),
        );
        
        assert_eq!(trigger.trigger_name, "tr_update_timestamp");
        assert_eq!(trigger.table_name, "users");
        assert_eq!(trigger.timing, "BEFORE");
        assert_eq!(trigger.event, "UPDATE");
    }
}
