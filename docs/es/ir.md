# Especificación de Representación Intermedia (IR) de AUDD

**Versión:** 1.0.0  
**Última actualización:** 2026-01-25

## Descripción general

La Representación Intermedia (IR) de AUDD es un modelo de schema canónico diseñado para permitir la comparación y unificación de fuentes de datos heterogéneas. El IR sirve como un contrato interno que normaliza schemas de diferentes fuentes (bases de datos, archivos, APIs) en una estructura uniforme.

## Principios de diseño

1. **Compatibilidad primero**: El IR prioriza la compatibilidad estructural sobre la perfección semántica
2. **Extensibilidad**: Utiliza campos de metadata para extensiones futuras sin cambios incompatibles
3. **Normalización**: Los identificadores y tipos se normalizan para una comparación consistente
4. **Versionado**: El versionado explícito permite la evolución mientras se mantiene la compatibilidad hacia atrás

## Estructuras principales

### SourceSchema

Representa un schema completo de fuente de datos (base de datos, archivo, colección).

**Campos:**
- `source_name` (String): Nombre normalizado de la fuente de datos
- `source_type` (String): Tipo de fuente (ej., "mysql", "postgresql", "csv", "json")
- `entities` (Vec<EntitySchema>): Lista de entidades (tablas/colecciones) en esta fuente
- `ir_version` (String): Versión de la especificación IR (ej., "1.0.0")
- `metadata` (HashMap<String, Value>): Metadata extensible para información específica de la fuente

**Ejemplo:**
```json
{
  "source_name": "customers_db",
  "source_type": "mysql",
  "entities": [...],
  "ir_version": "1.0.0",
  "metadata": {}
}
```

### EntitySchema

Representa una tabla, colección o estructura equivalente dentro de una fuente.

**Campos:**
- `entity_name` (String): Nombre normalizado de la entidad
- `entity_type` (String): Tipo de entidad ("table", "collection", "sheet", etc.)
- `fields` (Vec<FieldSchema>): Lista de campos/columnas en esta entidad
- `keys` (Vec<Key>): Claves primarias y únicas (MVP: soporte básico)
- `metadata` (HashMap<String, Value>): Metadata específica de la entidad

**Ejemplo:**
```json
{
  "entity_name": "users",
  "entity_type": "table",
  "fields": [...],
  "keys": [
    {
      "key_type": "primary",
      "field_names": ["id"]
    }
  ],
  "metadata": {}
}
```

### FieldSchema

Representa un campo/columna dentro de una entidad.

**Campos:**
- `field_name` (String): Nombre normalizado del campo
- `canonical_type` (CanonicalType): Tipo de dato canónico
- `nullable` (bool): Si el campo acepta valores nulos
- `constraints` (Vec<Constraint>): Restricciones adicionales en el campo
- `metadata` (HashMap<String, Value>): Metadata específica del campo (ej., original_name, original_type)

**Ejemplo:**
```json
{
  "field_name": "email",
  "canonical_type": "String",
  "nullable": false,
  "constraints": [
    {
      "constraint_type": "MaxLength",
      "value": 255
    }
  ],
  "metadata": {
    "original_name": "Email",
    "original_type": "VARCHAR(255)"
  }
}
```

### CanonicalType

Enumeración de tipos de datos canónicos que abstraen los tipos específicos de cada fuente.

**Tipos (Subconjunto MVP):**
- `Boolean`: Valores verdadero/falso
- `Int32`: Entero con signo de 32 bits
- `Int64`: Entero con signo de 64 bits
- `Float32`: Punto flotante de 32 bits
- `Float64`: Punto flotante de 64 bits
- `Decimal`: Decimal de precisión arbitraria con (precision, scale)
- `String`: Texto de longitud variable
- `Text`: Texto grande (CLOB, TEXT, etc.)
- `Binary`: Datos binarios (BLOB)
- `Date`: Fecha sin hora
- `Time`: Hora sin fecha
- `DateTime`: Fecha y hora
- `Timestamp`: Timestamp con zona horaria
- `Json`: Datos JSON
- `Uuid`: UUID/GUID
- `Unknown`: Alternativa para tipos no mapeados

**Parámetros de tipo:**
- `Decimal { precision: u16, scale: u16 }`: Para números decimales precisos
- `String` con restricción MaxLength: Para equivalentes de VARCHAR

### Constraint

Representa restricciones y reglas de validación en los campos.

**Tipos (Subconjunto MVP):**
- `MaxLength(usize)`: Longitud máxima de cadena
- `MinLength(usize)`: Longitud mínima de cadena
- `Precision(u16, u16)`: Precisión y escala decimal
- `Unique`: Restricción de unicidad
- `DefaultValue(Value)`: Valor predeterminado
- `Check(String)`: Expresión de verificación (almacenada como cadena para MVP)

**Ejemplo:**
```json
[
  { "constraint_type": "MaxLength", "value": 100 },
  { "constraint_type": "Unique" }
]
```

