//! AUDD Intermediate Representation (IR)
//!
//! This crate provides canonical schema structures for representing
//! heterogeneous data sources in a uniform way.
//!
//! # Overview
//!
//! The IR consists of:
//! - `SourceSchema`: Top-level schema for a data source
//! - `EntitySchema`: Schema for tables/collections/entities
//! - `FieldSchema`: Schema for individual fields/columns
//! - `CanonicalType`: Normalized type system
//! - `Constraint`: Field constraints and validations
//!
//! # Example
//!
//! ```
//! use audd_ir::{SourceSchema, EntitySchema, FieldSchema, CanonicalType};
//!
//! let field = FieldSchema::builder()
//!     .field_name("user_id")
//!     .canonical_type(CanonicalType::Int32)
//!     .nullable(false)
//!     .build();
//!
//! let entity = EntitySchema::builder()
//!     .entity_name("users")
//!     .entity_type("table")
//!     .add_field(field)
//!     .build();
//!
//! let source = SourceSchema::builder()
//!     .source_name("myapp_db")
//!     .source_type("mysql")
//!     .add_entity(entity)
//!     .build();
//! ```

mod constraint;
mod key;
mod normalization;
mod schema;
mod types;

pub use constraint::Constraint;
pub use key::Key;
pub use normalization::{map_type_to_canonical, normalize_identifier};
pub use schema::{EntitySchema, FieldSchema, SourceSchema};
pub use types::CanonicalType;

/// Current version of the IR specification
pub const IR_VERSION: &str = "1.0.0";
