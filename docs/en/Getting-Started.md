# Getting Started with AUDD

**🌐 Language / Idioma:**  
📘 [Español](../Getting-Started.md) | 📗 **English**

---

Welcome to AUDD (Dynamic Data Unification Algorithm). This guide will take you from zero to running your first schema comparison in less than 30 minutes.

## What is AUDD?

AUDD is a command-line tool and Rust library that compares data schemas from different sources (databases, CSV/JSON/XML files) and generates:
- Automatically unified schemas
- Difference and conflict reports
- Intelligent resolution suggestions
- Auditable logs of all decisions

**Use cases:**
- Migrate data between different systems
- Compare development vs production schemas
- Integrate data from multiple sources
- Audit schema changes

---

## Prerequisites

### Install Rust

AUDD requires Rust 1.70 or higher. If you don't have Rust installed:

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
Download and install from: https://rustup.rs/

**Verify installation:**
```bash
rustc --version
cargo --version
```

You should see something like:
```
rustc 1.70.0 (or higher)
cargo 1.70.0 (or higher)
```

### Optional Tools

To work with databases, you might need:
- SQLite: Included by default
- MySQL client: `sudo apt-get install libmysqlclient-dev` (Linux)
- PostgreSQL client: `sudo apt-get install libpq-dev` (Linux)

---

## Installation

### Option 1: From Source (Recommended)

```bash
# 1. Clone the repository
git clone https://github.com/jmcasimar/AUDD.git
cd AUDD

# 2. Build in release mode (optimized)
cargo build --release

# 3. The binary will be at:
ls -lh target/release/audd
```

**Estimated time:** 5-10 minutes (first time downloads dependencies)

### Option 2: Install from Cargo (Future)

```bash
# When published to crates.io:
cargo install audd
```

### Verify Installation

```bash
# If you built from source:
./target/release/audd --version

# Or add to PATH to use simply "audd":
export PATH="$PWD/target/release:$PATH"
audd --version
```

You should see:
```
audd 0.1.0
```

---

## Your First Comparison (Hello World)

### Step 1: Explore Test Data

AUDD includes sample files in `fixtures/adapters/`:

```bash
cd AUDD
ls -l fixtures/adapters/
```

You'll see:
- `users.csv` - Users in CSV format
- `users.json` - Same users in JSON
- `users.xml` - Same users in XML
- `schema.sql` - Sample SQL schema

**View users.csv content:**
```bash
cat fixtures/adapters/users.csv
```

```csv
id,name,email,age,created_at
1,Alice,alice@example.com,30,2024-01-01
2,Bob,bob@example.com,25,2024-01-02
```

**View users.json content:**
```bash
cat fixtures/adapters/users.json
```

```json
{
  "users": [
    {"id": 1, "name": "Alice", "email": "alice@example.com", "age": 30},
    {"id": 2, "name": "Bob", "email": "bob@example.com", "age": 25}
  ]
}
```

### Step 2: Inspect a Schema

Before comparing, let's see how AUDD interprets a file:

```bash
./target/release/audd inspect --source fixtures/adapters/users.csv
```

**Expected output:**
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
        {"field_name": "age", "canonical_type": {"type": "string"}, "nullable": true},
        {"field_name": "created_at", "canonical_type": {"type": "string"}, "nullable": true}
      ]
    }
  ],
  "ir_version": "1.0.0"
}
```

**Note:** By default, CSV infers all fields as `string`. Advanced type detection is a future feature.

### Step 3: Compare Two Sources

Now let's compare CSV and JSON:

```bash
./target/release/audd compare \
  --source-a fixtures/adapters/users.csv \
  --source-b fixtures/adapters/users.json \
  --out my_first_report
```

**Expected console output:**
```
🔍 AUDD Compare
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Loading schema A from fixtures/adapters/users.csv...
✓ Schema A loaded: users (1 entities)
Loading schema B from fixtures/adapters/users.json...
✓ Schema B loaded: users (1 entities)

Comparing schemas...
✓ Comparison complete!
  - Matches: 4
  - Exclusives: 1
  - Conflicts: 0

✅ Comparison completed successfully!
Output files written to: my_first_report
```

### Step 4: Explore the Results

```bash
ls -l my_first_report/
```

You'll see 4 generated files:

1. **unified_schema.json** - Unified schema (combination of both sources)
2. **diff.json** - Technical comparison details
3. **decision_log.json** - Record of automatic decisions
4. **report.md** - Human-readable report

**View the Markdown report:**
```bash
cat my_first_report/report.md
```

```markdown
# AUDD Comparison Report

**Generated:** 2026-01-26 15:35:24 UTC
**Source A:** users (csv)
**Source B:** users (json)

## Summary

- **Matches:** 4 fields matched perfectly
- **Exclusives:** 1 field exists in only one source
- **Conflicts:** 0 type conflicts detected

## Details

### Matched Fields
- `id` (string ↔ string) ✓
- `name` (string ↔ string) ✓
- `email` (string ↔ string) ✓
- `age` (string ↔ string) ✓

### Exclusive Fields
- `created_at` (only in source A) - Added to unified schema

### Conflicts
No conflicts detected.

## Unified Schema

