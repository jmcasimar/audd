# Database Adapters Documentation

## Overview

AUDD provides database adapters that allow you to extract schema metadata directly from databases and convert them to the AUDD Intermediate Representation (IR). This enables schema comparison between databases and other data sources.

## Supported Database Engines

### Current Support

- **SQLite** - Full support for schema extraction including foreign keys, indexes, views, and triggers
- **MySQL/MariaDB** - Full support for schema extraction including foreign keys, indexes, views, stored procedures, and triggers
- **PostgreSQL** - Full support for schema extraction including foreign keys, indexes, views, stored procedures, and triggers
- **MongoDB** - Full support with schema inference from document sampling, including indexes and views
- **Microsoft SQL Server** - Full support for schema extraction including foreign keys, indexes, views, stored procedures, and triggers (optional feature flag)

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

### PostgreSQL

```
postgres://user:password@host:port/database
postgresql://user:password@host:port/database  # Alias
postgres://user:password@host/database  # Port defaults to 5432
```

**Examples:**
```bash
# With explicit port
postgres://admin:secret@localhost:5432/myapp_db

# Default port (5432)
postgres://user:pass@localhost/myapp_db

# Remote PostgreSQL server
postgres://dbuser:dbpass@pg.example.com/production_db
```

### MongoDB

```
mongodb://host:port/database
mongodb://user:password@host:port/database
mongodb+srv://cluster/database  # MongoDB Atlas
```

**Examples:**
```bash
# Local MongoDB
mongodb://localhost:27017/myapp_db

# With authentication
mongodb://admin:secret@localhost:27017/myapp_db

# MongoDB Atlas cluster
mongodb+srv://cluster0.mongodb.net/production_db

# With connection options
mongodb://localhost:27017/mydb?retryWrites=true&w=majority
```

**Note:** MongoDB schema is inferred by sampling documents (default: 100 documents per collection).

### Microsoft SQL Server

```
sqlserver://user:password@host:port/database
mssql://user:password@host:port/database  # Alias (normalized to sqlserver)
sqlserver://user:password@host/database  # Port defaults to 1433
```

**Examples:**
```bash
# With explicit port
sqlserver://sa:YourPassword@localhost:1433/myapp_db

# Default port (1433)
sqlserver://user:pass@localhost/myapp_db

# Remote SQL Server
sqlserver://dbuser:dbpass@sqlserver.example.com/production_db

# Using mssql prefix (normalized to sqlserver)
mssql://sa:pass@localhost/myapp_db
```

**Note:** SQL Server connector requires the `sqlserver` feature flag to be enabled. It is not included in the default features.

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

#### PostgreSQL Example

```bash
# Load schema from PostgreSQL database
audd load --source "db:postgres://user:password@localhost/mydb"

# With explicit port
audd load --source "db:postgres://admin:secret@localhost:5432/myapp"
```

#### MongoDB Example

```bash
# Load schema from MongoDB database
audd load --source "db:mongodb://localhost:27017/mydb"

# MongoDB Atlas
audd load --source "db:mongodb+srv://cluster0.mongodb.net/production"
```

#### SQL Server Example

```bash
# Load schema from SQL Server database
audd load --source "db:sqlserver://sa:password@localhost/mydb"

# With explicit port
audd load --source "db:sqlserver://user:pass@localhost:1433/myapp"

# Using mssql prefix
audd load --source "db:mssql://sa:password@server/mydb"
```

**Note:** SQL Server support requires enabling the `sqlserver` feature flag when building AUDD.

#### Legacy Format (with separate --conn flag)

```bash
# SQLite with separate connection parameter
audd load --source db:sqlite --conn /path/to/database.db

# MySQL with separate connection parameter
audd load --source db:mysql --conn user:password@localhost/mydb

# PostgreSQL with separate connection parameter
audd load --source db:postgres --conn user:password@localhost:5432/mydb

# MongoDB with separate connection parameter
audd load --source db:mongodb --conn localhost:27017/mydb
```

