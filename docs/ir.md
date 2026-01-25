# AUDD Intermediate Representation (IR) Specification

**Version:** 1.0.0  
**Last Updated:** 2026-01-25

## Overview

The AUDD Intermediate Representation (IR) is a canonical schema model designed to enable comparison and unification of heterogeneous data sources. The IR serves as an internal contract that normalizes schemas from different sources (databases, files, APIs) into a uniform structure.

## Design Principles

1. **Compatibility First**: The IR prioritizes structural compatibility over semantic perfection
2. **Extensibility**: Uses metadata fields for future extensions without breaking changes
3. **Normalization**: Identifiers and types are normalized for consistent comparison
4. **Versioning**: Explicit versioning allows evolution while maintaining backward compatibility

## Core Structures

### SourceSchema

Represents a complete data source schema (database, file, collection).

**Fields:**
- `source_name` (String): Normalized name of the data source
- `source_type` (String): Type of source (e.g., "mysql", "postgresql", "csv", "json")
- `entities` (Vec<EntitySchema>): List of entities (tables/collections) in this source
- `ir_version` (String): Version of the IR specification (e.g., "1.0.0")
- `metadata` (HashMap<String, Value>): Extensible metadata for source-specific information

**Example:**
```json
{
  "source_name": "customers_db",
  "source_type": "mysql",
  "entities": [...],
  "ir_version": "1.0.0",
  "metadata": {}
}
```

### EntitySchema

Represents a table, collection, or equivalent structure within a source.

**Fields:**
- `entity_name` (String): Normalized name of the entity
- `entity_type` (String): Type of entity ("table", "collection", "sheet", etc.)
- `fields` (Vec<FieldSchema>): List of fields/columns in this entity
- `keys` (Vec<Key>): Primary and unique keys (MVP: basic support)
- `metadata` (HashMap<String, Value>): Entity-specific metadata

**Example:**
```json
{
  "entity_name": "users",
  "entity_type": "table",
  "fields": [...],
  "keys": [
    {
      "key_type": "primary",
      "field_names": ["id"]
    }
  ],
  "metadata": {}
}
```

### FieldSchema

Represents a field/column within an entity.

**Fields:**
- `field_name` (String): Normalized field name
- `canonical_type` (CanonicalType): Canonical data type
- `nullable` (bool): Whether the field accepts null values
- `constraints` (Vec<Constraint>): Additional constraints on the field
- `metadata` (HashMap<String, Value>): Field-specific metadata (e.g., original_name, original_type)

**Example:**
```json
{
  "field_name": "email",
  "canonical_type": "String",
  "nullable": false,
  "constraints": [
    {
      "constraint_type": "MaxLength",
      "value": 255
    }
  ],
  "metadata": {
    "original_name": "Email",
    "original_type": "VARCHAR(255)"
  }
}
```

### CanonicalType

Enumeration of canonical data types that abstract over source-specific types.

**Types (MVP Subset):**
- `Boolean`: True/false values
- `Int32`: 32-bit signed integer
- `Int64`: 64-bit signed integer
- `Float32`: 32-bit floating point
- `Float64`: 64-bit floating point
- `Decimal`: Arbitrary precision decimal with (precision, scale)
- `String`: Variable-length text
- `Text`: Large text (CLOB, TEXT, etc.)
- `Binary`: Binary data (BLOB)
- `Date`: Date without time
- `Time`: Time without date
- `DateTime`: Date and time
- `Timestamp`: Timestamp with timezone
- `Json`: JSON data
- `Uuid`: UUID/GUID
- `Unknown`: Fallback for unmapped types

**Type Parameters:**
- `Decimal { precision: u16, scale: u16 }`: For precise decimal numbers
- `String` with MaxLength constraint: For VARCHAR equivalents

### Constraint

Represents constraints and validation rules on fields.

**Types (MVP Subset):**
- `MaxLength(usize)`: Maximum string length
- `MinLength(usize)`: Minimum string length
- `Precision(u16, u16)`: Decimal precision and scale
- `Unique`: Unique constraint
- `DefaultValue(Value)`: Default value
- `Check(String)`: Check expression (stored as string for MVP)

**Example:**
```json
[
  { "constraint_type": "MaxLength", "value": 100 },
  { "constraint_type": "Unique" }
]
```

