# Documentación de Adapters de Base de Datos

## Descripción General

AUDD proporciona adapters de base de datos que le permiten extraer metadatos de schema directamente desde bases de datos y convertirlos a la Representación Intermedia (IR) de AUDD. Esto habilita la comparación de schemas entre bases de datos y otras fuentes de datos.

## Motores de Base de Datos Soportados

### Soporte Actual

- **SQLite** - Soporte completo para extracción de schema incluyendo foreign keys, indexes, views y triggers
- **MySQL/MariaDB** - Soporte completo para extracción de schema incluyendo foreign keys, indexes, views, stored procedures y triggers
- **PostgreSQL** - Soporte completo para extracción de schema incluyendo foreign keys, indexes, views, stored procedures y triggers
- **MongoDB** - Soporte completo con inferencia de schema desde muestreo de documentos, incluyendo indexes y views
- **Microsoft SQL Server** - Soporte completo para extracción de schema incluyendo foreign keys, indexes, views, stored procedures y triggers (bandera de característica opcional)

## Formatos de Connection String

### SQLite

```
sqlite:///absolute/path/to/database.db
sqlite://relative/path/to/database.db
```

**Ejemplos:**
```bash
# Ruta absoluta
sqlite:///var/lib/data/myapp.db

# Ruta relativa
sqlite://data/myapp.db

# Ruta absoluta en Windows
sqlite:///C:/Data/myapp.db
```

### MySQL/MariaDB

```
mysql://user:password@host:port/database
mysql://user:password@host/database  # Port defaults to 3306
```

**Ejemplos:**
```bash
# Con puerto explícito
mysql://admin:secret@localhost:3306/myapp_db

# Puerto por defecto (3306)
mysql://admin:secret@localhost/myapp_db

# Host remoto
mysql://user:pass@db.example.com/production_db

# MariaDB (usa el mismo formato de conexión)
mysql://user:pass@mariadb-server/myapp_db
```

**Nota:** Los connection strings de MariaDB usan el prefijo `mysql://` ya que son compatibles.

### PostgreSQL

```
postgres://user:password@host:port/database
postgresql://user:password@host:port/database  # Alias
postgres://user:password@host/database  # Port defaults to 5432
```

**Ejemplos:**
```bash
# Con puerto explícito
postgres://admin:secret@localhost:5432/myapp_db

# Puerto por defecto (5432)
postgres://user:pass@localhost/myapp_db

# Servidor PostgreSQL remoto
postgres://dbuser:dbpass@pg.example.com/production_db
```

### MongoDB

```
mongodb://host:port/database
mongodb://user:password@host:port/database
mongodb+srv://cluster/database  # MongoDB Atlas
```

**Ejemplos:**
```bash
# MongoDB local
mongodb://localhost:27017/myapp_db

# Con autenticación
mongodb://admin:secret@localhost:27017/myapp_db

# Cluster de MongoDB Atlas
mongodb+srv://cluster0.mongodb.net/production_db

# Con opciones de conexión
mongodb://localhost:27017/mydb?retryWrites=true&w=majority
```

**Nota:** El schema de MongoDB se infiere mediante muestreo de documentos (por defecto: 100 documentos por colección).

### Microsoft SQL Server

```
sqlserver://user:password@host:port/database
mssql://user:password@host:port/database  # Alias (normalized to sqlserver)
sqlserver://user:password@host/database  # Port defaults to 1433
```

**Ejemplos:**
```bash
# Con puerto explícito
sqlserver://sa:YourPassword@localhost:1433/myapp_db

# Puerto por defecto (1433)
sqlserver://user:pass@localhost/myapp_db

# SQL Server remoto
sqlserver://dbuser:dbpass@sqlserver.example.com/production_db

# Usando el prefijo mssql (normalizado a sqlserver)
mssql://sa:pass@localhost/myapp_db
```

**Nota:** El connector de SQL Server requiere que la bandera de característica `sqlserver` esté habilitada. No está incluida en las características por defecto.

## Uso del CLI

### Cargar Schema desde Base de Datos

#### Ejemplo SQLite

