# Soporte de Archivos de Configuración

AUDD CLI soporta archivos de configuración para personalizar el comportamiento sin necesidad de pasar banderas cada vez.

## Ubicaciones de Archivos de Configuración

AUDD buscará automáticamente archivos de configuración en las siguientes ubicaciones (en orden):

1. `./audd.toml` - Directorio actual
2. `~/.audd.toml` - Directorio home
3. `~/.config/audd/config.toml` - Directorio de configuración XDG

Alternativamente, puede especificar una ruta personalizada del archivo de configuración:

```bash
audd --config /ruta/a/config/personalizado.toml compare ...
```

## Generar una Configuración de Muestra

Para generar un archivo de configuración de muestra:

```bash
# Generar en el directorio actual
audd generate-config

# Generar en una ubicación personalizada
audd generate-config --out ~/.audd.toml
```

Esto crea un archivo TOML con todas las opciones disponibles y sus valores por defecto.

## Opciones de Configuración

### Sección [compare]

```toml
[compare]
# Umbral de similitud para emparejar entidades/fields (0.0 a 1.0)
similarity_threshold = 0.8

# Directorio de salida por defecto para resultados de comparación
default_output_dir = "output"
```

### Sección [resolution]

```toml
[resolution]
# Umbral de confianza para auto-aceptar sugerencias (0.0 a 1.0)
# Solo las sugerencias con confianza >= a este valor serán auto-aceptadas
confidence_threshold = 0.9

# Prefijo para IDs de decisión (útil para rastrear diferentes ejecuciones)
decision_id_prefix = "auto_dec"

# Si se permiten sugerencias riesgosas (ej., conversiones de tipo con pérdida)
allow_risky_suggestions = false
```

### Sección [output]

```toml
[output]
# Controlar qué archivos de salida se generan
generate_unified_schema = true
generate_diff = true
generate_decision_log = true
generate_report = true
```

## Reglas de Precedencia

Cuando la misma configuración se especifica en múltiples lugares, se aplica la siguiente precedencia:

1. **Banderas CLI** (prioridad más alta)
2. **Archivo de configuración** (especificado con `--config` o auto-cargado)
3. **Valores por defecto** (prioridad más baja)

### Ejemplo

Si tiene un archivo de configuración con:
```toml
[resolution]
confidence_threshold = 0.85
```

Y ejecuta:
```bash
audd compare --confidence-threshold 0.95 ...
```

Se usará el valor de la bandera CLI `0.95`, sobrescribiendo el valor del archivo de configuración.

## Ejemplo Completo

### 1. Generar archivo de configuración

```bash
audd generate-config --out ~/.audd.toml
```

### 2. Editar la configuración

```toml
[compare]
default_output_dir = "/var/audd/output"

[resolution]
confidence_threshold = 0.85
decision_id_prefix = "prod_dec"
allow_risky_suggestions = false

[output]
generate_unified_schema = true
generate_diff = true
generate_decision_log = true
generate_report = true
```

### 3. Usar la configuración

```bash
# La configuración se carga automáticamente desde ~/.audd.toml
audd compare --source-a data1.csv --source-b data2.json

# La salida va a /var/audd/output (de la configuración)
# Las decisiones usan IDs "prod_dec_*" (de la configuración)
# El umbral de confianza es 0.85 (de la configuración)
```

### 4. Sobrescribir configuraciones específicas

```bash
# Usar un umbral de confianza diferente para esta ejecución
audd compare \
  --source-a data1.csv \
  --source-b data2.json \
  --confidence-threshold 0.95

# Sobrescribir directorio de salida para esta ejecución
audd compare \
  --source-a data1.csv \
  --source-b data2.json \
  --out /tmp/comparison
```

## Casos de Uso

### Desarrollo vs Producción

**Configuración de desarrollo** (`audd-dev.toml`):
```toml
[resolution]
confidence_threshold = 0.75  # Más agresivo
allow_risky_suggestions = true

[output]
generate_report = false  # Omitir reportes en desarrollo
```

**Configuración de producción** (`audd-prod.toml`):
```toml
[resolution]
confidence_threshold = 0.9  # Conservador
allow_risky_suggestions = false

[output]
generate_report = true  # Siempre generar reportes
```

Uso:
```bash
# Desarrollo
audd --config audd-dev.toml compare ...

# Producción
audd --config audd-prod.toml compare ...
```

### Estandarización del Equipo

Coloque un `audd.toml` compartido en su repositorio del proyecto:

```toml
# Configuración estándar del proyecto
[compare]
default_output_dir = "schema_comparison"

[resolution]
confidence_threshold = 0.88
decision_id_prefix = "team_dec"
```

Todos en el equipo obtienen un comportamiento consistente sin memorizar banderas.

## Solución de Problemas

### Archivo de configuración no se está cargando

Verifique que:
1. El archivo existe en una de las ubicaciones de búsqueda
2. El archivo tiene sintaxis TOML válida
3. Los permisos del archivo permiten lectura

Para verificar qué configuración se está usando, puede revisar el comportamiento:
```bash
# Sin configuración: usa el directorio de salida por defecto "output"
# Con configuración: usa el directorio de salida configurado
audd compare --source-a a.csv --source-b b.json
```

### Configuración inválida

Si el archivo de configuración tiene sintaxis TOML inválida, verá un error:
```
❌ Error loading config file: Failed to parse configuration file: ...
```

Corrija el error de sintaxis e intente nuevamente.

### Probar configuración

Use el comando `generate-config` para ver el formato correcto:
```bash
audd generate-config --out /tmp/reference.toml
cat /tmp/reference.toml
```

## Referencia de Schema

Para el schema TOML completo, vea el archivo de configuración de muestra generado o el código fuente en `crates/audd-cli/src/config.rs`.