### Key

Represents primary keys, foreign keys, and unique constraints.

**Fields (MVP Subset):**
- `key_type` (String): "primary", "unique", "foreign" (foreign support limited in MVP)
- `field_names` (Vec<String>): Fields comprising the key
- `metadata` (HashMap<String, Value>): Additional key information

**Example:**
```json
{
  "key_type": "primary",
  "field_names": ["user_id"]
}
```

## Type Mapping Table

### MySQL → CanonicalType

| MySQL Type | Canonical Type | Notes |
|------------|---------------|-------|
| TINYINT(1) | Boolean | When length is 1 |
| TINYINT | Int32 | |
| SMALLINT | Int32 | |
| MEDIUMINT | Int32 | |
| INT | Int32 | |
| BIGINT | Int64 | |
| FLOAT | Float32 | |
| DOUBLE | Float64 | |
| DECIMAL(p,s) | Decimal(p,s) | |
| CHAR(n) | String | With MaxLength(n) |
| VARCHAR(n) | String | With MaxLength(n) |
| TEXT | Text | |
| TINYTEXT | Text | |
| MEDIUMTEXT | Text | |
| LONGTEXT | Text | |
| BLOB | Binary | |
| DATE | Date | |
| TIME | Time | |
| DATETIME | DateTime | |
| TIMESTAMP | Timestamp | |
| JSON | Json | |
| BINARY(n) | Binary | |
| VARBINARY(n) | Binary | |

### PostgreSQL → CanonicalType

| PostgreSQL Type | Canonical Type | Notes |
|----------------|---------------|-------|
| boolean | Boolean | |
| smallint | Int32 | |
| integer | Int32 | |
| bigint | Int64 | |
| real | Float32 | |
| double precision | Float64 | |
| numeric(p,s) | Decimal(p,s) | |
| decimal(p,s) | Decimal(p,s) | |
| char(n) | String | With MaxLength(n) |
| varchar(n) | String | With MaxLength(n) |
| text | Text | |
| bytea | Binary | |
| date | Date | |
| time | Time | |
| timestamp | DateTime | |
| timestamptz | Timestamp | |
| json | Json | |
| jsonb | Json | |
| uuid | Uuid | |

### SQLite → CanonicalType

| SQLite Type | Canonical Type | Notes |
|------------|---------------|-------|
| INTEGER | Int64 | SQLite uses 64-bit |
| REAL | Float64 | |
| TEXT | Text | |
| BLOB | Binary | |
| NUMERIC | Decimal(38,10) | Default precision |

## Normalization Rules

### Identifier Normalization

The `normalize_identifier()` function applies the following transformations:

1. **Trim whitespace**: Remove leading/trailing spaces
2. **Lowercase**: Convert to lowercase
3. **Collapse spaces**: Replace multiple spaces with single underscore
4. **Snake case**: Convert camelCase/PascalCase to snake_case
5. **Remove accents**: Convert accented characters to ASCII equivalents (optional, configurable)

**Examples:**
- `"UserEmail"` → `"user_email"`
- `"  Product Name  "` → `"product_name"`
- `"firstName"` → `"first_name"`
- `"Customer ID"` → `"customer_id"`

### Type Normalization

Source-specific type names are mapped to canonical types using the tables above. Original type information is preserved in the `metadata` field.

## Versioning Strategy

### IR Version Format

The IR uses semantic versioning: `MAJOR.MINOR.PATCH`

- **MAJOR**: Incompatible changes to core structures
- **MINOR**: Backward-compatible additions (new fields, new types)
- **PATCH**: Bug fixes, documentation updates

### Backward Compatibility

- New optional fields can be added in MINOR versions
- The `metadata` HashMap allows extensions without version bumps
- Parsers should ignore unknown fields gracefully

### Version Checks

When loading IR from JSON:
1. Parse the `ir_version` field
2. Check MAJOR version compatibility
3. Warn on MINOR/PATCH version mismatches
4. Proceed if compatible, fail if not

## Metadata Usage

The `metadata` field in all structures is a `HashMap<String, Value>` that stores:

1. **Original information**: Source-specific details (original names, types)
2. **Source-specific attributes**: Indexes, collations, constraints not in MVP
3. **Extensions**: Future features without schema changes
4. **Annotations**: User or tool annotations

