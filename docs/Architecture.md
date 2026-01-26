# AUDD Architecture

**🌐 Idioma / Language:**  
📘 **Español** | 📗 [English](en/Architecture.md)

---

## Visión General

AUDD (Algoritmo de Unificación Dinámica de Datos) es un sistema modular construido en Rust que permite la comparación y unificación de esquemas de datos heterogéneos. La arquitectura sigue principios de diseño que priorizan la extensibilidad, la separación de responsabilidades y la audibilidad de decisiones.

### Principios de Diseño

1. **Representación Canónica (IR)**: Todos los esquemas se normalizan a un modelo intermedio
2. **Adaptadores Plugables**: Nuevas fuentes de datos se pueden añadir sin modificar el core
3. **Comparación Determinística**: Resultados reproducibles para facilitar testing y auditoría
4. **Resolución Transparente**: Todas las decisiones están documentadas y son auditables
5. **Separación de Concerns**: Cada crate tiene una responsabilidad claramente definida

---

## Arquitectura de Capas

```
┌─────────────────────────────────────────────────────────────┐
│                     CLI / API Layer                          │
│                   (audd-cli crate)                           │
│  - Command parsing (clap)                                   │
│  - User interaction                                         │
│  - Output formatting                                        │
└────────────────────┬────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────┐
│              Orchestration Layer                             │
│  - Workflow coordination                                    │
│  - Error handling & reporting                               │
│  - Configuration management                                 │
└────┬───────────────┬──────────────────┬─────────────────────┘
     │               │                  │
     │               │                  │
┌────▼──────┐  ┌────▼─────────┐  ┌────▼──────────────────────┐
│  Adapters │  │  Comparison  │  │  Resolution Engine         │
│  Layer    │  │  Engine      │  │  (audd_resolution)         │
│           │  │ (audd_compare)│  │  - Conflict detection     │
│  File:    │  │  - Matching  │  │  - Suggestion generation  │
│  - CSV    │  │  - Diffing   │  │  - Confidence scoring     │
│  - JSON   │  │  - Unified   │  │  - Decision tracking      │
│  - XML    │  │    schema    │  └───────────────────────────┘
│  - SQL    │  │  - Metrics   │
│  - Remote │  └──────────────┘
│           │
│  Database:│
│  - SQLite │
│  - MySQL  │
│  - Postgres│
│  - MongoDB│
│  - SQLServer│
│  - Firebird│
└─────┬─────┘
      │
┌─────▼──────────────────────────────────────────────────────┐
│          Intermediate Representation (IR)                   │
│                (audd_ir crate)                              │
│  - SourceSchema: Normalized schema model                   │
│  - CanonicalType: Unified type system                      │
│  - EntitySchema: Tables/Collections representation         │
│  - FieldSchema: Column/Field metadata                      │
└─────────────────────────────────────────────────────────────┘
```

---

## Componentes Principales

### 1. audd_ir (Intermediate Representation)

**Propósito:** Modelo canónico para representar esquemas de cualquier fuente

**Tipos Principales:**

```rust
pub struct SourceSchema {
    pub source_name: String,
    pub source_type: String,
    pub entities: Vec<EntitySchema>,
    pub ir_version: String,
    pub metadata: HashMap<String, Value>,
}

pub struct EntitySchema {
    pub entity_name: String,
    pub entity_type: String,
    pub fields: Vec<FieldSchema>,
    pub keys: Vec<Key>,
    pub metadata: HashMap<String, Value>,
}

pub struct FieldSchema {
    pub field_name: String,
    pub canonical_type: CanonicalType,
    pub nullable: bool,
    pub default_value: Option<Value>,
    pub metadata: HashMap<String, Value>,
}

pub enum CanonicalType {
    Boolean,
    Int32,
    Int64,
    Float32,
    Float64,
    Decimal { precision: u8, scale: u8 },
    String { max_length: Option<u32> },
    Text,
    Date,
    Time,
    DateTime,
    Timestamp,
    UUID,
    Binary,
    Json,
    Array { element_type: Box<CanonicalType> },
    Unknown,
}
```

**Responsabilidades:**
- Definir estructura de datos canónica
- Serialización/deserialización JSON
- Normalización de identificadores
- Validación de esquemas

---

### 2. audd_adapters_file (File Adapters)

