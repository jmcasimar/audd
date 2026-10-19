# AUDD Comparison Report

**Generated:** 1970-01-01 00:00:00 UTC  
**Schema A:** grow_travel  
**Schema B:** grow_venta  
**Report Version:** 1.0.0

---

## Executive Summary

### Compatibility Overview

- **Matches**: 17
- **Exclusives**: 27 (22 from A, 5 from B)
- **Conflicts**: 4
- **Compatibility Score**: 81.0%
- **Safe Addition Rate**: 100.0%
- **Conflict Rate**: 19.0%

### Risk Assessment

**Overall Risk Level**: 🔥 **High**

- 3 high-severity conflict(s) detected
- Conflict rate is 19.0% (above 10% threshold)

### Top Conflicts by Entity

| Entity | Conflicts | Highest Severity |
|--------|-----------|------------------|
| pago | 2 | 🔥 High |
| cliente | 1 | 🔥 High |
| categoria | 1 | 🔥 High |

---

## Detailed Breakdown

### Matches

- **Total**: 17
- **Exact name matches**: 0
- **Average confidence**: 1.00

### Exclusives

- **From Schema A**: 22 (22 safe to add)
- **From Schema B**: 5 (5 safe to add)
- **Safe Addition Rate**: 100.0%

### Conflicts

- **Total**: 4
- **By Type**:
  - NullabilityMismatch: 1
  - TypeIncompatible: 3
- **By Severity**:
  - High: 3
  - Medium: 1

---

## Technical Details

### Matches

| Entity | Field | Match Type | Score | Index A | Index B |
|--------|-------|------------|-------|---------|---------|
| categoria | (entity) | semantic_match | 1.00 | 4 | 0 |
| categoria | id | semantic_match | 1.00 | 0 | 0 |
| categoria | nombre | semantic_match | 1.00 | 1 | 1 |
| categoria | descripcion | semantic_match | 1.00 | 2 | 2 |
| cliente | (entity) | semantic_match | 1.00 | 6 | 1 |
| cliente | id | semantic_match | 1.00 | 0 | 0 |
| cliente | codigo | semantic_match | 1.00 | 1 | 1 |
| cliente | nombre | semantic_match | 1.00 | 2 | 2 |
| cliente | telefono | semantic_match | 1.00 | 3 | 3 |
| cliente | correo | semantic_match | 1.00 | 4 | 4 |
| cliente | rfc | semantic_match | 1.00 | 5 | 5 |
| cliente | created_at | semantic_match | 1.00 | 8 | 6 |
| pago | (entity) | semantic_match | 1.00 | 11 | 2 |
| pago | id | semantic_match | 1.00 | 0 | 0 |
| pago | referencia | semantic_match | 1.00 | 1 | 4 |
| pago | monto | semantic_match | 1.00 | 2 | 3 |
| pago | fecha | semantic_match | 1.00 | 3 | 5 |

### Exclusives

| Entity | Field | Side | Safe to Add | Index |
|--------|-------|------|-------------|-------|
| cliente | fecha_nacimiento | A | ✅ Yes | 6 |
| cliente | pasaporte | A | ✅ Yes | 7 |
| pago | metodo_pago | A | ✅ Yes | 4 |
| pago | estatus | A | ✅ Yes | 5 |
| pago | venta_id | B | ✅ Yes | 1 |
| pago | forma_pago | B | ✅ Yes | 2 |
| aerolinea | (entity) | A | ✅ Yes | 0 |
| aerolinea_frecuente | (entity) | A | ✅ Yes | 1 |
| aeropuerto | (entity) | A | ✅ Yes | 2 |
| avion | (entity) | A | ✅ Yes | 3 |
| clase_vuelo | (entity) | A | ✅ Yes | 5 |
| factura | (entity) | A | ✅ Yes | 7 |
| guia_turistico | (entity) | A | ✅ Yes | 8 |
| habitacion | (entity) | A | ✅ Yes | 9 |
| hotel | (entity) | A | ✅ Yes | 10 |
| pago_vuelo | (entity) | A | ✅ Yes | 12 |
| paquete_turistico | (entity) | A | ✅ Yes | 13 |
| pasajero | (entity) | A | ✅ Yes | 14 |
| reserva_hotel | (entity) | A | ✅ Yes | 15 |
| reserva_paquete | (entity) | A | ✅ Yes | 16 |
| reserva_vuelo | (entity) | A | ✅ Yes | 17 |
| seguro_viaje | (entity) | A | ✅ Yes | 18 |
| tour | (entity) | A | ✅ Yes | 19 |
| vuelo | (entity) | A | ✅ Yes | 20 |
| producto | (entity) | B | ✅ Yes | 3 |
| venta | (entity) | B | ✅ Yes | 4 |
| venta_detalle | (entity) | B | ✅ Yes | 5 |

