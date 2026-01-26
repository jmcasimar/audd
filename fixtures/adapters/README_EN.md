# File Adapter Fixtures

This directory contains sample files for testing and demonstrating the AUDD file adapters.

## Files

### users.csv
Sample CSV file with typical user data (5 columns).

**Usage:**
```bash
audd load --source fixtures/adapters/users.csv
```

### users.json
Sample JSON file with an array of user objects. Demonstrates type inference.

**Usage:**
```bash
audd load --source fixtures/adapters/users.json
```

### users.xml
Sample XML file with user records. Shows basic tag extraction.

**Usage:**
```bash
audd load --source fixtures/adapters/users.xml
```

### schema.sql
Sample SQL DDL with two tables (users and posts). Demonstrates:
- PRIMARY KEY constraints
- NOT NULL constraints
- UNIQUE constraints
- Type mappings

**Usage:**
```bash
audd load --source fixtures/adapters/schema.sql
```

## Testing

These fixtures are used by integration tests in `crates/audd_adapters_file/tests/`.

Run tests with:
```bash
cargo test -p audd_adapters_file
```
