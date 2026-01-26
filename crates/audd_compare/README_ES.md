# AUDD Compare

Motor de comparación para el proyecto AUDD (Automatic Unification of Data Definitions).

## Descripción General

`audd_compare` proporciona un motor de comparación sofisticado para analizar dos representaciones de schema e identificar:
- **Coincidencias**: Campos/entidades que son compatibles entre esquemas
- **Exclusivos**: Campos/entidades presentes en solo un schema
- **Conflictos**: Incompatibilidades que requieren resolución

## Características

### Capacidades Centrales

- **Múltiples Estrategias de Coincidencia**
  - Coincidencia exacta de nombres
  - Coincidencia normalizada (insensible a mayúsculas, snake_case/camelCase)
  - Coincidencia basada en similitud (algoritmo Jaro-Winkler)

- **Análisis de Compatibilidad de Tipos**
  - Tipos idénticos
  - Tipos compatibles (ej., String ↔ Text)
  - Ampliación segura (ej., Int32 → Int64)
  - Conversión requerida (ej., Int64 → Int32)
  - Tipos incompatibles

- **Detección de Conflictos**
  - Incompatibilidades de tipos
  - Desajustes de nulabilidad
  - Conflictos de restricciones (unique, length, precision)
  - Colisiones de normalización

- **Generación de Schema Unificado**
  - Combina ambos esquemas en una única representación
  - Rastrea orígenes de campos (A, B o Ambos)
  - Marca estados de campos (Matched, Exclusive, Conflicted)

### Configuración

El motor de comparación es altamente configurable:

```rust
use audd_compare::CompareConfig;

// Configuración predeterminada (coincidencia exacta + normalizada, todas las verificaciones)
let config = CompareConfig::default();

// Todas las características habilitadas (incluye coincidencia de similitud)
let config = CompareConfig::all_features()
    .with_similarity_threshold(0.8);

// Configuración mínima (solo coincidencia exacta)
let config = CompareConfig::minimal();

// Configuración estricta (todas las verificaciones, umbral alto)
let config = CompareConfig::strict();
```

## Uso

### Ejemplo Básico

```rust
use audd_compare::{compare, CompareConfig};
use audd_ir::{SourceSchema, EntitySchema, FieldSchema, CanonicalType};

// Crear schema A
let schema_a = SourceSchema::builder()
    .source_name("db_a")
    .source_type("mysql")
    .add_entity(
        EntitySchema::builder()
            .entity_name("users")
            .add_field(
                FieldSchema::builder()
                    .field_name("id")
                    .canonical_type(CanonicalType::Int32)
                    .nullable(false)
                    .build()
            )
            .build()
    )
    .build();

// Crear schema B
let schema_b = SourceSchema::builder()
    .source_name("db_b")
    .source_type("postgresql")
    .add_entity(
        EntitySchema::builder()
            .entity_name("users")
            .add_field(
                FieldSchema::builder()
                    .field_name("id")
                    .canonical_type(CanonicalType::Int64)
                    .nullable(false)
                    .build()
            )
            .build()
    )
    .build();

// Comparar
let config = CompareConfig::default();
let result = compare(&schema_a, &schema_b, &config);

// Analizar resultados
println!("Matches: {}", result.summary.total_matches);
println!("Exclusives: {}", result.summary.total_exclusives);
println!("Conflicts: {}", result.summary.total_conflicts);
```

### Generar Schema Unificado

```rust
use audd_compare::UnifiedSchema;

let unified = UnifiedSchema::from_comparison(&schema_a, &schema_b, &result);

// Exportar a JSON
let json = unified.to_json()?;
```

## Ejemplos

Ejecuta el ejemplo de demostración:

```bash
cargo run --example compare_demo
```

## Pruebas

El crate incluye cobertura de pruebas completa:

```bash
# Ejecutar todas las pruebas
cargo test -p audd_compare

# Ejecutar solo pruebas unitarias
cargo test -p audd_compare --lib

# Ejecutar solo pruebas de integración
cargo test -p audd_compare --test integration_test
```

## Arquitectura

### Módulos

- **config**: Opciones de configuración para el motor de comparación
- **conflict**: Tipos de conflicto y rastreo de evidencia
- **engine**: Lógica de comparación principal
- **matcher**: Algoritmos de coincidencia de entidades y campos
- **result**: Estructuras Match, Exclusive y ComparisonResult
- **types**: Análisis de compatibilidad de tipos
- **unified**: Construcción de schema unificado

### Flujo de Datos

```
Schema A + Schema B + Config
         ↓
    Matching Engine
         ↓
    ┌────┴────┬──────────┬──────────┐
    ↓         ↓          ↓          ↓
  Matches  Exclusives Conflicts  Summary
         ↓
   ComparisonResult
         ↓
   UnifiedSchema
```

## Dependencias

- `audd_ir`: Representación intermedia para esquemas
- `serde`: Serialización/deserialización
- `strsim`: Algoritmos de similitud de cadenas

## Licencia

MIT
