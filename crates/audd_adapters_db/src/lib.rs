//! AUDD Database Adapters
//!
//! This crate provides connectors for extracting schema metadata from various
//! database engines and converting them to the AUDD Intermediate Representation (IR).
//!
//! # Supported Databases
//!
//! - **SQLite** - via the `sqlite` feature (enabled by default)
//! - **MySQL/MariaDB** - via the `mysql` feature (enabled by default)
//! - **PostgreSQL** - via the `postgres` feature (enabled by default)
//! - **MongoDB** - via the `mongodb` feature (enabled by default)
//! - **Microsoft SQL Server** - via the `sqlserver` feature (enabled by default)
//!
//! # Examples
//!
//! ## Loading schema from SQLite
//!
//! ```no_run
//! use audd_adapters_db::{create_connector, DbSchemaConnector};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let connector = create_connector("sqlite:///path/to/database.db")?;
//! let schema = connector.load()?;
//! println!("Database: {}", schema.source_name);
//! println!("Tables: {}", schema.entities.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Loading schema from PostgreSQL
//!
//! ```no_run
//! use audd_adapters_db::{create_connector, DbSchemaConnector};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let connector = create_connector("postgres://user:password@localhost:5432/mydb")?;
//! let schema = connector.load()?;
//! println!("Database: {}", schema.source_name);
//! # Ok(())
//! # }
//! ```
//!
//! ## Loading schema from MongoDB
//!
//! ```no_run
//! use audd_adapters_db::{create_connector, DbSchemaConnector};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let connector = create_connector("mongodb://localhost:27017/mydb")?;
//! let schema = connector.load()?;
//! println!("Database: {}", schema.source_name);
//! # Ok(())
//! # }
//! ```
//!
//! # Connection String Formats
//!
//! - **SQLite**: `sqlite:///absolute/path/to/db.sqlite` or `sqlite://relative/path/to/db.sqlite`
//! - **MySQL**: `mysql://user:password@host:port/database` (port defaults to 3306 if omitted)
//! - **PostgreSQL**: `postgres://user:password@host:port/database`
//! - **MongoDB**: `mongodb://host:port/database` or `mongodb+srv://host/database`
//! - **SQL Server**: `sqlserver://user:password@host:port/database` or `mssql://user:password@host:port/database`
//!
//! # Features
//!
//! - `sqlite` - Enable SQLite support (default)
//! - `mysql` - Enable MySQL/MariaDB support (default)
//! - `postgres` - Enable PostgreSQL support (default)
//! - `mongodb` - Enable MongoDB support (default)
//! - `sqlserver` - Enable Microsoft SQL Server support (default)

mod connector;
mod error;
mod factory;

#[cfg(feature = "sqlite")]
pub mod sqlite;

#[cfg(feature = "mysql")]
pub mod mysql;

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "mongodb")]
pub mod mongodb;

#[cfg(feature = "sqlserver")]
pub mod sqlserver;

pub use connector::{parse_connection_string, DbSchemaConnector};
pub use error::{DbError, DbResult};
pub use factory::create_connector;