The unified schema includes all 5 fields from both sources.
See `unified_schema.json` for full details.
```

### Step 5: Understand the Unified Schema

```bash
cat my_first_report/unified_schema.json | head -30
```

The unified schema marks the origin of each field:
- `"origin": "BOTH"` - Field exists in both sources (match)
- `"origin": "A"` - Field only in source A (exclusive)
- `"origin": "B"` - Field only in source B (exclusive)

---

## Next Level: Databases

### SQLite Example

```bash
# Inspect a SQLite database
./target/release/audd inspect --source "db:sqlite:///path/to/your/database.db"

# Compare file vs database
./target/release/audd compare \
  --source-a fixtures/adapters/users.csv \
  --source-b "db:sqlite:///path/to/your/database.db" \
  --out csv_vs_db
```

### MySQL Example

```bash
# Format: db:mysql://user:password@host/database_name
./target/release/audd inspect \
  --source "db:mysql://root:password@localhost/my_database"

# Compare two databases
./target/release/audd compare \
  --source-a "db:mysql://user:pass@localhost/development_db" \
  --source-b "db:mysql://user:pass@localhost/production_db" \
  --out dev_vs_prod
```

### Supported Connection Formats

- **SQLite:** `db:sqlite:///absolute/path/to/file.db`
- **MySQL:** `db:mysql://user:password@host:port/database`
- **PostgreSQL:** `db:postgresql://user:password@host:port/database`
- **MongoDB:** `db:mongodb://user:password@host:port/database`
- **SQL Server:** `db:sqlserver://user:password@host:port/database`
- **Firebird:** `db:firebird://user:password@host:port/path/database.fdb`

---

## Advanced Configuration

### Generate Configuration File

```bash
./target/release/audd generate-config
```

This creates `audd.toml`:

```toml
[compare]
similarity_threshold = 0.8
default_output_dir = "output"

[resolution]
confidence_threshold = 0.9
decision_id_prefix = "auto_dec"
allow_risky_suggestions = false

[output]
generate_unified_schema = true
generate_diff = true
generate_decision_log = true
generate_report = true
```

### Use Custom Configuration

```bash
# Edit audd.toml according to your needs
nano audd.toml

# AUDD will load it automatically from ./audd.toml
./target/release/audd compare --source-a a.csv --source-b b.json

# Or specify custom location
./target/release/audd --config my-config.toml compare ...
```

See [CONFIG.md](CONFIG.md) for complete configuration documentation.

---

## Main Commands

```bash
# View general help
audd --help

# View help for a specific command
audd compare --help
audd inspect --help
audd load --help

# Inspect and export IR to file
audd inspect --source data.csv --out ir_output.json

# Load and display schema in console
audd load --source data.json

# Compare with custom confidence threshold
audd compare \
  --source-a a.csv \
  --source-b b.json \
  --confidence-threshold 0.95 \
  --out results
```

---

## Typical Workflow

### 1. Initial Exploration
```bash
# Understand what each source contains
audd inspect --source old_system.csv --out old_ir.json
audd inspect --source new_system.json --out new_ir.json
```

### 2. Comparison
```bash
# Generate difference report
audd compare \
  --source-a old_system.csv \
  --source-b new_system.json \
  --out migration_analysis
```

### 3. Results Analysis
```bash
# Read the Markdown report
cat migration_analysis/report.md

# Review conflicts in detail
cat migration_analysis/diff.json | grep -A 10 "conflicts"

# Verify automatic decisions
cat migration_analysis/decision_log.json
```

### 4. Migration Planning
Use the report and unified schema to:
- Identify fields requiring transformation
- Plan type conversions
- Document differences for stakeholders

---

## Common Troubleshooting

### Error: "Cannot open file"

```bash
# Verify the file exists
ls -l fixtures/adapters/users.csv

# Use absolute path if in doubt
audd inspect --source /home/user/AUDD/fixtures/adapters/users.csv
```

### Error: "Unsupported format"

AUDD detects formats by extension. Supported extensions:
- `.csv` - CSV
- `.json` - JSON
- `.xml` - XML
- `.sql` - SQL DDL
- `db:...` - Database connections

### Error: "Cannot connect to database"

```bash
# Verify connection format
# SQLite: Absolute path after ://
audd inspect --source "db:sqlite:///home/user/database.db"

# MySQL: Verify credentials
audd inspect --source "db:mysql://user:password@localhost/database"

# Verify the service is running
systemctl status mysql  # Linux
```

### Build error

```bash
# Clean and rebuild
cargo clean
cargo build --release

# Update Rust
rustup update
```

---

## Next Steps

Now that you've run your first comparison:

1. **Explore advanced examples:** See [examples/cli/README.md](../examples/cli/README.md)
2. **Read about architecture:** See [Architecture.md](Architecture.md)
3. **Learn advanced configuration:** See [CONFIG.md](CONFIG.md)
4. **Explore use cases:** See [Usage-Examples.md](Usage-Examples.md)
5. **Contribute to the project:** See [CONTRIBUTING.md](../CONTRIBUTING.md)

---

## Get Help

- **Documentation:** [docs/](.)
- **Common problems:** [FAQ.md](FAQ.md)
- **GitHub Issues:** https://github.com/jmcasimar/AUDD/issues
- **Discussions:** https://github.com/jmcasimar/AUDD/discussions

---

**Congratulations! You're now ready to use AUDD in your data integration projects.**