**Common metadata keys:**
- `original_name`: Original identifier before normalization
- `original_type`: Source-specific type string
- `collation`: Database collation (MySQL, PostgreSQL)
- `charset`: Character set
- `auto_increment`: Auto-increment/sequence info
- `comment`: Field/table comments

## Serialization Format

The IR is serialized to JSON for:
- Debugging and inspection
- Testing with fixtures
- Persistence and caching
- Inter-process communication

**JSON Schema considerations:**
- Field order should be deterministic for snapshot testing
- Pretty-print for human readability in fixtures
- Compact format for production use

## Extension Points

### Future Additions (Post-MVP)

1. **Relations**: Foreign key mappings, cardinality
2. **Indexes**: Index definitions and types
3. **Views**: Virtual entities and their definitions
4. **Partitioning**: Partition information
5. **Triggers/Procedures**: Stored logic representation
6. **Statistics**: Row counts, cardinality estimates
7. **Semantic annotations**: Business glossary mappings

### Adding New Canonical Types

To add a new canonical type:
1. Add enum variant to `CanonicalType`
2. Update type mapping tables
3. Add tests for the new type
4. Document in this specification
5. Bump MINOR version

## Implementation Guidelines

### For Adapter Authors

When creating an adapter to convert a source schema to IR:

1. Extract schema information using source-specific APIs
2. Normalize entity and field names using `normalize_identifier()`
3. Map source types to canonical types using mapping tables
4. Preserve original information in `metadata` fields
5. Set appropriate `nullable` and `constraints` values
6. Validate the resulting IR structure

### For Comparison Engine Authors

When comparing two IR schemas:

1. Match entities by normalized names
2. Match fields within entities by normalized names
3. Compare canonical types for compatibility
4. Check nullability and constraints
5. Use metadata for tie-breaking and reporting
6. Handle type compatibility rules (e.g., Int32 ↔ Int64)

## Testing Strategy

### Unit Tests

- Normalization of identifiers (20+ test cases)
- Type mapping for each supported database
- IR construction and validation
- Serialization round-trips (IR → JSON → IR)

### Integration Tests

- Load fixtures and validate structure
- Compare known-compatible schemas
- Compare known-incompatible schemas
- Verify metadata preservation

### Fixtures

Located in `/fixtures/ir/`:
- `simple_a.json`: Basic schema with common types
- `simple_b.json`: Compatible schema with slight variations

## Examples

### Complete Example: Simple Users Table

**MySQL Source:**
```sql
CREATE TABLE Users (
  id INT PRIMARY KEY AUTO_INCREMENT,
  email VARCHAR(255) NOT NULL UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**IR Representation:**
```json
{
  "source_name": "myapp_db",
  "source_type": "mysql",
  "ir_version": "1.0.0",
  "entities": [
    {
      "entity_name": "users",
      "entity_type": "table",
      "fields": [
        {
          "field_name": "id",
          "canonical_type": "Int32",
          "nullable": false,
          "constraints": [],
          "metadata": {
            "original_name": "id",
            "original_type": "INT",
            "auto_increment": true
          }
        },
        {
          "field_name": "email",
          "canonical_type": "String",
          "nullable": false,
          "constraints": [
            { "constraint_type": "MaxLength", "value": 255 },
            { "constraint_type": "Unique" }
          ],
          "metadata": {
            "original_name": "email",
            "original_type": "VARCHAR(255)"
          }
        },
        {
          "field_name": "created_at",
          "canonical_type": "Timestamp",
          "nullable": false,
          "constraints": [],
          "metadata": {
            "original_name": "created_at",
            "original_type": "TIMESTAMP",
            "default": "CURRENT_TIMESTAMP"
          }
        }
      ],
      "keys": [
        {
          "key_type": "primary",
          "field_names": ["id"]
        }
      ],
      "metadata": {}
    }
  ],
  "metadata": {}
}
```

## References

- [EPIC 02: Canonical Schema IR](../README.md)
- [Serde Documentation](https://serde.rs/)
- [JSON Schema Specification](https://json-schema.org/)

---

**Document Status:** Complete (v1.0.0)  
**Next Review:** After EPIC 02 completion
