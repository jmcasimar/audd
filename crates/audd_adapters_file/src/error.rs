//! Error types for file adapters

use std::io;
use thiserror::Error;

/// Result type for adapter operations
pub type AdapterResult<T> = Result<T, AdapterError>;

/// Errors that can occur during schema adaptation
#[derive(Error, Debug)]
pub enum AdapterError {
    /// I/O error reading file
    #[error("Failed to read file: {0}")]
    IoError(#[from] io::Error),

    /// CSV parsing error
    #[error("CSV parsing error: {0}")]
    CsvError(#[from] csv::Error),

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// XML parsing error
    #[error("XML parsing error: {0}")]
    XmlError(String),

    /// SQL parsing error
    #[error("SQL parsing error: {0}")]
    SqlError(String),

    /// Unsupported file format
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),

    /// Invalid structure
    #[error("Invalid structure: {0}")]
    InvalidStructure(String),

    /// Empty data source
    #[error("Empty data source: {0}")]
    EmptyData(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AdapterError::UnsupportedFormat("xyz".to_string());
        assert_eq!(err.to_string(), "Unsupported file format: xyz");

        let err = AdapterError::InvalidStructure("no headers".to_string());
        assert_eq!(err.to_string(), "Invalid structure: no headers");
    }
}
