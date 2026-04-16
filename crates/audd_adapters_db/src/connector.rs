//! Core trait for database schema connectors

use audd_ir::SourceSchema;
use crate::error::DbResult;

/// Trait for extracting schema metadata from database engines
/// 
/// Each database engine implements this trait to provide a unified
/// interface for schema extraction.
///
/// # Examples
///
/// ```no_run
/// use audd_adapters_db::{DbSchemaConnector, parse_connection_string};
/// use audd_adapters_db::DbResult;
///
/// # fn example() -> DbResult<()> {
/// let (engine, conn_str) = parse_connection_string("sqlite:///path/to/db.sqlite")?;
/// // Get appropriate connector based on engine
/// // let connector = get_connector(engine, conn_str)?;
/// // let schema = connector.load()?;
/// # Ok(())
/// # }
/// ```
pub trait DbSchemaConnector {
    /// Load schema from the database
    ///
    /// # Returns
    ///
    /// A `SourceSchema` representing the database schema structure
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Connection to the database fails
    /// - Schema metadata cannot be extracted
    /// - Type mapping fails
    fn load(&self) -> DbResult<SourceSchema>;
}

/// Parse a database connection string into engine type and connection details
///
/// # Supported formats
///
/// - SQLite: `sqlite:///path/to/database.db` (absolute) or `sqlite://path/to/database.db` (relative)
/// - MySQL: `mysql://user:password@host:port/database` or `mysql://user:password@host/database`
/// - PostgreSQL: `postgresql://user:password@host:port/database` or `postgresql://...`
/// - MongoDB: `mongodb://user:password@host:port/database` or `mongodb+srv://...`
/// - SQL Server: `sqlserver://user:password@host:port/database` or `mssql://...`
/// - Firebird: `firebird://host:/path/to/database.fdb` or `firebird:///path/to/database.fdb`
///
/// # Examples
///
/// ```
/// use audd_adapters_db::parse_connection_string;
///
/// let (engine, conn) = parse_connection_string("sqlite:///data/app.db").unwrap();
/// assert_eq!(engine, "sqlite");
/// assert_eq!(conn, "/data/app.db");
///
/// let (engine, conn) = parse_connection_string("mysql://user:pass@localhost/mydb").unwrap();
/// assert_eq!(engine, "mysql");
/// assert_eq!(conn, "user:pass@localhost/mydb");
/// ```
///
/// # Errors
///
/// Returns an error if the connection string format is invalid or unsupported
pub fn parse_connection_string(conn_str: &str) -> DbResult<(String, String)> {
    use crate::error::DbError;

    // Split on :// to get engine and rest
    let parts: Vec<&str> = conn_str.splitn(2, "://").collect();
    
    if parts.len() != 2 {
        return Err(DbError::InvalidConnectionString(
            "Missing '://' separator".to_string(),
        ));
    }

    let engine = parts[0].to_lowercase();
    let conn_details = parts[1];

    // Validate engine is supported
    match engine.as_str() {
        "sqlite" => {
            // SQLite format: sqlite:///path or sqlite://path
            // After split on "://", conn_details will be:
            // - For sqlite:///absolute/path: "/absolute/path" (two slashes stripped)
            // - For sqlite://relative/path: "relative/path" 
            // We need to restore the leading slash for absolute paths
            let path = if conn_details.starts_with('/') {
                // Already absolute path from sqlite:///
                conn_details
            } else {
                // Relative path from sqlite://
                conn_details
            };
            Ok((engine, path.to_string()))
        }
        "mysql" | "mariadb" => {
            // MySQL format: mysql://user:pass@host:port/database
            // Normalize mariadb to mysql
            let normalized_engine = if engine == "mariadb" {
                "mysql".to_string()
            } else {
                engine
            };
            Ok((normalized_engine, conn_details.to_string()))
        }
        "postgresql" | "postgres" => {
            // PostgreSQL format: postgresql://user:pass@host:port/database
            // Normalize postgresql to postgres
            let normalized_engine = "postgres".to_string();
            Ok((normalized_engine, conn_details.to_string()))
        }
        "mongodb" | "mongo" | "mongodb+srv" => {
            // MongoDB format: mongodb://user:pass@host:port/database
            // or mongodb+srv://... for Atlas clusters
            let normalized_engine = if engine == "mongodb+srv" {
                // Keep the +srv variant for MongoDB Atlas
                engine
            } else if engine == "mongo" {
                "mongodb".to_string()
            } else {
                engine
            };
            Ok((normalized_engine, conn_details.to_string()))
        }
        "sqlserver" | "mssql" => {
            // SQL Server format: sqlserver://user:pass@host:port/database
            // Normalize mssql to sqlserver
            let normalized_engine = "sqlserver".to_string();
            Ok((normalized_engine, conn_details.to_string()))
        }
        "firebird" => {
            // Firebird format: firebird://host:/path/to/db.fdb or firebird:///path/to/db.fdb
            Ok((engine, conn_details.to_string()))
        }
        _ => Err(DbError::UnsupportedEngine(engine)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sqlite_connection() {
        // Absolute path with triple slash
        let result = parse_connection_string("sqlite:///data/test.db");
        assert!(result.is_ok());
        let (engine, conn) = result.unwrap();
        assert_eq!(engine, "sqlite");
        assert_eq!(conn, "/data/test.db");

        // Relative path with double slash
        let result = parse_connection_string("sqlite://data/test.db");
        assert!(result.is_ok());
        let (engine, conn) = result.unwrap();
        assert_eq!(engine, "sqlite");
        assert_eq!(conn, "data/test.db");
    }

    #[test]
    fn test_parse_mysql_connection() {
        let result = parse_connection_string("mysql://user:password@localhost:3306/mydb");
        assert!(result.is_ok());
        let (engine, conn) = result.unwrap();
        assert_eq!(engine, "mysql");
        assert_eq!(conn, "user:password@localhost:3306/mydb");
    }

    #[test]
    fn test_parse_mariadb_normalizes_to_mysql() {
        let result = parse_connection_string("mariadb://user:pass@host/db");
        assert!(result.is_ok());
        let (engine, _) = result.unwrap();
        assert_eq!(engine, "mysql"); // Should normalize to mysql
    }

    #[test]
    fn test_parse_invalid_format() {
        let result = parse_connection_string("invalidformat");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_unsupported_engine() {
        let result = parse_connection_string("oracle://connection");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_postgres_and_mongodb() {
        // PostgreSQL and MongoDB are now supported
        let result = parse_connection_string("postgresql://localhost/db");
        assert!(result.is_ok());
        let (engine, _) = result.unwrap();
        assert_eq!(engine, "postgres");

        let result = parse_connection_string("postgresql://localhost/db");
        assert!(result.is_ok());
        let (engine, _) = result.unwrap();
        assert_eq!(engine, "postgres");

        let result = parse_connection_string("mongodb://localhost/db");
        assert!(result.is_ok());
        let (engine, _) = result.unwrap();
        assert_eq!(engine, "mongodb");

        let result = parse_connection_string("mongodb+srv://host/db");
        assert!(result.is_ok());
        let (engine, _) = result.unwrap();
        assert_eq!(engine, "mongodb+srv");
    }
}
