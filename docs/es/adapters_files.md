# Documentación de Adapters de Archivos

**Versión:** 1.0.0  
**Última Actualización:** 2026-01-25

## Descripción General

Los adapters de archivos de AUDD permiten la extracción de schemas desde formatos de archivo comunes (CSV, JSON, XML, SQL/DDL) y la conversión a la Representación Intermedia (IR) de AUDD. Esto proporciona adopción inmediata sin conectores de base de datos y permite fixtures reproducibles para evaluación académica.

## Formatos Soportados

### Adapter CSV

**Extensión:** `.csv`

**Enfoque:**
- Los encabezados se convierten en nombres de fields
- Todos los fields tienen como predeterminado el tipo `String` (inferencia de tipos opcional en iteraciones futuras)
- El nombre de la entidad se deriva del nombre del archivo
- Todos los fields se marcan como nullable por defecto

**Ejemplo:**

Archivo de entrada `users.csv`:
```csv
id,name,email,age
1,Alice,alice@example.com,30
2,Bob,bob@example.com,25
```

IR generado:
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
        {"field_name": "age", "canonical_type": {"type": "string"}, "nullable": true}
      ]
    }
  ]
}
```

**Uso del CLI:**
```bash
audd load --source file:users.csv
```

**Limitaciones:**
- Sin inferencia de tipos (todos los fields son String)
- Sin soporte para múltiples tablas (un CSV = una entidad)
- Los encabezados son requeridos
- Sin soporte para fields entre comillas con saltos de línea (solo CSV básico)

---

### Adapter JSON

**Extensiones:** `.json`

**Enfoque:**
- Soporta un solo objeto o array de objetos
- Las claves del (primer) objeto se convierten en nombres de fields
- Inferencia de tipos básica: boolean, number (int/float), string, nested (tipo JSON)
- Las estructuras profundamente anidadas se tratan como tipo JSON
- El nombre de la entidad se deriva del nombre del archivo

**Ejemplo 1: Objeto Único**

Archivo de entrada `config.json`:
```json
{
  "id": 1,
  "name": "Alice",
  "active": true,
  "score": 95.5
}
```

**Ejemplo 2: Array de Objetos**

Archivo de entrada `users.json`:
```json
[
  {"id": 1, "name": "Alice", "active": true},
  {"id": 2, "name": "Bob", "active": false}
]
```

IR generado:
- `id` → Int64
- `name` → String
- `active` → Boolean
- `score` → Float64

**Uso del CLI:**
```bash
audd load --source file:users.json
```

**Limitaciones:**
- Solo se soportan objetos planos o poco profundos (MVP)
- Arrays heterogéneos no soportados (schema inferido del primer elemento)
- Sin tipos de unión o schemas anidados complejos
- Los arrays vacíos producen un error

---

### Adapter XML (MVP)

**Extensión:** `.xml`

**Enfoque:**
- Las etiquetas hijo de primer nivel de `<record>`, `<item>` o `<row>` se convierten en fields
- Todos los fields tienen como predeterminado el tipo String
- Los atributos se convierten en fields con sufijo `_attr`
- Asume estructura homogénea en todos los registros

**Ejemplo:**

Archivo de entrada `users.xml`:
```xml
<?xml version="1.0"?>
<users>
  <record id="1">
    <name>Alice</name>
    <email>alice@example.com</email>
  </record>
  <record id="2">
    <name>Bob</name>
    <email>bob@example.com</email>
  </record>
</users>
```

Fields del IR generados:
- `id_attr` (del atributo)
- `name`
- `email`

**Uso del CLI:**
```bash
audd load --source file:users.xml
```

**Limitaciones:**
- MVP: Solo estructura básica, sin XPath complejo o namespaces
- Sin validación contra XSD/DTD
- Todos los fields son tipo String
- Asume estructura de registro uniforme
- Elementos anidados más allá de profundidad 3 no se extraen como fields separados

---

### Adapter SQL/DDL

**Extensiones:** `.sql`, `.ddl`

**Enfoque:**
- Analiza sentencias `CREATE TABLE`
- Extrae nombres y tipos de columnas
- Mapea tipos SQL a tipos canónicos
- Soporta restricciones básicas: PRIMARY KEY, NOT NULL, UNIQUE
- Se soportan múltiples tablas (un CREATE TABLE = una entidad)

**Ejemplo:**

Archivo de entrada `schema.sql`:
```sql
CREATE TABLE users (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE,
    age INT,
    created_at TIMESTAMP
);

