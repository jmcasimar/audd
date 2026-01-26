# Ejemplos de CLI

## Uso Básico

### Generar Archivo de Configuración

Crea un archivo de configuración para personalizar el comportamiento de AUDD:

```bash
# Generar configuración predeterminada
audd generate-config

# Generar en ubicación personalizada
audd generate-config --out ~/.audd.toml
```

Ver [CONFIG.md](../../docs/es/CONFIG.md) para documentación detallada de configuración.

### Inspeccionar una única fuente de datos

Carga e inspecciona el IR (Representación Intermedia) de una fuente de datos:

```bash
# Inspeccionar un archivo CSV
audd inspect --source fixtures/adapters/users.csv

# Inspeccionar un archivo JSON y guardar a archivo
audd inspect --source fixtures/adapters/users.json --out ir_output.json

# Inspeccionar un archivo SQL DDL
audd inspect --source fixtures/adapters/schema.sql
```

### Cargar y mostrar schema

```bash
# Cargar desde CSV
audd load --source fixtures/adapters/users.csv

# Cargar desde JSON
audd load --source fixtures/adapters/users.json

# Cargar desde XML
audd load --source fixtures/adapters/users.xml
```

### Comparar dos fuentes de datos

Compara esquemas de diferentes fuentes y genera reportes de unificación:

```bash
# Comparar CSV y JSON
audd compare \
  --source-a fixtures/adapters/users.csv \
  --source-b fixtures/adapters/users.json \
  --out output

# Esto crea:
# - output/unified_schema.json    - Schema unificado combinando ambas fuentes
# - output/diff.json               - Resultados detallados de comparación
# - output/decision_log.json       - Registro de todas las decisiones de resolución
# - output/report.md               - Reporte markdown legible
```

## Fuentes de Bases de Datos

### SQLite

```bash
# Inspeccionar base de datos SQLite
audd inspect --source db:sqlite:///path/to/database.db

# Comparar dos bases de datos SQLite
audd compare \
  --source-a db:sqlite:///path/to/db1.db \
  --source-b db:sqlite:///path/to/db2.db \
  --out comparison_output
```

### MySQL

```bash
# Inspeccionar base de datos MySQL
audd inspect --source "db:mysql://user:password@localhost/dbname"

# Comparar MySQL y PostgreSQL
audd compare \
  --source-a "db:mysql://user:pass@localhost/db1" \
  --source-b "db:postgres://user:pass@localhost/db2" \
  --out output
```

## Uso Avanzado

### Directorio de salida personalizado

```bash
# Especificar directorio de salida personalizado
audd compare \
  --source-a data1.csv \
  --source-b data2.json \
  --out /tmp/my_comparison
```

### Fuentes mixtas

```bash
# Comparar archivo vs base de datos
audd compare \
  --source-a fixtures/adapters/users.csv \
  --source-b "db:sqlite:///production.db" \
  --out file_vs_db_comparison
```

## Entender los Archivos de Salida

### unified_schema.json

El schema unificado (C) que fusiona ambas fuentes A y B:

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
            "canonical_type": {
              "type": "integer"
            },
            "nullable": false
          },
          "origin": "BOTH",
          "state": "matched"
        }
      ]
    }
  ]
}
```

### diff.json

Resultados completos de comparación mostrando coincidencias, exclusivos y conflictos:

```json
{
  "matches": [...],
  "exclusives": [...],
  "conflicts": [...]
}
```

### decision_log.json

Registro auditable de todas las decisiones de resolución:

```json
{
  "metadata": {
    "version": "1.0.0",
    "total_decisions": 3,
    "accepted_decisions": 3
  },
  "decisions": [...]
}
```

### report.md

Resumen legible en formato Markdown:

```markdown
# AUDD Comparison Report

## Summary

- **Matches**: 6
- **Exclusives**: 1
- **Conflicts**: 3

# Decision Log
...
```

## Flujos de Trabajo Comunes

### Flujo de trabajo de desarrollo

1. Inspeccionar ambas fuentes para entender la estructura:
```bash
audd inspect --source app_schema.sql --out schema_a.json
audd inspect --source legacy_data.csv --out schema_b.json
```

2. Comparar y analizar:
```bash
audd compare \
  --source-a app_schema.sql \
  --source-b legacy_data.csv \
  --out migration_plan
```

3. Revisar el report.md y decision_log.json para entender conflictos

### Planificación de migración

```bash
# Comparar base de datos de producción actual con nuevo schema
audd compare \
  --source-a "db:postgres://user:pass@prod.example.com/db" \
  --source-b new_schema.sql \
  --out migration_analysis

# Revisar los archivos generados para planear la migración
cat migration_analysis/report.md
```

## Manejo de Errores

El CLI proporciona mensajes de error claros:

```bash
# Archivo inválido
$ audd inspect --source nonexistent.csv
❌ Error: Failed to load schema from source 'nonexistent.csv': ...

# Conexión de base de datos inválida
$ audd inspect --source "db:mysql://invalid"
❌ Error: Failed to load schema from source 'db:mysql://invalid': ...
```

## Archivos de Configuración

AUDD soporta archivos de configuración para ajustes persistentes. Ver [CONFIG.md](../../docs/es/CONFIG.md) para documentación completa.

### Inicio Rápido con Archivos de Configuración

```bash
# 1. Generar un archivo de configuración
audd generate-config --out audd.toml

# 2. Editar el archivo de configuración
cat audd.toml
# [resolution]
# confidence_threshold = 0.9
# decision_id_prefix = "auto_dec"

# 3. Usar la configuración (cargada automáticamente desde ./audd.toml)
audd compare --source-a a.csv --source-b b.json

# 4. O especificar un archivo de configuración personalizado
audd --config /path/to/config.toml compare --source-a a.csv --source-b b.json
```

### Precedencia de Configuración

Los ajustes se aplican en este orden (prioridad de mayor a menor):
1. **Flags de CLI** - `--confidence-threshold 0.95`
2. **Archivo de configuración** - Desde `--config` o auto-cargado
3. **Valores predeterminados** - Predeterminados incorporados

### Ejemplo: Umbral de Confianza Personalizado

Archivo de configuración (`team-config.toml`):
```toml
[resolution]
confidence_threshold = 0.85
decision_id_prefix = "team_dec"

[compare]
default_output_dir = "comparisons"
```

Uso:
```bash
# Usa los ajustes de configuración del equipo
audd --config team-config.toml compare --source-a a.csv --source-b b.json

# Sobrescribir umbral de confianza solo para esta ejecución
audd --config team-config.toml compare \
  --source-a a.csv \
  --source-b b.json \
  --confidence-threshold 0.95
```

Para opciones de configuración detalladas y ejemplos, ver la [Documentación de Configuración](../../docs/es/CONFIG.md).
