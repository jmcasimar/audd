# Adaptadores de Archivos Remotos

AUDD soporta la carga de schemas desde archivos remotos a través de URLs HTTP/HTTPS y Google Sheets públicos.

## Protocolos Soportados

- **URLs HTTP/HTTPS**: Descargas directas de archivos
- **Google Sheets**: Hojas de cálculo públicas (exportadas automáticamente como CSV)

## Formatos Soportados

Los archivos remotos soportan los mismos formatos que los archivos locales:
- CSV (`.csv`)
- JSON (`.json`)
- XML (`.xml`)
- SQL/DDL (`.sql`, `.ddl`)

## Formatos de Cadena de Conexión

### URLs HTTP/HTTPS

Utilice una URL estándar que apunte al archivo:

```bash
audd load --source "https://example.com/data.csv"
audd load --source "https://api.example.com/schema.json"
audd load --source "https://storage.example.com/schema.sql"
```

### Google Sheets (Públicos)

Utilice la URL de Google Sheets directamente. El archivo será exportado automáticamente como CSV:

```bash
audd load --source "https://docs.google.com/spreadsheets/d/SHEET_ID/edit"
audd load --source "https://docs.google.com/spreadsheets/d/SHEET_ID/edit#gid=0"
```

**Nota**: El Google Sheet debe ser públicamente accesible (compartido con "Cualquiera con el enlace puede ver").

## Ejemplos de Uso del CLI

### Carga Básica desde URL

```bash
# Cargar desde URL HTTP
audd load --source "https://example.com/employees.csv"

# Cargar desde URL HTTPS con parámetros de consulta
audd load --source "https://api.example.com/data.json?version=latest"

# Cargar desde Google Sheets
audd load --source "https://docs.google.com/spreadsheets/d/1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms/edit"
```

### Comparar Fuentes Remotas y Locales

```bash
# Comparar CSV remoto con base de datos local
audd compare \
  --source-a "https://example.com/prod-schema.csv" \
  --source-b "db:sqlite:///local.db"

# Comparar dos Google Sheets
audd compare \
  --source-a "https://docs.google.com/spreadsheets/d/SHEET_A/edit" \
  --source-b "https://docs.google.com/spreadsheets/d/SHEET_B/edit"
```

### Especificación Explícita de Formato

Si la URL no tiene una extensión de archivo clara, puede utilizar el API programático con una indicación explícita de formato:

```rust
use audd_adapters_file::RemoteAdapter;

let adapter = RemoteAdapter::with_format(
    "https://api.example.com/schema",
    "json"
);
let schema = adapter.load_schema()?;
```

## API Programático

### Uso Básico

```rust
use audd_adapters_file::{load_schema_from_url, AdapterError};

fn main() -> Result<(), AdapterError> {
    // Cargar desde URL HTTP
    let schema = load_schema_from_url("https://example.com/data.csv")?;
    println!("Loaded {} entities", schema.entities.len());
    
    // Cargar desde Google Sheets
    let sheet_url = "https://docs.google.com/spreadsheets/d/SHEET_ID/edit";
    let schema = load_schema_from_url(sheet_url)?;
    println!("Loaded from Google Sheets: {}", schema.source_name);
    
    Ok(())
}
```

### Con Indicación de Formato

```rust
use audd_adapters_file::load_schema_from_url_with_format;

// Cargar desde URL sin extensión clara
let schema = load_schema_from_url_with_format(
    "https://api.example.com/data",
    "json"
)?;
```

### Adapter Personalizado

```rust
use audd_adapters_file::RemoteAdapter;

// Crear adapter con auto-detección
let adapter = RemoteAdapter::new("https://example.com/data.csv");
let schema = adapter.load_schema()?;

// Crear adapter con formato explícito
let adapter = RemoteAdapter::with_format(
    "https://api.example.com/endpoint",
    "json"
);
let schema = adapter.load_schema()?;
```

## Detección de Formato

El adapter detecta automáticamente el formato de archivo utilizando:

1. **Indicación explícita de formato** (si se proporciona mediante `with_format()`)
2. **Detección de Google Sheets** (siempre tratado como CSV)
3. **Extensión de archivo de la URL** (por ejemplo, `.csv`, `.json`, `.xml`, `.sql`)

Si el formato no puede ser detectado, se devuelve un error con un mensaje útil.

## Manejo de Errores

Errores comunes y soluciones:

### Errores HTTP

```
❌ Error loading remote schema: Failed to fetch URL https://example.com/data.csv: HTTP error 404
```

**Solución**: Verifique que la URL sea correcta y accesible.

### Errores de Detección de Formato

```
❌ Error loading remote schema: Cannot detect format from URL: https://example.com/data
```

**Solución**: Utilice `load_schema_from_url_with_format()` con una indicación explícita de formato.

### Acceso Denegado (Google Sheets)

```
❌ Error loading remote schema: Failed to fetch URL: HTTP error 403
```

