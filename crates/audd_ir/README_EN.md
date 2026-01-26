# audd_ir - AUDD Intermediate Representation

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Canonical schema representation for the AUDD (Algoritmo de Unificación Dinámica de Datos) project.

## Overview

`audd_ir` provides a unified intermediate representation (IR) for schemas from heterogeneous data sources. It enables comparison, mapping, and unification of schemas across different databases, file formats, and data systems.

## Features

- **Canonical Type System**: Abstract representation of common data types
- **Identifier Normalization**: Convert identifiers to a standard format (snake_case)
- **Type Mapping**: Map database-specific types to canonical types
- **Schema Building**: Ergonomic builder pattern for constructing schemas
- **Serialization**: JSON export/import for debugging and testing
- **Type Compatibility**: Check compatibility between different types
- **Extensibility**: Metadata fields for custom extensions

## Supported Databases

- MySQL
- PostgreSQL
- SQLite

Additional database support can be added by implementing type mappings.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
audd_ir = "0.1"
```

## Quick Start

```rust
use audd_ir::{SourceSchema, EntitySchema, FieldSchema, CanonicalType};

// Build a schema
let field = FieldSchema::builder()
    .field_name("user_id")
    .canonical_type(CanonicalType::Int32)
    .nullable(false)
    .build();

let entity = EntitySchema::builder()
    .entity_name("users")
    .entity_type("table")
    .add_field(field)
    .build();

let source = SourceSchema::builder()
    .source_name("myapp_db")
    .source_type("mysql")
    .add_entity(entity)
    .build();

// Serialize to JSON
let json = source.to_json().unwrap();
println!("{}", json);

// Deserialize from JSON
let loaded = SourceSchema::from_json(&json).unwrap();
assert_eq!(source, loaded);
```

## Examples

### Normalize Identifiers

```rust
use audd_ir::normalize_identifier;

assert_eq!(normalize_identifier("UserEmail"), "user_email");
assert_eq!(normalize_identifier("Product Name"), "product_name");
assert_eq!(normalize_identifier("firstName"), "first_name");
```

### Map Database Types

```rust
use audd_ir::{map_type_to_canonical, CanonicalType};

assert_eq!(
    map_type_to_canonical("mysql", "VARCHAR(255)"),
    CanonicalType::String
);

assert_eq!(
    map_type_to_canonical("postgresql", "UUID"),
    CanonicalType::Uuid
);
```

### Check Type Compatibility

```rust
use audd_ir::CanonicalType;

assert!(CanonicalType::Int32.is_compatible_with(&CanonicalType::Int64));
assert!(CanonicalType::String.is_compatible_with(&CanonicalType::Text));
```

### Complete Example

Run the included example:

```bash
cargo run --example ir_demo
```

## Architecture

The IR consists of several key components:

- **SourceSchema**: Top-level container for a data source
- **EntitySchema**: Represents tables, collections, or equivalent structures
- **FieldSchema**: Represents individual fields/columns
- **CanonicalType**: Unified type system
- **Constraint**: Field constraints (MaxLength, Unique, etc.)
- **Key**: Primary, unique, and foreign keys

## Documentation

Full documentation is available at [docs/ir.md](../../docs/ir.md).

## Testing

Run the test suite:

```bash
cargo test --package audd_ir
```

This runs:
- 29 unit tests
- 4 integration tests with fixtures
- 4 doc tests

All 37 tests should pass.

## Fixtures

Example IR schemas are available in `fixtures/ir/`:

- `simple_a.json`: MySQL users table
- `simple_b.json`: PostgreSQL users table

These demonstrate the IR format and can be used for testing.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.

## Version

Current IR version: **1.0.0**

The IR uses semantic versioning. See [docs/ir.md](../../docs/ir.md) for versioning strategy.
