# audd_ir - Representación Intermedia de AUDD

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Representación de schema canónico para el proyecto AUDD (Algoritmo de Unificación Dinámica de Datos).

## Descripción General

`audd_ir` proporciona una representación intermedia (IR) unificada para esquemas de fuentes de datos heterogéneas. Permite la comparación, mapeo y unificación de esquemas entre diferentes bases de datos, formatos de archivo y sistemas de datos.

## Características

- **Sistema de Tipos Canónico**: Representación abstracta de tipos de datos comunes
- **Normalización de Identificadores**: Convierte identificadores a un formato estándar (snake_case)
- **Mapeo de Tipos**: Mapea tipos específicos de bases de datos a tipos canónicos
- **Construcción de Schema**: Patrón builder ergonómico para construir esquemas
- **Serialización**: Exportación/importación JSON para depuración y pruebas
- **Compatibilidad de Tipos**: Verifica compatibilidad entre diferentes tipos
- **Extensibilidad**: Campos de metadata para extensiones personalizadas

## Bases de Datos Soportadas

- MySQL
- PostgreSQL
- SQLite

Se puede agregar soporte para bases de datos adicionales implementando mapeos de tipos.

## Instalación

Agrega esto a tu `Cargo.toml`:

```toml
[dependencies]
audd_ir = "0.1"
```

## Inicio Rápido

```rust
use audd_ir::{SourceSchema, EntitySchema, FieldSchema, CanonicalType};

// Construir un schema
let field = FieldSchema::builder()
    .field_name("user_id")
    .canonical_type(CanonicalType::Int32)
    .nullable(false)
    .build();

let entity = EntitySchema::builder()
    .entity_name("users")
    .entity_type("table")
    .add_field(field)
    .build();

let source = SourceSchema::builder()
    .source_name("myapp_db")
    .source_type("mysql")
    .add_entity(entity)
    .build();

// Serializar a JSON
let json = source.to_json().unwrap();
println!("{}", json);

// Deserializar desde JSON
let loaded = SourceSchema::from_json(&json).unwrap();
assert_eq!(source, loaded);
```

## Ejemplos

### Normalizar Identificadores

```rust
use audd_ir::normalize_identifier;

assert_eq!(normalize_identifier("UserEmail"), "user_email");
assert_eq!(normalize_identifier("Product Name"), "product_name");
assert_eq!(normalize_identifier("firstName"), "first_name");
```

### Mapear Tipos de Bases de Datos

```rust
use audd_ir::{map_type_to_canonical, CanonicalType};

assert_eq!(
    map_type_to_canonical("mysql", "VARCHAR(255)"),
    CanonicalType::String
);

assert_eq!(
    map_type_to_canonical("postgresql", "UUID"),
    CanonicalType::Uuid
);
```

### Verificar Compatibilidad de Tipos

```rust
use audd_ir::CanonicalType;

assert!(CanonicalType::Int32.is_compatible_with(&CanonicalType::Int64));
assert!(CanonicalType::String.is_compatible_with(&CanonicalType::Text));
```

### Ejemplo Completo

Ejecuta el ejemplo incluido:

```bash
cargo run --example ir_demo
```

## Arquitectura

El IR consiste en varios componentes clave:

- **SourceSchema**: Contenedor de nivel superior para una fuente de datos
- **EntitySchema**: Representa tablas, colecciones o estructuras equivalentes
- **FieldSchema**: Representa campos/columnas individuales
- **CanonicalType**: Sistema de tipos unificado
- **Constraint**: Restricciones de campo (MaxLength, Unique, etc.)
- **Key**: Claves primarias, únicas y foráneas

## Documentación

La documentación completa está disponible en [docs/ir.md](../../docs/ir.md).

## Pruebas

Ejecuta la suite de pruebas:

```bash
cargo test --package audd_ir
```

Esto ejecuta:
- 29 pruebas unitarias
- 4 pruebas de integración con fixtures
- 4 pruebas de documentación

Las 37 pruebas deberían pasar.

## Fixtures

Esquemas IR de ejemplo están disponibles en `fixtures/ir/`:

- `simple_a.json`: Tabla users de MySQL
- `simple_b.json`: Tabla users de PostgreSQL

Estos demuestran el formato IR y pueden usarse para pruebas.

## Contribuir

¡Las contribuciones son bienvenidas! Por favor consulta [CONTRIBUTING.md](../../CONTRIBUTING.md) para lineamientos.

## Licencia

Licenciado bajo la Licencia MIT. Ver [LICENSE](../../LICENSE) para detalles.

## Versión

Versión IR actual: **1.0.0**

El IR usa versionado semántico. Ver [docs/ir.md](../../docs/ir.md) para la estrategia de versionado.