### Compare Schemas from Different Sources

You can compare schemas from different database engines or between databases and files:

```bash
# Compare SQLite with PostgreSQL
audd compare \
  --source-a "db:sqlite:///local/app.db" \
  --source-b "db:postgres://user:pass@remote.com/prod_db"

# Compare MongoDB with MySQL
audd compare \
  --source-a "db:mongodb://localhost:27017/development" \
  --source-b "db:mysql://user:pass@staging/myapp"

# Compare database with CSV file
audd compare \
  --source-a "db:postgres://user:pass@localhost/current" \
  --source-b "schema.csv"

# Compare two PostgreSQL databases
audd compare \
  --source-a "db:postgres://user:pass@staging:5432/myapp" \
  --source-b "db:postgres://user:pass@production:5432/myapp"

# Compare SQL Server with MySQL
audd compare \
  --source-a "db:sqlserver://sa:pass@localhost/devdb" \
  --source-b "db:mysql://user:pass@production/myapp"

# Compare SQL Server development to production
audd compare \
  --source-a "db:sqlserver://user:pass@dev-server/myapp" \
  --source-b "db:sqlserver://user:pass@prod-server/myapp"
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
- **Foreign Keys**: Foreign key relationships with referenced tables/columns
- **Indexes**: Regular, full-text, and spatial indexes
- **Views**: View names and SQL definitions
- **Stored Procedures**: Procedures and functions with return types and definitions
- **Triggers**: Database triggers with timing, events, table associations, and definitions

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

**Advanced Features:**
- Foreign keys extracted from `INFORMATION_SCHEMA.KEY_COLUMN_USAGE`
- Regular indexes (non-unique) from `INFORMATION_SCHEMA.STATISTICS`
- Full-text indexes (FULLTEXT type)
- Spatial indexes (SPATIAL type)
- Views from `INFORMATION_SCHEMA.VIEWS`
- Stored procedures and functions from `INFORMATION_SCHEMA.ROUTINES`
- Triggers from `INFORMATION_SCHEMA.TRIGGERS`

### PostgreSQL

The PostgreSQL adapter extracts the following metadata:

- **Tables**: All base tables in the public schema
- **Columns**: Name, type, nullability, precision/scale
- **Primary Keys**: Single and composite primary keys
- **Unique Constraints**: Unique constraints
- **Foreign Keys**: Foreign key relationships with referenced tables/columns
- **Indexes**: Regular, unique, partial (filtered), GIN, GIST indexes
- **Views**: Regular views and materialized views with SQL definitions
- **Stored Procedures**: Functions and procedures with return types and definitions
- **Triggers**: Database triggers with timing, events, and definitions

**Type Mapping:**
- SMALLINT, INTEGER → Int32
- BIGINT → Int64
- SMALLSERIAL, SERIAL → Int32
- BIGSERIAL → Int64
- REAL → Float32
- DOUBLE PRECISION → Float64
- NUMERIC, DECIMAL → Decimal (with precision/scale)
- MONEY → Decimal(19,2)
- CHARACTER, CHARACTER VARYING, VARCHAR → String
- TEXT → Text
- BYTEA → Binary
- BOOLEAN → Boolean
- DATE → Date
- TIME → Time
- TIMESTAMP → DateTime
- TIMESTAMP WITH TIME ZONE → Timestamp
- JSON, JSONB → Json
- UUID → Uuid
- ARRAY → Unknown (preserves element type info)
- User-defined types → Unknown (preserves original type name)

**Advanced Features:**
- Partial/filtered indexes with WHERE conditions
- Materialized views (marked with `is_materialized` flag)
- GIN and GIST indexes (mapped to FullText type)
- Async operations using tokio runtime

### MongoDB

The MongoDB adapter infers schema by sampling documents:

- **Collections**: All collections in the database
- **Fields**: Detected from sampled documents (default: 100 per collection)
- **Types**: Inferred from BSON types in documents
- **Primary Key**: Automatic _id detection
- **Nullable**: Inferred based on presence of null values

**Sampling Behavior:**
- Default sample size: 100 documents per collection
- Configurable via API
- Fields present in < 100% of documents marked as nullable
- Mixed types reported as Unknown with type list

**Type Mapping:**
- Int32, Int64 → Int32, Int64
- Double → Float64
- Decimal128 → Decimal(34,0)
- String → String
- Boolean → Boolean
- Binary → Binary
- DateTime → DateTime
- Timestamp → Timestamp
- ObjectId → String
- Nested documents/arrays → Json
- Mixed types → Unknown (with type list)

**Advanced Features:**
- Indexes extracted from `listIndexes()` command
  - Single field and compound indexes
  - Text indexes (full-text search)
  - 2dsphere indexes (geospatial/spatial)
  - Hashed indexes
  - Unique indexes
  - Partial/filtered indexes with filter expressions
- Views extracted from `listCollections()` 
  - Aggregation pipeline views
  - View definitions as formatted pipelines
- Async operations using tokio runtime

**Note:** MongoDB validators (JSON Schema and query validators) are not currently extracted but could be added in a future enhancement.

### Microsoft SQL Server

The SQL Server adapter extracts the following metadata:

- **Tables**: All user tables from dbo schema
- **Columns**: Name, type, nullability, defaults
- **Primary Keys**: Single and composite primary keys
- **Foreign Keys**: With referenced table and column information
- **Indexes**: Regular, unique, full-text, spatial, and filtered indexes
- **Views**: View definitions from INFORMATION_SCHEMA
- **Stored Procedures**: Functions and procedures with definitions
- **Triggers**: Timing, events, and SQL definitions

**Type Mapping:**
- BIT → Boolean
- TINYINT, SMALLINT, INT → Int32
- BIGINT → Int64
- DECIMAL(p,s), NUMERIC(p,s) → Decimal{p,s}
- MONEY, SMALLMONEY → Decimal{19,4}
- REAL → Float32
- FLOAT → Float64
- CHAR, VARCHAR, NCHAR, NVARCHAR → String
- VARCHAR(MAX), TEXT, NTEXT → Text
- BINARY, VARBINARY, IMAGE → Binary
- DATE → Date
- TIME → Time
- DATETIME, DATETIME2, SMALLDATETIME, DATETIMEOFFSET → DateTime
- UNIQUEIDENTIFIER → Uuid
- JSON, XML → Json
- GEOGRAPHY, GEOMETRY → Unknown (with spatial type info)

**Advanced Features:**
- Foreign keys extracted from `sys.foreign_keys` and `sys.foreign_key_columns`
  - Supports composite foreign keys
  - Referenced table and column metadata
- Indexes extracted from `sys.indexes` and `sys.index_columns`
  - Regular (non-unique) indexes
  - Unique indexes  
  - Full-text indexes
  - Spatial indexes
  - Filtered indexes with filter definitions
  - Excludes primary key and unique constraint indexes
- Views extracted from `INFORMATION_SCHEMA.VIEWS`
  - View names and SQL definitions
- Stored procedures from `INFORMATION_SCHEMA.ROUTINES`
  - Functions and procedures
  - Routine types and return types
  - SQL definitions
- Triggers from `sys.triggers`
  - BEFORE/AFTER/INSTEAD OF timing
  - INSERT/UPDATE/DELETE events
  - SQL definitions
- Async operations using tiberius and tokio runtime

**Note:** SQL Server support requires enabling the `sqlserver` feature flag when building AUDD. Add `features = ["sqlserver"]` to your Cargo.toml or use `--features sqlserver` when building.

## Error Handling

The database adapters provide clear error messages for common issues:

### Connection Errors

```
❌ Error loading schema: Failed to create database connector: 
   Database connection error: Failed to open SQLite database: unable to open database file