```bash
# Cargar schema desde base de datos SQLite
audd load --source "db:sqlite:///path/to/database.db"

# Especificar formato de salida
audd load --source "db:sqlite:///data/app.db" --format json
```

#### Ejemplo MySQL

```bash
# Cargar schema desde base de datos MySQL
audd load --source "db:mysql://user:password@localhost/mydb"

# Con puerto explícito
audd load --source "db:mysql://admin:secret@localhost:3306/myapp"
```

#### Ejemplo PostgreSQL

```bash
# Cargar schema desde base de datos PostgreSQL
audd load --source "db:postgres://user:password@localhost/mydb"

# Con puerto explícito
audd load --source "db:postgres://admin:secret@localhost:5432/myapp"
```

#### Ejemplo MongoDB

```bash
# Cargar schema desde base de datos MongoDB
audd load --source "db:mongodb://localhost:27017/mydb"

# MongoDB Atlas
audd load --source "db:mongodb+srv://cluster0.mongodb.net/production"
```

#### Ejemplo SQL Server

```bash
# Cargar schema desde base de datos SQL Server
audd load --source "db:sqlserver://sa:password@localhost/mydb"

# Con puerto explícito
audd load --source "db:sqlserver://user:pass@localhost:1433/myapp"

# Usando el prefijo mssql
audd load --source "db:mssql://sa:password@server/mydb"
```

**Nota:** El soporte de SQL Server requiere habilitar la bandera de característica `sqlserver` al compilar AUDD.

#### Formato Legacy (con bandera --conn separada)

```bash
# SQLite con parámetro de conexión separado
audd load --source db:sqlite --conn /path/to/database.db

# MySQL con parámetro de conexión separado
audd load --source db:mysql --conn user:password@localhost/mydb

# PostgreSQL con parámetro de conexión separado
audd load --source db:postgres --conn user:password@localhost:5432/mydb

# MongoDB con parámetro de conexión separado
audd load --source db:mongodb --conn localhost:27017/mydb
```

### Comparar Schemas de Diferentes Fuentes

Puede comparar schemas de diferentes motores de base de datos o entre bases de datos y archivos:

```bash
# Comparar SQLite con PostgreSQL
audd compare \
  --source-a "db:sqlite:///local/app.db" \
  --source-b "db:postgres://user:pass@remote.com/prod_db"

# Comparar MongoDB con MySQL
audd compare \
  --source-a "db:mongodb://localhost:27017/development" \
  --source-b "db:mysql://user:pass@staging/myapp"

# Comparar base de datos con archivo CSV
audd compare \
  --source-a "db:postgres://user:pass@localhost/current" \
  --source-b "schema.csv"

# Comparar dos bases de datos PostgreSQL
audd compare \
  --source-a "db:postgres://user:pass@staging:5432/myapp" \
  --source-b "db:postgres://user:pass@production:5432/myapp"

# Comparar SQL Server con MySQL
audd compare \
  --source-a "db:sqlserver://sa:pass@localhost/devdb" \
  --source-b "db:mysql://user:pass@production/myapp"

# Comparar SQL Server desarrollo con producción
audd compare \
  --source-a "db:sqlserver://user:pass@dev-server/myapp" \
  --source-b "db:sqlserver://user:pass@prod-server/myapp"
```

## Detalles de Extracción de Schema

### SQLite

El adapter de SQLite extrae los siguientes metadatos:

- **Tables**: Todas las tablas de usuario (excluyendo tablas del sistema sqlite_*)
- **Columns**: Nombre, tipo, nulabilidad
- **Primary Keys**: Primary keys simples y compuestas
- **Unique Constraints**: Indexes únicos (excluyendo los autogenerados)

**Mapeo de Tipos:**
- INTEGER → Int64
- TEXT, CLOB → Text
- VARCHAR, CHAR → String
- BLOB → Binary
- REAL, FLOAT, DOUBLE → Float64
- NUMERIC, DECIMAL → Decimal(10,2)
- DATE → Date
- DATETIME, TIMESTAMP → DateTime
- BOOLEAN → Boolean

### MySQL/MariaDB

El adapter de MySQL extrae los siguientes metadatos:

