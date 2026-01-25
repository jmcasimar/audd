# File Adapters Documentation

**Version:** 1.0.0  
**Last Updated:** 2026-01-25

## Overview

The AUDD file adapters enable schema extraction from common file formats (CSV, JSON, XML, SQL/DDL) and conversion to the AUDD Intermediate Representation (IR). This provides immediate adoption without database connectors and enables reproducible fixtures for academic evaluation.

## Supported Formats

### CSV Adapter

**Extension:** `.csv`

**Approach:**
- Headers become field names
- All fields default to `String` type (type inference optional in future iterations)
- Entity name derived from filename
- All fields marked as nullable by default

**Example:**

Input file `users.csv`:
```csv
id,name,email,age
1,Alice,alice@example.com,30
2,Bob,bob@example.com,25
```

Generated IR:
```json
{
  "source_name": "users",
  "source_type": "csv",
  "entities": [
    {
      "entity_name": "users",
      "entity_type": "table",
      "fields": [
        {"field_name": "id", "canonical_type": {"type": "string"}, "nullable": true},
        {"field_name": "name", "canonical_type": {"type": "string"}, "nullable": true},
        {"field_name": "email", "canonical_type": {"type": "string"}, "nullable": true},
        {"field_name": "age", "canonical_type": {"type": "string"}, "nullable": true}
      ]
    }
  ]
}
```

**CLI Usage:**
```bash
audd load --source file:users.csv
```

**Limitations:**
- No type inference (all fields are String)
- No multi-table support (one CSV = one entity)
- Headers are required
- No support for quoted fields with newlines (basic CSV only)

---

### JSON Adapter

**Extensions:** `.json`

**Approach:**
- Supports single object or array of objects
- Keys from (first) object become field names
- Basic type inference: boolean, number (int/float), string, nested (JSON type)
- Deeply nested structures treated as JSON type
- Entity name derived from filename

**Example 1: Single Object**

Input file `config.json`:
```json
{
  "id": 1,
  "name": "Alice",
  "active": true,
  "score": 95.5
}
```

**Example 2: Array of Objects**

Input file `users.json`:
```json
[
  {"id": 1, "name": "Alice", "active": true},
  {"id": 2, "name": "Bob", "active": false}
]
```

Generated IR:
- `id` → Int64
- `name` → String
- `active` → Boolean
- `score` → Float64

**CLI Usage:**
```bash
audd load --source file:users.json
```

**Limitations:**
- Only flat or shallow objects supported (MVP)
- Heterogeneous arrays not supported (schema inferred from first element)
- No union types or complex nested schemas
- Empty arrays produce an error

---

### XML Adapter (MVP)

**Extension:** `.xml`

**Approach:**
- First-level child tags of `<record>`, `<item>`, or `<row>` become fields
- All fields default to String type
- Attributes become fields with `_attr` suffix
- Assumes homogeneous structure across all records

**Example:**

Input file `users.xml`:
```xml
<?xml version="1.0"?>
<users>
  <record id="1">
    <name>Alice</name>
    <email>alice@example.com</email>
  </record>
  <record id="2">
    <name>Bob</name>
    <email>bob@example.com</email>
  </record>
</users>
```

Generated IR fields:
- `id_attr` (from attribute)
- `name`
- `email`

**CLI Usage:**
```bash
audd load --source file:users.xml
```

**Limitations:**
- MVP: Basic structure only, no complex XPath or namespaces
- No validation against XSD/DTD
- All fields are String type
- Assumes uniform record structure
- Nested elements beyond depth 3 not extracted as separate fields

---

### SQL/DDL Adapter

**Extensions:** `.sql`, `.ddl`

**Approach:**
- Parses `CREATE TABLE` statements
- Extracts column names and types
- Maps SQL types to canonical types
- Supports basic constraints: PRIMARY KEY, NOT NULL, UNIQUE
- Multiple tables supported (one CREATE TABLE = one entity)

**Example:**

Input file `schema.sql`:
```sql
CREATE TABLE users (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE,
    age INT,
    created_at TIMESTAMP
);

CREATE TABLE posts (
    id INT PRIMARY KEY,
    title TEXT NOT NULL,
    published BOOLEAN
);
```

Generated IR:
- Two entities: `users` and `posts`
- `id` fields: Int32, not nullable (PRIMARY KEY implies NOT NULL)
- `name`: String, not nullable
- `email`: String, nullable, with UNIQUE constraint
- Type mappings applied (INT → Int32, VARCHAR → String, TIMESTAMP → Timestamp, etc.)

**CLI Usage:**
```bash
audd load --source file:schema.sql
```

**Type Mappings:**

