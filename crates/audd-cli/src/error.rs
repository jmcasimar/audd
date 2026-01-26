//! Error types for the CLI

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Failed to load schema from source '{source}': {details}")]
    SchemaLoad {
        source: String,
        #[source]
        details: anyhow::Error,
    },

    #[error("Failed to create output directory '{path}': {details}")]
    OutputDirCreation {
        path: String,
        #[source]
        details: std::io::Error,
    },

    #[error("Failed to write output file '{path}': {details}")]
    OutputWrite {
        path: String,
        #[source]
        details: std::io::Error,
    },

    #[error("Invalid source format: {0}")]
    #[allow(dead_code)]
    InvalidSource(String),

    #[error("Comparison failed: {0}")]
    #[allow(dead_code)]
    ComparisonFailed(String),

    #[error("Resolution failed: {0}")]
    #[allow(dead_code)]
    ResolutionFailed(String),

    #[error("Failed to parse configuration file: {0}")]
    ConfigParseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type CliResult<T> = Result<T, CliError>;