- **Tables**: Todas las tablas base en la base de datos especificada
- **Columns**: Nombre, tipo, nulabilidad, valores por defecto
- **Primary Keys**: Primary keys simples y compuestas
- **Unique Constraints**: Indexes únicos
- **Foreign Keys**: Relaciones de foreign key con tablas/columnas referenciadas
- **Indexes**: Indexes regulares, full-text y spatial
- **Views**: Nombres de views y definiciones SQL
- **Stored Procedures**: Procedimientos y funciones con tipos de retorno y definiciones
- **Triggers**: Triggers de base de datos con timing, eventos, asociaciones de tablas y definiciones

**Mapeo de Tipos:**
- TINYINT, SMALLINT, MEDIUMINT, INT → Int32
- BIGINT → Int64
- FLOAT → Float32
- DOUBLE, REAL → Float64
- DECIMAL, NUMERIC → Decimal (with precision/scale)
- CHAR, VARCHAR → String
- TEXT, MEDIUMTEXT, LONGTEXT → Text
- BLOB, BINARY, VARBINARY → Binary
- DATE → Date
- TIME → Time
- DATETIME → DateTime
- TIMESTAMP → Timestamp
- JSON → Json
- TINYINT(1) → Boolean

**Características Avanzadas:**
- Foreign keys extraídas de `INFORMATION_SCHEMA.KEY_COLUMN_USAGE`
- Indexes regulares (no únicos) de `INFORMATION_SCHEMA.STATISTICS`
- Indexes full-text (tipo FULLTEXT)
- Indexes spatial (tipo SPATIAL)
- Views de `INFORMATION_SCHEMA.VIEWS`
- Stored procedures y funciones de `INFORMATION_SCHEMA.ROUTINES`
- Triggers de `INFORMATION_SCHEMA.TRIGGERS`

### PostgreSQL

El adapter de PostgreSQL extrae los siguientes metadatos:

- **Tables**: Todas las tablas base en el schema público
- **Columns**: Nombre, tipo, nulabilidad, precision/scale
- **Primary Keys**: Primary keys simples y compuestas
- **Unique Constraints**: Restricciones únicas
- **Foreign Keys**: Relaciones de foreign key con tablas/columnas referenciadas
- **Indexes**: Indexes regulares, únicos, parciales (filtrados), GIN, GIST
- **Views**: Views regulares y views materializadas con definiciones SQL
- **Stored Procedures**: Funciones y procedimientos con tipos de retorno y definiciones
- **Triggers**: Triggers de base de datos con timing, eventos y definiciones

**Mapeo de Tipos:**
- SMALLINT, INTEGER → Int32
- BIGINT → Int64
- SMALLSERIAL, SERIAL → Int32
- BIGSERIAL → Int64
- REAL → Float32
- DOUBLE PRECISION → Float64
- NUMERIC, DECIMAL → Decimal (with precision/scale)
- MONEY → Decimal(19,2)
- CHARACTER, CHARACTER VARYING, VARCHAR → String
- TEXT → Text
- BYTEA → Binary
- BOOLEAN → Boolean
- DATE → Date
- TIME → Time
- TIMESTAMP → DateTime
- TIMESTAMP WITH TIME ZONE → Timestamp
- JSON, JSONB → Json
- UUID → Uuid
- ARRAY → Unknown (preserves element type info)
- User-defined types → Unknown (preserves original type name)

**Características Avanzadas:**
- Indexes parciales/filtrados con condiciones WHERE
- Views materializadas (marcadas con bandera `is_materialized`)
- Indexes GIN y GIST (mapeados al tipo FullText)
- Operaciones asíncronas usando tokio runtime

### MongoDB

El adapter de MongoDB infiere el schema mediante muestreo de documentos:

- **Collections**: Todas las colecciones en la base de datos
- **Fields**: Detectados desde documentos muestreados (por defecto: 100 por colección)
- **Types**: Inferidos desde tipos BSON en documentos
- **Primary Key**: Detección automática de _id
- **Nullable**: Inferido basado en la presencia de valores null

**Comportamiento de Muestreo:**
- Tamaño de muestra por defecto: 100 documentos por colección
- Configurable vía API
- Fields presentes en < 100% de documentos marcados como nullable
- Tipos mixtos reportados como Unknown con lista de tipos

