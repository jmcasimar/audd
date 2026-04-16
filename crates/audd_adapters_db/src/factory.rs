//! Factory for creating database connectors based on connection strings

use crate::connector::{parse_connection_string, DbSchemaConnector};
use crate::error::{DbError, DbResult};

#[cfg(feature = "sqlite")]
use crate::sqlite::SqliteConnector;

#[cfg(feature = "mysql")]
use crate::mysql::MysqlConnector;

#[cfg(feature = "postgres")]
use crate::postgres::PostgresConnector;

#[cfg(feature = "mongodb")]
use crate::mongodb::MongoDbConnector;

#[cfg(feature = "sqlserver")]
use crate::sqlserver::SqlServerConnector;

#[cfg(feature = "firebird")]
use crate::firebird::FirebirdConnector;

/// Create a database connector from a connection string
///
/// # Arguments
///
/// * `conn_str` - Full connection string with engine prefix
///   - SQLite: `sqlite:///path/to/database.db`
///   - MySQL: `mysql://user:password@host:port/database`
///   - PostgreSQL: `postgres://user:password@host:port/database`
///   - MongoDB: `mongodb://host:port/database`
///   - SQL Server: `sqlserver://user:password@host:port/database`
///   - Firebird: `firebird://host:/path/to/database.fdb` (note: username and password are separate parameters)
///
/// # Examples
///
/// ```no_run
/// use audd_adapters_db::{create_connector, DbSchemaConnector};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let connector = create_connector("sqlite:///data/app.db")?;
/// let schema = connector.load()?;
/// println!("Loaded schema: {}", schema.source_name);
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - Connection string format is invalid
/// - Database engine is not supported or feature not enabled
/// - Connection to the database fails
pub fn create_connector(conn_str: &str) -> DbResult<Box<dyn DbSchemaConnector>> {
    let (engine, conn_details) = parse_connection_string(conn_str)?;

    match engine.as_str() {
        "sqlite" => {
            #[cfg(feature = "sqlite")]
            {
                let connector = SqliteConnector::new(&conn_details)?;
                Ok(Box::new(connector))
            }
            #[cfg(not(feature = "sqlite"))]
            {
                Err(DbError::FeatureNotEnabled(
                    "sqlite - enable the 'sqlite' feature".to_string(),
                ))
            }
        }
        "mysql" => {
            #[cfg(feature = "mysql")]
            {
                let connector = MysqlConnector::new(&conn_details)?;
                Ok(Box::new(connector))
            }
            #[cfg(not(feature = "mysql"))]
            {
                Err(DbError::FeatureNotEnabled(
                    "mysql - enable the 'mysql' feature".to_string(),
                ))
            }
        }
        "postgres" => {
            #[cfg(feature = "postgres")]
            {
                let connector = crate::runtime::block_on(async {
                    PostgresConnector::new(&conn_details).await
                })?;
                Ok(Box::new(connector))
            }
            #[cfg(not(feature = "postgres"))]
            {
                Err(DbError::FeatureNotEnabled(
                    "postgres - enable the 'postgres' feature".to_string(),
                ))
            }
        }
        "mongodb" | "mongodb+srv" => {
            #[cfg(feature = "mongodb")]
            {
                let full_conn_str = format!("{}://{}", engine, conn_details);
                let connector = crate::runtime::block_on(async {
                    MongoDbConnector::new(&full_conn_str).await
                })?;
                Ok(Box::new(connector))
            }
            #[cfg(not(feature = "mongodb"))]
            {
                Err(DbError::FeatureNotEnabled(
                    "mongodb - enable the 'mongodb' feature".to_string(),
                ))
            }
        }
        "sqlserver" => {
            #[cfg(feature = "sqlserver")]
            {
                let connector = crate::runtime::block_on(async {
                    SqlServerConnector::new(&conn_details).await
                })?;
                Ok(Box::new(connector))
            }
            #[cfg(not(feature = "sqlserver"))]
            {
                Err(DbError::FeatureNotEnabled(
                    "sqlserver - enable the 'sqlserver' feature".to_string(),
                ))
            }
        }
        "firebird" => {
            #[cfg(feature = "firebird")]
            {
                // For Firebird, we need to extract username and password from the connection string
                // Format: firebird://username:password@host:/path/to/db.fdb
                // OR: firebird://username:password@/path/to/db.fdb (for embedded)
                
                // Parse format: username:password@database_path
                if let Some((credentials, db_path)) = conn_details.split_once('@') {
                    if let Some((username, password)) = credentials.split_once(':') {
                        let connector = FirebirdConnector::new(db_path, username, password)?;
                        Ok(Box::new(connector))
                    } else {
                        Err(DbError::InvalidConnectionString(
                            "Firebird connection string must include username:password (format: firebird://user:pass@host:/path/to/db.fdb)".to_string(),
                        ))
                    }
                } else {
                    Err(DbError::InvalidConnectionString(
                        "Firebird connection string must include username:password (format: firebird://user:pass@host:/path/to/db.fdb)".to_string(),
                    ))
                }
            }
            #[cfg(not(feature = "firebird"))]
            {
                Err(DbError::FeatureNotEnabled(
                    "firebird - enable the 'firebird' feature".to_string(),
                ))
            }
        }
        _ => Err(DbError::UnsupportedEngine(engine)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_create_sqlite() {
        // Test parsing works (connection will fail as DB doesn't exist)
        let result = create_connector("sqlite:///nonexistent.db");
        
        #[cfg(feature = "sqlite")]
        {
            // Should fail to open the database, but parsing should work
            assert!(result.is_ok() || matches!(result, Err(DbError::ConnectionError(_))));
        }
        
        #[cfg(not(feature = "sqlite"))]
        {
            assert!(matches!(result, Err(DbError::FeatureNotEnabled(_))));
        }
    }

    #[test]
    fn test_parse_and_create_mysql() {
        // Test parsing works (connection will fail)
        let result = create_connector("mysql://user:pass@localhost/testdb");
        
        #[cfg(feature = "mysql")]
        {
            // Should fail to connect, but parsing should work
            assert!(result.is_ok() || matches!(result, Err(DbError::ConnectionError(_))));
        }
        
        #[cfg(not(feature = "mysql"))]
        {
            assert!(matches!(result, Err(DbError::FeatureNotEnabled(_))));
        }
    }

    #[test]
    fn test_parse_and_create_postgres() {
        // Test parsing works (connection will fail)
        let result = create_connector("postgres://user:pass@localhost/testdb");
        
        #[cfg(feature = "postgres")]
        {
            // Should fail to connect, but parsing should work
            assert!(result.is_err()); // Will fail connection
        }
        
        #[cfg(not(feature = "postgres"))]
        {
            assert!(matches!(result, Err(DbError::FeatureNotEnabled(_))));
        }
    }

    #[test]
    fn test_parse_and_create_mongodb() {
        // Test parsing works
        let result = create_connector("mongodb://localhost/testdb");
        
        #[cfg(feature = "mongodb")]
        {
            // MongoDB client creation may succeed even without a server (lazy connection)
            // Just verify we can create the connector without a panic
            let _ = result;
        }
        
        #[cfg(not(feature = "mongodb"))]
        {
            assert!(matches!(result, Err(DbError::FeatureNotEnabled(_))));
        }
    }

    #[test]
    fn test_invalid_format() {
        let result = create_connector("invalid_format");
        assert!(matches!(result, Err(DbError::InvalidConnectionString(_))));
    }

    #[test]
    fn test_unsupported_engine() {
        let result = create_connector("oracle://connection");
        assert!(matches!(result, Err(DbError::UnsupportedEngine(_))));
    }
}

