# FAQ - Preguntas Frecuentes

**🌐 Idioma / Language:**  
📘 **Español** | 📗 [English](en/FAQ.md)

---

## Índice

- [General](#general)
- [Instalación y Configuración](#instalación-y-configuración)
- [Fuentes de Datos](#fuentes-de-datos)
- [Comparación y Resultados](#comparación-y-resultados)
- [Configuración Avanzada](#configuración-avanzada)
- [Solución de Problemas](#solución-de-problemas)
- [Contribución y Desarrollo](#contribución-y-desarrollo)

---

## General

### ¿Qué es AUDD?

AUDD (Algoritmo de Unificación Dinámica de Datos) es una herramienta CLI y biblioteca Rust para comparar esquemas de datos de diferentes fuentes (CSV, JSON, XML, bases de datos SQL/NoSQL) y generar:
- Esquemas unificados automáticos
- Reportes de diferencias y conflictos
- Sugerencias inteligentes de resolución
- Logs auditables de decisiones

### ¿Para qué sirve AUDD?

**Casos de uso principales:**
- **Migraciones de datos**: Comparar esquema antiguo vs nuevo antes de migrar
- **Integración de datos**: Unificar datos de múltiples fuentes heterogéneas
- **Auditoría de esquemas**: Verificar consistencia entre desarrollo y producción
- **Planificación de ETL**: Identificar transformaciones necesarias entre sistemas

### ¿Es AUDD de código abierto?

Sí, AUDD está licenciado bajo MIT License. Puedes usarlo, modificarlo y distribuirlo libremente.

### ¿En qué lenguaje está escrito?

AUDD está escrito completamente en Rust, lo que garantiza alto rendimiento y seguridad de memoria.

---

## Instalación y Configuración

### ¿Qué necesito para instalar AUDD?

**Requisitos:**
- Rust 1.70 o superior
- Cargo (incluido con Rust)
- Git (para clonar el repositorio)

**Instalar Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ¿Cómo instalo AUDD?

```bash
# 1. Clonar repositorio
git clone https://github.com/jmcasimar/AUDD.git
cd AUDD

# 2. Compilar
cargo build --release

# 3. Ejecutar
./target/release/audd --version
```

Ver [Getting-Started.md](Getting-Started.md) para guía completa.

### ¿Puedo instalar AUDD sin compilar?

Actualmente, solo está disponible mediante compilación desde código fuente. En el futuro se publicará en crates.io para instalación con `cargo install audd`.

### ¿Cómo actualizo AUDD?

```bash
cd AUDD
git pull origin main
cargo build --release
```

### ¿Necesito instalar bases de datos para usar AUDD?

No necesariamente:
- **SQLite**: Incluido por defecto, no requiere servidor
- **MySQL/PostgreSQL/MongoDB**: Solo si quieres conectarte a estas bases de datos
- **Archivos (CSV/JSON/XML)**: No requieren ninguna base de datos

---

## Fuentes de Datos

### ¿Qué formatos de archivo soporta AUDD?

| Formato | Extensión | Auto-detección | Ejemplo |
|---------|-----------|----------------|---------|
| CSV     | `.csv`    | ✓             | `datos.csv` |
| JSON    | `.json`   | ✓             | `datos.json` |
| XML     | `.xml`    | ✓             | `datos.xml` |
| SQL DDL | `.sql`    | ✓             | `schema.sql` |

### ¿Qué bases de datos puedo conectar?

**Bases soportadas:**
- SQLite (por defecto)
- MySQL (por defecto)
- PostgreSQL (por defecto)
- MongoDB (por defecto)
- SQL Server (requiere feature `sqlserver`)
- Firebird (requiere feature `firebird`)

### ¿Cómo conecto a una base de datos?

**Formato general:**
```
db:<tipo>://<usuario>:<password>@<host>:<puerto>/<base_de_datos>
```

**Ejemplos:**
```bash
# SQLite (ruta absoluta)
audd inspect --source "db:sqlite:///home/user/datos.db"

# MySQL
audd inspect --source "db:mysql://root:password@localhost:3306/mi_base"

# PostgreSQL
audd inspect --source "db:postgres://user:pass@localhost:5432/mi_base"

# MongoDB
audd inspect --source "db:mongodb://user:pass@localhost:27017/mi_base"
```

### ¿Puedo comparar un archivo con una base de datos?

Sí, AUDD puede comparar cualquier combinación de fuentes:

```bash
audd compare \
  --source-a datos.csv \
  --source-b "db:mysql://user:pass@localhost/produccion" \
  --out comparacion
```

### ¿Soporta archivos remotos (URLs)?

Sí, AUDD puede cargar archivos desde HTTP/HTTPS:

```bash
audd inspect --source "https://example.com/datos.csv"
audd inspect --source "https://docs.google.com/spreadsheets/...export?format=csv"
```

### ¿Qué pasa si mi CSV no tiene encabezados?

AUDD requiere que los archivos CSV tengan una fila de encabezados. Si no la tienen, añade una manualmente o usa herramientas como `sed`:

```bash
# Añadir encabezados genéricos
echo "col1,col2,col3" | cat - datos.csv > datos_con_headers.csv
```

### ¿Detecta automáticamente los tipos de datos en CSV?

Actualmente, CSV infiere todos los campos como `String`. La detección avanzada de tipos está en desarrollo. Para tipos específicos, usa:
- Archivos SQL DDL (`.sql`) que especifican tipos
- Bases de datos que ya tienen tipos definidos
- JSON que preserva tipos primitivos (number, boolean, string)

---

## Comparación y Resultados

### ¿Qué archivos genera el comando `compare`?

El comando `compare` genera 4 archivos por defecto:

1. **unified_schema.json** - Esquema unificado (combina ambas fuentes)
2. **diff.json** - Resultados detallados de comparación (matches, exclusives, conflicts)
3. **decision_log.json** - Registro de decisiones de resolución automática
4. **report.md** - Reporte legible en Markdown

**Ejemplo:**
```bash
audd compare --source-a a.csv --source-b b.json --out resultados
ls resultados/
# unified_schema.json  diff.json  decision_log.json  report.md
```

### ¿Qué significa "Match", "Exclusive" y "Conflict"?

- **Match**: Campo existe en ambas fuentes con el mismo tipo → Sin problemas
- **Exclusive**: Campo existe solo en una fuente → Se incluye en esquema unificado
- **Conflict**: Campo existe en ambas pero con tipos incompatibles → Requiere resolución

**Ejemplo de salida:**
```
✓ Comparison complete!
  - Matches: 6      (campos idénticos)
  - Exclusives: 2   (campos únicos)
  - Conflicts: 1    (tipos incompatibles)
```

### ¿Cómo interpreto el reporte Markdown?

El `report.md` contiene:

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

Lee la sección de conflictos para identificar qué requiere atención.

### ¿Qué es el "confidence_threshold"?

El `confidence_threshold` (umbral de confianza) determina qué sugerencias automáticas se aceptan:

- **Valor**: 0.0 a 1.0 (default: 0.9)
- **Lógica**: Solo sugerencias con `confidence >= threshold` se auto-aceptan

**Ejemplo:**
```bash
# Más conservador (solo sugerencias muy seguras)
audd compare --source-a a.csv --source-b b.json --confidence-threshold 0.95

# Más agresivo (acepta más sugerencias)
audd compare --source-a a.csv --source-b b.json --confidence-threshold 0.75
```

### ¿Cómo resuelve conflictos AUDD?

AUDD genera **sugerencias** automáticas:

1. **CastSafe** (0.95 confianza): Conversión segura (Int32 → Int64)
2. **CastRisky** (0.6 confianza): Conversión con riesgo (Float → Int)
3. **RenameField** (variable): Renombrar campo similar
4. **PreferType** (0.8 confianza): Preferir tipo de una fuente
5. **ManualIntervention** (0.0): Requiere decisión humana

Las sugerencias con confianza >= threshold se aplican automáticamente.

### ¿Puedo revisar qué decisiones se tomaron?

Sí, el `decision_log.json` registra todas las decisiones:

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

### ¿Qué hago si no estoy de acuerdo con una decisión automática?

1. **Revisa** el `decision_log.json` para entender la lógica
2. **Ajusta** el `confidence_threshold` para ser más conservador
3. **Edita** manualmente el `unified_schema.json` si es necesario
4. **Reporta** el caso si crees que la sugerencia es incorrecta (issue en GitHub)

---

## Configuración Avanzada

### ¿Cómo creo un archivo de configuración?

```bash
audd generate-config
```

Esto crea `audd.toml` en el directorio actual:

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

### ¿Dónde busca AUDD el archivo de configuración?

**Orden de búsqueda:**
1. `./audd.toml` (directorio actual)
2. `~/.audd.toml` (home del usuario)
3. `~/.config/audd/config.toml` (XDG config)
4. Especificado con `--config ruta/a/config.toml`

### ¿Qué significa "similarity_threshold"?

El `similarity_threshold` controla qué tan similares deben ser los nombres para considerarse un "match":

- **Valor**: 0.0 a 1.0 (default: 0.8)
- **Algoritmo**: Jaro-Winkler similarity
- **Ejemplo**: "user_id" vs "userId" → 0.87 (match si threshold ≤ 0.87)

```toml
[compare]
similarity_threshold = 0.9  # Más estricto (requiere más similitud)
```

### ¿Qué hace "allow_risky_suggestions"?

Controla si se permiten sugerencias con riesgo de pérdida de datos:

```toml
[resolution]
allow_risky_suggestions = false  # No permitir casts riesgosos (default)
allow_risky_suggestions = true   # Permitir (ej: Float → Int)
```

**Ejemplo de sugerencia riesgosa:**
- Convertir `Float64` a `Int32` (pierde decimales)
- Convertir `DateTime` a `Date` (pierde hora)

### ¿Puedo deshabilitar la generación de algunos archivos de salida?

Sí, en la configuración:

```toml
[output]
generate_unified_schema = true   # unified_schema.json
generate_diff = true             # diff.json
generate_decision_log = true     # decision_log.json
generate_report = true           # report.md
```

Cambiar a `false` para omitir ese archivo.

---

## Solución de Problemas

### Error: "No se puede abrir el archivo"

**Causa:** El archivo no existe o la ruta es incorrecta

**Solución:**
```bash
# Verificar que el archivo existe
ls -l datos.csv

# Usar ruta absoluta
audd inspect --source /home/usuario/proyecto/datos.csv

# O navegar al directorio del archivo
cd /home/usuario/proyecto
audd inspect --source datos.csv
```

### Error: "Unsupported format"

**Causa:** AUDD no reconoce la extensión del archivo

**Solución:**
- Verifica la extensión: `.csv`, `.json`, `.xml`, `.sql`
- Renombra el archivo con la extensión correcta
- Para bases de datos, usa el prefijo `db:`

```bash
# Incorrecto
audd inspect --source datos.txt

# Correcto
mv datos.txt datos.csv
audd inspect --source datos.csv
```

### Error: "Cannot connect to database"

**Causa:** Credenciales incorrectas, servicio no corriendo, o formato de conexión incorrecto

**Solución:**

1. **Verificar servicio:**
```bash
# MySQL
sudo systemctl status mysql

# PostgreSQL
sudo systemctl status postgresql
```

2. **Verificar credenciales:**
```bash
# Probar conexión manual
mysql -u usuario -p -h localhost nombre_base
psql -U usuario -h localhost nombre_base
```

3. **Verificar formato de conexión:**
```bash
# SQLite: Ruta ABSOLUTA después de ://
db:sqlite:///ruta/absoluta/archivo.db

# MySQL: Incluir puerto si no es 3306
db:mysql://usuario:password@localhost:3306/base
```

4. **Permisos de base de datos:**
Asegúrate de que el usuario tiene permisos `SELECT` en las tablas del esquema.

### Error: "Failed to parse CSV"

**Causas comunes:**
- CSV sin encabezados
- Delimitador incorrecto (ej: punto y coma en lugar de coma)
- Encoding incorrecto (no UTF-8)

**Solución:**
```bash
# Verificar primeras líneas
head -5 datos.csv

# Convertir a UTF-8 si es necesario
iconv -f ISO-8859-1 -t UTF-8 datos.csv > datos_utf8.csv

# Si usa otro delimitador, convertir a CSV estándar
sed 's/;/,/g' datos_separado_por_punto_coma.csv > datos.csv
```

### Error de compilación: "linker `cc` not found"

**Causa:** Falta el compilador C (necesario para algunas dependencias)

**Solución:**
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# Fedora/RHEL
sudo dnf install gcc

# macOS
xcode-select --install
```

### Error: "libmysqlclient not found"

**Causa:** Falta biblioteca cliente de MySQL

**Solución:**
```bash
# Ubuntu/Debian
sudo apt-get install libmysqlclient-dev

# Fedora/RHEL
sudo dnf install mysql-devel

# macOS
brew install mysql-client
```

### La comparación es muy lenta

**Optimizaciones:**

1. **Reducir scope:**
```bash
# En lugar de comparar toda la base, exportar solo tablas relevantes a SQL
mysqldump schema_only db tabla1 tabla2 > subset.sql
audd compare --source-a subset.sql --source-b datos.json
```

2. **Usar archivos en lugar de conexiones DB en vivo:**
```bash
# Exportar esquemas primero
audd inspect --source "db:mysql://..." --out esquema_a.json
audd inspect --source "db:postgres://..." --out esquema_b.json

# Comparar archivos IR
# (requiere herramienta externa o carga manual)
```

3. **Aumentar similarity_threshold** (reduce comparaciones):
```toml
[compare]
similarity_threshold = 0.9  # Match más estricto = menos comparaciones
```

### ¿Cómo puedo ver mensajes de debug?

```bash
# Activar logs verbosos
RUST_LOG=debug ./target/release/audd compare ...

# O solo para AUDD
RUST_LOG=audd=debug ./target/release/audd compare ...
```

---

## Contribución y Desarrollo

### ¿Cómo puedo contribuir a AUDD?

Ver [CONTRIBUTING.md](../CONTRIBUTING.md) para guía completa.

**Formas de contribuir:**
- Reportar bugs (GitHub Issues)
- Sugerir features (GitHub Discussions)
- Mejorar documentación
- Añadir tests
- Implementar nuevos adaptadores
- Traducir documentación

### ¿Cómo reporto un bug?

1. Ve a https://github.com/jmcasimar/AUDD/issues
2. Busca si ya existe un issue similar
3. Si no, crea uno nuevo con:
   - Descripción clara del problema
   - Pasos para reproducir
   - Versión de AUDD (`audd --version`)
   - Sistema operativo
   - Archivos de ejemplo (si aplica)

### ¿Cómo ejecuto los tests?

```bash
# Todos los tests
cargo test

# Tests de un crate específico
cargo test -p audd_compare

# Test específico
cargo test test_csv_comparison

# Con output verbose
cargo test -- --nocapture
```

### ¿Cómo añado soporte para un nuevo formato?

Ver [Architecture.md](Architecture.md) sección "Extensibility" para guía detallada.

**Pasos básicos:**
1. Crear nuevo adaptador en `crates/audd_adapters_file/src/`
2. Implementar trait `FileAdapter`
3. Registrar en `AdapterRegistry`
4. Añadir tests
5. Actualizar documentación

### ¿Puedo usar AUDD como biblioteca en mi proyecto Rust?

Sí, AUDD está diseñado como biblioteca modular:

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

## Recursos Adicionales

### Documentación
- [Getting Started](Getting-Started.md) - Guía de inicio
- [Architecture](Architecture.md) - Arquitectura del sistema
- [Configuration](CONFIG.md) - Configuración detallada
- [Usage Examples](Usage-Examples.md) - Ejemplos avanzados

### Soporte
- [GitHub Issues](https://github.com/jmcasimar/AUDD/issues) - Reportar bugs
- [GitHub Discussions](https://github.com/jmcasimar/AUDD/discussions) - Preguntas y discusión

### Comunidad
- [Contributing](../CONTRIBUTING.md) - Guía para contribuir
- [Code of Conduct](../CODE_OF_CONDUCT.md) - Código de conducta

---

**¿No encuentras tu pregunta?** Abre un [Discussion](https://github.com/jmcasimar/AUDD/discussions) en GitHub.

**Última actualización:** 2026-01-26
