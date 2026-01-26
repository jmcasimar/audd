# AUDD - Algoritmo de Unificación Dinámica de Datos

**Dynamic Data Unification Algorithm**

---

**🌐 Language / Idioma:**  
📘 [Español (Base)](docs/es/README_PRINCIPAL.md) | 📗 [English](README.md)

> **Note:** Spanish is the base language for this project. Documentation is maintained in both Spanish and English.  
> **Nota:** El español es el idioma base de este proyecto. La documentación se mantiene en español e inglés.

---

A Rust-based tool for intelligent data comparison and unification across heterogeneous sources.

## 🎯 Purpose

AUDD provides automated data reconciliation and schema mapping for datasets from different sources, enabling efficient data integration workflows.

## ✨ Features

- **File Adapters**: Load schemas from CSV, JSON, XML, and SQL/DDL files
- **Database Adapters**: Connect to SQLite, MySQL, PostgreSQL, MongoDB, SQL Server, and Firebird
- **Intermediate Representation (IR)**: Canonical schema model for heterogeneous sources
- **Auto-detection**: Automatic format detection from file extensions
- **Type Inference**: Smart type detection for JSON and SQL sources
- **Conflict Detection**: Advanced schema comparison and conflict identification
- **Resolution Engine**: Automated and manual conflict resolution strategies
- **Unified Schema Generation**: Automatic creation of unified schema (C) from sources A and B
- **Auditable Decisions**: Track and document all schema unification decisions
- **Multiple Output Formats**: JSON schemas, diff reports, decision logs, and Markdown reports
- **CLI and Library**: Use as command-line tool or Rust library

## 🚀 Quick Start

### Installation

**From source:**
```bash
git clone https://github.com/jmcasimar/AUDD.git
cd AUDD
cargo build --release
```

Binary available at: `target/release/audd`

### Usage

**Generate configuration file:**
```bash
# Create a config file with default settings
audd generate-config

# Customize behavior (optional)
# Edit audd.toml to set confidence thresholds, output options, etc.
```

**Inspect a schema (IR export):**
```bash
# Print to stdout
audd inspect --source users.csv

# Save to file
audd inspect --source schema.sql --out ir.json
```

**Load and display schema:**
```bash
audd load --source users.csv
audd load --source schema.sql
audd load --source data.json
```

**Compare two data sources:**
```bash
audd compare \
  --source-a data1.csv \
  --source-b data2.json \
  --out output

# Generates:
# - output/unified_schema.json  (Unified schema C)
# - output/diff.json             (Comparison results)
# - output/decision_log.json     (Resolution decisions)
# - output/report.md             (Human-readable report)
# - output/report.json           (Structured report, optional)

# Use custom config file
audd --config team-config.toml compare --source-a a.csv --source-b b.json

# Override confidence threshold
audd compare --source-a a.csv --source-b b.json --confidence-threshold 0.95
```

**Work with databases:**
```bash
# Inspect a database
audd inspect --source "db:sqlite:///path/to/db.sqlite"

# Compare file vs database
audd compare \
  --source-a users.csv \
  --source-b "db:mysql://user:pass@host/db" \
  --out comparison_output
```

**Get help:**
```bash
audd --help
audd compare --help
audd inspect --help
audd generate-config --help
```

### Example

```bash
# Compare CSV and JSON schemas
audd compare \
  --source-a fixtures/adapters/users.csv \
  --source-b fixtures/adapters/users.json \
  --out output

# Output:
# 🔍 AUDD Compare
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Loading schema A from fixtures/adapters/users.csv...
# ✓ Schema A loaded: users (1 entities)
# Loading schema B from fixtures/adapters/users.json...
# ✓ Schema B loaded: users (1 entities)
#
# Comparing schemas...
# ✓ Comparison complete!
#   - Matches: 6
#   - Exclusives: 1
#   - Conflicts: 3
#
# ✅ Comparison completed successfully!
# Output files written to: output
```

For more examples, see [`examples/cli/README.md`](examples/cli/README.md).

## 🏗️ Architecture

```
┌─────────────┐
│   CLI/API   │
└──────┬──────┘
       │
┌──────▼──────────────────────────┐
│  Data Ingestion & Parsing       │
│  (CSV, JSON, XML readers)       │
└──────┬──────────────────────────┘
       │
┌──────▼──────────────────────────┐
│  Schema Detection & Mapping     │
│  (Field alignment, type infer)  │
└──────┬──────────────────────────┘
       │
┌──────▼──────────────────────────┐
│  Comparison Engine              │
│  (Diff algorithm, matching)     │
└──────┬──────────────────────────┘
       │
┌──────▼──────────────────────────┐
│  Unification & Output           │
│  (Conflict resolution, export)  │
└─────────────────────────────────┘
```

## 📋 Roadmap (MVP)

- **Sprint 1:** Core data parsing and schema detection
- **Sprint 2:** Comparison algorithm and field matching
- **Sprint 3:** Unification engine and conflict resolution
- **Sprint 4:** Multi-format support and optimizations
- **Sprint 5:** Documentation and performance tuning

## 🛠️ Development

### Prerequisites
- Rust 1.70+
- Cargo

### Build
```bash
cargo build
```

### Test
```bash
cargo test
```

### Format & Lint
```bash
cargo fmt
cargo clippy
```

## 📚 Documentation

**Complete documentation is available in both languages:**

- 📘 **Spanish (Base):** [docs/es/](docs/es/)
- 📗 **English:** [docs/en/](docs/en/)

For detailed guides, see:
- [Configuration Guide](docs/en/CONFIG.md) | [Guía de Configuración](docs/es/CONFIG.md)
- [IR Specification](docs/en/ir.md) | [Especificación IR](docs/es/ir.md)
- [CLI Examples](examples/cli/README.md) | [Ejemplos CLI](examples/cli/README_ES.md)

## 📝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) | [Contribuciones](docs/es/CONTRIBUTING.md) for guidelines.

## 🔒 Security

See [SECURITY.md](SECURITY.md) | [Seguridad](docs/es/SECURITY.md) for reporting procedures.

## 📄 License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 👥 Authors

AUDD Contributors - See project repository for details.

## 🙏 Acknowledgments

This project is part of academic research on data integration and open-source transfer strategies.

---

**Status:** Early development (v0.1.0) - Core implementation in progress
