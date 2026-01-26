# Fixtures de Adaptadores de Archivo

Este directorio contiene archivos de muestra para probar y demostrar los adaptadores de archivo de AUDD.

## Archivos

### users.csv
Archivo CSV de muestra con datos típicos de usuario (5 columnas).

**Uso:**
```bash
audd load --source fixtures/adapters/users.csv
```

### users.json
Archivo JSON de muestra con un array de objetos de usuario. Demuestra inferencia de tipos.

**Uso:**
```bash
audd load --source fixtures/adapters/users.json
```

### users.xml
Archivo XML de muestra con registros de usuario. Muestra extracción básica de etiquetas.

**Uso:**
```bash
audd load --source fixtures/adapters/users.xml
```

### schema.sql
DDL SQL de muestra con dos tablas (users y posts). Demuestra:
- Restricciones PRIMARY KEY
- Restricciones NOT NULL
- Restricciones UNIQUE
- Mapeos de tipos

**Uso:**
```bash
audd load --source fixtures/adapters/schema.sql
```

## Pruebas

Estos fixtures son usados por las pruebas de integración en `crates/audd_adapters_file/tests/`.

Ejecuta las pruebas con:
```bash
cargo test -p audd_adapters_file
```
