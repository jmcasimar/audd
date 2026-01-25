//! AUDD Database Adapters
//!
//! This crate provides connectors for extracting schema metadata from various
//! database engines and converting them to the AUDD Intermediate Representation (IR).
//!
//! # Supported Databases
//!
//! - **SQLite** - via the `sqlite` feature (enabled by default)
//! - **MySQL/MariaDB** - via the `mysql` feature (enabled by default)
//!
//! # Roadmap
//!
//! - **PostgreSQL** - planned for future releases
//! - **MongoDB** - planned for future releases
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
//! ## Loading schema from MySQL
//!
//! ```no_run
//! use audd_adapters_db::{create_connector, DbSchemaConnector};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let connector = create_connector("mysql://user:password@localhost:3306/mydb")?;
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
//!
//! # Features
//!
//! - `sqlite` - Enable SQLite support (default)
//! - `mysql` - Enable MySQL/MariaDB support (default)

mod connector;
mod error;
mod factory;

#[cfg(feature = "sqlite")]
pub mod sqlite;

#[cfg(feature = "mysql")]
pub mod mysql;

pub use connector::{parse_connection_string, DbSchemaConnector};
pub use error::{DbError, DbResult};
pub use factory::create_connector;
