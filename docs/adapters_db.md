# Database Adapters Documentation

## Overview

AUDD provides database adapters that allow you to extract schema metadata directly from databases and convert them to the AUDD Intermediate Representation (IR). This enables schema comparison between databases and other data sources.

## Supported Database Engines

### Current Support (MVP)

- **SQLite** - Full support for schema extraction
- **MySQL/MariaDB** - Full support for schema extraction

### Roadmap (Future Releases)

- **PostgreSQL** - Planned
- **MongoDB** - Planned (will require special handling for schema-flexible collections)

## Connection String Formats

### SQLite

```
sqlite:///absolute/path/to/database.db
sqlite://relative/path/to/database.db
```

**Examples:**
```bash
# Absolute path
sqlite:///var/lib/data/myapp.db

# Relative path
sqlite://data/myapp.db

# Windows absolute path
sqlite:///C:/Data/myapp.db
```

### MySQL/MariaDB

```
mysql://user:password@host:port/database
mysql://user:password@host/database  # Port defaults to 3306
```

**Examples:**
```bash
# With explicit port
mysql://admin:secret@localhost:3306/myapp_db

# Default port (3306)
mysql://admin:secret@localhost/myapp_db

# Remote host
mysql://user:pass@db.example.com/production_db

# MariaDB (uses same connection format)
mysql://user:pass@mariadb-server/myapp_db
```

**Note:** MariaDB connection strings use the `mysql://` prefix as they are compatible.

## CLI Usage

### Load Schema from Database

#### SQLite Example

```bash
# Load schema from SQLite database
audd load --source "db:sqlite:///path/to/database.db"

# Specify output format
audd load --source "db:sqlite:///data/app.db" --format json
```

#### MySQL Example

```bash
# Load schema from MySQL database
audd load --source "db:mysql://user:password@localhost/mydb"

# With explicit port
audd load --source "db:mysql://admin:secret@localhost:3306/myapp"
```

#### Legacy Format (with separate --conn flag)

```bash
# SQLite with separate connection parameter
audd load --source db:sqlite --conn /path/to/database.db

# MySQL with separate connection parameter
audd load --source db:mysql --conn user:password@localhost/mydb
```

### Compare Schemas from Different Sources

You can compare schemas from different database engines or between databases and files:

```bash
# Compare SQLite with MySQL
audd compare \
  --source-a "db:sqlite:///local/app.db" \
  --source-b "db:mysql://user:pass@remote.com/prod_db"

# Compare database with CSV file
audd compare \
  --source-a "db:sqlite:///data/current.db" \
  --source-b "schema.csv"

# Compare two MySQL databases
audd compare \
  --source-a "db:mysql://user:pass@staging/myapp" \
  --source-b "db:mysql://user:pass@production/myapp"
```

## Schema Extraction Details

### SQLite

The SQLite adapter extracts the following metadata:

- **Tables**: All user tables (excluding sqlite_* system tables)
- **Columns**: Name, type, nullability
- **Primary Keys**: Single and composite primary keys
- **Unique Constraints**: Unique indexes (excluding auto-generated)

**Type Mapping:**
- INTEGER → Int64
- TEXT, CLOB → Text
- VARCHAR, CHAR → String
- BLOB → Binary
- REAL, FLOAT, DOUBLE → Float64
- NUMERIC, DECIMAL → Decimal(10,2)
- DATE → Date
- DATETIME, TIMESTAMP → DateTime
- BOOLEAN → Boolean

### MySQL/MariaDB

The MySQL adapter extracts the following metadata:

- **Tables**: All base tables in the specified database
- **Columns**: Name, type, nullability, defaults
- **Primary Keys**: Single and composite primary keys
- **Unique Constraints**: Unique indexes

**Type Mapping:**
- TINYINT, SMALLINT, MEDIUMINT, INT → Int32
- BIGINT → Int64
- FLOAT → Float32
- DOUBLE, REAL → Float64
- DECIMAL, NUMERIC → Decimal (with precision/scale)
- CHAR, VARCHAR → String
- TEXT, MEDIUMTEXT, LONGTEXT → Text
- BLOB, BINARY, VARBINARY → Binary
- DATE → Date
- TIME → Time
- DATETIME → DateTime
- TIMESTAMP → Timestamp
- JSON → Json
- TINYINT(1) → Boolean

## Error Handling

The database adapters provide clear error messages for common issues:

### Connection Errors

```
❌ Error loading schema: Failed to create database connector: 
   Database connection error: Failed to open SQLite database: unable to open database file
```

**Common causes:**
- Database file doesn't exist (SQLite)
- Incorrect credentials (MySQL)
- Database server not running (MySQL)
- Network issues (MySQL)

