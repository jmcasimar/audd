# AUDD Comparison Report

**Generated:** [TIMESTAMP]
**Schema A:** users  
**Schema B:** users  
**Report Version:** 1.0.0

---

## Executive Summary

### Compatibility Overview

- **Matches**: 6
- **Exclusives**: 1 (0 from A, 1 from B)
- **Conflicts**: 8
- **Compatibility Score**: 42.9%
- **Safe Addition Rate**: 100.0%
- **Conflict Rate**: 57.1%

### Risk Assessment

**Overall Risk Level**: 💀 **Critical**

- 3 high-severity conflict(s) detected
- Conflict rate is 57.1% (above 25% threshold)

### Top Conflicts by Entity

| Entity | Conflicts | Highest Severity |
|--------|-----------|------------------|
| users | 8 | 🔥 High |

---

## Detailed Breakdown

### Matches

- **Total**: 6
- **Exact name matches**: 0
- **Average confidence**: 1.00

### Exclusives

- **From Schema A**: 0 (0 safe to add)
- **From Schema B**: 1 (1 safe to add)
- **Safe Addition Rate**: 100.0%

### Conflicts

- **Total**: 8
- **By Type**:
  - NullabilityMismatch: 5
  - TypeIncompatible: 3
- **By Severity**:
  - High: 3
  - Medium: 5

---

## Technical Details

### Matches

| Entity | Field | Match Type | Score | Index A | Index B |
|--------|-------|------------|-------|---------|---------|
| users | (entity) | semantic_match | 1.00 | 0 | 0 |
| users | id | semantic_match | 1.00 | 0 | 0 |
| users | name | semantic_match | 1.00 | 1 | 1 |
| users | email | semantic_match | 1.00 | 2 | 2 |
| users | age | semantic_match | 1.00 | 3 | 3 |
| users | active | semantic_match | 1.00 | 4 | 4 |

### Exclusives

| Entity | Field | Side | Safe to Add | Index |
|--------|-------|------|-------------|-------|
| users | score | B | ✅ Yes | 5 |

### Conflicts

#### <a name="conflict-1"></a>Conflict #1

- **Entity**: users
- **Field**: id
- **Type**: TypeIncompatible
- **Severity**: 🔥 High
- **Evidence**:
  - **From Schema A**: Type: String
  - **From Schema B**: Type: Int64
  - **Rule**: Types must be compatible
- **Indexes**: A=0, B=0

#### <a name="conflict-2"></a>Conflict #2

- **Entity**: users
- **Field**: id
- **Type**: NullabilityMismatch
- **Severity**: ⚠️ Medium
- **Evidence**:
  - **From Schema A**: Nullable: true
  - **From Schema B**: Nullable: false
  - **Rule**: Nullability must match or be compatible
- **Indexes**: A=0, B=0

#### <a name="conflict-3"></a>Conflict #3

- **Entity**: users
- **Field**: name
- **Type**: NullabilityMismatch
- **Severity**: ⚠️ Medium
- **Evidence**:
  - **From Schema A**: Nullable: true
  - **From Schema B**: Nullable: false
  - **Rule**: Nullability must match or be compatible
- **Indexes**: A=1, B=1

#### <a name="conflict-4"></a>Conflict #4

- **Entity**: users
- **Field**: email
- **Type**: NullabilityMismatch
- **Severity**: ⚠️ Medium
- **Evidence**:
  - **From Schema A**: Nullable: true
  - **From Schema B**: Nullable: false
  - **Rule**: Nullability must match or be compatible
- **Indexes**: A=2, B=2

#### <a name="conflict-5"></a>Conflict #5

- **Entity**: users
- **Field**: age
- **Type**: TypeIncompatible
- **Severity**: 🔥 High
- **Evidence**:
  - **From Schema A**: Type: String
  - **From Schema B**: Type: Int64
  - **Rule**: Types must be compatible
- **Indexes**: A=3, B=3

#### <a name="conflict-6"></a>Conflict #6

- **Entity**: users
- **Field**: age
- **Type**: NullabilityMismatch
- **Severity**: ⚠️ Medium
- **Evidence**:
  - **From Schema A**: Nullable: true
  - **From Schema B**: Nullable: false
  - **Rule**: Nullability must match or be compatible
- **Indexes**: A=3, B=3

#### <a name="conflict-7"></a>Conflict #7

- **Entity**: users
- **Field**: active
- **Type**: TypeIncompatible
- **Severity**: 🔥 High
- **Evidence**:
  - **From Schema A**: Type: String
  - **From Schema B**: Type: Boolean
  - **Rule**: Types must be compatible
