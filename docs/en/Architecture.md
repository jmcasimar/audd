# AUDD Architecture

**🌐 Language / Idioma:**  
📘 [Español](../Architecture.md) | 📗 **English**

---

## Overview

AUDD (Dynamic Data Unification Algorithm) is a modular system built in Rust that enables comparison and unification of heterogeneous data schemas. The architecture follows design principles prioritizing extensibility, separation of concerns, and decision auditability.

### Design Principles

1. **Canonical Representation (IR)**: All schemas normalized to an intermediate model
2. **Pluggable Adapters**: New data sources can be added without modifying the core
3. **Deterministic Comparison**: Reproducible results for testing and auditing
4. **Transparent Resolution**: All decisions are documented and auditable
5. **Separation of Concerns**: Each crate has a clearly defined responsibility

---

## Layer Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     CLI / API Layer                          │
│                   (audd-cli crate)                           │
│  - Command parsing (clap)                                   │
│  - User interaction                                         │
│  - Output formatting                                        │
└────────────────────┬────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────┐
│              Orchestration Layer                             │
│  - Workflow coordination                                    │
│  - Error handling & reporting                               │
│  - Configuration management                                 │
└────┬───────────────┬──────────────────┬─────────────────────┘
     │               │                  │
     │               │                  │
┌────▼──────┐  ┌────▼─────────┐  ┌────▼──────────────────────┐
│  Adapters │  │  Comparison  │  │  Resolution Engine         │
│  Layer    │  │  Engine      │  │  (audd_resolution)         │
│           │  │ (audd_compare)│  │  - Conflict detection     │
│  File:    │  │  - Matching  │  │  - Suggestion generation  │
│  - CSV    │  │  - Diffing   │  │  - Confidence scoring     │
│  - JSON   │  │  - Unified   │  │  - Decision tracking      │
│  - XML    │  │    schema    │  └───────────────────────────┘
│  - SQL    │  │  - Metrics   │
│  - Remote │  └──────────────┘
│           │
│  Database:│
│  - SQLite │
│  - MySQL  │
│  - Postgres│
│  - MongoDB│
│  - SQLServer│
│  - Firebird│
└─────┬─────┘
      │
┌─────▼──────────────────────────────────────────────────────┐
│          Intermediate Representation (IR)                   │
│                (audd_ir crate)                              │
│  - SourceSchema: Normalized schema model                   │
│  - CanonicalType: Unified type system                      │
│  - EntitySchema: Tables/Collections representation         │
│  - FieldSchema: Column/Field metadata                      │
└─────────────────────────────────────────────────────────────┘
```

---

## Main Components

### 1. audd_ir (Intermediate Representation)

**Purpose:** Canonical model to represent schemas from any source

**Main Types:**

```rust
pub struct SourceSchema {
    pub source_name: String,
    pub source_type: String,
    pub entities: Vec<EntitySchema>,
    pub ir_version: String,
    pub metadata: HashMap<String, Value>,
}

pub enum CanonicalType {
    Boolean,
    Int32,
    Int64,
    Float32,
    Float64,
    Decimal { precision: u8, scale: u8 },
    String { max_length: Option<u32> },
    Text,
    Date,
    Time,
    DateTime,
    Timestamp,
    UUID,
    Binary,
    Json,
    Array { element_type: Box<CanonicalType> },
    Unknown,
}
```

**Responsibilities:**
- Define canonical data structure
- JSON serialization/deserialization
- Identifier normalization
- Schema validation

---

### 2. audd_adapters_file (File Adapters)

**Purpose:** Convert files to IR

**Supported Formats:**

| Format | Extension | Auto-detection | Type Inference |
|---------|-----------|----------------|----------------|
| CSV     | `.csv`    | ✓             | Basic (string) |
| JSON    | `.json`   | ✓             | ✓ (primitives) |
| XML     | `.xml`    | ✓             | Basic (string) |
| SQL DDL | `.sql`    | ✓             | ✓ (SQL types)  |

**Conversion Process:**

```
File → Parser → Schema Detector → IR Generator → SourceSchema
```

---

### 3. audd_adapters_db (Database Adapters)

**Purpose:** Extract database schemas to IR

**Supported Databases:**

| Database    | Feature Flag | Schema Extraction | Metadata |
|-------------|--------------|-------------------|----------|
| SQLite      | (default)    | ✓                | ✓        |
| MySQL       | (default)    | ✓                | ✓        |
| PostgreSQL  | (default)    | ✓                | ✓        |
| MongoDB     | (default)    | ✓ (inference)    | ✓        |
| SQL Server  | `sqlserver`  | ✓                | ✓        |
| Firebird    | `firebird`   | ✓                | ✓        |

**SQL Type Mapping → Canonical:**

| MySQL Type    | PostgreSQL Type | CanonicalType              |
|---------------|-----------------|----------------------------|
| TINYINT(1)    | BOOLEAN         | Boolean                    |
| INT           | INTEGER         | Int32                      |
| BIGINT        | BIGINT          | Int64                      |
| FLOAT         | REAL            | Float32                    |
| DOUBLE        | DOUBLE          | Float64                    |
| DECIMAL(p,s)  | NUMERIC(p,s)    | Decimal{precision, scale}  |
| VARCHAR(n)    | VARCHAR(n)      | String{max_length}         |
| TEXT          | TEXT            | Text                       |
| DATE          | DATE            | Date                       |
| DATETIME      | TIMESTAMP       | DateTime                   |
| JSON          | JSONB           | Json                       |

---

### 4. audd_compare (Comparison Engine)

**Purpose:** Compare two IR schemas and generate unified schema

**Comparison Algorithm:**

```
Input: SourceSchema A, SourceSchema B
Output: ComparisonResult, UnifiedSchema

