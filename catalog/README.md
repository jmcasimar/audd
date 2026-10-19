# Catálogo de resoluciones AUDD

Este directorio contiene el catálogo de reglas de resolución y equivalencias de dominio que AUDD utiliza para sugerir cómo resolver conflictos entre esquemas.

## Estructura

```
catalog/
  resolutions.toml    # Reglas de resolución por defecto
  domains/
    pos.toml          # Equivalencias para punto de venta
```

## Uso

```bash
# Usar el catálogo por defecto (directorio catalog/ junto al ejecutable)
audd compare --source-a A --source-b B --out out

# Usar un catálogo personalizado
audd compare --source-a A --source-b B --out out --catalog ./mi_catalogo

# Activar un dominio de equivalencias
audd compare --source-a A --source-b B --out out --domain pos
```

## Formato de reglas

```toml
[[rule]]
id = "R-TYPE-001"
conflict_type = "TypeIncompatible"
when = { from = "Int32", to = "Int64" }
suggestion = "CastSafe"
params = { target = "Int64" }
confidence = 0.95
rationale = "Ampliación sin pérdida"
source = "default"
version = 1
```

## Prioridad

Las reglas del catálogo se evalúan en orden de aparición. La primera regla cuyo `conflict_type` y condiciones `when` coincidan con un conflicto determina la sugerencia. Si ninguna regla aplica, se usa el comportamiento por defecto del `SuggestionEngine`.

## Versionado

Cada regla tiene un `id` y una `version`. El reporte de comparación incluye la versión del catálogo usado.