- **Indexes**: A=4, B=4

#### <a name="conflict-8"></a>Conflict #8

- **Entity**: users
- **Field**: active
- **Type**: NullabilityMismatch
- **Severity**: ⚠️ Medium
- **Evidence**:
  - **From Schema A**: Nullable: true
  - **From Schema B**: Nullable: false
  - **Rule**: Nullability must match or be compatible
- **Indexes**: A=4, B=4

---

## Resolution Suggestions

### Summary

- **Total Suggestions**: 8
- **High Confidence** (≥0.85): 5
- **Medium Confidence** (0.60-0.84): 3
- **Auto-accepted**: 8

### Conflict #1 - Suggestions

#### Suggestion: sug_0

- **Kind**: CastRisky { from_type: "String", to_type: "Int64", warning: "Requires validating that all values can be parsed" }
- **Confidence**: 0.75 (Medium)
- **Impact**: High
- **Explanation**: Narrowing or lossy cast from String to Int64 - Requires validating that all values can be parsed
- **Evidence**:
  - Source type: String
  - Target type: Int64
  - Warning: Requires validating that all values can be parsed
- **Status**: ✅ Accepted

#### Suggestion: sug_1

- **Kind**: PreferType { preferred_type: "NULLABLE", alternative_type: "NOT NULL", rule: "Allow nullable to accommodate both schemas" }
- **Confidence**: 0.90 (High)
- **Impact**: Low
- **Explanation**: Make field nullable to accommodate schema A
- **Evidence**:
  - Schema A: NULLABLE
  - Schema B: NOT NULL
  - Nullable is more permissive
- **Status**: ✅ Accepted

### Conflict #2 - Suggestions

#### Suggestion: sug_0

- **Kind**: CastRisky { from_type: "String", to_type: "Int64", warning: "Requires validating that all values can be parsed" }
- **Confidence**: 0.75 (Medium)
- **Impact**: High
- **Explanation**: Narrowing or lossy cast from String to Int64 - Requires validating that all values can be parsed
- **Evidence**:
  - Source type: String
  - Target type: Int64
  - Warning: Requires validating that all values can be parsed
- **Status**: ✅ Accepted

#### Suggestion: sug_1

- **Kind**: PreferType { preferred_type: "NULLABLE", alternative_type: "NOT NULL", rule: "Allow nullable to accommodate both schemas" }
- **Confidence**: 0.90 (High)
- **Impact**: Low
- **Explanation**: Make field nullable to accommodate schema A
- **Evidence**:
  - Schema A: NULLABLE
  - Schema B: NOT NULL
  - Nullable is more permissive
- **Status**: ✅ Accepted

### Conflict #3 - Suggestions

#### Suggestion: sug_2

- **Kind**: PreferType { preferred_type: "NULLABLE", alternative_type: "NOT NULL", rule: "Allow nullable to accommodate both schemas" }
- **Confidence**: 0.90 (High)
- **Impact**: Low
- **Explanation**: Make field nullable to accommodate schema A
- **Evidence**:
  - Schema A: NULLABLE
  - Schema B: NOT NULL
  - Nullable is more permissive
- **Status**: ✅ Accepted

### Conflict #4 - Suggestions

#### Suggestion: sug_3

- **Kind**: PreferType { preferred_type: "NULLABLE", alternative_type: "NOT NULL", rule: "Allow nullable to accommodate both schemas" }
- **Confidence**: 0.90 (High)
- **Impact**: Low
- **Explanation**: Make field nullable to accommodate schema A
- **Evidence**:
  - Schema A: NULLABLE
  - Schema B: NOT NULL
  - Nullable is more permissive
- **Status**: ✅ Accepted

### Conflict #5 - Suggestions

#### Suggestion: sug_4

- **Kind**: CastRisky { from_type: "String", to_type: "Int64", warning: "Requires validating that all values can be parsed" }
- **Confidence**: 0.75 (Medium)
- **Impact**: High
- **Explanation**: Narrowing or lossy cast from String to Int64 - Requires validating that all values can be parsed
- **Evidence**:
  - Source type: String
  - Target type: Int64
  - Warning: Requires validating that all values can be parsed
- **Status**: ✅ Accepted

#### Suggestion: sug_5

- **Kind**: PreferType { preferred_type: "NULLABLE", alternative_type: "NOT NULL", rule: "Allow nullable to accommodate both schemas" }
- **Confidence**: 0.90 (High)
- **Impact**: Low
- **Explanation**: Make field nullable to accommodate schema A
- **Evidence**:
  - Schema A: NULLABLE
  - Schema B: NOT NULL
  - Nullable is more permissive