**Propósito:** Convertir archivos a IR

**Formatos Soportados:**

| Formato | Extensión | Auto-detección | Inferencia de Tipos |
|---------|-----------|----------------|---------------------|
| CSV     | `.csv`    | ✓             | Básica (string)     |
| JSON    | `.json`   | ✓             | ✓ (primitivos)      |
| XML     | `.xml`    | ✓             | Básica (string)     |
| SQL DDL | `.sql`    | ✓             | ✓ (tipos SQL)       |

**Proceso de Conversión:**

```
Archivo → Parser → Schema Detector → IR Generator → SourceSchema
```

**Ejemplo: CSV Adapter**
```rust
// Pseudocódigo simplificado
fn parse_csv(path: &Path) -> Result<SourceSchema> {
    let reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?;
    
    let fields = headers.iter().map(|name| {
        FieldSchema {
            field_name: normalize(name),
            canonical_type: CanonicalType::String { max_length: None },
            nullable: true,
            ..Default::default()
        }
    }).collect();
    
    Ok(SourceSchema {
        source_name: path.file_stem()?,
        source_type: "csv",
        entities: vec![EntitySchema {
            entity_name: path.file_stem()?,
            fields,
            ..Default::default()
        }],
        ir_version: "1.0.0",
        ..Default::default()
    })
}
```

**Característica Especial: Remote Files**
- Soporte para HTTP/HTTPS URLs
- Detección automática de formato
- Caché opcional de descargas
- Soporte Google Sheets (URLs públicas)

---

### 3. audd_adapters_db (Database Adapters)

**Propósito:** Extraer esquemas de bases de datos a IR

**Bases de Datos Soportadas:**

| Database    | Feature Flag | Extracción de Esquema | Metadatos |
|-------------|--------------|----------------------|-----------|
| SQLite      | (default)    | ✓                   | ✓         |
| MySQL       | (default)    | ✓                   | ✓         |
| PostgreSQL  | (default)    | ✓                   | ✓         |
| MongoDB     | (default)    | ✓ (inferencia)      | ✓         |
| SQL Server  | `sqlserver`  | ✓                   | ✓         |
| Firebird    | `firebird`   | ✓                   | ✓         |

**Proceso de Extracción:**

```
Connection String → Driver → Information Schema → IR Generator → SourceSchema
```

**Ejemplo: MySQL Adapter**
```sql
-- Queries usadas internamente para extraer esquema

-- 1. Listar tablas
SELECT TABLE_NAME 
FROM INFORMATION_SCHEMA.TABLES 
WHERE TABLE_SCHEMA = ?;

-- 2. Extraer columnas
SELECT COLUMN_NAME, DATA_TYPE, IS_NULLABLE, 
       CHARACTER_MAXIMUM_LENGTH, COLUMN_DEFAULT
FROM INFORMATION_SCHEMA.COLUMNS
WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ?
ORDER BY ORDINAL_POSITION;

-- 3. Obtener claves primarias
SELECT COLUMN_NAME
FROM INFORMATION_SCHEMA.KEY_COLUMN_USAGE
WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ? 
  AND CONSTRAINT_NAME = 'PRIMARY';
```

**Mapeo de Tipos SQL → Canonical:**

| MySQL Type    | PostgreSQL Type | CanonicalType              |
|---------------|-----------------|----------------------------|
| TINYINT(1)    | BOOLEAN         | Boolean                    |
| INT           | INTEGER         | Int32                      |
| BIGINT        | BIGINT          | Int64                      |
| FLOAT         | REAL            | Float32                    |
| DOUBLE        | DOUBLE          | Float64                    |
| DECIMAL(p,s)  | NUMERIC(p,s)    | Decimal{precision, scale}  |
| VARCHAR(n)    | VARCHAR(n)      | String{max_length}         |
| TEXT          | TEXT            | Text                       |
| DATE          | DATE            | Date                       |
| DATETIME      | TIMESTAMP       | DateTime                   |
| JSON          | JSONB           | Json                       |

---

### 4. audd_compare (Comparison Engine)

**Propósito:** Comparar dos esquemas IR y generar esquema unificado

**Algoritmo de Comparación:**

