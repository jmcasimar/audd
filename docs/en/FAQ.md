# FAQ - Frequently Asked Questions

**🌐 Language / Idioma:**  
📘 [Español](../FAQ.md) | 📗 **English**

---

## Table of Contents

- [General](#general)
- [Installation and Configuration](#installation-and-configuration)
- [Data Sources](#data-sources)
- [Comparison and Results](#comparison-and-results)
- [Advanced Configuration](#advanced-configuration)
- [Troubleshooting](#troubleshooting)
- [Contribution and Development](#contribution-and-development)

---

## General

### What is AUDD?

AUDD (Dynamic Data Unification Algorithm) is a CLI tool and Rust library for comparing data schemas from different sources (CSV, JSON, XML, SQL/NoSQL databases) and generating:
- Unified schemas automatically
- Difference and conflict reports
- Intelligent resolution suggestions
- Auditable decision logs

### What is AUDD used for?

**Main use cases:**
- **Data migrations**: Compare old vs new schema before migrating
- **Data integration**: Unify data from multiple heterogeneous sources
- **Schema auditing**: Verify consistency between development and production
- **ETL planning**: Identify necessary transformations between systems

### Is AUDD open source?

Yes, AUDD is licensed under the MIT License. You can use, modify, and distribute it freely.

### What language is it written in?

AUDD is written entirely in Rust, which ensures high performance and memory safety.

---

## Installation and Configuration

### What do I need to install AUDD?

**Requirements:**
- Rust 1.70 or higher
- Cargo (included with Rust)
- Git (to clone the repository)

**Install Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### How do I install AUDD?

```bash
# 1. Clone repository
git clone https://github.com/jmcasimar/AUDD.git
cd AUDD

# 2. Build
cargo build --release

# 3. Run
./target/release/audd --version
```

See [Getting-Started.md](Getting-Started.md) for the complete guide.

### Can I install AUDD without compiling?

Currently, it's only available by compiling from source. In the future, it will be published to crates.io for installation with `cargo install audd`.

### How do I update AUDD?

```bash
cd AUDD
git pull origin main
cargo build --release
```

### Do I need to install databases to use AUDD?

Not necessarily:
- **SQLite**: Included by default, no server required
- **MySQL/PostgreSQL/MongoDB**: Only if you want to connect to these databases
- **Files (CSV/JSON/XML)**: Don't require any database

---

## Data Sources

### What file formats does AUDD support?

| Format | Extension | Auto-detection | Example |
|---------|-----------|----------------|---------|
| CSV     | `.csv`    | ✓             | `data.csv` |
| JSON    | `.json`   | ✓             | `data.json` |
| XML     | `.xml`    | ✓             | `data.xml` |
| SQL DDL | `.sql`    | ✓             | `schema.sql` |

### What databases can I connect to?

**Supported databases:**
- SQLite (default)
- MySQL (default)
- PostgreSQL (default)
- MongoDB (default)
- SQL Server (requires `sqlserver` feature)
- Firebird (requires `firebird` feature)

### How do I connect to a database?

**General format:**
```
db:<type>://<user>:<password>@<host>:<port>/<database>
```

**Examples:**
```bash
# SQLite (absolute path)
audd inspect --source "db:sqlite:///home/user/data.db"

# MySQL
audd inspect --source "db:mysql://root:password@localhost:3306/my_db"

# PostgreSQL
audd inspect --source "db:postgresql://user:pass@localhost:5432/my_db"

# MongoDB
audd inspect --source "db:mongodb://user:pass@localhost:27017/my_db"
```

### Can I compare a file with a database?

Yes, AUDD can compare any combination of sources:

```bash
audd compare \
  --source-a data.csv \
  --source-b "db:mysql://user:pass@localhost/production" \
  --out comparison
```

### Does it support remote files (URLs)?

Yes, AUDD can load files from HTTP/HTTPS:

```bash
audd inspect --source "https://example.com/data.csv"
audd inspect --source "https://docs.google.com/spreadsheets/...export?format=csv"
```

### What if my CSV doesn't have headers?

AUDD requires CSV files to have a header row. If they don't, add one manually or use tools like `sed`:

```bash
# Add generic headers
echo "col1,col2,col3" | cat - data.csv > data_with_headers.csv
```

### Does it automatically detect data types in CSV?

Currently, CSV infers all fields as `String`. Advanced type detection is in development. For specific types, use:
- SQL DDL files (`.sql`) that specify types
- Databases that already have defined types
- JSON which preserves primitive types (number, boolean, string)

---

## Comparison and Results

### What files does the `compare` command generate?

The `compare` command generates 4 files by default:

1. **unified_schema.json** - Unified schema (combines both sources)
2. **diff.json** - Detailed comparison results (matches, exclusives, conflicts)
3. **decision_log.json** - Log of automatic resolution decisions
4. **report.md** - Human-readable report in Markdown

**Example:**
```bash
audd compare --source-a a.csv --source-b b.json --out results
ls results/
# unified_schema.json  diff.json  decision_log.json  report.md
```

### What do "Match", "Exclusive" and "Conflict" mean?

- **Match**: Field exists in both sources with the same type → No problems
- **Exclusive**: Field exists in only one source → Included in unified schema
- **Conflict**: Field exists in both but with incompatible types → Requires resolution

**Example output:**
```
✓ Comparison complete!
  - Matches: 6      (identical fields)
  - Exclusives: 2   (unique fields)
  - Conflicts: 1    (incompatible types)
```

### How do I interpret the Markdown report?

The `report.md` contains:

```markdown
# AUDD Comparison Report

## Summary
- Matches: 6 fields matched perfectly
- Exclusives: 1 field exists in only one source
- Conflicts: 3 type conflicts detected

## Details
### Matched Fields
- `id` (Int32 ↔ Int32) ✓
- `name` (String ↔ String) ✓

### Exclusive Fields
- `created_at` (only in source A)

### Conflicts
- `age`: String (A) vs Int32 (B)
  - Suggestion: Cast A.age to Int32 (confidence: 0.85)
```

Read the conflicts section to identify what requires attention.

### What is the "confidence_threshold"?

The `confidence_threshold` determines which automatic suggestions are accepted:

- **Value**: 0.0 to 1.0 (default: 0.9)
- **Logic**: Only suggestions with `confidence >= threshold` are auto-accepted

**Example:**
```bash
# More conservative (only very safe suggestions)
audd compare --source-a a.csv --source-b b.json --confidence-threshold 0.95

# More aggressive (accepts more suggestions)
audd compare --source-a a.csv --source-b b.json --confidence-threshold 0.75
```

### How does AUDD resolve conflicts?

AUDD generates **automatic suggestions**:

1. **CastSafe** (0.95 confidence): Safe conversion (Int32 → Int64)
2. **CastRisky** (0.6 confidence): Risky conversion (Float → Int)
3. **RenameField** (variable): Rename similar field
4. **PreferType** (0.8 confidence): Prefer type from one source
5. **ManualIntervention** (0.0): Requires human decision

Suggestions with confidence >= threshold are applied automatically.

### Can I review what decisions were made?

Yes, the `decision_log.json` records all decisions:

```json
{
  "metadata": {
    "total_decisions": 3,
    "accepted_decisions": 2,
    "rejected_decisions": 1
  },
  "decisions": [
    {
      "decision_id": "auto_dec_001",
      "conflict_type": "TypeMismatch",
      "suggested_action": "CastSafe",
      "confidence": 0.95,
      "accepted": true,
      "rationale": "Safe upcast from Int32 to Int64"
    }
  ]
}
```

### What if I disagree with an automatic decision?

1. **Review** the `decision_log.json` to understand the logic
2. **Adjust** the `confidence_threshold` to be more conservative
3. **Edit** the `unified_schema.json` manually if necessary
4. **Report** the case if you believe the suggestion is incorrect (GitHub issue)

---

## Advanced Configuration

### How do I create a configuration file?

```bash
audd generate-config
```

This creates `audd.toml` in the current directory:

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

### Where does AUDD look for the configuration file?

**Search order:**
1. `./audd.toml` (current directory)
2. `~/.audd.toml` (user home)
3. `~/.config/audd/config.toml` (XDG config)
4. Specified with `--config path/to/config.toml`

### What does "similarity_threshold" mean?

The `similarity_threshold` controls how similar names must be to be considered a "match":

- **Value**: 0.0 to 1.0 (default: 0.8)
- **Algorithm**: Jaro-Winkler similarity
- **Example**: "user_id" vs "userId" → 0.87 (match if threshold ≤ 0.87)

```toml
[compare]
similarity_threshold = 0.9  # Stricter (requires more similarity)
```

### What does "allow_risky_suggestions" do?

Controls whether suggestions with risk of data loss are allowed:

```toml
[resolution]
allow_risky_suggestions = false  # Don't allow risky casts (default)
allow_risky_suggestions = true   # Allow (e.g., Float → Int)
```

**Example of risky suggestion:**
- Convert `Float64` to `Int32` (loses decimals)
- Convert `DateTime` to `Date` (loses time)

### Can I disable generation of some output files?

Yes, in the configuration:

```toml
[output]
generate_unified_schema = true   # unified_schema.json
generate_diff = true             # diff.json
generate_decision_log = true     # decision_log.json
generate_report = true           # report.md
```

Set to `false` to skip that file.

---

## Troubleshooting

### Error: "Cannot open file"

**Cause:** The file doesn't exist or the path is incorrect

**Solution:**
```bash
# Verify the file exists
ls -l data.csv

# Use absolute path
audd inspect --source /home/user/project/data.csv

# Or navigate to the file's directory
cd /home/user/project
audd inspect --source data.csv
```

### Error: "Unsupported format"

**Cause:** AUDD doesn't recognize the file extension

**Solution:**
- Check the extension: `.csv`, `.json`, `.xml`, `.sql`
- Rename the file with the correct extension
- For databases, use the `db:` prefix

```bash
# Incorrect
audd inspect --source data.txt

# Correct
mv data.txt data.csv
audd inspect --source data.csv
```

### Error: "Cannot connect to database"

**Cause:** Incorrect credentials, service not running, or incorrect connection format

**Solution:**

1. **Check service:**
```bash
# MySQL
sudo systemctl status mysql

# PostgreSQL
sudo systemctl status postgresql
```

2. **Verify credentials:**
```bash
# Test manual connection
mysql -u user -p -h localhost db_name
psql -U user -h localhost db_name
```

3. **Verify connection format:**
```bash
# SQLite: ABSOLUTE path after ://
db:sqlite:///absolute/path/file.db

# MySQL: Include port if not 3306
db:mysql://user:password@localhost:3306/db
```

4. **Database permissions:**
Ensure the user has `SELECT` permissions on the schema tables.

### Error: "Failed to parse CSV"

**Common causes:**
- CSV without headers
- Incorrect delimiter (e.g., semicolon instead of comma)
- Incorrect encoding (not UTF-8)

**Solution:**
```bash
# Check first lines
head -5 data.csv

# Convert to UTF-8 if necessary
iconv -f ISO-8859-1 -t UTF-8 data.csv > data_utf8.csv

# If using another delimiter, convert to standard CSV
sed 's/;/,/g' data_semicolon.csv > data.csv
```

### Compilation error: "linker `cc` not found"

**Cause:** Missing C compiler (needed for some dependencies)

**Solution:**
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# Fedora/RHEL
sudo dnf install gcc

# macOS
xcode-select --install
```

### Error: "libmysqlclient not found"

**Cause:** Missing MySQL client library

**Solution:**
```bash
# Ubuntu/Debian
sudo apt-get install libmysqlclient-dev

# Fedora/RHEL
sudo dnf install mysql-devel

# macOS
brew install mysql-client
```

### Comparison is very slow

**Optimizations:**

1. **Reduce scope:**
```bash
# Instead of comparing entire DB, export only relevant tables to SQL
mysqldump schema_only db table1 table2 > subset.sql
audd compare --source-a subset.sql --source-b data.json
```

2. **Use files instead of live DB connections:**
```bash
# Export schemas first
audd inspect --source "db:mysql://..." --out schema_a.json
audd inspect --source "db:postgresql://..." --out schema_b.json

# Compare IR files
# (requires external tool or manual loading)
```

3. **Increase similarity_threshold** (reduces comparisons):
```toml
[compare]
similarity_threshold = 0.9  # Stricter match = fewer comparisons
```

### How can I see debug messages?

```bash
# Enable verbose logs
RUST_LOG=debug ./target/release/audd compare ...

# Or only for AUDD
RUST_LOG=audd=debug ./target/release/audd compare ...
```

---

## Contribution and Development

### How can I contribute to AUDD?

See [CONTRIBUTING.md](CONTRIBUTING.md) for the complete guide.

**Ways to contribute:**
- Report bugs (GitHub Issues)
- Suggest features (GitHub Discussions)
- Improve documentation
- Add tests
- Implement new adapters
- Translate documentation

### How do I report a bug?

1. Go to https://github.com/jmcasimar/AUDD/issues
2. Search if a similar issue already exists
3. If not, create a new one with:
   - Clear description of the problem
   - Steps to reproduce
   - AUDD version (`audd --version`)
   - Operating system
   - Sample files (if applicable)

### How do I run the tests?

```bash
# All tests
cargo test

# Tests for a specific crate
cargo test -p audd_compare

# Specific test
cargo test test_csv_comparison

# With verbose output
cargo test -- --nocapture
```

### How do I add support for a new format?

See [Architecture.md](Architecture.md) "Extensibility" section for detailed guide.

**Basic steps:**
1. Create new adapter in `crates/audd_adapters_file/src/`
2. Implement `FileAdapter` trait
3. Register in `AdapterRegistry`
4. Add tests
5. Update documentation

### Can I use AUDD as a library in my Rust project?

Yes, AUDD is designed as a modular library:

```toml
# Cargo.toml
[dependencies]
audd_ir = { path = "path/to/AUDD/crates/audd_ir" }
audd_compare = { path = "path/to/AUDD/crates/audd_compare" }
```

```rust
use audd_ir::SourceSchema;
use audd_compare::compare_schemas;

fn main() {
    let schema_a: SourceSchema = load_from_somewhere();
    let schema_b: SourceSchema = load_from_somewhere_else();
    
    let result = compare_schemas(&schema_a, &schema_b, 0.8);
    println!("{:?}", result);
}
```

---

## Additional Resources

### Documentation
- [Getting Started](Getting-Started.md) - Getting started guide
- [Architecture](Architecture.md) - System architecture
- [Configuration](CONFIG.md) - Detailed configuration
- [Usage Examples](Usage-Examples.md) - Advanced examples

### Support
- [GitHub Issues](https://github.com/jmcasimar/AUDD/issues) - Report bugs
- [GitHub Discussions](https://github.com/jmcasimar/AUDD/discussions) - Questions and discussion

### Community
- [Contributing](CONTRIBUTING.md) - Contributing guide
- [Code of Conduct](../CODE_OF_CONDUCT.md) - Code of conduct

---

**Can't find your question?** Open a [Discussion](https://github.com/jmcasimar/AUDD/discussions) on GitHub.

**Last updated:** 2026-01-26