**Mapeo de Tipos:**
- Int32, Int64 → Int32, Int64
- Double → Float64
- Decimal128 → Decimal(34,0)
- String → String
- Boolean → Boolean
- Binary → Binary
- DateTime → DateTime
- Timestamp → Timestamp
- ObjectId → String
- Nested documents/arrays → Json
- Mixed types → Unknown (with type list)

**Características Avanzadas:**
- Indexes extraídos del comando `listIndexes()`
  - Indexes de campo simple y compuestos
  - Indexes text (búsqueda full-text)
  - Indexes 2dsphere (geoespaciales/spatial)
  - Indexes hashed
  - Indexes únicos
  - Indexes parciales/filtrados con expresiones de filtro
- Views extraídas de `listCollections()` 
  - Views de pipeline de agregación
  - Definiciones de views como pipelines formateados
- Operaciones asíncronas usando tokio runtime

**Nota:** Los validadores de MongoDB (JSON Schema y validadores de query) no se extraen actualmente pero podrían agregarse en una mejora futura.

### Microsoft SQL Server

El adapter de SQL Server extrae los siguientes metadatos:

- **Tables**: Todas las tablas de usuario del schema dbo
- **Columns**: Nombre, tipo, nulabilidad, valores por defecto
- **Primary Keys**: Primary keys simples y compuestas
- **Foreign Keys**: Con información de tabla y columna referenciadas
- **Indexes**: Indexes regulares, únicos, full-text, spatial y filtrados
- **Views**: Definiciones de views desde INFORMATION_SCHEMA
- **Stored Procedures**: Funciones y procedimientos con definiciones
- **Triggers**: Timing, eventos y definiciones SQL

**Mapeo de Tipos:**
- BIT → Boolean
- TINYINT, SMALLINT, INT → Int32
- BIGINT → Int64
- DECIMAL(p,s), NUMERIC(p,s) → Decimal{p,s}
- MONEY, SMALLMONEY → Decimal{19,4}
- REAL → Float32
- FLOAT → Float64
- CHAR, VARCHAR, NCHAR, NVARCHAR → String
- VARCHAR(MAX), TEXT, NTEXT → Text
- BINARY, VARBINARY, IMAGE → Binary
- DATE → Date
- TIME → Time
- DATETIME, DATETIME2, SMALLDATETIME, DATETIMEOFFSET → DateTime
- UNIQUEIDENTIFIER → Uuid
- JSON, XML → Json
- GEOGRAPHY, GEOMETRY → Unknown (with spatial type info)

**Características Avanzadas:**
- Foreign keys extraídas de `sys.foreign_keys` y `sys.foreign_key_columns`
  - Soporta foreign keys compuestas
  - Metadatos de tabla y columna referenciadas
- Indexes extraídos de `sys.indexes` y `sys.index_columns`
  - Indexes regulares (no únicos)
  - Indexes únicos  
  - Indexes full-text
  - Indexes spatial
  - Indexes filtrados con definiciones de filtro
  - Excluye indexes de primary key y unique constraint
- Views extraídas de `INFORMATION_SCHEMA.VIEWS`
  - Nombres de views y definiciones SQL
- Stored procedures de `INFORMATION_SCHEMA.ROUTINES`
  - Funciones y procedimientos
  - Tipos de rutina y tipos de retorno
  - Definiciones SQL
- Triggers de `sys.triggers`
  - Timing BEFORE/AFTER/INSTEAD OF
  - Eventos INSERT/UPDATE/DELETE
  - Definiciones SQL
- Operaciones asíncronas usando tiberius y tokio runtime

**Nota:** El soporte de SQL Server requiere habilitar la bandera de característica `sqlserver` al compilar AUDD. Agregue `features = ["sqlserver"]` a su Cargo.toml o use `--features sqlserver` al compilar.

## Manejo de Errores

Los adapters de base de datos proporcionan mensajes de error claros para problemas comunes:

### Errores de Conexión

```
❌ Error loading schema: Failed to create database connector: 
   Database connection error: Failed to open SQLite database: unable to open database file
```

