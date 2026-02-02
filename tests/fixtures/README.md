# AUDD Test Fixtures

This directory contains test data and fixtures for the AUDD test suite.

## Organization

```
fixtures/
├── databases/        # Database test fixtures (SQLite, SQL scripts)
├── scenarios/        # End-to-end test scenarios
├── e2e/             # End-to-end workflow test data
└── README.md        # This file
```

## Datasets

### 1. Simple Users (Minimal Dataset)

**Purpose**: Basic functionality validation

**Location**: `../fixtures/adapters/users.*`

**Schema**:
- Entity: `users`
- Fields: `id`, `username`, `email`, `age`
- Rows: 3

**Use Cases**:
- CSV parsing
- JSON parsing
- Type inference
- Basic comparison

### 2. E-Commerce (Realistic Dataset)

**Purpose**: Real-world schema simulation

**Location**: `scenarios/ecommerce/`

**Schema**:
- Tables: `users`, `products`, `orders`
- Relationships: FK constraints
- Variations: naming conventions, type differences

**Use Cases**:
- Multi-table scenarios
- Foreign key detection
- Name normalization
- Type conflict resolution

### 3. Edge Cases

**Purpose**: Boundary and error condition testing

**Datasets**:
- Empty files
- Malformed CSV/JSON
- Large schemas (100+ entities)
- Unicode/encoding test data
- Missing/null values

## Adding New Fixtures

When adding fixtures:

1. Create a dedicated subdirectory
2. Add a README.md describing:
   - Purpose
   - Schema structure
   - Test scenarios covered
   - Expected results
3. Keep datasets minimal (only data needed for tests)
4. Version control all fixtures (no gitignore)
5. Document in this file

## Fixture Standards

- Use realistic but anonymized data
- Keep file sizes small (<1MB for most cases)
- Use consistent naming: `<entity>.<format>`
- Include schema diagrams for complex datasets
- Provide both "clean" and "dirty" versions where appropriate
