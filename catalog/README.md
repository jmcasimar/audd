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
# Usar el catálogo por defecto (directorio catalog/ del directorio de trabajo)
audd compare --source-a A --source-b B --out out

# Usar un catálogo personalizado
audd compare --source-a A --source-b B --out out --catalog ./mi_catalogo

# Activar un dominio de equivalencias
audd compare --source-a A --source-b B --out out --domain pos

# Continuar sin catálogo si tiene un error de formato (muestra una advertencia)
audd compare --source-a A --source-b B --out out --catalog ./mi_catalogo --catalog-optional

# Convertir las decisiones aceptadas en reglas candidatas (source = "learned"), para revisión
audd catalog learn --from out/decision_log.json --out reglas_candidatas.toml
```

Las reglas aprendidas nunca se agregan solas al catálogo: hay que revisarlas y copiarlas a mano.

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

En las reglas de tipo, `from` se compara con el tipo de A y `to` con el de B, por nombre
exacto (los parámetros como `Decimal(10, 2)` se ignoran). `symmetric = true` hace que la
regla también aplique con A y B intercambiados, y `|` separa alternativas
(`to = "DateTime|Timestamp"`).

## Prioridad

Las reglas del catálogo se evalúan en orden de aparición. La primera regla cuyo `conflict_type` y condiciones `when` coincidan con un conflicto determina la sugerencia. Si ninguna regla aplica, se usa el comportamiento por defecto del `SuggestionEngine`.

## Versionado

Cada regla tiene un `id` y una `version`, y `resolutions.toml` declara la versión del catálogo
(`version = "1.0.0"`). `report.md` indica la versión usada y `decision_log.json` guarda, en
`metadata.custom.catalog`, la ruta, la versión y la versión de cada regla.

## Trazabilidad

Cada sugerencia indica su origen en el campo `origin` de `suggestions.json` y en la evidencia
y los metadatos de cada decisión de `decision_log.json`: `catalog:<id de regla>` o
`engine:default`.
