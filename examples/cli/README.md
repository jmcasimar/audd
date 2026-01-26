# CLI Examples

## Basic Usage

### Inspect a single data source

Load and inspect the IR (Intermediate Representation) of a data source:

```bash
# Inspect a CSV file
audd inspect --source fixtures/adapters/users.csv

# Inspect a JSON file and save to file
audd inspect --source fixtures/adapters/users.json --out ir_output.json

# Inspect a SQL DDL file
audd inspect --source fixtures/adapters/schema.sql
```

### Load and display schema

```bash
# Load from CSV
audd load --source fixtures/adapters/users.csv

# Load from JSON
audd load --source fixtures/adapters/users.json

# Load from XML
audd load --source fixtures/adapters/users.xml
```

### Compare two data sources

Compare schemas from different sources and generate unification reports:

```bash
# Compare CSV and JSON
audd compare \
  --source-a fixtures/adapters/users.csv \
  --source-b fixtures/adapters/users.json \
  --out output

# This creates:
# - output/unified_schema.json    - Unified schema combining both sources
# - output/diff.json               - Detailed comparison results
# - output/decision_log.json       - Record of all resolution decisions
# - output/report.md               - Human-readable markdown report
```

## Database Sources

### SQLite

```bash
# Inspect SQLite database
audd inspect --source db:sqlite:///path/to/database.db

# Compare two SQLite databases
audd compare \
  --source-a db:sqlite:///path/to/db1.db \
  --source-b db:sqlite:///path/to/db2.db \
  --out comparison_output
```

### MySQL

```bash
# Inspect MySQL database
audd inspect --source "db:mysql://user:password@localhost/dbname"

# Compare MySQL and PostgreSQL
audd compare \
  --source-a "db:mysql://user:pass@localhost/db1" \
  --source-b "db:postgres://user:pass@localhost/db2" \
  --out output
```

## Advanced Usage

### Custom output directory

```bash
# Specify custom output directory
audd compare \
  --source-a data1.csv \
  --source-b data2.json \
  --out /tmp/my_comparison
```

### Mixed sources

```bash
# Compare file vs database
audd compare \
  --source-a fixtures/adapters/users.csv \
  --source-b "db:sqlite:///production.db" \
  --out file_vs_db_comparison
```

## Understanding Output Files

### unified_schema.json

The unified schema (C) that merges both sources A and B:

```json
{
  "schema_name": "users_users_unified",
  "entities": [
    {
      "entity_name": "users",
      "fields": [
        {
          "field": {
            "field_name": "id",
            "canonical_type": {
              "type": "integer"
            },
            "nullable": false
          },
          "origin": "BOTH",
          "state": "matched"
        }
      ]
    }
  ]
}
```

### diff.json

Complete comparison results showing matches, exclusives, and conflicts:

```json
{
  "matches": [...],
  "exclusives": [...],
  "conflicts": [...]
}
```

### decision_log.json

Auditable record of all resolution decisions:

```json
{
  "metadata": {
    "version": "1.0.0",
    "total_decisions": 3,
    "accepted_decisions": 3
  },
  "decisions": [...]
}
```

### report.md

Human-readable summary in Markdown format:

```markdown
# AUDD Comparison Report

## Summary

- **Matches**: 6
- **Exclusives**: 1
- **Conflicts**: 3

# Decision Log
...
```

## Common Workflows

### Development workflow

1. Inspect both sources to understand structure:
```bash
audd inspect --source app_schema.sql --out schema_a.json
audd inspect --source legacy_data.csv --out schema_b.json
```

2. Compare and analyze:
```bash
audd compare \
  --source-a app_schema.sql \
  --source-b legacy_data.csv \
  --out migration_plan
```

3. Review the report.md and decision_log.json to understand conflicts

### Migration planning

```bash
# Compare current production DB with new schema
audd compare \
  --source-a "db:postgres://user:pass@prod.example.com/db" \
  --source-b new_schema.sql \
  --out migration_analysis

# Review the generated files to plan migration
cat migration_analysis/report.md
```

## Error Handling

The CLI provides clear error messages:

```bash
# Invalid file
$ audd inspect --source nonexistent.csv
❌ Error: Failed to load schema from source 'nonexistent.csv': ...

# Invalid database connection
$ audd inspect --source "db:mysql://invalid"
❌ Error: Failed to load schema from source 'db:mysql://invalid': ...
```