```
Input: SourceSchema A, SourceSchema B
Output: ComparisonResult, UnifiedSchema

1. Entity Matching:
   - Comparar nombres de entidades usando similitud Jaro-Winkler
   - Umbral: similarity_threshold (default 0.8)
   - Resultado: pares de entidades (matched, exclusive_a, exclusive_b)

2. Field Matching (por cada par de entidades):
   - Comparar nombres de campos (Jaro-Winkler)
   - Resultado: matched_fields, exclusive_a, exclusive_b

3. Conflict Detection (por cada par de campos matched):
   - Comparar tipos canónicos
   - Verificar nullable
   - Resultado: conflicts con detalles

4. Unified Schema Generation:
   - Incluir todos los campos matched (origin: BOTH)
   - Incluir campos exclusivos (origin: A o B)
   - Marcar conflictos como needs_resolution
```

**Similitud Jaro-Winkler:**
- Algoritmo robusto para detectar similitud de strings
- Tolera errores tipográficos y variaciones
- Valor: 0.0 (sin similitud) a 1.0 (idéntico)
- Ejemplo: "user_id" vs "userId" → 0.87

**Tipos de Resultados:**

```rust
pub enum FieldOrigin {
    A,           // Solo en fuente A
    B,           // Solo en fuente B
    BOTH,        // En ambas fuentes (matched)
}

pub enum FieldState {
    Matched,              // Sin conflictos
    Exclusive,            // Solo en una fuente
    Conflict { details }, // Tipos incompatibles
}

pub struct ComparisonResult {
    pub matches: Vec<FieldMatch>,
    pub exclusives: Vec<ExclusiveField>,
    pub conflicts: Vec<Conflict>,
}
```

**Generación de Métricas:**
- Total de campos comparados
- Porcentaje de coincidencias
- Número de conflictos por tipo
- Nivel de compatibilidad global

---

### 5. audd_resolution (Resolution Engine)

**Propósito:** Generar sugerencias para resolver conflictos

**Motor de Sugerencias:**

```rust
pub enum Suggestion {
    // Conversión segura (sin pérdida de datos)
    CastSafe {
        from: CanonicalType,
        to: CanonicalType,
        confidence: f64,
    },
    
    // Conversión con riesgo (posible pérdida)
    CastRisky {
        from: CanonicalType,
        to: CanonicalType,
        confidence: f64,
        risk_description: String,
    },
    
    // Renombrar campo
    RenameField {
        from: String,
        to: String,
        confidence: f64,
    },
    
    // Preferir tipo de una fuente
    PreferType {
        source: FieldOrigin,
        reason: String,
        confidence: f64,
    },
    
    // Intervención manual necesaria
    ManualIntervention {
        reason: String,
    },
}
```

**Matriz de Conversiones Seguras:**

| From → To       | Safe? | Ejemplo                      |
|-----------------|-------|------------------------------|
| Int32 → Int64   | ✓     | 100 → 100                    |
| Int64 → Int32   | ✗     | Overflow posible             |
| Int32 → Float64 | ✓     | 42 → 42.0                    |
| Float64 → Int32 | ✗     | Pérdida de decimales         |
| String → Text   | ✓     | Sin restricción de longitud  |
| Text → String   | ✗     | Posible truncamiento         |
| Date → DateTime | ✓     | 2024-01-01 → 2024-01-01 00:00|
| DateTime → Date | ✗     | Pérdida de información horaria|

**Cálculo de Confianza:**

```rust
fn calculate_confidence(suggestion: &Suggestion) -> f64 {
    match suggestion {
        Suggestion::CastSafe { .. } => 0.95,  // Alta confianza
        Suggestion::RenameField { from, to, .. } => {
            // Basado en similitud de nombres
            jaro_winkler(from, to)
        },
        Suggestion::CastRisky { .. } => 0.6,  // Confianza moderada
        Suggestion::ManualIntervention { .. } => 0.0,
        _ => 0.8,
    }
}
```

**Decision Log (Auditoría):**

```json
{
  "metadata": {
    "version": "1.0.0",
    "total_decisions": 3,
    "accepted_decisions": 2,
    "rejected_decisions": 1,
    "generated_at": "2026-01-26T15:35:24Z"
  },
  "decisions": [
    {
      "decision_id": "auto_dec_001",
      "conflict_type": "TypeMismatch",
      "suggested_action": "CastSafe",
      "confidence": 0.95,
      "accepted": true,
      "rationale": "Safe upcast from Int32 to Int64",
      "timestamp": "2026-01-26T15:35:24Z"
    }
  ]
}
```

