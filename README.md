# AUDD - Algoritmo de Unificación Dinámica de Datos

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![CI](https://github.com/jmcasimar/AUDD/workflows/CI/badge.svg)](https://github.com/jmcasimar/AUDD/actions)

**Algoritmo de Unificación Dinámica de Datos**

---

**🌐 Idioma / Language:**  
📘 **Español (Base)** | 📗 [English](docs/en/README.md)

> **Nota:** El español es el idioma base de este proyecto. La documentación se mantiene sincronizada en español e inglés conforme evoluciona el proyecto.

---

Una herramienta basada en Rust para la comparación y unificación inteligente de datos entre fuentes heterogéneas.

## 🎯 Propósito

AUDD proporciona reconciliación automatizada de datos y mapeo de esquemas para conjuntos de datos de diferentes fuentes, permitiendo flujos de trabajo eficientes de integración de datos.

## ✨ Características

- **Adaptadores de Archivo**: Carga esquemas desde archivos CSV, JSON, XML y SQL/DDL
- **Adaptadores de Base de Datos**: Conecta con SQLite, MySQL, PostgreSQL, MongoDB, SQL Server y Firebird
- **Representación Intermedia (IR)**: Modelo de schema canónico para fuentes heterogéneas
- **Auto-detección**: Detección automática de formato desde extensiones de archivo
- **Inferencia de Tipos**: Detección inteligente de tipos para fuentes JSON y SQL
- **Detección de Conflictos**: Comparación avanzada de esquemas e identificación de conflictos
- **Motor de Resolución**: Estrategias automatizadas y manuales de resolución de conflictos
- **Generación de Schema Unificado**: Creación automática de schema unificado (C) desde fuentes A y B
- **Decisiones Auditables**: Rastrea y documenta todas las decisiones de unificación de esquemas
- **Múltiples Formatos de Salida**: Esquemas JSON, reportes de diferencias, logs de decisiones y reportes Markdown
- **CLI y Biblioteca**: Úsalo como herramienta de línea de comandos o biblioteca Rust

## 🚀 Inicio Rápido

### Instalación

**Desde código fuente:**
```bash
git clone https://github.com/jmcasimar/AUDD.git
cd AUDD
cargo build --release
```

Binario disponible en: `target/release/audd`

### Uso

**Generar archivo de configuración:**
```bash
# Crear un archivo de configuración con valores predeterminados
audd generate-config

# Personalizar comportamiento (opcional)
# Editar audd.toml para establecer umbrales de confianza, opciones de salida, etc.
```

**Inspeccionar un schema (exportación IR):**
```bash
# Imprimir a stdout
audd inspect --source users.csv

# Guardar a archivo
audd inspect --source schema.sql --out ir.json
```

**Cargar y mostrar schema:**
```bash
audd load --source users.csv
audd load --source schema.sql
audd load --source data.json
```

**Comparar dos fuentes de datos:**
```bash
audd compare \
  --source-a data1.csv \
  --source-b data2.json \
  --out output

# Genera:
# - output/unified_schema.json  (Schema unificado C)
# - output/diff.json             (Resultados de comparación)
# - output/decision_log.json     (Decisiones de resolución)
# - output/report.md             (Reporte legible)
# - output/report.json           (Reporte estructurado, opcional)

# Usar archivo de configuración personalizado
audd --config team-config.toml compare --source-a a.csv --source-b b.json

# Sobrescribir umbral de confianza
audd compare --source-a a.csv --source-b b.json --confidence-threshold 0.95
```

**Trabajar con bases de datos:**
```bash
# Inspeccionar una base de datos
audd inspect --source "db:sqlite:///path/to/db.sqlite"

# Comparar archivo vs base de datos
audd compare \
  --source-a users.csv \
  --source-b "db:mysql://user:pass@host/db" \
  --out comparison_output
```

**Obtener ayuda:**
```bash
audd --help
audd compare --help
audd inspect --help
audd generate-config --help
```

### Ejemplo

```bash
# Comparar esquemas CSV y JSON
audd compare \
  --source-a fixtures/adapters/users.csv \
  --source-b fixtures/adapters/users.json \
  --out output

# Salida:
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

Para más ejemplos, ver [`examples/cli/README.md`](examples/cli/README.md).

## 🏗️ Arquitectura

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

Para información arquitectónica detallada, ver [docs/Architecture.md](docs/Architecture.md).

## 📚 Documentación

**Guías Principales:**
- 🚀 [**Inicio Rápido**](docs/Getting-Started.md) - Tu primera comparación en 30 minutos
- 🏗️ [**Arquitectura**](docs/Architecture.md) - Diseño del sistema y componentes
- ⚙️ [**Configuración**](docs/CONFIG.md) - Opciones y personalización avanzada
- 💡 [**Ejemplos de Uso**](docs/Usage-Examples.md) - Casos de uso reales y workflows
- 🤝 [**Contribuir**](docs/Contributing.md) - Guía para colaboradores
- ❓ [**FAQ**](docs/FAQ.md) - Preguntas frecuentes y solución de problemas

**Documentación Técnica:**
- [Intermediate Representation (IR)](docs/ir.md) - Especificación del modelo canónico
- [Adaptadores de Archivo](docs/adapters_files.md) - CSV, JSON, XML, SQL
- [Adaptadores de Base de Datos](docs/adapters_db.md) - MySQL, PostgreSQL, MongoDB, etc.
- [Motor de Comparación](docs/audit_report.md) - Algoritmos y estrategias
- [Estructura de Reportes](docs/reporting.md) - Formatos de salida

**Versión en Inglés:** Toda la documentación está disponible en [docs/en/](docs/en/)

## 📋 Roadmap (MVP)

- **Sprint 1:** Análisis central de datos y detección de esquemas
- **Sprint 2:** Algoritmo de comparación y coincidencia de campos
- **Sprint 3:** Motor de unificación y resolución de conflictos
- **Sprint 4:** Soporte multi-formato y optimizaciones
- **Sprint 5:** Documentación y ajuste de rendimiento

## 🛠️ Desarrollo

### Prerequisitos
- Rust 1.70+
- Cargo

### Compilar
```bash
cargo build
```

### Probar
```bash
cargo test
```

### Formatear y Lint
```bash
cargo fmt
cargo clippy
```

## 📝 Contribuir

Ver [CONTRIBUTING.md](CONTRIBUTING.md) para lineamientos.

## 🔒 Seguridad

Ver [SECURITY.md](SECURITY.md) para procedimientos de reporte.

## 📄 Licencia

Licenciado bajo la Licencia MIT. Ver [LICENSE](LICENSE) para detalles.

## 👥 Autores

Contribuidores de AUDD - Ver repositorio del proyecto para detalles.

## 🙏 Agradecimientos

Este proyecto es parte de investigación académica sobre integración de datos y estrategias de transferencia de código abierto.

---

**Estado:** Desarrollo temprano (v0.1.0) - Implementación central en progreso
