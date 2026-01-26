# Ejemplos de Uso - AUDD

**🌐 Idioma / Language:**  
📘 **Español** | 📗 [English](en/Usage-Examples.md)

---

Esta guía proporciona ejemplos prácticos y escenarios del mundo real para usar AUDD en diferentes contextos de integración y migración de datos.

## Tabla de Contenidos

- [Ejemplos Básicos](#ejemplos-básicos)
- [Trabajar con Bases de Datos](#trabajar-con-bases-de-datos)
- [Escenarios del Mundo Real](#escenarios-del-mundo-real)
- [Configuración Avanzada](#configuración-avanzada)
- [Integración en Flujos de Trabajo](#integración-en-flujos-de-trabajo)

---

## Ejemplos Básicos

### 1. Generar Archivo de Configuración

Crear un archivo de configuración para personalizar el comportamiento de AUDD:

```bash
# Crear configuración con valores predeterminados
audd generate-config

# Crear en ubicación personalizada
audd generate-config --out ~/.audd.toml

# Crear configuración de equipo
audd generate-config --out team-config.toml
```

Ver [CONFIG.md](CONFIG.md) para documentación detallada de configuración.

### 2. Inspeccionar una Fuente de Datos Individual

Cargar e inspeccionar la Representación Intermedia (IR) de una fuente de datos:

```bash
# Inspeccionar archivo CSV
audd inspect --source fixtures/adapters/users.csv

# Inspeccionar JSON y guardar en archivo
audd inspect --source fixtures/adapters/users.json --out ir_output.json

# Inspeccionar archivo SQL DDL
audd inspect --source fixtures/adapters/schema.sql

# Inspeccionar archivo XML
audd inspect --source fixtures/adapters/users.xml --out users_ir.json
```

**Salida esperada (JSON):**
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

### 3. Cargar y Mostrar Esquema

```bash
# Cargar desde CSV
audd load --source fixtures/adapters/users.csv

# Cargar desde JSON
audd load --source fixtures/adapters/users.json

# Cargar desde XML
audd load --source fixtures/adapters/users.xml
```

### 4. Comparar Dos Fuentes de Datos

Comparar esquemas de diferentes fuentes y generar reportes de unificación:

```bash
# Comparar CSV y JSON
audd compare \
  --source-a fixtures/adapters/users.csv \
  --source-b fixtures/adapters/users.json \
  --out output

# Esto crea:
# - output/unified_schema.json    - Esquema unificado combinando ambas fuentes
# - output/diff.json               - Resultados detallados de comparación
# - output/decision_log.json       - Registro de todas las decisiones de resolución
# - output/report.md               - Reporte Markdown legible
```

**Salida en consola:**
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

## Trabajar con Bases de Datos

### SQLite

```bash
# Inspeccionar base de datos SQLite
audd inspect --source "db:sqlite:///path/to/database.db"

# Comparar dos bases de datos SQLite
audd compare \
  --source-a "db:sqlite:///path/to/db1.db" \
  --source-b "db:sqlite:///path/to/db2.db" \
  --out sqlite_comparison

# Comparar archivo CSV con SQLite
audd compare \
  --source-a fixtures/adapters/users.csv \
  --source-b "db:sqlite:///production.db" \
  --out csv_vs_sqlite
```

### MySQL

```bash
# Inspeccionar base de datos MySQL
audd inspect --source "db:mysql://user:password@localhost/dbname"

# Comparar MySQL local con remoto
audd compare \
  --source-a "db:mysql://dev:pass@localhost/dev_db" \
  --source-b "db:mysql://user:pass@prod.server.com/prod_db" \
  --out dev_vs_prod

# Exportar esquema MySQL a IR para análisis
audd inspect \
  --source "db:mysql://root:password@localhost/ecommerce" \
  --out ecommerce_schema.json
```

### PostgreSQL

```bash
# Inspeccionar PostgreSQL
audd inspect --source "db:postgres://user:pass@localhost:5432/dbname"

# Comparar PostgreSQL staging vs producción
audd compare \
  --source-a "db:postgres://user:pass@staging.example.com/app" \
  --source-b "db:postgres://user:pass@prod.example.com/app" \
  --out staging_vs_prod

# Comparar con puerto personalizado
audd inspect --source "db:postgres://user:pass@localhost:5433/custom_db"
```

### MongoDB

```bash
# Inspeccionar colección MongoDB
audd inspect --source "db:mongodb://user:pass@localhost:27017/dbname"

# Comparar MongoDB con archivo JSON
audd compare \
  --source-a "db:mongodb://admin:pass@localhost:27017/analytics" \
  --source-b analytics_export.json \
  --out mongo_vs_json
```

### Fuentes Mixtas

```bash
# CSV vs Base de Datos
audd compare \
  --source-a legacy_data.csv \
  --source-b "db:mysql://user:pass@localhost/new_system" \
  --out migration_analysis

# JSON API export vs PostgreSQL
audd compare \
  --source-a api_schema.json \
  --source-b "db:postgres://user:pass@db.example.com/api_db" \
  --out api_validation

# SQL DDL vs Base de Datos en vivo
audd compare \
  --source-a planned_schema.sql \
  --source-b "db:sqlite:///current.db" \
  --out schema_evolution
```

---

## Escenarios del Mundo Real

### Escenario 1: Migración de Sistema Legacy

**Contexto:** Migrar de un sistema CSV legacy a una base de datos MySQL moderna.

```bash
# Paso 1: Inspeccionar datos legacy
audd inspect --source legacy/customers.csv --out legacy_schema.json
audd inspect --source legacy/orders.csv --out legacy_orders.json

# Paso 2: Comparar con esquema nuevo
audd compare \
  --source-a legacy/customers.csv \
  --source-b "db:mysql://admin:pass@localhost/new_crm" \
  --out migration/customers_analysis

audd compare \
  --source-a legacy/orders.csv \
  --source-b new_system/orders.sql \
  --out migration/orders_analysis

# Paso 3: Revisar reportes
cat migration/customers_analysis/report.md
cat migration/orders_analysis/report.md

# Paso 4: Revisar conflictos y decisiones
cat migration/customers_analysis/diff.json | grep -A 5 "conflicts"
cat migration/customers_analysis/decision_log.json
```

**Resultado esperado:**
- Identificar campos que requieren transformación
- Detectar tipos incompatibles (ej: edad como String → Int)
- Planificar conversiones necesarias
- Documentar diferencias para stakeholders

### Escenario 2: Auditoría de Consistencia Dev vs Prod

**Contexto:** Verificar que el esquema de desarrollo coincide con producción.

```bash
# Comparar ambientes
audd compare \
  --source-a "db:postgres://dev:pass@dev.company.com/app_db" \
  --source-b "db:postgres://readonly:pass@prod.company.com/app_db" \
  --out audit/dev_prod_$(date +%Y%m%d)

# Usar configuración conservadora
audd --config audit-config.toml compare \
  --source-a "db:postgres://dev:pass@dev.company.com/app_db" \
  --source-b "db:postgres://readonly:pass@prod.company.com/app_db" \
  --confidence-threshold 0.95 \
  --out audit/critical_check
```

**Configuración de auditoría** (`audit-config.toml`):
```toml
[compare]
similarity_threshold = 0.95  # Muy estricto

[resolution]
confidence_threshold = 0.95
allow_risky_suggestions = false  # No permitir sugerencias riesgosas

[output]
generate_unified_schema = true
generate_diff = true
generate_decision_log = true
generate_report = true
```

### Escenario 3: Integración Multi-Fuente

**Contexto:** Integrar datos de clientes de 3 sistemas diferentes (CRM, ERP, E-commerce).

```bash
# Crear directorio de proyecto
mkdir -p integration/customer_360
cd integration/customer_360

# Paso 1: Inspeccionar cada fuente
audd inspect \
  --source "db:mysql://user:pass@crm.company.com/crm" \
  --out sources/crm_schema.json

audd inspect \
  --source "db:postgres://user:pass@erp.company.com/erp" \
  --out sources/erp_schema.json

audd inspect \
  --source https://api.shop.company.com/export/customers.json \
  --out sources/ecommerce_schema.json

# Paso 2: Comparar CRM vs ERP
audd compare \
  --source-a "db:mysql://user:pass@crm.company.com/crm" \
  --source-b "db:postgres://user:pass@erp.company.com/erp" \
  --out comparisons/crm_vs_erp

# Paso 3: Comparar CRM vs E-commerce
audd compare \
  --source-a "db:mysql://user:pass@crm.company.com/crm" \
  --source-b https://api.shop.company.com/export/customers.json \
  --out comparisons/crm_vs_ecommerce

# Paso 4: Analizar resultados
echo "=== CRM vs ERP ===" 
cat comparisons/crm_vs_erp/report.md

echo "=== CRM vs E-commerce ===" 
cat comparisons/crm_vs_ecommerce/report.md

# Paso 5: Usar esquemas unificados como base para MDM (Master Data Management)
cp comparisons/crm_vs_erp/unified_schema.json master_customer_schema.json
```

### Escenario 4: Planificación de ETL

**Contexto:** Planificar transformaciones ETL entre fuente y destino.

```bash
# Comparar fuente de datos (CSV exportado) con destino (Data Warehouse)
audd compare \
  --source-a extracts/sales_data_2024.csv \
  --source-b "db:postgres://etl:pass@warehouse.company.com/dwh" \
  --out etl_planning/sales_transformation

# Revisar transformaciones necesarias
cat etl_planning/sales_transformation/report.md

# Generar configuración conservadora para ETL crítico
audd generate-config --out etl-config.toml
# Editar: confidence_threshold = 0.95, allow_risky_suggestions = false

# Re-ejecutar con configuración de ETL
audd --config etl-config.toml compare \
  --source-a extracts/sales_data_2024.csv \
  --source-b "db:postgres://etl:pass@warehouse.company.com/dwh" \
  --out etl_planning/sales_transformation_strict
```

### Escenario 5: Validación de API REST

**Contexto:** Validar que el esquema de respuestas JSON de una API coincide con la documentación.

```bash
# Descargar respuesta de API
curl -o api_response.json https://api.example.com/v2/users

# Comparar con esquema documentado
audd compare \
  --source-a api_documentation/users_schema.json \
  --source-b api_response.json \
  --out validation/api_schema_check

# Automatizar validación en CI/CD
if grep -q '"conflicts": \[\]' validation/api_schema_check/diff.json; then
  echo "✓ API schema matches documentation"
  exit 0
else
  echo "✗ API schema conflicts detected"
  cat validation/api_schema_check/report.md
  exit 1
fi
```

### Escenario 6: Migración Incremental

**Contexto:** Migración por fases, validando cada tabla antes de migrar.

```bash
# Tabla por tabla
for table in users orders products invoices; do
  echo "Analyzing $table..."
  
  audd compare \
    --source-a "old_system/${table}.csv" \
    --source-b "db:mysql://admin:pass@localhost/new_system" \
    --out migration/tables/${table}_analysis
  
  # Revisar reporte
  cat migration/tables/${table}_analysis/report.md
  
  # Si no hay conflictos, marcar como listo
  if ! grep -q "Conflicts:" migration/tables/${table}_analysis/report.md; then
    echo "✓ $table ready for migration" >> migration/status.log
  else
    echo "✗ $table requires manual review" >> migration/status.log
  fi
done

# Revisar estado general
cat migration/status.log
```

---

## Configuración Avanzada

### Uso con Directorio de Salida Personalizado

```bash
# Especificar directorio de salida personalizado
audd compare \
  --source-a data1.csv \
  --source-b data2.json \
  --out /tmp/my_comparison

# Usar timestamp en nombre de directorio
OUTPUT_DIR="comparisons/$(date +%Y%m%d_%H%M%S)"
audd compare \
  --source-a source_a.json \
  --source-b source_b.json \
  --out "$OUTPUT_DIR"
```

### Configuraciones por Equipo/Proyecto

```bash
# Configuración de equipo de desarrollo
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

# Configuración de equipo de producción
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

# Usar configuración apropiada
audd --config dev-team-config.toml compare ...
audd --config prod-team-config.toml compare ...
```

### Override de Configuración por Línea de Comandos

```bash
# Usar config de equipo pero override threshold para este caso
audd --config team-config.toml compare \
  --source-a critical_data.csv \
  --source-b "db:postgres://user:pass@prod/db" \
  --confidence-threshold 0.98 \
  --out critical_comparison

# Override de directorio de salida
audd --config team-config.toml compare \
  --source-a a.csv \
  --source-b b.json \
  --out /mnt/shared/team_comparisons/project_x
```

---

## Integración en Flujos de Trabajo

### Script de Validación Pre-Deployment

```bash
#!/bin/bash
# validate_schema.sh - Validar esquema antes de deployment

set -e

echo "🔍 Validating schema before deployment..."

# Comparar esquema nuevo con producción
audd compare \
  --source-a deployment/new_schema.sql \
  --source-b "db:postgres://readonly:pass@prod.db.company.com/app" \
  --confidence-threshold 0.95 \
  --out validation/pre_deployment_check

# Verificar conflictos
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

### Integración CI/CD (GitHub Actions)

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
          
          # Fail si hay conflictos
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

### Script de Auditoría Semanal

```bash
#!/bin/bash
# weekly_audit.sh - Auditoría semanal automática

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
AUDIT_DIR="audits/weekly/$TIMESTAMP"
mkdir -p "$AUDIT_DIR"

echo "🔍 Running weekly schema audit - $TIMESTAMP"

# Lista de comparaciones a realizar
declare -A COMPARISONS=(
  ["dev_vs_staging"]="db:postgres://dev:pass@dev.db/app|db:postgres://staging:pass@staging.db/app"
  ["staging_vs_prod"]="db:postgres://staging:pass@staging.db/app|db:postgres://readonly:pass@prod.db/app"
)

for name in "${!COMPARISONS[@]}"; do
  IFS='|' read -r source_a source_b <<< "${COMPARISONS[$name]}"
  
  echo "Comparing: $name"
  audd compare \
    --source-a "$source_a" \
    --source-b "$source_b" \
    --out "$AUDIT_DIR/$name"
done

# Generar reporte consolidado
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

# Enviar por email o notificación
echo "📧 Audit complete. Report saved to: $AUDIT_DIR/consolidated_report.md"
```

### Monitoreo de Drift de Esquema

```bash
#!/bin/bash
# schema_drift_monitor.sh - Detectar drift entre ambientes

BASELINE="baseline/production_schema.json"
CURRENT="db:postgres://readonly:pass@prod.db.company.com/app"

# Primera vez: establecer baseline
if [ ! -f "$BASELINE" ]; then
  echo "Creating baseline..."
  audd inspect --source "$CURRENT" --out "$BASELINE"
  exit 0
fi

# Comparar con baseline
audd compare \
  --source-a "$BASELINE" \
  --source-b "$CURRENT" \
  --out monitoring/drift_check

# Alertar si hay diferencias
if ! grep -q '"exclusives": \[\]' monitoring/drift_check/diff.json || \
   ! grep -q '"conflicts": \[\]' monitoring/drift_check/diff.json; then
  echo "⚠️  ALERT: Schema drift detected!"
  cat monitoring/drift_check/report.md
  
  # Enviar alerta (ejemplo con curl a Slack webhook)
  # curl -X POST -H 'Content-type: application/json' \
  #   --data '{"text":"Schema drift detected in production!"}' \
  #   $SLACK_WEBHOOK_URL
else
  echo "✓ No schema drift detected"
fi
```

---

## Entendiendo los Archivos de Salida

### unified_schema.json

El esquema unificado (C) que combina ambas fuentes A y B:

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

Resultados completos de comparación mostrando matches, exclusives y conflictos:

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

Registro auditable de todas las decisiones de resolución:

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

Reporte legible en formato Markdown:

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

## Consejos y Mejores Prácticas

### 1. Empezar con Inspección

Siempre inspecciona las fuentes individualmente antes de comparar:

```bash
audd inspect --source source_a.csv --out a_schema.json
audd inspect --source source_b.json --out b_schema.json
# Revisar esquemas individuales antes de comparar
cat a_schema.json
cat b_schema.json
```

### 2. Usar Configuración Apropiada al Contexto

- **Desarrollo**: Umbrales más bajos, permitir sugerencias riesgosas
- **Staging**: Umbrales medios, analizar antes de aceptar
- **Producción**: Umbrales altos, muy conservador

### 3. Documentar Decisiones

Guardar los `decision_log.json` como parte de la documentación del proyecto:

```bash
cp output/decision_log.json docs/schema_decisions_$(date +%Y%m%d).json
git add docs/schema_decisions_*.json
git commit -m "docs: Add schema comparison decisions"
```

### 4. Automatizar Validaciones Regulares

Configurar auditorías automáticas (cron, CI/CD) para detectar drift temprano.

### 5. Versionamiento de Esquemas

```bash
# Crear snapshot de esquema actual
audd inspect --source "db:postgres://user:pass@prod/db" \
  --out schema_snapshots/v1.2.0_$(date +%Y%m%d).json
```

---

## Recursos Adicionales

- [Getting Started](Getting-Started.md) - Guía de inicio
- [FAQ](FAQ.md) - Preguntas frecuentes
- [Configuration](CONFIG.md) - Configuración detallada
- [Architecture](Architecture.md) - Arquitectura del sistema

---

**¿Necesitas más ejemplos?** Abre un [Discussion](https://github.com/jmcasimar/AUDD/discussions) en GitHub.

**Última actualización:** 2026-01-26
