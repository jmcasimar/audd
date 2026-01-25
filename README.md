# AUDD - Algoritmo de Unificación Dinámica de Datos

**Dynamic Data Unification Algorithm**

A Rust-based tool for intelligent data comparison and unification across heterogeneous sources.

## 🎯 Purpose

AUDD provides automated data reconciliation and schema mapping for datasets from different sources, enabling efficient data integration workflows.

## ✨ Features

- Dynamic schema detection and alignment
- Multi-format support (CSV, JSON, XML)
- Intelligent field mapping
- Conflict resolution strategies
- CLI and library interfaces

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

**Compare two data sources:**
```bash
audd compare --source1 data1.csv --source2 data2.json
```

**Get help:**
```bash
audd --help
audd compare --help
```

### Example

```bash
# Compare customer data from two systems
audd compare \
  --source1 crm_export.csv \
  --source2 erp_data.json \
  --format json
```

**Output (stub):**
```
🔍 AUDD Compare (Stub Implementation)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Source 1: crm_export.csv
Source 2: erp_data.json
Format:   json

✓ Comparison completed successfully!

Note: This is a stub implementation.
Full comparison logic will be implemented in upcoming sprints.
```

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

## 📝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 🔒 Security

See [SECURITY.md](SECURITY.md) for reporting procedures.

## 📄 License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 👥 Authors

AUDD Contributors - See project repository for details.

## 🙏 Acknowledgments

This project is part of academic research on data integration and open-source transfer strategies.

---

**Status:** Early development (v0.1.0) - Core implementation in progress