```

**Common causes:**
- Database file doesn't exist (SQLite)
- Incorrect credentials (MySQL, PostgreSQL, MongoDB)
- Database server not running (MySQL, PostgreSQL, MongoDB)
- Network issues (all network databases)

**Solutions:**
- Verify the database path/connection string
- Check database permissions
- Ensure database server is running
- Verify network connectivity

### Invalid Connection String

```
❌ Error loading schema: Failed to create database connector: 
   Invalid connection string: Missing database name. 
   Expected format: sqlite://<path>, mysql://<user>:<pass>@<host>/<db>, 
   postgres://<user>:<pass>@<host>/<db>, or mongodb://<host>/<db>
```

**Solution:** Check the connection string format matches the documented patterns.

### Unsupported Engine

```
❌ Error loading schema: Failed to create database connector: 
   Unsupported database engine: oracle (Supported: sqlite, mysql, postgres, mongodb, sqlserver)
```

**Solution:** Use a supported database engine.

## Features and Limitations

### Current Features

✅ Extract table/collection schemas  
✅ Column types, nullability  
✅ Primary keys (single and composite)  
✅ Unique constraints  
✅ Type mapping to canonical IR types  
✅ Error handling with helpful messages  
✅ Schema inference for MongoDB  
✅ PostgreSQL full support  
✅ MongoDB document sampling  

### Limitations

❌ Foreign key relationships (planned)  
❌ Indexes (non-unique)  
❌ Views  
❌ Stored procedures  
❌ Triggers  
❌ Complex constraints (CHECK, etc.)  
❌ MongoDB validators and JSON schema  

### Future Enhancements

- Foreign key extraction
- Index analysis
- View metadata
- Advanced constraint detection
- Connection pooling for network databases
- SSL/TLS connection options
- MongoDB validation rules extraction

## Performance

### Extraction Speed

The metadata extraction is optimized for speed:

- **SQLite**: Uses efficient PRAGMA queries
- **MySQL**: Uses INFORMATION_SCHEMA with indexed queries
- **PostgreSQL**: Uses information_schema and pg_catalog
- **MongoDB**: Configurable document sampling (default: 100 docs)

**Target:** < 2 seconds for databases with up to 100 tables/collections

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

### PostgreSQL Issues

**Problem:** "connection refused"
- Verify PostgreSQL server is running
- Check that PostgreSQL is listening on the correct host/port
- Verify pg_hba.conf allows connections from your host
- Check firewall rules

**Problem:** "authentication failed"
- Verify username and password
- Check PostgreSQL authentication method in pg_hba.conf
- Ensure user has SELECT permissions on information_schema

**Problem:** "database does not exist"
- Verify database name is spelled correctly
- Check user has CONNECT privilege on the database
- Ensure database exists using psql

### MongoDB Issues

**Problem:** "connection timed out"
- Verify MongoDB server is running
- Check MongoDB is listening on the correct host/port
- Verify firewall rules allow connections
- Check network connectivity

**Problem:** "authentication failed"
- Verify username and password
- Check user has read permissions on the database
- Ensure authentication database is correct

**Problem:** "no collections found"
- Verify database name is correct
- Check that collections exist in the database
- Ensure user has list_collections permission

**Problem:** "schema appears incomplete"
- Increase sample size (MongoDB uses sampling)
- Some fields may not appear in all documents
- Consider sampling more documents for better coverage

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

// PostgreSQL
let connector = create_connector("postgres://user:pass@localhost:5432/mydb")?;
let schema = connector.load()?;

// MongoDB
let connector = create_connector("mongodb://localhost:27017/mydb")?;
let schema = connector.load()?;
```

See the crate documentation for more details on the API.

## Support and Feedback

For issues, feature requests, or questions:
- File an issue on GitHub
- Check existing documentation
- Review error messages carefully

## Version History

- **v0.1.0** - SQLite and MySQL/MariaDB support
- **v0.2.0** - PostgreSQL and MongoDB support (schema inference)
- Future: Foreign keys, views, advanced constraints