### Key

Representa claves primarias, claves foráneas y restricciones únicas.

**Campos (Subconjunto MVP):**
- `key_type` (String): "primary", "unique", "foreign" (soporte limitado de foreign en MVP)
- `field_names` (Vec<String>): Campos que componen la clave
- `metadata` (HashMap<String, Value>): Información adicional de la clave

**Ejemplo:**
```json
{
  "key_type": "primary",
  "field_names": ["user_id"]
}
```

## Tabla de mapeo de tipos

### MySQL → CanonicalType

| Tipo MySQL | Canonical Type | Notas |
|------------|---------------|-------|
| TINYINT(1) | Boolean | Cuando la longitud es 1 |
| TINYINT | Int32 | |
| SMALLINT | Int32 | |
| MEDIUMINT | Int32 | |
| INT | Int32 | |
| BIGINT | Int64 | |
| FLOAT | Float32 | |
| DOUBLE | Float64 | |
| DECIMAL(p,s) | Decimal(p,s) | |
| CHAR(n) | String | Con MaxLength(n) |
| VARCHAR(n) | String | Con MaxLength(n) |
| TEXT | Text | |
| TINYTEXT | Text | |
| MEDIUMTEXT | Text | |
| LONGTEXT | Text | |
| BLOB | Binary | |
| DATE | Date | |
| TIME | Time | |
| DATETIME | DateTime | |
| TIMESTAMP | Timestamp | |
| JSON | Json | |
| BINARY(n) | Binary | |
| VARBINARY(n) | Binary | |

### PostgreSQL → CanonicalType

| Tipo PostgreSQL | Canonical Type | Notas |
|----------------|---------------|-------|
| boolean | Boolean | |
| smallint | Int32 | |
| integer | Int32 | |
| bigint | Int64 | |
| real | Float32 | |
| double precision | Float64 | |
| numeric(p,s) | Decimal(p,s) | |
| decimal(p,s) | Decimal(p,s) | |
| char(n) | String | Con MaxLength(n) |
| varchar(n) | String | Con MaxLength(n) |
| text | Text | |
| bytea | Binary | |
| date | Date | |
| time | Time | |
| timestamp | DateTime | |
| timestamptz | Timestamp | |
| json | Json | |
| jsonb | Json | |
| uuid | Uuid | |

### SQLite → CanonicalType

| Tipo SQLite | Canonical Type | Notas |
|------------|---------------|-------|
| INTEGER | Int64 | SQLite usa 64 bits |
| REAL | Float64 | |
| TEXT | Text | |
| BLOB | Binary | |
| NUMERIC | Decimal(38,10) | Precisión predeterminada |

## Reglas de normalización

### Normalización de identificadores

La función `normalize_identifier()` aplica las siguientes transformaciones:

1. **Trim whitespace**: Eliminar espacios al inicio/final
2. **Lowercase**: Convertir a minúsculas
3. **Collapse spaces**: Reemplazar múltiples espacios con un guion bajo
4. **Snake case**: Convertir camelCase/PascalCase a snake_case
5. **Remove accents**: Convertir caracteres acentuados a equivalentes ASCII (opcional, configurable)

**Ejemplos:**
- `"UserEmail"` → `"user_email"`
- `"  Product Name  "` → `"product_name"`
- `"firstName"` → `"first_name"`
- `"Customer ID"` → `"customer_id"`

### Normalización de tipos

Los nombres de tipos específicos de la fuente se mapean a tipos canónicos utilizando las tablas anteriores. La información del tipo original se preserva en el campo `metadata`.

## Estrategia de versionado

### Formato de versión IR

El IR utiliza versionado semántico: `MAJOR.MINOR.PATCH`

- **MAJOR**: Cambios incompatibles en las estructuras principales
- **MINOR**: Adiciones compatibles hacia atrás (nuevos campos, nuevos tipos)
- **PATCH**: Correcciones de errores, actualizaciones de documentación

### Compatibilidad hacia atrás

- Los nuevos campos opcionales pueden agregarse en versiones MINOR
- El HashMap `metadata` permite extensiones sin incrementos de versión
- Los analizadores deben ignorar campos desconocidos de manera elegante

### Verificaciones de versión

Al cargar IR desde JSON:
1. Analizar el campo `ir_version`
2. Verificar compatibilidad de versión MAJOR
3. Advertir sobre discrepancias en versiones MINOR/PATCH
4. Continuar si es compatible, fallar si no lo es

## Uso de metadata

El campo `metadata` en todas las estructuras es un `HashMap<String, Value>` que almacena:

1. **Información original**: Detalles específicos de la fuente (nombres originales, tipos)
2. **Atributos específicos de la fuente**: Índices, colaciones, restricciones no incluidas en MVP
3. **Extensiones**: Características futuras sin cambios en el schema
4. **Anotaciones**: Anotaciones de usuario o herramientas

