# Usage Examples - AUDD

**🌐 Language / Idioma:**  
📘 [Español](../Usage-Examples.md) | 📗 **English**

---

This guide provides practical examples and real-world scenarios for using AUDD in different data integration and migration contexts.

## Table of Contents

- [Basic Examples](#basic-examples)
- [Working with Databases](#working-with-databases)
- [Real-World Scenarios](#real-world-scenarios)
- [Advanced Configuration](#advanced-configuration)
- [Workflow Integration](#workflow-integration)

---

## Basic Examples

### 1. Generate Configuration File

Create a configuration file to customize AUDD behavior:

```bash
# Create configuration with default values
audd generate-config

# Create in custom location
audd generate-config --out ~/.audd.toml

# Create team configuration
audd generate-config --out team-config.toml
```

See [CONFIG.md](CONFIG.md) for detailed configuration documentation.

### 2. Inspect a Single Data Source

Load and inspect the Intermediate Representation (IR) of a data source:

```bash
# Inspect CSV file
audd inspect --source fixtures/adapters/users.csv

# Inspect JSON and save to file
audd inspect --source fixtures/adapters/users.json --out ir_output.json

# Inspect SQL DDL file
audd inspect --source fixtures/adapters/schema.sql

# Inspect XML file
audd inspect --source fixtures/adapters/users.xml --out users_ir.json
```

**Expected output (JSON):**
```json
{
  "source_name": "users",
  "source_type": "csv",
  "entities": [
    {
      "entity_name": "users",
      "entity_type": "table",
      "fields": [
        {
          "field_name": "id",
          "canonical_type": {"type": "string"},
          "nullable": true
        },
        {
          "field_name": "name",
          "canonical_type": {"type": "string"},
          "nullable": true
        }
      ]
    }
  ],
  "ir_version": "1.0.0"
}
```

### 3. Load and Display Schema

```bash
# Load from CSV
audd load --source fixtures/adapters/users.csv

# Load from JSON
audd load --source fixtures/adapters/users.json

# Load from XML
audd load --source fixtures/adapters/users.xml
```

### 4. Compare Two Data Sources

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

**Console output:**
```
🔍 AUDD Compare
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Loading schema A from fixtures/adapters/users.csv...
✓ Schema A loaded: users (1 entities)
Loading schema B from fixtures/adapters/users.json...
✓ Schema B loaded: users (1 entities)

Comparing schemas...
✓ Comparison complete!
  - Matches: 6
  - Exclusives: 1
  - Conflicts: 3

✅ Comparison completed successfully!
Output files written to: output
```

---

## Working with Databases

### SQLite

```bash
# Inspect SQLite database
audd inspect --source "db:sqlite:///path/to/database.db"

# Compare two SQLite databases
audd compare \
  --source-a "db:sqlite:///path/to/db1.db" \
  --source-b "db:sqlite:///path/to/db2.db" \
  --out sqlite_comparison

# Compare CSV file with SQLite
audd compare \
  --source-a fixtures/adapters/users.csv \
  --source-b "db:sqlite:///production.db" \
  --out csv_vs_sqlite
```

### MySQL

```bash
# Inspect MySQL database
audd inspect --source "db:mysql://user:password@localhost/dbname"

# Compare local MySQL with remote
audd compare \
  --source-a "db:mysql://dev:pass@localhost/dev_db" \
  --source-b "db:mysql://user:pass@prod.server.com/prod_db" \
  --out dev_vs_prod

# Export MySQL schema to IR for analysis
audd inspect \
  --source "db:mysql://root:password@localhost/ecommerce" \
  --out ecommerce_schema.json
```

### PostgreSQL

```bash
# Inspect PostgreSQL
audd inspect --source "db:postgresql://user:pass@localhost:5432/dbname"

# Compare PostgreSQL staging vs production
audd compare \
  --source-a "db:postgresql://user:pass@staging.example.com/app" \
  --source-b "db:postgresql://user:pass@prod.example.com/app" \
  --out staging_vs_prod

# Compare with custom port
audd inspect --source "db:postgresql://user:pass@localhost:5433/custom_db"
```

### MongoDB

```bash
# Inspect MongoDB collection
audd inspect --source "db:mongodb://user:pass@localhost:27017/dbname"

# Compare MongoDB with JSON file
audd compare \
  --source-a "db:mongodb://admin:pass@localhost:27017/analytics" \
  --source-b analytics_export.json \
  --out mongo_vs_json
```

### Mixed Sources

```bash
# CSV vs Database
audd compare \
  --source-a legacy_data.csv \
  --source-b "db:mysql://user:pass@localhost/new_system" \
  --out migration_analysis

# JSON API export vs PostgreSQL
audd compare \
  --source-a api_schema.json \
  --source-b "db:postgresql://user:pass@db.example.com/api_db" \
  --out api_validation

# SQL DDL vs Live Database
audd compare \
  --source-a planned_schema.sql \
  --source-b "db:sqlite:///current.db" \
  --out schema_evolution
```

---

## Real-World Scenarios

### Scenario 1: Legacy System Migration

**Context:** Migrating from a legacy CSV system to a modern MySQL database.

```bash
# Step 1: Inspect legacy data
audd inspect --source legacy/customers.csv --out legacy_schema.json
audd inspect --source legacy/orders.csv --out legacy_orders.json

# Step 2: Compare with new schema
audd compare \
  --source-a legacy/customers.csv \
  --source-b "db:mysql://admin:pass@localhost/new_crm" \
  --out migration/customers_analysis

audd compare \
  --source-a legacy/orders.csv \
  --source-b new_system/orders.sql \
  --out migration/orders_analysis

# Step 3: Review reports
cat migration/customers_analysis/report.md
cat migration/orders_analysis/report.md

# Step 4: Review conflicts and decisions
cat migration/customers_analysis/diff.json | grep -A 5 "conflicts"
cat migration/customers_analysis/decision_log.json
```

**Expected outcome:**
- Identify fields requiring transformation
- Detect incompatible types (e.g., age as String → Int)
- Plan necessary conversions
- Document differences for stakeholders

### Scenario 2: Dev vs Prod Consistency Audit

**Context:** Verify that development schema matches production.

```bash
# Compare environments
audd compare \
  --source-a "db:postgresql://dev:pass@dev.company.com/app_db" \
  --source-b "db:postgresql://readonly:pass@prod.company.com/app_db" \
  --out audit/dev_prod_$(date +%Y%m%d)

# Use conservative configuration
audd --config audit-config.toml compare \
  --source-a "db:postgresql://dev:pass@dev.company.com/app_db" \
  --source-b "db:postgresql://readonly:pass@prod.company.com/app_db" \
  --confidence-threshold 0.95 \
  --out audit/critical_check
```

**Audit configuration** (`audit-config.toml`):
```toml
[compare]
similarity_threshold = 0.95  # Very strict

[resolution]
confidence_threshold = 0.95
allow_risky_suggestions = false  # Don't allow risky suggestions

[output]
generate_unified_schema = true
generate_diff = true
generate_decision_log = true
generate_report = true
```

### Scenario 3: Multi-Source Integration

**Context:** Integrate customer data from 3 different systems (CRM, ERP, E-commerce).

```bash
# Create project directory
mkdir -p integration/customer_360
cd integration/customer_360

# Step 1: Inspect each source
audd inspect \
  --source "db:mysql://user:pass@crm.company.com/crm" \
  --out sources/crm_schema.json

audd inspect \
  --source "db:postgresql://user:pass@erp.company.com/erp" \
  --out sources/erp_schema.json

audd inspect \
  --source https://api.shop.company.com/export/customers.json \
  --out sources/ecommerce_schema.json

# Step 2: Compare CRM vs ERP
audd compare \
  --source-a "db:mysql://user:pass@crm.company.com/crm" \
  --source-b "db:postgresql://user:pass@erp.company.com/erp" \
  --out comparisons/crm_vs_erp

# Step 3: Compare CRM vs E-commerce
audd compare \
  --source-a "db:mysql://user:pass@crm.company.com/crm" \
  --source-b https://api.shop.company.com/export/customers.json \
  --out comparisons/crm_vs_ecommerce

# Step 4: Analyze results
echo "=== CRM vs ERP ===" 
cat comparisons/crm_vs_erp/report.md

echo "=== CRM vs E-commerce ===" 
cat comparisons/crm_vs_ecommerce/report.md

# Step 5: Use unified schemas as basis for MDM (Master Data Management)
cp comparisons/crm_vs_erp/unified_schema.json master_customer_schema.json
```

### Scenario 4: ETL Planning

**Context:** Plan ETL transformations between source and destination.

```bash
# Compare data source (exported CSV) with destination (Data Warehouse)
audd compare \
  --source-a extracts/sales_data_2024.csv \
  --source-b "db:postgresql://etl:pass@warehouse.company.com/dwh" \
  --out etl_planning/sales_transformation

# Review necessary transformations
cat etl_planning/sales_transformation/report.md

# Generate conservative configuration for critical ETL
audd generate-config --out etl-config.toml
# Edit: confidence_threshold = 0.95, allow_risky_suggestions = false

# Re-run with ETL configuration
audd --config etl-config.toml compare \
  --source-a extracts/sales_data_2024.csv \
  --source-b "db:postgresql://etl:pass@warehouse.company.com/dwh" \
  --out etl_planning/sales_transformation_strict
```

### Scenario 5: REST API Validation

**Context:** Validate that JSON response schema from an API matches documentation.

```bash
# Download API response
curl -o api_response.json https://api.example.com/v2/users

# Compare with documented schema
audd compare \
  --source-a api_documentation/users_schema.json \
  --source-b api_response.json \
  --out validation/api_schema_check

# Automate validation in CI/CD
if grep -q '"conflicts": \[\]' validation/api_schema_check/diff.json; then
  echo "✓ API schema matches documentation"
  exit 0
else
  echo "✗ API schema conflicts detected"
  cat validation/api_schema_check/report.md
  exit 1
fi
```

### Scenario 6: Incremental Migration

**Context:** Phased migration, validating each table before migrating.

```bash
# Table by table
for table in users orders products invoices; do
  echo "Analyzing $table..."
  
  audd compare \
    --source-a "old_system/${table}.csv" \
    --source-b "db:mysql://admin:pass@localhost/new_system" \
    --out migration/tables/${table}_analysis
  
  # Review report
  cat migration/tables/${table}_analysis/report.md
  
  # If no conflicts, mark as ready
  if ! grep -q "Conflicts:" migration/tables/${table}_analysis/report.md; then
    echo "✓ $table ready for migration" >> migration/status.log
  else
    echo "✗ $table requires manual review" >> migration/status.log
  fi
done

# Review overall status
cat migration/status.log
```

---

## Advanced Configuration

### Using Custom Output Directory

```bash
# Specify custom output directory
audd compare \
  --source-a data1.csv \
  --source-b data2.json \
  --out /tmp/my_comparison

# Use timestamp in directory name
OUTPUT_DIR="comparisons/$(date +%Y%m%d_%H%M%S)"
audd compare \
  --source-a source_a.json \
  --source-b source_b.json \
  --out "$OUTPUT_DIR"
```

### Team/Project Configurations

```bash
# Development team configuration
cat > dev-team-config.toml << EOF
[compare]
default_output_dir = "schema_comparisons"
similarity_threshold = 0.75

[resolution]
confidence_threshold = 0.85
decision_id_prefix = "dev_dec"
allow_risky_suggestions = true

[output]
generate_unified_schema = true
generate_diff = true
generate_decision_log = true
generate_report = true
EOF

# Production team configuration
cat > prod-team-config.toml << EOF
[compare]
default_output_dir = "production_audits"
similarity_threshold = 0.9

[resolution]
confidence_threshold = 0.95
decision_id_prefix = "prod_dec"
allow_risky_suggestions = false

[output]
generate_unified_schema = true
generate_diff = true
generate_decision_log = true
generate_report = true
EOF

# Use appropriate configuration
audd --config dev-team-config.toml compare ...
audd --config prod-team-config.toml compare ...
```

### Configuration Override via Command Line

```bash
# Use team config but override threshold for this case
audd --config team-config.toml compare \
  --source-a critical_data.csv \
  --source-b "db:postgresql://user:pass@prod/db" \
  --confidence-threshold 0.98 \
  --out critical_comparison

# Override output directory
audd --config team-config.toml compare \
  --source-a a.csv \
  --source-b b.json \
  --out /mnt/shared/team_comparisons/project_x
```

---

## Workflow Integration

### Pre-Deployment Validation Script

```bash
#!/bin/bash
# validate_schema.sh - Validate schema before deployment

set -e

echo "🔍 Validating schema before deployment..."

# Compare new schema with production
audd compare \
  --source-a deployment/new_schema.sql \
  --source-b "db:postgresql://readonly:pass@prod.db.company.com/app" \
  --confidence-threshold 0.95 \
  --out validation/pre_deployment_check

# Check for conflicts
if grep -q '"conflicts": \[\]' validation/pre_deployment_check/diff.json; then
  echo "✅ No conflicts detected. Safe to deploy."
  exit 0
else
  echo "⚠️  Conflicts detected. Review required."
  echo ""
  cat validation/pre_deployment_check/report.md
  exit 1
fi
```

### CI/CD Integration (GitHub Actions)

```yaml
# .github/workflows/schema_validation.yml
name: Schema Validation

on:
  pull_request:
    paths:
      - 'db/migrations/**'
      - 'schema/**'

jobs:
  validate-schema:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install AUDD
        run: |
          git clone https://github.com/jmcasimar/AUDD.git
          cd AUDD
          cargo build --release
          sudo cp target/release/audd /usr/local/bin/
      
      - name: Validate Schema Changes
        env:
          DB_URL: ${{ secrets.STAGING_DB_URL }}
        run: |
          audd compare \
            --source-a schema/current.sql \
            --source-b "$DB_URL" \
            --out schema_validation
          
          # Fail if conflicts exist
          if ! grep -q '"conflicts": \[\]' schema_validation/diff.json; then
            echo "Schema conflicts detected!"
            cat schema_validation/report.md
            exit 1
          fi
      
      - name: Upload Validation Report
        uses: actions/upload-artifact@v3
        with:
          name: schema-validation-report
          path: schema_validation/
```

### Weekly Audit Script

```bash
#!/bin/bash
# weekly_audit.sh - Automated weekly audit

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
AUDIT_DIR="audits/weekly/$TIMESTAMP"
mkdir -p "$AUDIT_DIR"

echo "🔍 Running weekly schema audit - $TIMESTAMP"

# List of comparisons to perform
declare -A COMPARISONS=(
  ["dev_vs_staging"]="db:postgresql://dev:pass@dev.db/app|db:postgresql://staging:pass@staging.db/app"
  ["staging_vs_prod"]="db:postgresql://staging:pass@staging.db/app|db:postgresql://readonly:pass@prod.db/app"
)

for name in "${!COMPARISONS[@]}"; do
  IFS='|' read -r source_a source_b <<< "${COMPARISONS[$name]}"
  
  echo "Comparing: $name"
  audd compare \
    --source-a "$source_a" \
    --source-b "$source_b" \
    --out "$AUDIT_DIR/$name"
done

# Generate consolidated report
{
  echo "# Weekly Schema Audit Report"
  echo "**Date:** $(date)"
  echo ""
  
  for name in "${!COMPARISONS[@]}"; do
    echo "## $name"
    cat "$AUDIT_DIR/$name/report.md"
    echo ""
  done
} > "$AUDIT_DIR/consolidated_report.md"

# Send via email or notification
echo "📧 Audit complete. Report saved to: $AUDIT_DIR/consolidated_report.md"
```

### Schema Drift Monitoring

```bash
#!/bin/bash
# schema_drift_monitor.sh - Detect drift between environments

BASELINE="baseline/production_schema.json"
CURRENT="db:postgresql://readonly:pass@prod.db.company.com/app"

# First time: establish baseline
if [ ! -f "$BASELINE" ]; then
  echo "Creating baseline..."
  audd inspect --source "$CURRENT" --out "$BASELINE"
  exit 0
fi

# Compare with baseline
audd compare \
  --source-a "$BASELINE" \
  --source-b "$CURRENT" \
  --out monitoring/drift_check

# Alert if there are differences
if ! grep -q '"exclusives": \[\]' monitoring/drift_check/diff.json || \
   ! grep -q '"conflicts": \[\]' monitoring/drift_check/diff.json; then
  echo "⚠️  ALERT: Schema drift detected!"
  cat monitoring/drift_check/report.md
  
  # Send alert (example with curl to Slack webhook)
  # curl -X POST -H 'Content-type: application/json' \
  #   --data '{"text":"Schema drift detected in production!"}' \
  #   $SLACK_WEBHOOK_URL
else
  echo "✓ No schema drift detected"
fi
```

---

## Understanding Output Files

### unified_schema.json

The unified schema (C) that combines both sources A and B:

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
            "canonical_type": {"type": "integer"},
            "nullable": false
          },
          "origin": "BOTH",
          "state": "matched"
        },
        {
          "field": {
            "field_name": "created_at",
            "canonical_type": {"type": "datetime"},
            "nullable": true
          },
          "origin": "A",
          "state": "exclusive"
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
  "matches": [
    {
      "entity_a": "users",
      "entity_b": "users",
      "field_a": "id",
      "field_b": "id",
      "type_a": {"type": "integer"},
      "type_b": {"type": "integer"}
    }
  ],
  "exclusives": [
    {
      "entity": "users",
      "field": "created_at",
      "source": "A",
      "canonical_type": {"type": "datetime"}
    }
  ],
  "conflicts": [
    {
      "entity_a": "users",
      "entity_b": "users",
      "field_a": "age",
      "field_b": "age",
      "type_a": {"type": "string"},
      "type_b": {"type": "integer"},
      "conflict_type": "TypeMismatch"
    }
  ]
}
```

### decision_log.json

Auditable record of all resolution decisions:

```json
{
  "metadata": {
    "version": "1.0.0",
    "timestamp": "2026-01-26T15:30:00Z",
    "total_decisions": 3,
    "accepted_decisions": 3,
    "rejected_decisions": 0
  },
  "decisions": [
    {
      "decision_id": "auto_dec_001",
      "timestamp": "2026-01-26T15:30:01Z",
      "conflict_type": "TypeMismatch",
      "entity_name": "users",
      "field_name": "id",
      "type_a": {"type": "string"},
      "type_b": {"type": "integer"},
      "suggested_action": "CastSafe",
      "confidence": 0.95,
      "accepted": true,
      "rationale": "Safe cast from string to integer based on data analysis"
    }
  ]
}
```

### report.md

Human-readable report in Markdown format:

```markdown
# AUDD Comparison Report

**Generated:** 2026-01-26 15:35:24 UTC  
**Source A:** users.csv (csv)  
**Source B:** users.json (json)

## Summary

- **Matches:** 6 fields matched perfectly
- **Exclusives:** 1 field exists in only one source
- **Conflicts:** 3 type conflicts detected

## Details

### Matched Fields
- `id` (String ↔ String) ✓
- `name` (String ↔ String) ✓
- `email` (String ↔ String) ✓

### Exclusive Fields
- `created_at` (only in source A) - Added to unified schema

### Conflicts
1. **age**: String (A) vs Int32 (B)
   - **Suggestion:** CastSafe - Convert A.age to Int32
   - **Confidence:** 0.85
   - **Status:** Accepted

## Decision Log

Total decisions: 3  
Accepted: 3  
Rejected: 0

See `decision_log.json` for complete details.

## Unified Schema

The unified schema includes all 7 fields from both sources.  
See `unified_schema.json` for full schema definition.
```

---

## Tips and Best Practices

### 1. Start with Inspection

Always inspect sources individually before comparing:

```bash
audd inspect --source source_a.csv --out a_schema.json
audd inspect --source source_b.json --out b_schema.json
# Review individual schemas before comparing
cat a_schema.json
cat b_schema.json
```

### 2. Use Context-Appropriate Configuration

- **Development**: Lower thresholds, allow risky suggestions
- **Staging**: Medium thresholds, analyze before accepting
- **Production**: High thresholds, very conservative

### 3. Document Decisions

Save `decision_log.json` as part of project documentation:

```bash
cp output/decision_log.json docs/schema_decisions_$(date +%Y%m%d).json
git add docs/schema_decisions_*.json
git commit -m "docs: Add schema comparison decisions"
```

### 4. Automate Regular Validations

Set up automated audits (cron, CI/CD) to detect drift early.

### 5. Schema Versioning

```bash
# Create snapshot of current schema
audd inspect --source "db:postgresql://user:pass@prod/db" \
  --out schema_snapshots/v1.2.0_$(date +%Y%m%d).json
```

---

## Additional Resources

- [Getting Started](Getting-Started.md) - Getting started guide
- [FAQ](FAQ.md) - Frequently asked questions
- [Configuration](CONFIG.md) - Detailed configuration
- [Architecture](Architecture.md) - System architecture

---

**Need more examples?** Open a [Discussion](https://github.com/jmcasimar/AUDD/discussions) on GitHub.

**Last updated:** 2026-01-26