---

### 6. audd-cli (Command-Line Interface)

**Propósito:** Interfaz de usuario y orquestación de workflows

**Estructura de Comandos:**

```rust
enum Command {
    Load {
        source: String,
    },
    Inspect {
        source: String,
        out: Option<PathBuf>,
    },
    Compare {
        source_a: String,
        source_b: String,
        out: PathBuf,
        confidence_threshold: Option<f64>,
    },
    GenerateConfig {
        out: Option<PathBuf>,
    },
}
```

**Workflow Interno (Compare Command):**

```
1. Parse CLI arguments (clap)
2. Load configuration (file + CLI overrides)
3. Load schema A (adaptadores)
   └─ Detect format → Select adapter → Parse → Generate IR
4. Load schema B
   └─ Detect format → Select adapter → Parse → Generate IR
5. Compare schemas (audd_compare)
   └─ Match entities → Match fields → Detect conflicts
6. Generate suggestions (audd_resolution)
   └─ Analyze conflicts → Generate suggestions → Score confidence
7. Create unified schema
   └─ Merge matched → Include exclusives → Mark conflicts
8. Generate outputs
   ├─ unified_schema.json (SourceSchema)
   ├─ diff.json (ComparisonResult)
   ├─ decision_log.json (DecisionLog)
   ├─ report.md (Markdown summary)
   └─ report.json (optional, structured)
9. Display summary to user
```

**Gestión de Configuración:**

```
Precedencia (mayor a menor):
1. CLI flags (--confidence-threshold 0.95)
2. Config file (--config custom.toml)
3. Auto-loaded config (./audd.toml, ~/.audd.toml, ~/.config/audd/config.toml)
4. Default values
```

---

## Flujos de Datos

### Flujo de Inspección

```
User Input (file/db path)
    ↓
Format Detection
    ↓
Adapter Selection
    ↓
Schema Extraction
    ↓
IR Generation
    ↓
JSON Output / Console Display
```

### Flujo de Comparación

```
Source A Input → Adapter A → IR-A ─┐
                                    ├→ Comparison Engine → Results
Source B Input → Adapter B → IR-B ─┘                           ↓
                                                    ┌───────────┴──────────┐
                                                    │                      │
                                              Resolution Engine    Unified Schema
                                                    ↓                      ↓
                                              Suggestions            Merged IR
                                                    ↓                      ↓
                                              Decision Log          unified_schema.json
                                                    ↓
                                              Output Files
                                              (diff.json, report.md, etc.)
```

---

## Extensibilidad

### Añadir Nuevo Adaptador de Archivo

```rust
// 1. Implementar trait Adapter
pub trait FileAdapter {
    fn can_handle(&self, path: &Path) -> bool;
    fn parse(&self, path: &Path) -> Result<SourceSchema>;
}

// 2. Implementar para nuevo formato
pub struct YamlAdapter;

impl FileAdapter for YamlAdapter {
    fn can_handle(&self, path: &Path) -> bool {
        path.extension().map_or(false, |e| e == "yaml" || e == "yml")
    }
    
    fn parse(&self, path: &Path) -> Result<SourceSchema> {
        // Lógica de parsing YAML → IR
    }
}

// 3. Registrar en AdapterRegistry
registry.register(Box::new(YamlAdapter));
```

### Añadir Nueva Base de Datos

```rust
// 1. Añadir feature en Cargo.toml
[features]
oracle = ["oracle-driver"]

// 2. Implementar DbAdapter trait
pub struct OracleAdapter {
    connection_string: String,
}

impl DbAdapter for OracleAdapter {
    fn extract_schema(&self) -> Result<SourceSchema> {
        // 1. Conectar a Oracle
        // 2. Consultar ALL_TABLES, ALL_TAB_COLUMNS
        // 3. Mapear tipos Oracle → CanonicalType
        // 4. Construir IR
    }
}

// 3. Añadir en dispatcher de conexiones
match scheme {
    "oracle" => OracleAdapter::new(conn_str).extract_schema(),
    // ...
}
```

### Añadir Nuevo Tipo de Sugerencia