**Causas comunes:**
- El archivo de base de datos no existe (SQLite)
- Credenciales incorrectas (MySQL, PostgreSQL, MongoDB)
- Servidor de base de datos no está ejecutándose (MySQL, PostgreSQL, MongoDB)
- Problemas de red (todas las bases de datos en red)

**Soluciones:**
- Verifique la ruta/connection string de la base de datos
- Revise los permisos de la base de datos
- Asegúrese de que el servidor de base de datos esté ejecutándose
- Verifique la conectividad de red

### Connection String Inválido

```
❌ Error loading schema: Failed to create database connector: 
   Invalid connection string: Missing database name. 
   Expected format: sqlite://<path>, mysql://<user>:<pass>@<host>/<db>, 
   postgres://<user>:<pass>@<host>/<db>, or mongodb://<host>/<db>
```

**Solución:** Verifique que el formato del connection string coincida con los patrones documentados.

### Motor No Soportado

```
❌ Error loading schema: Failed to create database connector: 
   Unsupported database engine: oracle (Supported: sqlite, mysql, postgres, mongodb, sqlserver)
```

**Solución:** Use un motor de base de datos soportado.

## Características y Limitaciones

### Características Actuales

✅ Extraer schemas de tablas/colecciones  
✅ Tipos de columnas, nulabilidad  
✅ Primary keys (simples y compuestas)  
✅ Restricciones únicas  
✅ Mapeo de tipos a tipos canónicos IR  
✅ Manejo de errores con mensajes útiles  
✅ Inferencia de schema para MongoDB  
✅ Soporte completo de PostgreSQL  
✅ Muestreo de documentos MongoDB  

### Limitaciones

❌ Relaciones de foreign key (planificado)  
❌ Indexes (no únicos)  
❌ Views  
❌ Stored procedures  
❌ Triggers  
❌ Restricciones complejas (CHECK, etc.)  
❌ Validadores y JSON schema de MongoDB  

### Mejoras Futuras

- Extracción de foreign keys
- Análisis de indexes
- Metadatos de views
- Detección avanzada de restricciones
- Connection pooling para bases de datos en red
- Opciones de conexión SSL/TLS
- Extracción de reglas de validación de MongoDB

## Rendimiento

### Velocidad de Extracción

La extracción de metadatos está optimizada para velocidad:

- **SQLite**: Usa queries PRAGMA eficientes
- **MySQL**: Usa INFORMATION_SCHEMA con queries indexadas
- **PostgreSQL**: Usa information_schema y pg_catalog
- **MongoDB**: Muestreo de documentos configurable (por defecto: 100 docs)

**Objetivo:** < 2 segundos para bases de datos con hasta 100 tablas/colecciones

### Mejores Prácticas

1. **Use conexiones de base de datos de solo lectura** cuando sea posible
2. **Evite consultar durante horas pico** para bases de datos de producción
3. **Considere views de base de datos** para limitar el schema expuesto
4. **Pruebe los connection strings** con un comando de carga simple primero

## Ejemplos

### Ejemplo de Flujo de Trabajo Completo

```bash
# 1. Cargar schema desde base de datos SQLite de desarrollo
audd load --source "db:sqlite:///dev/app.db" > dev_schema.json

# 2. Cargar schema desde base de datos MySQL de staging
audd load --source "db:mysql://readonly:pass@staging.db/app" > staging_schema.json

# 3. Comparar schemas
audd compare \
  --source-a "db:sqlite:///dev/app.db" \
  --source-b "db:mysql://readonly:pass@staging.db/app"

# 4. Exportar a JSON para control de versiones
audd load --source "db:sqlite:///dev/app.db" --format json > schema_v1.0.json
```

### Integración con File Adapters

```bash
# Extraer schema desde base de datos y guardar como SQL DDL
audd load --source "db:sqlite:///data/app.db" > current_schema.json

# Comparar con schema SQL histórico
audd compare \
  --source-a "db:sqlite:///data/app.db" \
  --source-b "migrations/v1.0_schema.sql"
```

## Consideraciones de Seguridad

1. **Nunca codifique contraseñas en duro** en scripts o historial de comandos
2. **Use variables de entorno** para credenciales sensibles:
   ```bash
   export DB_USER="admin"
   export DB_PASS="secret"
   audd load --source "db:mysql://$DB_USER:$DB_PASS@localhost/mydb"
   ```
