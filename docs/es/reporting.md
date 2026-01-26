# Estructura de Reportes de AUDD

**Versión:** 1.0.0  
**Estado:** Estable

## Descripción General

Este documento define la estructura y formato de los reportes de comparación de AUDD. Los reportes se generan en formato Markdown (con exportación opcional a JSON) y consisten en dos secciones principales:

1. **Reporte Ejecutivo** - Resumen de alto nivel para toma de decisiones rápida
2. **Reporte Técnico** - Evidencia detallada y trazabilidad para auditoría

## Secciones del Reporte

### 1. Resumen Ejecutivo

Vista general rápida de los resultados de comparación con métricas e indicadores clave.

#### 1.1 Encabezado
- Título del reporte
- Marca de tiempo de generación
- Identificadores de schema (A y B)
- Versión del reporte

#### 1.2 Panorama de Compatibilidad
Métricas de alto nivel:
- **Total de Coincidencias**: Número de fields/entities compatibles
- **Exclusivos Seguros**: Fields que pueden agregarse de manera segura al schema unificado
- **Total de Conflictos**: Número de incompatibilidades que requieren resolución
- **Puntuación de Compatibilidad**: Porcentaje de coincidencias exitosas

**Fórmula:**
```
Compatibility Score = (Matches / (Matches + Conflicts)) × 100%
Safe Addition Rate = (Safe Exclusives / Total Exclusives) × 100%
Conflict Rate = (Conflicts / (Matches + Conflicts)) × 100%
```

#### 1.3 Evaluación de Riesgos
- **Nivel de Riesgo General**: Low | Medium | High | Critical
  - Low: < 10% conflict rate, sin conflictos de severidad critical
  - Medium: 10-25% conflict rate O tiene conflictos de severidad high
  - High: 25-50% conflict rate O tiene conflictos de severidad critical
  - Critical: > 50% conflict rate O múltiples conflictos critical

#### 1.4 Principales Conflictos por Entity
Tabla mostrando entities con más conflictos (top 5 o todos si son menos):
- Nombre del entity
- Número de conflictos
- Nivel de severidad más alto
- Estado de resolución (si existen sugerencias)

### 2. Desglose Detallado

Estadísticas desglosadas por categoría.

#### 2.1 Coincidencias
- Conteo total
- Desglose por razón de coincidencia (exact, normalized, similarity)
- Puntuación promedio de confianza de coincidencias

#### 2.2 Exclusivos
- Conteo total del Schema A
- Conteo total del Schema B
- Conteo marcados como seguros para agregar
- Conteo que requiere revisión

#### 2.3 Conflictos
- Conteo total
- Desglose por tipo de conflicto (type_incompatible, nullability_mismatch, etc.)
- Desglose por severidad (low, medium, high, critical)

### 3. Detalles Técnicos

Listado completo para pista de auditoría y trazabilidad.

#### 3.1 Listado de Coincidencias
Formato de tabla:
| Entity | Field | Match Type | Score | Index A | Index B |
|--------|-------|------------|-------|---------|---------|

#### 3.2 Listado de Exclusivos
Formato de tabla:
| Entity | Field | Side | Safe to Add | Index |
|--------|-------|------|-------------|-------|

#### 3.3 Listado de Conflictos
Para cada conflicto:
- **Conflict ID**: Número secuencial para referencia
- **Entity**: Nombre del entity
- **Field**: Nombre del field (si aplica)
- **Type**: Tipo de conflicto
- **Severity**: Nivel de severidad
- **Evidence**:
  - From Schema A: Descripción
  - From Schema B: Descripción
  - Rule: Regla que fue violada
- **Indexes**: Ubicación en los schemas (A, B)

### 4. Sugerencias de Resolución (si están disponibles)

Para cada conflicto con sugerencias:

#### 4.1 Tabla de Sugerencias
| Conflict ID | Suggestion | Confidence | Impact | Status |
|-------------|------------|------------|--------|--------|

