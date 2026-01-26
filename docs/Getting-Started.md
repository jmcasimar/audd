# Getting Started with AUDD

**🌐 Idioma / Language:**  
📘 **Español** | 📗 [English](en/Getting-Started.md)

---

Bienvenido a AUDD (Algoritmo de Unificación Dinámica de Datos). Esta guía te llevará desde cero hasta ejecutar tu primera comparación de esquemas en menos de 30 minutos.

## ¿Qué es AUDD?

AUDD es una herramienta de línea de comandos y biblioteca Rust que compara esquemas de datos de diferentes fuentes (bases de datos, archivos CSV/JSON/XML) y genera:
- Esquemas unificados automáticamente
- Reportes de diferencias y conflictos
- Sugerencias inteligentes de resolución
- Logs auditables de todas las decisiones

**Casos de uso:**
- Migrar datos entre sistemas diferentes
- Comparar esquemas de desarrollo vs producción
- Integrar datos de múltiples fuentes
- Auditar cambios de esquema

---

## Prerequisitos

### Instalar Rust

AUDD requiere Rust 1.70 o superior. Si no tienes Rust instalado:

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
Descarga e instala desde: https://rustup.rs/

**Verificar instalación:**
```bash
rustc --version
cargo --version
```

Deberías ver algo como:
```
rustc 1.70.0 (o superior)
cargo 1.70.0 (o superior)
```

### Herramientas Opcionales

Para trabajar con bases de datos, podrías necesitar:
- SQLite: Incluido por defecto
- MySQL client: `sudo apt-get install libmysqlclient-dev` (Linux)
- PostgreSQL client: `sudo apt-get install libpq-dev` (Linux)

---

## Instalación

### Opción 1: Desde Código Fuente (Recomendado)

```bash
# 1. Clonar el repositorio
git clone https://github.com/jmcasimar/AUDD.git
cd AUDD

# 2. Compilar en modo release (optimizado)
cargo build --release

# 3. El binario estará en:
ls -lh target/release/audd
```

**Tiempo estimado:** 5-10 minutos (la primera vez descarga dependencias)

### Opción 2: Instalar desde Cargo (Futuro)

```bash
# Cuando se publique en crates.io:
cargo install audd
```

### Verificar Instalación

```bash
# Si compilaste desde código fuente:
./target/release/audd --version

# O añade al PATH para usar simplemente "audd":
export PATH="$PWD/target/release:$PATH"
audd --version
```

Deberías ver:
```
audd 0.1.0
```

---

## Tu Primera Comparación (Hello World)

### Paso 1: Explorar los Datos de Prueba

AUDD incluye archivos de ejemplo en `fixtures/adapters/`:

```bash
cd AUDD
ls -l fixtures/adapters/
```

Verás:
- `users.csv` - Usuarios en formato CSV
- `users.json` - Mismos usuarios en JSON
- `users.xml` - Mismos usuarios en XML
- `schema.sql` - Esquema SQL de ejemplo

**Ver contenido de users.csv:**
```bash
cat fixtures/adapters/users.csv
```

```csv
id,name,email,age,created_at
1,Alice,alice@example.com,30,2024-01-01
2,Bob,bob@example.com,25,2024-01-02
```

**Ver contenido de users.json:**
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

### Paso 2: Inspeccionar un Esquema

Antes de comparar, veamos cómo AUDD interpreta un archivo:

```bash
./target/release/audd inspect --source fixtures/adapters/users.csv
```

**Salida esperada:**
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

**Nota:** Por defecto, CSV infiere todos los campos como `string`. La detección avanzada de tipos es una característica futura.

### Paso 3: Comparar Dos Fuentes

Ahora comparemos CSV y JSON:

```bash
./target/release/audd compare \
  --source-a fixtures/adapters/users.csv \
  --source-b fixtures/adapters/users.json \
  --out mi_primer_reporte
```

**Salida esperada en consola:**
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
Output files written to: mi_primer_reporte
```

### Paso 4: Explorar los Resultados

```bash
ls -l mi_primer_reporte/
```

Verás 4 archivos generados:

1. **unified_schema.json** - Esquema unificado (combinación de ambas fuentes)
2. **diff.json** - Detalles técnicos de la comparación
3. **decision_log.json** - Registro de decisiones automáticas
4. **report.md** - Reporte legible para humanos

**Ver el reporte Markdown:**
```bash
cat mi_primer_reporte/report.md
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

### Paso 5: Entender el Esquema Unificado

```bash
cat mi_primer_reporte/unified_schema.json | head -30
```

El esquema unificado marca el origen de cada campo:
- `"origin": "BOTH"` - Campo existe en ambas fuentes (match)
- `"origin": "A"` - Campo solo en fuente A (exclusivo)
- `"origin": "B"` - Campo solo en fuente B (exclusivo)

---

## Siguiente Nivel: Bases de Datos

### Ejemplo con SQLite

```bash
# Inspeccionar una base de datos SQLite
./target/release/audd inspect --source "db:sqlite:///ruta/a/tu/base.db"

# Comparar archivo vs base de datos
./target/release/audd compare \
  --source-a fixtures/adapters/users.csv \
  --source-b "db:sqlite:///ruta/a/tu/base.db" \
  --out csv_vs_db
```