3. **Use usuarios de base de datos de solo lectura** para extracción de schema
4. **Evite registrar connection strings** que contengan contraseñas
5. **Considere usar túneles SSH** para conexiones de base de datos remotas

## Solución de Problemas

### Problemas de SQLite

**Problema:** "unable to open database file"
- Verifique que la ruta del archivo sea correcta
- Verifique los permisos del archivo
- Asegúrese de que el directorio padre exista

**Problema:** "database is locked"
- Cierre otras conexiones a la base de datos
- Use el modo WAL para SQLite si se necesita acceso concurrente

### Problemas de MySQL

**Problema:** "Access denied for user"
- Verifique el nombre de usuario y contraseña
- Revise que el usuario tenga permisos SELECT en INFORMATION_SCHEMA
- Asegúrese de que el usuario tenga acceso desde su host

**Problema:** "Can't connect to MySQL server"
- Verifique que el servidor MySQL esté ejecutándose
- Revise las reglas del firewall
- Verifique que el host y puerto sean correctos

**Problema:** "Unknown database"
- Asegúrese de que el nombre de la base de datos esté escrito correctamente
- Verifique que la base de datos exista
- Revise que el usuario tenga acceso a la base de datos

### Problemas de PostgreSQL

**Problema:** "connection refused"
- Verifique que el servidor PostgreSQL esté ejecutándose
- Revise que PostgreSQL esté escuchando en el host/puerto correcto
- Verifique que pg_hba.conf permita conexiones desde su host
- Revise las reglas del firewall

**Problema:** "authentication failed"
- Verifique el nombre de usuario y contraseña
- Revise el método de autenticación de PostgreSQL en pg_hba.conf
- Asegúrese de que el usuario tenga permisos SELECT en information_schema

**Problema:** "database does not exist"
- Verifique que el nombre de la base de datos esté escrito correctamente
- Revise que el usuario tenga privilegio CONNECT en la base de datos
- Asegúrese de que la base de datos exista usando psql

### Problemas de MongoDB

**Problema:** "connection timed out"
- Verifique que el servidor MongoDB esté ejecutándose
- Revise que MongoDB esté escuchando en el host/puerto correcto
- Verifique que las reglas del firewall permitan conexiones
- Revise la conectividad de red

**Problema:** "authentication failed"
- Verifique el nombre de usuario y contraseña
- Revise que el usuario tenga permisos de lectura en la base de datos
- Asegúrese de que la base de datos de autenticación sea correcta

**Problema:** "no collections found"
- Verifique que el nombre de la base de datos sea correcto
- Revise que existan colecciones en la base de datos
- Asegúrese de que el usuario tenga permiso list_collections

**Problema:** "schema appears incomplete"
- Incremente el tamaño de la muestra (MongoDB usa muestreo)
- Algunos fields pueden no aparecer en todos los documentos
- Considere muestrear más documentos para mejor cobertura

## Uso de API

Para uso programático en código Rust:

```rust
use audd_adapters_db::{create_connector, DbSchemaConnector};

// SQLite
let connector = create_connector("sqlite:///path/to/db.sqlite")?;
let schema = connector.load()?;

// MySQL
let connector = create_connector("mysql://user:pass@localhost/mydb")?;
let schema = connector.load()?;

// PostgreSQL
let connector = create_connector("postgres://user:pass@localhost:5432/mydb")?;
let schema = connector.load()?;

// MongoDB
let connector = create_connector("mongodb://localhost:27017/mydb")?;
let schema = connector.load()?;
```

Consulte la documentación del crate para más detalles sobre la API.

## Soporte y Retroalimentación

Para problemas, solicitudes de características o preguntas:
- Registre un issue en GitHub
- Revise la documentación existente
- Revise los mensajes de error cuidadosamente

## Historial de Versiones

- **v0.1.0** - Soporte de SQLite y MySQL/MariaDB
- **v0.2.0** - Soporte de PostgreSQL y MongoDB (inferencia de schema)
- Futuro: Foreign keys, views, restricciones avanzadas