| SQL Type | Canonical Type |
|----------|---------------|
| INT, INTEGER, SMALLINT, MEDIUMINT | Int32 |
| BIGINT, LONG | Int64 |
| FLOAT, REAL | Float32 |
| DOUBLE, DOUBLE PRECISION | Float64 |
| DECIMAL, NUMERIC | Decimal(10,2) |
| BOOLEAN, BOOL | Boolean |
| CHAR, VARCHAR, TEXT, NVARCHAR | String |
| CLOB, LONGTEXT, MEDIUMTEXT | Text |
| BLOB, BINARY, VARBINARY | Binary |
| DATE | Date |
| TIME | Time |
| DATETIME, TIMESTAMP | Timestamp |
| JSON | Json |
| UUID | Uuid |

**Limitations:**
- Subset of SQL DDL only (not a full SQL parser)
- No support for:
  - ALTER TABLE
  - Foreign key constraints (parsed but not represented in IR yet)
  - CHECK constraints
  - DEFAULT values (except in metadata)
  - Complex SQL dialects (MySQL/PostgreSQL/SQLite-specific extensions)
  - CREATE INDEX
  - Views, triggers, stored procedures
- Minimal support for `IF NOT EXISTS`, `CONSTRAINT` clauses
- Comments and whitespace variations may affect parsing

---

## Not Supported (Current Limitations)

### All Formats
- Semantic business logic extraction (e.g., detecting "email" vs. "phone" fields)
- Relationship inference between entities
- Data validation rules beyond basic constraints

### CSV
- Type inference (planned for future iteration)
- Multi-file aggregation
- Column metadata (units, formats, etc.)

### JSON
- Deeply nested or recursive structures
- Polymorphic/heterogeneous arrays
- JSON Schema validation
- JSON-LD or semantic annotations

### XML
- XPath queries
- XML Schema (XSD) validation
- Namespaces
- Mixed content (text + elements)
- Complex element hierarchies

### SQL/DDL
- Full SQL dialect support (MySQL, PostgreSQL, SQL Server, Oracle)
- Foreign keys represented in IR
- Indexes
- Views and materialized views
- Stored procedures and triggers
- Advanced constraints (CHECK, exclusion)

---

## API Usage

### Rust API

```rust
use audd_adapters_file::load_schema_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Auto-detect format from extension
    let schema = load_schema_from_file("users.csv")?;
    
    println!("Source: {} ({})", schema.source_name, schema.source_type);
    
    for entity in &schema.entities {
        println!("Entity: {}", entity.entity_name);
        for field in &entity.fields {
            println!("  - {}: {:?}", field.field_name, field.canonical_type);
        }
    }
    
    Ok(())
}
```

### Manual Adapter Selection

```rust
use audd_adapters_file::{CsvAdapter, JsonAdapter, SqlAdapter, XmlAdapter, SchemaAdapter};

// CSV
let csv_adapter = CsvAdapter::new();
let schema = csv_adapter.load(Path::new("data.csv"))?;

// JSON
let json_adapter = JsonAdapter::new();
let schema = json_adapter.load(Path::new("data.json"))?;

// XML
let xml_adapter = XmlAdapter::new();
let schema = xml_adapter.load(Path::new("data.xml"))?;

// SQL
let sql_adapter = SqlAdapter::new();
let schema = sql_adapter.load(Path::new("schema.sql"))?;
```

---

## Error Handling

Adapters can return the following errors:

- **`IoError`**: File not found or cannot be read
- **`CsvError`**: Invalid CSV format
- **`JsonError`**: Invalid JSON syntax
- **`XmlError`**: Malformed XML
- **`SqlError`**: SQL parsing error
- **`UnsupportedFormat`**: File extension not recognized
- **`InvalidStructure`**: File structure doesn't match expected format
- **`EmptyData`**: No data or fields found in file

Example error handling:

```rust
use audd_adapters_file::{load_schema_from_file, AdapterError};

match load_schema_from_file("data.csv") {
    Ok(schema) => println!("Loaded: {}", schema.source_name),
    Err(AdapterError::UnsupportedFormat(ext)) => {
        eprintln!("Format '{}' not supported", ext);
    }
    Err(AdapterError::EmptyData(msg)) => {
        eprintln!("Empty file: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## Fixtures

Sample fixtures are available in `/fixtures/adapters/`:

- `users.csv` - Sample CSV with multiple columns
- `users.json` - Sample JSON array of objects
- `users.xml` - Sample XML with records
- `schema.sql` - Sample SQL DDL with multiple tables

These fixtures are used for testing and demonstration purposes.

---

## Future Enhancements

### Planned (Post-MVP)
- CSV type inference (smart detection of integers, dates, booleans)
- JSON: Better handling of nested structures
- XML: XPath-based field extraction
- SQL: More dialect-specific support (MySQL, PostgreSQL)
- Configuration options (e.g., custom type mappings, null handling)

### Under Consideration
- Excel/XLSX adapter
- Parquet adapter
- YAML adapter
- Avro adapter
- Protobuf schema adapter

---

## Contributing

When adding a new adapter:

1. Implement the `SchemaAdapter` trait
2. Add tests (unit + integration)
3. Register in `factory.rs` for auto-detection
4. Add fixture examples
5. Update this documentation
6. Ensure error messages are clear and actionable

---

## License

See LICENSE file in the repository root.