- **Status**: ✅ Accepted

### Conflict #6 - Suggestions

#### Suggestion: sug_4

- **Kind**: CastRisky { from_type: "String", to_type: "Int64", warning: "Requires validating that all values can be parsed" }
- **Confidence**: 0.75 (Medium)
- **Impact**: High
- **Explanation**: Narrowing or lossy cast from String to Int64 - Requires validating that all values can be parsed
- **Evidence**:
  - Source type: String
  - Target type: Int64
  - Warning: Requires validating that all values can be parsed
- **Status**: ✅ Accepted

#### Suggestion: sug_5

- **Kind**: PreferType { preferred_type: "NULLABLE", alternative_type: "NOT NULL", rule: "Allow nullable to accommodate both schemas" }
- **Confidence**: 0.90 (High)
- **Impact**: Low
- **Explanation**: Make field nullable to accommodate schema A
- **Evidence**:
  - Schema A: NULLABLE
  - Schema B: NOT NULL
  - Nullable is more permissive
- **Status**: ✅ Accepted

### Conflict #7 - Suggestions

#### Suggestion: sug_6

- **Kind**: CastRisky { from_type: "String", to_type: "Boolean", warning: "Requires validating that all values can be parsed" }
- **Confidence**: 0.75 (Medium)
- **Impact**: High
- **Explanation**: Narrowing or lossy cast from String to Boolean - Requires validating that all values can be parsed
- **Evidence**:
  - Source type: String
  - Target type: Boolean
  - Warning: Requires validating that all values can be parsed
- **Status**: ✅ Accepted

#### Suggestion: sug_7

- **Kind**: PreferType { preferred_type: "NULLABLE", alternative_type: "NOT NULL", rule: "Allow nullable to accommodate both schemas" }
- **Confidence**: 0.90 (High)
- **Impact**: Low
- **Explanation**: Make field nullable to accommodate schema A
- **Evidence**:
  - Schema A: NULLABLE
  - Schema B: NOT NULL
  - Nullable is more permissive
- **Status**: ✅ Accepted

### Conflict #8 - Suggestions

#### Suggestion: sug_6

- **Kind**: CastRisky { from_type: "String", to_type: "Boolean", warning: "Requires validating that all values can be parsed" }
- **Confidence**: 0.75 (Medium)
- **Impact**: High
- **Explanation**: Narrowing or lossy cast from String to Boolean - Requires validating that all values can be parsed
- **Evidence**:
  - Source type: String
  - Target type: Boolean
  - Warning: Requires validating that all values can be parsed
- **Status**: ✅ Accepted

#### Suggestion: sug_7

- **Kind**: PreferType { preferred_type: "NULLABLE", alternative_type: "NOT NULL", rule: "Allow nullable to accommodate both schemas" }
- **Confidence**: 0.90 (High)
- **Impact**: Low
- **Explanation**: Make field nullable to accommodate schema A
- **Evidence**:
  - Schema A: NULLABLE
  - Schema B: NOT NULL
  - Nullable is more permissive
- **Status**: ✅ Accepted

---

## Decision Log

### Summary

- **Total Decisions**: 8
- **Accepted**: 8 (100.0%)
- **Rejected**: 0 (0.0%)
- **By Source**:
  - System (auto): 8
  - User (manual): 0

### Decisions

#### Decision: auto_dec_1

- **Suggestion**: sug_0 (users::id)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: [TIMESTAMP]

#### Decision: auto_dec_2

- **Suggestion**: sug_1 (users::id)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: [TIMESTAMP]

#### Decision: auto_dec_3

- **Suggestion**: sug_2 (users::name)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: [TIMESTAMP]

#### Decision: auto_dec_4

- **Suggestion**: sug_3 (users::email)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: [TIMESTAMP]

#### Decision: auto_dec_5

- **Suggestion**: sug_4 (users::age)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: [TIMESTAMP]

#### Decision: auto_dec_6

- **Suggestion**: sug_5 (users::age)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: [TIMESTAMP]

#### Decision: auto_dec_7

- **Suggestion**: sug_6 (users::active)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: [TIMESTAMP]

#### Decision: auto_dec_8

- **Suggestion**: sug_7 (users::active)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: [TIMESTAMP]

---

## Recommendations

- 🔥 **CRITICAL**: Immediate manual review required before proceeding
- 🔥 Address 3 high-severity conflict(s) before auto-unification
- ⚠️ Low compatibility score - consider manual schema alignment
- 📄 See `decision_log.json` for complete decision history and programmatic access


---

*Report generated by AUDD v0.1.0*