CREATE TABLE posts (
    id INT PRIMARY KEY,
    title TEXT NOT NULL,
    published BOOLEAN
);
```

IR generado:
- Dos entidades: `users` y `posts`
- Fields `id`: Int32, no nullable (PRIMARY KEY implica NOT NULL)
- `name`: String, no nullable
- `email`: String, nullable, con restricción UNIQUE
- Mapeos de tipos aplicados (INT → Int32, VARCHAR → String, TIMESTAMP → Timestamp, etc.)

**Uso del CLI:**
```bash
audd load --source file:schema.sql
```

**Mapeos de Tipos:**

| Tipo SQL | Tipo Canónico |
|----------|---------------|
| INT, INTEGER, SMALLINT, MEDIUMINT | Int32 |
| BIGINT, LONG | Int64 |
| FLOAT, REAL | Float32 |
| DOUBLE, DOUBLE PRECISION | Float64 |
| DECIMAL, NUMERIC | Decimal(10,2) |
| BOOLEAN, BOOL | Boolean |
| CHAR, VARCHAR, TEXT, NVARCHAR | String |
| CLOB, LONGTEXT, MEDIUMTEXT | Text |
| BLOB, BINARY, VARBINARY | Binary |
| DATE | Date |
| TIME | Time |
| DATETIME, TIMESTAMP | Timestamp |
| JSON | Json |
| UUID | Uuid |

**Limitaciones:**
- Solo un subconjunto de SQL DDL (no es un parser completo de SQL)
- Sin soporte para:
  - ALTER TABLE
  - Restricciones de clave foránea (analizadas pero aún no representadas en IR)
  - Restricciones CHECK
  - Valores DEFAULT (excepto en metadata)
  - Dialectos SQL complejos (extensiones específicas de MySQL/PostgreSQL/SQLite)
  - CREATE INDEX
  - Vistas, triggers, procedimientos almacenados
- Soporte mínimo para cláusulas `IF NOT EXISTS`, `CONSTRAINT`
- Los comentarios y variaciones de espacios en blanco pueden afectar el análisis

---

## No Soportado (Limitaciones Actuales)

### Todos los Formatos
- Extracción de lógica de negocio semántica (ej., detectar fields "email" vs. "phone")
- Inferencia de relaciones entre entidades
- Reglas de validación de datos más allá de restricciones básicas

### CSV
- Inferencia de tipos (planeada para iteración futura)
- Agregación de múltiples archivos
- Metadata de columnas (unidades, formatos, etc.)

### JSON
- Estructuras profundamente anidadas o recursivas
- Arrays polimórficos/heterogéneos
- Validación de JSON Schema
- JSON-LD o anotaciones semánticas

### XML
- Consultas XPath
- Validación de XML Schema (XSD)
- Namespaces
- Contenido mixto (texto + elementos)
- Jerarquías de elementos complejas

### SQL/DDL
- Soporte completo de dialectos SQL (MySQL, PostgreSQL, SQL Server, Oracle)
- Claves foráneas representadas en IR
- Índices
- Vistas y vistas materializadas
- Procedimientos almacenados y triggers
- Restricciones avanzadas (CHECK, exclusión)

---

## Uso de la API

### API de Rust

```rust
use audd_adapters_file::load_schema_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Auto-detectar formato desde la extensión
    let schema = load_schema_from_file("users.csv")?;
    
    println!("Source: {} ({})", schema.source_name, schema.source_type);
    
    for entity in &schema.entities {
        println!("Entity: {}", entity.entity_name);
        for field in &entity.fields {
            println!("  - {}: {:?}", field.field_name, field.canonical_type);
        }
    }
    
    Ok(())
}
```

### Selección Manual de Adapter

```rust
use audd_adapters_file::{CsvAdapter, JsonAdapter, SqlAdapter, XmlAdapter, SchemaAdapter};

// CSV
let csv_adapter = CsvAdapter::new();
let schema = csv_adapter.load(Path::new("data.csv"))?;

// JSON
let json_adapter = JsonAdapter::new();
let schema = json_adapter.load(Path::new("data.json"))?;

// XML
let xml_adapter = XmlAdapter::new();
let schema = xml_adapter.load(Path::new("data.xml"))?;

// SQL
let sql_adapter = SqlAdapter::new();
let schema = sql_adapter.load(Path::new("schema.sql"))?;
```

---

## Manejo de Errores

Los adapters pueden devolver los siguientes errores:

- **`IoError`**: Archivo no encontrado o no se puede leer
- **`CsvError`**: Formato CSV inválido
- **`JsonError`**: Sintaxis JSON inválida
- **`XmlError`**: XML mal formado
- **`SqlError`**: Error de análisis SQL
- **`UnsupportedFormat`**: Extensión de archivo no reconocida
- **`InvalidStructure`**: La estructura del archivo no coincide con el formato esperado
- **`EmptyData`**: No se encontraron datos o fields en el archivo

Ejemplo de manejo de errores:

```rust
use audd_adapters_file::{load_schema_from_file, AdapterError};

match load_schema_from_file("data.csv") {
    Ok(schema) => println!("Loaded: {}", schema.source_name),
    Err(AdapterError::UnsupportedFormat(ext)) => {
        eprintln!("Format '{}' not supported", ext);
    }
    Err(AdapterError::EmptyData(msg)) => {
        eprintln!("Empty file: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## Fixtures

Los fixtures de muestra están disponibles en `/fixtures/adapters/`:

- `users.csv` - CSV de muestra con múltiples columnas
- `users.json` - JSON de muestra con array de objetos
- `users.xml` - XML de muestra con registros
- `schema.sql` - SQL DDL de muestra con múltiples tablas

Estos fixtures se utilizan para propósitos de prueba y demostración.

---

## Mejoras Futuras

### Planeadas (Post-MVP)
- Inferencia de tipos CSV (detección inteligente de enteros, fechas, booleanos)
- JSON: Mejor manejo de estructuras anidadas
- XML: Extracción de fields basada en XPath
- SQL: Más soporte específico de dialectos (MySQL, PostgreSQL)
- Opciones de configuración (ej., mapeos de tipos personalizados, manejo de nulos)

### En Consideración
- Adapter Excel/XLSX
- Adapter Parquet
- Adapter YAML
- Adapter Avro
- Adapter de schema Protobuf

---

## Contribuir

Al agregar un nuevo adapter:

1. Implemente el trait `SchemaAdapter`
2. Agregue pruebas (unitarias + integración)
3. Regístrelo en `factory.rs` para auto-detección
4. Agregue ejemplos de fixtures
5. Actualice esta documentación
6. Asegúrese de que los mensajes de error sean claros y accionables

---

## Licencia

Consulte el archivo LICENSE en la raíz del repositorio.
