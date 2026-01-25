//! Error types for database adapters

use std::fmt;

/// Result type for database adapter operations
pub type DbResult<T> = Result<T, DbError>;

/// Errors that can occur during database schema extraction
#[derive(Debug)]
pub enum DbError {
    /// Connection error
    ConnectionError(String),
    
    /// Query execution error
    QueryError(String),
    
    /// Invalid connection string
    InvalidConnectionString(String),
    
    /// Unsupported database engine
    UnsupportedEngine(String),
    
    /// Schema extraction error
    ExtractionError(String),
    
    /// Type mapping error
    TypeMappingError(String),
    
    /// Feature not enabled
    FeatureNotEnabled(String),
    
    /// Other errors
    Other(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionError(msg) => write!(f, "Database connection error: {}", msg),
            Self::QueryError(msg) => write!(f, "Query execution error: {}", msg),
            Self::InvalidConnectionString(msg) => {
                write!(f, "Invalid connection string: {}. ", msg)?;
                write!(
                    f,
                    "Expected format: sqlite://<path> or mysql://<user>:<pass>@<host>/<db>"
                )
            }
            Self::UnsupportedEngine(engine) => {
                write!(f, "Unsupported database engine: {}", engine)?;
                write!(f, " (Supported: sqlite, mysql)")
            }
            Self::ExtractionError(msg) => write!(f, "Schema extraction error: {}", msg),
            Self::TypeMappingError(msg) => write!(f, "Type mapping error: {}", msg),
            Self::FeatureNotEnabled(feature) => {
                write!(
                    f,
                    "Feature '{}' not enabled. Enable it in Cargo.toml features",
                    feature
                )
            }
            Self::Other(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for DbError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = DbError::ConnectionError("connection refused".to_string());
        assert!(err.to_string().contains("connection refused"));

        let err = DbError::UnsupportedEngine("oracle".to_string());
        assert!(err.to_string().contains("oracle"));
        assert!(err.to_string().contains("Supported"));
    }

    #[test]
    fn test_invalid_connection_string() {
        let err = DbError::InvalidConnectionString("malformed".to_string());
        let msg = err.to_string();
        assert!(msg.contains("malformed"));
        assert!(msg.contains("Expected format"));
    }
}