```rust
pub enum Suggestion {
    // ... existentes ...
    
    // Nueva sugerencia
    MergeFields {
        field_a: String,
        field_b: String,
        strategy: MergeStrategy,
        confidence: f64,
    },
}

// Implementar lógica de generación
impl SuggestionEngine {
    fn generate_merge_suggestion(&self, conflict: &Conflict) -> Option<Suggestion> {
        // Detectar patrones que sugieren merge
        // Calcular confianza
        // Retornar sugerencia
    }
}
```

---

## Consideraciones de Rendimiento

### Optimizaciones Implementadas

1. **Streaming parsers**: CSV y JSON usan parsers streaming para archivos grandes
2. **Lazy loading**: Adaptadores cargan esquemas on-demand
3. **String interning**: Nombres de campos/tipos normalizados compartidos
4. **Connection pooling**: Reuso de conexiones DB (cuando aplicable)

### Límites Conocidos

| Aspecto              | Límite Actual        | Notas                           |
|----------------------|----------------------|---------------------------------|
| Tamaño archivo CSV   | ~1GB                 | Depende de memoria disponible   |
| Campos por entidad   | Sin límite hard      | Performance degrada >10k campos |
| Entidades por schema | Sin límite hard      | Performance degrada >1k tablas  |
| Profundidad JSON     | 128 niveles          | Límite del parser serde_json    |

### Recomendaciones

- **Archivos grandes**: Usar base de datos en lugar de archivos
- **Muchas tablas**: Comparar subconjuntos específicos
- **Tipos complejos**: Simplificar antes de comparar
- **Performance crítica**: Considerar cache de IR intermedio

---

## Testing y Calidad

### Estrategia de Testing

```
Unit Tests (por módulo)
    ├─ audd_ir: Serialización, validación
    ├─ audd_adapters_file: Parsing de cada formato
    ├─ audd_adapters_db: Extracción de esquema (mocks)
    ├─ audd_compare: Algoritmos de matching
    └─ audd_resolution: Generación de sugerencias

Integration Tests
    ├─ CLI end-to-end (fixtures reales)
    ├─ Workflows completos (load → compare → output)
    └─ Multi-formato (CSV vs JSON vs DB)

Golden Tests
    ├─ Outputs determinísticos
    ├─ Comparación contra snapshots conocidos
    └─ Detecta regresiones en resultados
```

### CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu, windows, macos]
    steps:
      - cargo fmt --check
      - cargo clippy -- -D warnings
      - cargo test --all-features
      - cargo build --release
```

---

## Decisiones de Diseño

### ¿Por qué Rust?

- **Performance**: Necesario para procesar grandes volúmenes de datos
- **Safety**: Type system previene bugs comunes en parsing/conversión
- **Concurrency**: Preparado para paralelización futura
- **Tooling**: Cargo, rustfmt, clippy facilitan desarrollo

### ¿Por qué IR Canónica?

- **Desacopla** fuentes de lógica de comparación
- **Simplifica** añadir nuevos adaptadores
- **Normaliza** diferencias sintácticas irrelevantes
- **Facilita** testing con fixtures controlados

### ¿Por qué Jaro-Winkler?

- **Robusto** contra variaciones comunes (camelCase vs snake_case)
- **Rápido** O(n) para strings típicos
- **Calibrado** para nombres de campos (prefiere coincidencias al inicio)

---

## Roadmap Futuro

### Características Planificadas

- **Inferencia avanzada de tipos** para CSV
- **Soporte incremental** (comparar cambios delta)
- **Modo interactivo** (resolver conflictos en CLI)
- **Generación de scripts de migración** (SQL ALTER TABLE, etc.)
- **API REST** (servicio web para comparaciones)
- **Soporte para schemas complejos** (relaciones, constraints)
- **Machine learning** para sugerencias más inteligentes

### Mejoras de Performance

- **Paralelización** de comparación de entidades
- **Cache distribuida** para esquemas grandes
- **Streaming comparisons** para datasets masivos

---

## Referencias

- **Jaro-Winkler**: Algoritmo de similitud de strings
- **IR Pattern**: Inspirado en compiladores (AST → IR → Target)
- **Adapter Pattern**: Gang of Four design patterns
- **JSON Schema**: Para validación de IR

---

**Última actualización:** 2026-01-26  
**Versión de arquitectura:** 1.0  
**Estado:** Implementación MVP completada