1. Entity Matching:
   - Compare entity names using Jaro-Winkler similarity
   - Threshold: similarity_threshold (default 0.8)
   - Result: entity pairs (matched, exclusive_a, exclusive_b)

2. Field Matching (for each entity pair):
   - Compare field names (Jaro-Winkler)
   - Result: matched_fields, exclusive_a, exclusive_b

3. Conflict Detection (for each matched field pair):
   - Compare canonical types
   - Verify nullable
   - Result: conflicts with details

4. Unified Schema Generation:
   - Include all matched fields (origin: BOTH)
   - Include exclusive fields (origin: A or B)
   - Mark conflicts as needs_resolution
```

**Jaro-Winkler Similarity:**
- Robust algorithm for detecting string similarity
- Tolerates typos and variations
- Value: 0.0 (no similarity) to 1.0 (identical)
- Example: "user_id" vs "userId" → 0.87

**Result Types:**

```rust
pub enum FieldOrigin {
    A,           // Only in source A
    B,           // Only in source B
    BOTH,        // In both sources (matched)
}

pub enum FieldState {
    Matched,              // No conflicts
    Exclusive,            // Only in one source
    Conflict { details }, // Incompatible types
}
```

---

### 5. audd_resolution (Resolution Engine)

**Purpose:** Generate suggestions to resolve conflicts

**Suggestion Engine:**

```rust
pub enum Suggestion {
    // Safe conversion (no data loss)
    CastSafe {
        from: CanonicalType,
        to: CanonicalType,
        confidence: f64,
    },
    
    // Risky conversion (possible data loss)
    CastRisky {
        from: CanonicalType,
        to: CanonicalType,
        confidence: f64,
        risk_description: String,
    },
    
    // Rename field
    RenameField {
        from: String,
        to: String,
        confidence: f64,
    },
    
    // Prefer type from one source
    PreferType {
        source: FieldOrigin,
        reason: String,
        confidence: f64,
    },
    
    // Manual intervention needed
    ManualIntervention {
        reason: String,
    },
}
```

**Safe Conversions Matrix:**

| From → To       | Safe? | Example                      |
|-----------------|-------|------------------------------|
| Int32 → Int64   | ✓     | 100 → 100                    |
| Int64 → Int32   | ✗     | Possible overflow            |
| Int32 → Float64 | ✓     | 42 → 42.0                    |
| Float64 → Int32 | ✗     | Loss of decimals             |
| String → Text   | ✓     | No length restriction        |
| Text → String   | ✗     | Possible truncation          |
| Date → DateTime | ✓     | 2024-01-01 → 2024-01-01 00:00|
| DateTime → Date | ✗     | Loss of time information     |

**Confidence Calculation:**

```rust
fn calculate_confidence(suggestion: &Suggestion) -> f64 {
    match suggestion {
        Suggestion::CastSafe { .. } => 0.95,  // High confidence
        Suggestion::RenameField { from, to, .. } => {
            // Based on name similarity
            jaro_winkler(from, to)
        },
        Suggestion::CastRisky { .. } => 0.6,  // Moderate confidence
        Suggestion::ManualIntervention { .. } => 0.0,
        _ => 0.8,
    }
}
```

---

### 6. audd-cli (Command-Line Interface)

**Purpose:** User interface and workflow orchestration

**Internal Workflow (Compare Command):**

```
1. Parse CLI arguments (clap)
2. Load configuration (file + CLI overrides)
3. Load schema A (adapters)
   └─ Detect format → Select adapter → Parse → Generate IR