#### 4.2 Sugerencias Detalladas
Para cada sugerencia:
- **Suggestion ID**: Identificador único
- **For Conflict**: Referencia al conflict ID
- **Kind**: Tipo de sugerencia (cast_safe, cast_risky, rename_field, etc.)
- **Confidence**: Puntuación (0.0-1.0)
- **Impact**: Minimal | Low | Medium | High | Critical
- **Explanation**: Descripción legible para humanos
- **Evidence**: Hechos de respaldo

### 5. Registro de Decisiones (si está disponible)

Resumen de decisiones de resolución:
- **Total Decisions**: Conteo
- **Accepted**: Conteo y porcentaje
- **Rejected**: Conteo y porcentaje
- **Auto-accepted**: Decisiones tomadas por el sistema (alta confianza)
- **Manual**: Decisiones tomadas por usuarios

Para cada decisión:
- **Decision ID**: Identificador único
- **Suggestion**: Referencia a sugerencia
- **Status**: Accepted/Rejected
- **Source**: System o User
- **Rationale**: Razón de la decisión
- **Timestamp**: Cuándo se tomó la decisión

### 6. Recomendaciones

Próximos pasos accionables basados en el análisis:
- Si existen conflictos: Sugerir revisar conflictos de alta severidad primero
- Si hay muchos exclusivos: Recomendar revisar flags safe_to_add
- Si la compatibilidad es baja: Sugerir alineación manual de schema
- Referencia a decision_log.json para acceso programático

## Formatos de Salida

### Markdown (Primario)
- Archivo: `report.md`
- Legible para humanos con tablas y formato
- Incluye anclas internas para navegación
- Autocontenido (entendible sin archivos JSON)

### JSON (Opcional)
- Archivo: `report.json`
- Datos estructurados para consumo programático
- Mismas secciones que Markdown
- Incluye todos los datos crudos

## Convenciones

### Formato
- Usar tablas para datos estructurados
- Usar badges/emojis para indicadores de estado:
  - ✅ Success/Accepted
  - ⚠️ Warning/Medium severity
  - ❌ Error/Rejected
  - 🔍 Info/Review needed
  - ⚡ High priority
  - 🔥 Critical

### Nomenclatura
- Encabezados de sección usan title case
- Nombres de field usan snake_case en secciones técnicas
- Nombres de entity preservan capitalización original

### Anclas
- Cada sección principal tiene un ancla: `#executive-summary`, `#technical-details`, etc.
- Los conflictos tienen anclas: `#conflict-1`, `#conflict-2`, etc.

## Ejemplo de Salida