### Ejemplo con MySQL

```bash
# Formato: db:mysql://usuario:contraseña@host/nombre_db
./target/release/audd inspect \
  --source "db:mysql://root:password@localhost/mi_base"

# Comparar dos bases de datos
./target/release/audd compare \
  --source-a "db:mysql://user:pass@localhost/db_desarrollo" \
  --source-b "db:mysql://user:pass@localhost/db_produccion" \
  --out dev_vs_prod
```

### Formatos de Conexión Soportados

- **SQLite:** `db:sqlite:///ruta/absoluta/archivo.db`
- **MySQL:** `db:mysql://usuario:password@host:puerto/base`
- **PostgreSQL:** `db:postgres://usuario:password@host:puerto/base`
- **MongoDB:** `db:mongodb://usuario:password@host:puerto/base`
- **SQL Server:** `db:sqlserver://usuario:password@host:puerto/base`
- **Firebird:** `db:firebird://usuario:password@host:puerto/ruta/base.fdb`

---

## Configuración Avanzada

### Generar Archivo de Configuración

```bash
./target/release/audd generate-config
```

Esto crea `audd.toml`:

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

### Usar Configuración Personalizada

```bash
# Editar audd.toml según tus necesidades
nano audd.toml

# AUDD lo cargará automáticamente desde ./audd.toml
./target/release/audd compare --source-a a.csv --source-b b.json

# O especificar ubicación personalizada
./target/release/audd --config mi-config.toml compare ...
```

Ver [CONFIG.md](CONFIG.md) para documentación completa de configuración.

---

## Comandos Principales

```bash
# Ver ayuda general
audd --help

# Ver ayuda de un comando específico
audd compare --help
audd inspect --help
audd load --help

# Inspeccionar y exportar IR a archivo
audd inspect --source datos.csv --out ir_output.json

# Cargar y mostrar esquema en consola
audd load --source datos.json

# Comparar con umbral de confianza personalizado
audd compare \
  --source-a a.csv \
  --source-b b.json \
  --confidence-threshold 0.95 \
  --out resultados
```

---

## Flujo de Trabajo Típico

### 1. Exploración Inicial
```bash
# Entender qué contiene cada fuente
audd inspect --source sistema_viejo.csv --out viejo_ir.json
audd inspect --source sistema_nuevo.json --out nuevo_ir.json
```

### 2. Comparación
```bash
# Generar reporte de diferencias
audd compare \
  --source-a sistema_viejo.csv \
  --source-b sistema_nuevo.json \
  --out analisis_migracion
```

### 3. Análisis de Resultados
```bash
# Leer el reporte Markdown
cat analisis_migracion/report.md

# Revisar conflictos en detalle
cat analisis_migracion/diff.json | grep -A 10 "conflicts"

# Verificar decisiones automáticas
cat analisis_migracion/decision_log.json
```

### 4. Planificación de Migración
Usar el reporte y esquema unificado para:
- Identificar campos que requieren transformación
- Planificar conversiones de tipos
- Documentar diferencias para stakeholders

---

## Solución de Problemas Comunes

### Error: "No se puede abrir el archivo"

```bash
# Verificar que el archivo existe
ls -l fixtures/adapters/users.csv

# Usar ruta absoluta si hay dudas
audd inspect --source /home/usuario/AUDD/fixtures/adapters/users.csv
```

### Error: "Formato no soportado"

AUDD detecta formatos por extensión. Extensiones soportadas:
- `.csv` - CSV
- `.json` - JSON
- `.xml` - XML
- `.sql` - SQL DDL
- `db:...` - Conexiones a bases de datos

### Error: "No se puede conectar a la base de datos"

```bash
# Verificar formato de conexión
# SQLite: Ruta absoluta después de ://
audd inspect --source "db:sqlite:///home/usuario/base.db"

# MySQL: Verificar credenciales
audd inspect --source "db:mysql://usuario:password@localhost/base"

# Verificar que el servicio esté corriendo
systemctl status mysql  # Linux
```

### Error de compilación

```bash
# Limpiar y recompilar
cargo clean
cargo build --release

# Actualizar Rust
rustup update
```

---

## Próximos Pasos

Ahora que has ejecutado tu primera comparación:

1. **Explora ejemplos avanzados:** Ver [examples/cli/README.md](../examples/cli/README.md)
2. **Lee sobre arquitectura:** Ver [Architecture.md](Architecture.md)
3. **Aprende configuración avanzada:** Ver [CONFIG.md](CONFIG.md)
4. **Explora casos de uso:** Ver [Usage-Examples.md](Usage-Examples.md)
5. **Contribuye al proyecto:** Ver [CONTRIBUTING.md](../CONTRIBUTING.md)

---

## Obtener Ayuda

- **Documentación:** [docs/](.)
- **Problemas comunes:** [FAQ.md](FAQ.md)
- **Issues de GitHub:** https://github.com/jmcasimar/AUDD/issues
- **Discusiones:** https://github.com/jmcasimar/AUDD/discussions

---

**¡Felicidades! Ya estás listo para usar AUDD en tus proyectos de integración de datos.**