**Solutions:**
- Verify the database path/connection string
- Check database permissions
- Ensure database server is running
- Verify network connectivity

### Invalid Connection String

```
❌ Error loading schema: Failed to create database connector: 
   Invalid connection string: Missing database name. 
   Expected format: sqlite://<path> or mysql://<user>:<pass>@<host>/<db>
```

**Solution:** Check the connection string format matches the documented patterns.

### Unsupported Engine

```
❌ Error loading schema: Failed to create database connector: 
   Unsupported database engine: oracle (Supported: sqlite, mysql)
```

**Solution:** Use a supported database engine or wait for future releases.

## Features and Limitations

### Current Features

✅ Extract table schemas  
✅ Column types, nullability  
✅ Primary keys (single and composite)  
✅ Unique constraints  
✅ Type mapping to canonical IR types  
✅ Error handling with helpful messages  

### Limitations (MVP)

❌ Foreign key relationships (planned)  
❌ Indexes (non-unique)  
❌ Views  
❌ Stored procedures  
❌ Triggers  
❌ Complex constraints (CHECK, etc.)  

### Future Enhancements

- PostgreSQL support
- MongoDB support (with schema inference)
- Foreign key extraction
- Index analysis
- View metadata
- Advanced constraint detection
- Connection pooling for MySQL
- SSL/TLS connection options

## Performance

### Extraction Speed

The metadata extraction is optimized for speed:

- **SQLite**: Uses efficient PRAGMA queries
- **MySQL**: Uses INFORMATION_SCHEMA with indexed queries

**Target:** < 2 seconds for databases with up to 100 tables

### Best Practices

1. **Use read-only database connections** when possible
2. **Avoid querying during peak hours** for production databases
3. **Consider database views** to limit exposed schema
4. **Test connection strings** with a simple load command first

## Examples

### Complete Workflow Example

```bash
# 1. Load schema from SQLite development database
audd load --source "db:sqlite:///dev/app.db" > dev_schema.json

# 2. Load schema from MySQL staging database
audd load --source "db:mysql://readonly:pass@staging.db/app" > staging_schema.json

# 3. Compare schemas
audd compare \
  --source-a "db:sqlite:///dev/app.db" \
  --source-b "db:mysql://readonly:pass@staging.db/app"

# 4. Export to JSON for version control
audd load --source "db:sqlite:///dev/app.db" --format json > schema_v1.0.json
```

### Integration with File Adapters

```bash
# Extract schema from database and save as SQL DDL
audd load --source "db:sqlite:///data/app.db" > current_schema.json

# Compare with historical SQL schema
audd compare \
  --source-a "db:sqlite:///data/app.db" \
  --source-b "migrations/v1.0_schema.sql"
```

## Security Considerations

1. **Never hardcode passwords** in scripts or command history
2. **Use environment variables** for sensitive credentials:
   ```bash
   export DB_USER="admin"
   export DB_PASS="secret"
   audd load --source "db:mysql://$DB_USER:$DB_PASS@localhost/mydb"
   ```
3. **Use read-only database users** for schema extraction
4. **Avoid logging connection strings** that contain passwords
5. **Consider using SSH tunnels** for remote database connections

## Troubleshooting

### SQLite Issues

**Problem:** "unable to open database file"
- Check file path is correct
- Verify file permissions
- Ensure parent directory exists

**Problem:** "database is locked"
- Close other connections to the database
- Use WAL mode for SQLite if concurrent access is needed

### MySQL Issues

**Problem:** "Access denied for user"
- Verify username and password
- Check user has SELECT permissions on INFORMATION_SCHEMA
- Ensure user has access from your host

**Problem:** "Can't connect to MySQL server"
- Verify MySQL server is running
- Check firewall rules
- Verify host and port are correct

**Problem:** "Unknown database"
- Ensure database name is spelled correctly
- Verify database exists
- Check user has access to the database

## API Usage

For programmatic usage in Rust code:

```rust
use audd_adapters_db::{create_connector, DbSchemaConnector};

// SQLite
let connector = create_connector("sqlite:///path/to/db.sqlite")?;
let schema = connector.load()?;

// MySQL
let connector = create_connector("mysql://user:pass@localhost/mydb")?;
let schema = connector.load()?;
```

See the crate documentation for more details on the API.

## Support and Feedback

For issues, feature requests, or questions:
- File an issue on GitHub
- Check existing documentation
- Review error messages carefully

## Version History

- **v0.1.0** (MVP) - SQLite and MySQL/MariaDB support
- Future: PostgreSQL, MongoDB, enhanced features