**Claves de metadata comunes:**
- `original_name`: Identificador original antes de la normalización
- `original_type`: Cadena de tipo específica de la fuente
- `collation`: Colación de la base de datos (MySQL, PostgreSQL)
- `charset`: Conjunto de caracteres
- `auto_increment`: Información de auto-incremento/secuencia
- `comment`: Comentarios de campo/tabla

## Formato de serialización

El IR se serializa a JSON para:
- Depuración e inspección
- Pruebas con fixtures
- Persistencia y almacenamiento en caché
- Comunicación entre procesos

**Consideraciones del schema JSON:**
- El orden de los campos debe ser determinístico para pruebas de instantáneas
- Pretty-print para legibilidad humana en fixtures
- Formato compacto para uso en producción

## Puntos de extensión

### Adiciones futuras (Post-MVP)

1. **Relations**: Mapeos de claves foráneas, cardinalidad
2. **Indexes**: Definiciones y tipos de índices
3. **Views**: Entidades virtuales y sus definiciones
4. **Partitioning**: Información de particionamiento
5. **Triggers/Procedures**: Representación de lógica almacenada
6. **Statistics**: Recuentos de filas, estimaciones de cardinalidad
7. **Semantic annotations**: Mapeos de glosario de negocios

### Agregar nuevos tipos canónicos

Para agregar un nuevo tipo canónico:
1. Agregar variante enum a `CanonicalType`
2. Actualizar tablas de mapeo de tipos
3. Agregar pruebas para el nuevo tipo
4. Documentar en esta especificación
5. Incrementar versión MINOR

## Guías de implementación

### Para autores de adaptadores

Al crear un adaptador para convertir un schema de fuente a IR:

1. Extraer información de schema usando APIs específicas de la fuente
2. Normalizar nombres de entidades y campos usando `normalize_identifier()`
3. Mapear tipos de fuente a tipos canónicos usando tablas de mapeo
4. Preservar información original en campos `metadata`
5. Establecer valores apropiados de `nullable` y `constraints`
6. Validar la estructura IR resultante

### Para autores de motores de comparación

Al comparar dos schemas IR:

1. Emparejar entidades por nombres normalizados
2. Emparejar campos dentro de entidades por nombres normalizados
3. Comparar tipos canónicos para compatibilidad
4. Verificar nullability y restricciones
5. Usar metadata para desempate y reportes
6. Manejar reglas de compatibilidad de tipos (ej., Int32 ↔ Int64)

## Estrategia de pruebas

### Pruebas unitarias

- Normalización de identificadores (más de 20 casos de prueba)
- Mapeo de tipos para cada base de datos soportada
- Construcción y validación de IR
- Ciclos completos de serialización (IR → JSON → IR)

### Pruebas de integración

- Cargar fixtures y validar estructura
- Comparar schemas conocidos como compatibles
- Comparar schemas conocidos como incompatibles
- Verificar preservación de metadata

### Fixtures

Ubicados en `/fixtures/ir/`:
- `simple_a.json`: Schema básico con tipos comunes
- `simple_b.json`: Schema compatible con ligeras variaciones

## Ejemplos

### Ejemplo completo: Tabla simple de usuarios

**Fuente MySQL:**
```sql
CREATE TABLE Users (
  id INT PRIMARY KEY AUTO_INCREMENT,
  email VARCHAR(255) NOT NULL UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**Representación IR:**
```json
{
  "source_name": "myapp_db",
  "source_type": "mysql",
  "ir_version": "1.0.0",
  "entities": [
    {
      "entity_name": "users",
      "entity_type": "table",
      "fields": [
        {
          "field_name": "id",
          "canonical_type": "Int32",
          "nullable": false,
          "constraints": [],
          "metadata": {
            "original_name": "id",
            "original_type": "INT",
            "auto_increment": true
          }
        },
        {
          "field_name": "email",
          "canonical_type": "String",
          "nullable": false,
          "constraints": [
            { "constraint_type": "MaxLength", "value": 255 },
            { "constraint_type": "Unique" }
          ],
          "metadata": {
            "original_name": "email",
            "original_type": "VARCHAR(255)"
          }
        },
        {
          "field_name": "created_at",
          "canonical_type": "Timestamp",
          "nullable": false,
          "constraints": [],
          "metadata": {
            "original_name": "created_at",
            "original_type": "TIMESTAMP",
            "default": "CURRENT_TIMESTAMP"
          }
        }
      ],
      "keys": [
        {
          "key_type": "primary",
          "field_names": ["id"]
        }
      ],
      "metadata": {}
    }
  ],
  "metadata": {}
}
```

## Referencias

- [EPIC 02: Canonical Schema IR](../README.md)
- [Documentación de Serde](https://serde.rs/)
- [Especificación de JSON Schema](https://json-schema.org/)

---

**Estado del documento:** Completo (v1.0.0)  
**Próxima revisión:** Después de completar EPIC 02