4. Load schema B
   └─ Detect format → Select adapter → Parse → Generate IR
5. Compare schemas (audd_compare)
   └─ Match entities → Match fields → Detect conflicts
6. Generate suggestions (audd_resolution)
   └─ Analyze conflicts → Generate suggestions → Score confidence
7. Create unified schema
   └─ Merge matched → Include exclusives → Mark conflicts
8. Generate outputs
   ├─ unified_schema.json (SourceSchema)
   ├─ diff.json (ComparisonResult)
   ├─ decision_log.json (DecisionLog)
   ├─ report.md (Markdown summary)
   └─ report.json (optional, structured)
9. Display summary to user
```

---

## Data Flows

### Inspection Flow

```
User Input (file/db path)
    ↓
Format Detection
    ↓
Adapter Selection
    ↓
Schema Extraction
    ↓
IR Generation
    ↓
JSON Output / Console Display
```

### Comparison Flow

```
Source A Input → Adapter A → IR-A ─┐
                                    ├→ Comparison Engine → Results
Source B Input → Adapter B → IR-B ─┘                           ↓
                                                    ┌───────────┴──────────┐
                                                    │                      │
                                              Resolution Engine    Unified Schema
                                                    ↓                      ↓
                                              Suggestions            Merged IR
                                                    ↓                      ↓
                                              Decision Log          unified_schema.json
                                                    ↓
                                              Output Files
                                              (diff.json, report.md, etc.)
```

---

## Extensibility

### Adding a New File Adapter

```rust
// 1. Implement Adapter trait
pub trait FileAdapter {
    fn can_handle(&self, path: &Path) -> bool;
    fn parse(&self, path: &Path) -> Result<SourceSchema>;
}

// 2. Implement for new format
pub struct YamlAdapter;

impl FileAdapter for YamlAdapter {
    fn can_handle(&self, path: &Path) -> bool {
        path.extension().map_or(false, |e| e == "yaml" || e == "yml")
    }
    
    fn parse(&self, path: &Path) -> Result<SourceSchema> {
        // YAML parsing logic → IR
    }
}

// 3. Register in AdapterRegistry
registry.register(Box::new(YamlAdapter));
```

### Adding a New Database

```rust
// 1. Add feature in Cargo.toml
[features]
oracle = ["oracle-driver"]

// 2. Implement DbAdapter trait
pub struct OracleAdapter {
    connection_string: String,
}

impl DbAdapter for OracleAdapter {
    fn extract_schema(&self) -> Result<SourceSchema> {
        // 1. Connect to Oracle
        // 2. Query ALL_TABLES, ALL_TAB_COLUMNS
        // 3. Map Oracle types → CanonicalType
        // 4. Build IR
    }
}
```

---

## Performance Considerations

### Implemented Optimizations

1. **Streaming parsers**: CSV and JSON use streaming parsers for large files
2. **Lazy loading**: Adapters load schemas on-demand
3. **String interning**: Normalized field/type names are shared
4. **Connection pooling**: DB connection reuse (when applicable)

### Known Limits

| Aspect              | Current Limit        | Notes                           |
|---------------------|----------------------|---------------------------------|
| CSV file size       | ~1GB                 | Depends on available memory     |
| Fields per entity   | No hard limit        | Performance degrades >10k fields|
| Entities per schema | No hard limit        | Performance degrades >1k tables |
| JSON depth          | 128 levels           | serde_json parser limit         |

---

## Design Decisions

### Why Rust?

- **Performance**: Necessary for processing large data volumes
- **Safety**: Type system prevents common bugs in parsing/conversion
- **Concurrency**: Ready for future parallelization
- **Tooling**: Cargo, rustfmt, clippy facilitate development

### Why Canonical IR?

- **Decouples** sources from comparison logic
- **Simplifies** adding new adapters
- **Normalizes** irrelevant syntactic differences
- **Facilitates** testing with controlled fixtures

### Why Jaro-Winkler?

- **Robust** against common variations (camelCase vs snake_case)
- **Fast** O(n) for typical strings
- **Calibrated** for field names (prefers matches at the beginning)

---

## Future Roadmap

### Planned Features

- **Advanced type inference** for CSV
- **Incremental support** (compare delta changes)
- **Interactive mode** (resolve conflicts in CLI)
- **Migration script generation** (SQL ALTER TABLE, etc.)
- **REST API** (web service for comparisons)
- **Complex schema support** (relationships, constraints)
- **Machine learning** for smarter suggestions

---

**Last updated:** 2026-01-26  
**Architecture version:** 1.0  
**Status:** MVP implementation completed
