# audd_resolution

Sugerencias de resolución y rastreo de decisiones para conflictos de comparación de schema en AUDD.

## Descripción General

Este crate proporciona la base para resolución asistida y toma de decisiones auditable en AUDD:

- **Sugerencias**: Recomendaciones explicables para resolver conflictos de schema
- **Decisiones**: Rastreo auditable de elecciones de resolución (humana o automatizada)
- **Logs de Decisiones**: Pista de auditoría completa de conflicto → sugerencia → decisión

## Características

- **Múltiples Tipos de Sugerencias**:
  - `CastSafe`: Conversiones de tipo seguras (ej., Int32 → Int64)
  - `CastRisky`: Conversiones riesgosas con advertencias (ej., Int64 → Int32)
  - `RenameField`: Renombrado de campos para evitar colisiones
  - `PreferType`: Elegir tipo preferido basado en política
  - `SplitField` / `MergeFields`: Cambios estructurales (solo sugerencias)
  - `NoSuggestion`: Marcador explícito cuando no hay auto-resolución disponible

- **Rastreo de Confianza e Impacto**:
  - Niveles de confianza: Alto (0.9-1.0), Medio (0.6-0.89), Bajo (0.0-0.59)
  - Niveles de impacto: Minimal, Low, Medium, High, Critical

- **Decisiones Auditables**:
  - Rastreo de fuente (User vs System)
  - Marcas de tiempo
  - Razonamiento y metadata
  - Rastreo de estado (Pending, Applied, Rejected, Superseded)

- **Políticas Configurables**:
  - Preferencia de tipo (preferir mayor, mayor precisión, schema A/B)
  - Política de nulabilidad (siempre nullable, siempre no null, preferir schema A/B)
  - Política de longitud (usar max, usar min, preferir schema A/B)
  - Control de sugerencias riesgosas
  - Umbrales de confianza

- **Formatos de Exportación**:
  - JSON: Exportación de datos estructurados
  - Markdown: Resúmenes legibles

## Inicio Rápido

```rust
use audd_compare::Conflict;
use audd_resolution::{
    DecisionLog, Decision, DecisionSource, 
    Suggestion, SuggestionEngine,
};

// Generar sugerencias desde un conflicto
let conflict = Conflict::type_incompatible(
    "users".to_string(),
    "id".to_string(),
    "Int32".to_string(),
    "Int64".to_string(),
    0,
    1,
);

let engine = SuggestionEngine::new();
let suggestions = engine.suggest(&conflict);

// Tomar una decisión
let decision = Decision::accept(
    "dec1".to_string(),
    suggestions[0].clone(),
    "Approved for production".to_string(),
    DecisionSource::User {
        username: "admin".to_string(),
    },
);

// Rastrear en log de decisiones
let mut log = DecisionLog::new();
log.add_decision(decision);

// Exportar
let json = log.to_json().unwrap();
let markdown = log.to_markdown();
```

## Ejemplos

Ejecuta el ejemplo completo:

```bash
cargo run --package audd_resolution --example resolution_workflow
```

## Configuración

Crea políticas de resolución personalizadas:

```rust
use audd_resolution::{ResolutionConfig, SuggestionEngine};

// Modo conservador - solo sugerencias seguras
let config = ResolutionConfig::conservative();
let engine = SuggestionEngine::with_config(config);

// Preferir schema A para todos los conflictos
let config = ResolutionConfig::prefer_schema_a();
let engine = SuggestionEngine::with_config(config);

// Configuración personalizada
let config = ResolutionConfig {
    type_preference: TypePreferencePolicy::PreferLarger,
    nullability_policy: NullabilityPolicy::AlwaysNullable,
    length_policy: LengthPolicy::UseMaximum,
    allow_risky_suggestions: false,
    min_confidence: 0.8,
    generate_alternatives: true,
};
let engine = SuggestionEngine::with_config(config);
```

## Pruebas

El crate incluye cobertura de pruebas completa:

```bash
# Ejecutar todas las pruebas
cargo test --package audd_resolution

# Ejecutar pruebas de integración
cargo test --package audd_resolution --test integration_test

# Ejecutar pruebas de cobertura
cargo test --package audd_resolution --test coverage_test
```

**Estadísticas de Pruebas:**
- Total de pruebas: 62
- Pruebas unitarias: 38
- Pruebas de cobertura: 13
- Pruebas de fixtures: 5
- Pruebas de integración: 5
- Pruebas de documentación: 1

**Métricas de Cobertura:**
- Fixtures completos: ≥90% de cobertura lograda
- Fixtures de conversión de tipos: 100% de cobertura
- Fixtures de colisión de nombres: 100% de cobertura
- Todos los tipos de conflicto producen al menos una sugerencia

## Arquitectura

### Componentes Centrales

1. **Suggestion** (`suggestion.rs`)
   - Estructura de datos para recomendaciones de resolución
   - Incluye confianza, impacto, evidencia y explicación
   - Múltiples tipos de sugerencias para diferentes tipos de conflicto

2. **Decision** (`decision.rs`)
   - Rastrea aceptación/rechazo de sugerencias
   - Registra fuente (usuario o sistema), razonamiento y marca de tiempo
   - Rastreo de estado para gestión de ciclo de vida

3. **DecisionLog** (`decision_log.rs`)
   - Colección de decisiones con metadata
   - Exportar a JSON y Markdown
   - Interfaz de consulta para filtrar decisiones

4. **SuggestionEngine** (`engine.rs`)
   - Genera sugerencias desde conflictos
   - Comportamiento configurable vía ResolutionConfig
   - Estrategias de sugerencia específicas por tipo

5. **ResolutionConfig** (`config.rs`)
   - Políticas para preferencia de tipo, nulabilidad, longitud
   - Control sobre sugerencias riesgosas y umbrales de confianza
   - Configuraciones preestablecidas (default, conservative, prefer A/B)

### Flujo de Generación de Sugerencias

```
Conflict → SuggestionEngine → [Suggestion, ...]
                ↓
           Configuration
                ↓
         Type Analysis → CastSafe/CastRisky
         Name Analysis → RenameField
    Nullability Check → PreferType (nullable)
       Constraint Check → NoSuggestion
```

### Flujo de Rastreo de Decisiones

```
Suggestions → User/System Review → Decision
                                      ↓
                              DecisionLog → Export
                                            - JSON
                                            - Markdown
```

## Estado

**Estado de Implementación: COMPLETO** ✅

Todos los issues de EPIC 04 han sido implementados:

- ✅ Issue 04.1: Modelos de datos centrales (Suggestion/Decision/DecisionLog)
- ✅ Issue 04.2: Sugerencias de incompatibilidad de tipos (conversiones)
- ✅ Issue 04.3: Sugerencias de colisión de nombres (renombrado)
- ✅ Issue 04.4: Preferencias de tipos configurables
- ✅ Issue 04.5: Integración con flujo de comparación
- ✅ Issue 04.6: Fixtures y pruebas de cobertura

## Mejoras Futuras

Posibles mejoras futuras (fuera del alcance del MVP):

- Interfaz de usuario para toma de decisiones interactiva
- Machine learning para ranking de sugerencias
- Estrategias de sugerencia personalizadas vía plugins
- Integración con sistemas de control de versiones
- Aplicación automática de decisiones aprobadas
- Capacidades de rollback para decisiones aplicadas

## Licencia

Licenciado bajo la Licencia MIT. Ver [LICENSE](../../LICENSE) para detalles.