```markdown
# Reporte de Comparación de AUDD

**Generado:** 2024-01-15 14:30:00 UTC  
**Schema A:** users.csv  
**Schema B:** users.json  
**Versión del Reporte:** 1.0.0

---

## Resumen Ejecutivo

### Panorama de Compatibilidad

- **Matches**: 6
- **Exclusives**: 4 (3 de A, 1 de B)
- **Conflicts**: 3
- **Compatibility Score**: 66.7%
- **Safe Addition Rate**: 75.0%
- **Conflict Rate**: 33.3%

### Evaluación de Riesgos

**Nivel de Riesgo General**: ⚠️ **Medium**

- Conflict rate es 33.3% (sobre el umbral del 25%)
- 1 conflicto de severidad high detectado
- 2 conflictos de severidad medium detectados

### Principales Conflictos por Entity

| Entity | Conflicts | Highest Severity | Status |
|--------|-----------|------------------|--------|
| users  | 3         | High             | 2 sugerencias disponibles |

---

## Desglose Detallado

### Coincidencias
- **Total**: 6
- **Coincidencias de nombre exacto**: 5
- **Coincidencias normalizadas**: 1
- **Confianza promedio**: 0.95

### Exclusivos
- **Del Schema A**: 3 (2 seguros para agregar)
- **Del Schema B**: 1 (1 seguro para agregar)
- **Safe Addition Rate**: 75.0%

### Conflictos
- **Total**: 3
- **Por Tipo**:
  - Type incompatible: 2
  - Nullability mismatch: 1
- **Por Severidad**:
  - High: 1
  - Medium: 2

---

## Detalles Técnicos

### Coincidencias

| Entity | Field | Match Type | Score | Index A | Index B |
|--------|-------|------------|-------|---------|---------|
| users  | id    | exact_name | 1.00  | 0       | 0       |
| users  | name  | exact_name | 1.00  | 1       | 1       |
...

### Exclusivos

| Entity | Field    | Side | Safe to Add | Index |
|--------|----------|------|-------------|-------|
| users  | password | A    | ✅ Yes      | 5     |
...

### Conflictos

#### Conflict #1

- **Entity**: users
- **Field**: age
- **Type**: type_incompatible
- **Severity**: 🔥 High
- **Evidence**:
  - **From Schema A**: Type: String
  - **From Schema B**: Type: Int32
  - **Rule**: Types must be compatible
- **Indexes**: A=2, B=2

...

---

## Sugerencias de Resolución

### Resumen
- **Total Suggestions**: 2
- **High Confidence**: 1
- **Medium Confidence**: 1
- **Auto-accepted**: 1

### Conflict #1 - Sugerencias

#### Suggestion: sug_001
- **Kind**: prefer_type
- **Confidence**: 0.90 (High)
- **Impact**: Medium
- **Explanation**: Preferir tipo Int32, convertir valores String durante migración
- **Evidence**:
  - Preferred type: Int32
  - Alternative type: String
  - Rule: Prefer numeric types for age fields
- **Status**: ✅ Auto-accepted

...

---

## Registro de Decisiones

### Resumen
- **Total Decisions**: 1
- **Accepted**: 1 (100%)
- **Rejected**: 0 (0%)
- **Por Fuente**:
  - System (auto): 1
  - User (manual): 0

### Decisiones

#### Decision: dec_001
- **Suggestion**: sug_001 (Conflict #1)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Confidence 0.90 excede el umbral 0.85
- **Timestamp**: 2024-01-15 14:30:05 UTC

---

## Recomendaciones

- 🔍 Revisar los 2 conflictos restantes sin sugerencias auto-aceptadas
- ⚠️ Conflicto de alta severidad en users.age requiere revisión manual
- ✅ La mayoría de los fields son compatibles (compatibilidad del 66.7%)
- 📄 Consulte decision_log.json para historial completo de decisiones

---

*Reporte generado por AUDD v0.1.0*
```

## Métricas e Indicadores

### Métricas Principales
1. **Compatibility Score**: Tasa de éxito de coincidencia de fields
2. **Conflict Rate**: Porcentaje de fields con conflictos
3. **Safe Addition Rate**: Porcentaje de exclusivos seguros para agregar
4. **Resolution Rate**: Porcentaje de conflictos con sugerencias aceptadas

### Indicadores de Calidad
1. **Average Match Confidence**: Confianza promedio de todas las coincidencias
2. **Suggestion Coverage**: Porcentaje de conflictos con sugerencias
3. **Auto-acceptance Rate**: Porcentaje de sugerencias auto-aceptadas

### Indicadores de Riesgo
1. **Critical Conflict Count**: Número de conflictos de severidad critical
2. **High Severity Rate**: Porcentaje de conflictos high/critical
3. **Unresolved Conflict Count**: Conflictos sin sugerencias aceptadas

## Mantenimiento

Esta estructura está versionada y debe permanecer estable. Los cambios requieren:
1. Incremento de versión
2. Consideraciones de compatibilidad hacia atrás
3. Actualización de archivos de prueba golden
4. Documentación de ruta de migración

## Referencias

- EPIC 03: Comparison engine
- EPIC 04: Suggestions and resolution
- EPIC 07: CLI outputs