**Solución**: Asegúrese de que el Google Sheet sea públicamente accesible (compartido con "Cualquiera con el enlace puede ver").

## Consideraciones de Seguridad

### HTTPS Recomendado

Utilice siempre URLs HTTPS cuando sea posible para garantizar la integridad y privacidad de los datos:

```bash
# ✅ Bueno: HTTPS
audd load --source "https://example.com/data.csv"

# ⚠️  Advertencia: HTTP (inseguro)
audd load --source "http://example.com/data.csv"
```

### Solo Datos Públicos

Los adaptadores de archivos remotos están diseñados para datos públicamente accesibles:

- ✅ Google Sheets públicos
- ✅ Endpoints HTTP(S) públicos
- ✅ Portales de datos abiertos
- ❌ Endpoints privados/autenticados (no soportados en la versión actual)

### Privacidad de Datos

Al cargar desde URLs remotas:
- Los datos se descargan temporalmente a un archivo temporal local
- Los archivos temporales se limpian automáticamente después del procesamiento
- No se almacenan ni transmiten credenciales (excepto en la URL misma)

## Detalles de Implementación

### Proceso de Descarga

1. **Validación de URL**: Verifica si la URL es de Google Sheets o HTTP(S) estándar
2. **Detección de formato**: Determina el formato de archivo desde la URL o indicación
3. **Solicitud HTTP**: Descarga el contenido del archivo utilizando la biblioteca `ureq`
4. **Archivo temporal**: Escribe el contenido en un archivo temporal con la extensión apropiada
5. **Delegación de adapter**: Utiliza el adapter de archivo estándar (CSV, JSON, XML, SQL) para analizar
6. **Limpieza**: El archivo temporal se elimina automáticamente

### Exportación de Google Sheets

Para URLs de Google Sheets:
- Extrae el ID de la hoja de la URL
- Convierte a URL de exportación: `https://docs.google.com/spreadsheets/d/{SHEET_ID}/export?format=csv`
- Descarga como CSV
- Procesa utilizando el adapter CSV

### Formatos de URL de Sheet Soportados

Todos los formatos de URL estándar de Google Sheets son soportados:
```
https://docs.google.com/spreadsheets/d/SHEET_ID/edit
https://docs.google.com/spreadsheets/d/SHEET_ID/edit#gid=0
https://docs.google.com/spreadsheets/d/SHEET_ID/edit?usp=sharing
```

## Comparación con Conectores de Base de Datos

| Característica | Archivos Remotos | Conectores de Base de Datos |
|----------------|------------------|----------------------------|
| Acceso de Red | HTTP/HTTPS | Protocolos de base de datos |
| Autenticación | Basada en URL | Credenciales en cadena de conexión |
| Formato de Datos | Archivos (CSV, JSON, XML, SQL) | Schemas de base de datos |
| Conexión | Sin estado | Con estado |
| Rendimiento | Descarga + análisis | Consultas directas de schema |

## Mejoras Futuras

Adiciones potenciales futuras:
- Soporte de autenticación (API keys, OAuth)
- Acceso a Google Sheets privados
- Encabezados HTTP personalizados
- Reporte de progreso de descarga
- Mecanismo de caché
- Soporte FTP/SFTP

## Ejemplos

### Casos de Uso del Mundo Real

#### Portal de Datos Públicos

```bash
# Cargar schema desde datos abiertos gubernamentales
audd load --source "https://data.gov/catalog/dataset.csv"
```

#### Colaboración con Google Sheets

```bash
# El equipo utiliza Google Sheets para documentación de schema
audd compare \
  --source-a "https://docs.google.com/spreadsheets/d/PROD_SHEET/edit" \
  --source-b "db:postgres://localhost/production"
```

#### Exportación de Schema de API

```bash
# El API exporta el schema como JSON
audd load --source "https://api.example.com/v1/schema/export.json"
```

#### Comparación entre Formatos

```bash
# Comparar JSON remoto con archivo SQL local
audd compare \
  --source-a "https://example.com/schema.json" \
  --source-b "local-schema.sql"
```

## Pruebas

El adapter remoto incluye pruebas completas:

```bash
# Ejecutar todas las pruebas del adapter de archivos (incluyendo remotos)
cd crates/audd_adapters_file
cargo test

# Ejecutar pruebas específicas del adapter remoto
cargo test remote_adapter::tests
```

La cobertura de pruebas incluye:
- Detección y conversión de URLs de Google Sheets
- Detección de formato desde URLs
- Manejo de errores para formatos no soportados
- Casos extremos (URLs con parámetros de consulta, etc.)

## Dependencias

El soporte de archivos remotos requiere:
- `ureq`: Biblioteca cliente HTTP (síncrona, ligera)
- `tempfile`: Gestión de archivos temporales

Estas se incluyen automáticamente al utilizar el crate `audd_adapters_file`.
