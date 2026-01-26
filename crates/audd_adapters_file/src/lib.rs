//! AUDD File Adapters
//!
//! This crate provides adapters that read schemas from common file formats
//! and convert them to the AUDD Intermediate Representation (IR).
//!
//! # Supported Formats
//!
//! - **CSV**: Headers → fields, with optional type inference
//! - **JSON**: Flat objects or arrays with homogeneous keys
//! - **XML**: Basic tags and attributes (MVP subset)
//! - **SQL/DDL**: Subset of CREATE TABLE statements
//! - **Remote Files**: HTTP/HTTPS URLs and Google Sheets (public)
//!
//! # Example
//!
//! ```no_run
//! use audd_adapters_file::{load_schema_from_file, AdapterError};
//!
//! # fn main() -> Result<(), AdapterError> {
//! // Auto-detect format from extension
//! let schema = load_schema_from_file("data.csv")?;
//! println!("Loaded schema: {}", schema.source_name);
//! # Ok(())
//! # }
//! ```
//!
//! # Remote Files Example
//!
//! ```no_run
//! use audd_adapters_file::{RemoteAdapter, SchemaAdapter, AdapterError};
//!
//! # fn main() -> Result<(), AdapterError> {
//! // Load from HTTP URL
//! let adapter = RemoteAdapter::new("https://example.com/data.csv");
//! let schema = adapter.load_schema()?;
//!
//! // Load from Google Sheets (public)
//! let adapter = RemoteAdapter::new("https://docs.google.com/spreadsheets/d/SHEET_ID/edit");
//! let schema = adapter.load_schema()?;
//! # Ok(())
//! # }
//! ```

mod adapter;
mod csv_adapter;
mod error;
mod factory;
mod json_adapter;
mod remote_adapter;
mod sql_adapter;
mod xml_adapter;

pub use adapter::SchemaAdapter;
pub use csv_adapter::CsvAdapter;
pub use error::{AdapterError, AdapterResult};
pub use factory::{load_schema_from_file, load_schema_from_url, load_schema_from_url_with_format};
pub use json_adapter::JsonAdapter;
pub use remote_adapter::RemoteAdapter;
pub use sql_adapter::SqlAdapter;
pub use xml_adapter::XmlAdapter;
