# AUDD Comparison Report

**Generated:** 1970-01-01 00:00:00 UTC  
**Schema A:** a  
**Schema B:** b  
**Report Version:** 1.0.0

---

## Executive Summary

### Compatibility Overview

- **Matches**: 43
- **Exclusives**: 4 (2 from A, 2 from B)
- **Conflicts**: 1
- **Compatibility Score**: 97.7%
- **Safe Addition Rate**: 100.0%
- **Conflict Rate**: 2.3%

### Risk Assessment

**Overall Risk Level**: ✅ **Low**

- Low conflict rate and no high-severity issues

### Top Conflicts by Entity

| Entity | Conflicts | Highest Severity |
|--------|-----------|------------------|
| producto | 1 | ⚠️ Medium |

---

## Detailed Breakdown

### Matches

- **Total**: 43
- **Exact name matches**: 0
- **Average confidence**: 1.00

### Exclusives

- **From Schema A**: 2 (2 safe to add)
- **From Schema B**: 2 (2 safe to add)
- **Safe Addition Rate**: 100.0%

### Conflicts

- **Total**: 1
- **By Type**:
  - NullabilityMismatch: 1
- **By Severity**:
  - Medium: 1

---

## Technical Details

### Matches

| Entity | Field | Match Type | Score | Index A | Index B |
|--------|-------|------------|-------|---------|---------|
| categoria | (entity) | semantic_match | 1.00 | 0 | 0 |
| categoria | id | semantic_match | 1.00 | 0 | 0 |
| categoria | nombre | semantic_match | 1.00 | 1 | 1 |
| categoria | descripcion | semantic_match | 1.00 | 2 | 2 |
| cliente | (entity) | semantic_match | 1.00 | 1 | 1 |
| cliente | id | semantic_match | 1.00 | 0 | 0 |
| cliente | codigo | semantic_match | 1.00 | 1 | 1 |
| cliente | nombre | semantic_match | 1.00 | 2 | 2 |
| cliente | telefono | semantic_match | 1.00 | 3 | 3 |
| cliente | correo | semantic_match | 1.00 | 4 | 4 |
| cliente | rfc | semantic_match | 1.00 | 5 | 5 |
| cliente | created_at | semantic_match | 1.00 | 6 | 6 |
| pago | (entity) | semantic_match | 1.00 | 2 | 2 |
| pago | id | semantic_match | 1.00 | 0 | 0 |
| pago | venta_id | semantic_match | 1.00 | 1 | 1 |
| pago | forma_pago | semantic_match | 1.00 | 2 | 2 |
| pago | monto | semantic_match | 1.00 | 3 | 3 |
| pago | referencia | semantic_match | 1.00 | 4 | 4 |
| producto | (entity) | semantic_match | 1.00 | 3 | 3 |
| producto | id | semantic_match | 1.00 | 0 | 0 |
| producto | codigo_barras | semantic_match | 1.00 | 1 | 1 |
| producto | nombre | semantic_match | 1.00 | 2 | 2 |
| producto | descripcion | semantic_match | 1.00 | 3 | 3 |
| producto | precio | semantic_match | 1.00 | 4 | 4 |
| producto | existencia | semantic_match | 1.00 | 5 | 5 |
| producto | categoria_id | semantic_match | 1.00 | 6 | 6 |
| producto | activo | semantic_match | 1.00 | 7 | 7 |
| producto | created_at | semantic_match | 1.00 | 8 | 8 |
| venta | (entity) | semantic_match | 1.00 | 4 | 4 |
| venta | id | semantic_match | 1.00 | 0 | 0 |
| venta | folio | semantic_match | 1.00 | 1 | 1 |
| venta | cliente_id | semantic_match | 1.00 | 2 | 2 |
| venta | subtotal | semantic_match | 1.00 | 4 | 4 |
| venta | impuesto | semantic_match | 1.00 | 5 | 5 |
| venta | total | semantic_match | 1.00 | 6 | 6 |
| venta | estatus | semantic_match | 1.00 | 7 | 7 |
| venta_detalle | (entity) | semantic_match | 1.00 | 5 | 5 |
| venta_detalle | id | semantic_match | 1.00 | 0 | 0 |
| venta_detalle | venta_id | semantic_match | 1.00 | 1 | 1 |
| venta_detalle | producto_id | semantic_match | 1.00 | 2 | 2 |
| venta_detalle | cantidad | semantic_match | 1.00 | 3 | 3 |
| venta_detalle | precio_unitario | semantic_match | 1.00 | 4 | 4 |
| venta_detalle | importe | semantic_match | 1.00 | 5 | 5 |

### Exclusives

| Entity | Field | Side | Safe to Add | Index |
|--------|-------|------|-------------|-------|
| pago | fecha | A | ✅ Yes | 5 |
| pago | date | B | ✅ Yes | 5 |
| venta | fecha | A | ✅ Yes | 3 |
| venta | date | B | ✅ Yes | 3 |

### Conflicts

#### <a name="conflict-1"></a>Conflict #1

- **Entity**: producto
- **Field**: created_at
- **Type**: NullabilityMismatch
- **Severity**: ⚠️ Medium
- **Evidence**:
  - **From Schema A**: Nullable: false
  - **From Schema B**: Nullable: true
  - **Rule**: Nullability must match or be compatible
- **Indexes**: A=8, B=8

---

## Resolution Suggestions

### Summary

- **Total Suggestions**: 1
- **High Confidence** (≥0.85): 1
- **Medium Confidence** (0.60-0.84): 0
- **Auto-accepted**: 1

### Conflict #1 - Suggestions

#### Suggestion: sug_0

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

- **Total Decisions**: 1
- **Accepted**: 1 (100.0%)
- **Rejected**: 0 (0.0%)
- **By Source**:
  - System (auto): 1
  - User (manual): 0

### Decisions

#### Decision: auto_dec_1

- **Suggestion**: sug_0 (producto::created_at)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: 1970-01-01 00:00:00 UTC

---

## Recommendations

- ✅ Low risk detected - schemas are mostly compatible
- ✅ High compatibility score - schemas align well
- 📄 See `decision_log.json` for complete decision history and programmatic access


---

*Report generated by AUDD v0.1.0*