### Conflicts

#### <a name="conflict-1"></a>Conflict #1

- **Entity**: categoria
- **Field**: nombre
- **Type**: TypeIncompatible
- **Severity**: 🔥 High
- **Evidence**:
  - **From Schema A**: Type: Int64
  - **From Schema B**: Type: String
  - **Rule**: Types must be compatible
- **Indexes**: A=1, B=1

#### <a name="conflict-2"></a>Conflict #2

- **Entity**: cliente
- **Field**: telefono
- **Type**: TypeIncompatible
- **Severity**: 🔥 High
- **Evidence**:
  - **From Schema A**: Type: Int64
  - **From Schema B**: Type: String
  - **Rule**: Types must be compatible
- **Indexes**: A=3, B=3

#### <a name="conflict-3"></a>Conflict #3

- **Entity**: pago
- **Field**: referencia
- **Type**: NullabilityMismatch
- **Severity**: ⚠️ Medium
- **Evidence**:
  - **From Schema A**: Nullable: false
  - **From Schema B**: Nullable: true
  - **Rule**: Nullability must match or be compatible
- **Indexes**: A=1, B=4

#### <a name="conflict-4"></a>Conflict #4

- **Entity**: pago
- **Field**: monto
- **Type**: TypeIncompatible
- **Severity**: 🔥 High
- **Evidence**:
  - **From Schema A**: Type: String
  - **From Schema B**: Type: Decimal(10, 2)
  - **Rule**: Types must be compatible
- **Indexes**: A=2, B=3

---

## Resolution Suggestions

### Summary

- **Total Suggestions**: 3
- **High Confidence** (≥0.85): 3
- **Medium Confidence** (0.60-0.84): 0
- **Auto-accepted**: 3

### Conflict #1 - Suggestions

#### Suggestion: sug_1

- **Kind**: CastSafe { from_type: "Int64", to_type: "String" }
- **Confidence**: 0.90 (High)
- **Impact**: Low
- **Explanation**: Safe widening cast from Int64 to String - no data loss expected
- **Evidence**:
  - Source type: Int64
  - Target type: String
  - No data loss expected
- **Status**: ✅ Accepted

### Conflict #2 - Suggestions

#### Suggestion: sug_3

- **Kind**: CastSafe { from_type: "Int64", to_type: "String" }
- **Confidence**: 0.90 (High)
- **Impact**: Low
- **Explanation**: Safe widening cast from Int64 to String - no data loss expected
- **Evidence**:
  - Source type: Int64
  - Target type: String
  - No data loss expected
- **Status**: ✅ Accepted

### Conflict #3 - Suggestions

#### Suggestion: sug_4

- **Kind**: PreferType { preferred_type: "Nullable", alternative_type: "alternative", rule: "R-NULL-001" }
- **Confidence**: 0.90 (High)
- **Impact**: Medium
- **Explanation**: Acepta los datos de ambas fuentes
- **Evidence**:
  - catalog:R-NULL-001
- **Status**: ✅ Accepted

---

## Decision Log

### Summary

- **Total Decisions**: 3
- **Accepted**: 3 (100.0%)
- **Rejected**: 0 (0.0%)
- **By Source**:
  - System (auto): 3
  - User (manual): 0

### Decisions

#### Decision: auto_dec_1

- **Suggestion**: sug_1 (categoria::nombre)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: 1970-01-01 00:00:00 UTC

#### Decision: auto_dec_2

- **Suggestion**: sug_3 (cliente::telefono)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: 1970-01-01 00:00:00 UTC

#### Decision: auto_dec_3

- **Suggestion**: sug_4 (pago::referencia)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: 1970-01-01 00:00:00 UTC

---

## Recommendations

- ⚠️ **HIGH RISK**: Careful review of all conflicts recommended
- 🔥 Address 3 high-severity conflict(s) before auto-unification
- 🔍 1 conflict(s) remain without accepted resolutions
- ✅ High compatibility score - schemas align well
- 📄 See `decision_log.json` for complete decision history and programmatic access


---

*Report generated by AUDD v0.1.0*
