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
- **Conflicts**: 3
- **Compatibility Score**: 66.7%
- **Safe Addition Rate**: 100.0%
- **Conflict Rate**: 33.3%

### Risk Assessment

**Overall Risk Level**: 🔥 **High**

- 3 high-severity conflict(s) detected
- Conflict rate is 33.3% (above 25% threshold)

### Top Conflicts by Entity

| Entity | Conflicts | Highest Severity |
|--------|-----------|------------------|
| users | 3 | 🔥 High |

---

## Detailed Breakdown

### Matches

- **Total**: 6
- **Exact name matches**: 6
- **Average confidence**: 1.00

### Exclusives

- **From Schema A**: 0 (0 safe to add)
- **From Schema B**: 1 (1 safe to add)
- **Safe Addition Rate**: 100.0%

### Conflicts

- **Total**: 3
- **By Type**:
  - TypeIncompatible: 3
- **By Severity**:
  - High: 3

---

## Technical Details

### Matches

| Entity | Field | Match Type | Score | Index A | Index B |
|--------|-------|------------|-------|---------|---------|
| users | (entity) | exact_name | 1.00 | 0 | 0 |
| users | id | exact_name | 1.00 | 0 | 0 |
| users | name | exact_name | 1.00 | 1 | 1 |
| users | email | exact_name | 1.00 | 2 | 2 |
| users | age | exact_name | 1.00 | 3 | 3 |
| users | active | exact_name | 1.00 | 4 | 4 |

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
- **Field**: age
- **Type**: TypeIncompatible
- **Severity**: 🔥 High
- **Evidence**:
  - **From Schema A**: Type: String
  - **From Schema B**: Type: Int64
  - **Rule**: Types must be compatible
- **Indexes**: A=3, B=3

#### <a name="conflict-3"></a>Conflict #3

- **Entity**: users
- **Field**: active
- **Type**: TypeIncompatible
- **Severity**: 🔥 High
- **Evidence**:
  - **From Schema A**: Type: String
  - **From Schema B**: Type: Boolean
  - **Rule**: Types must be compatible
- **Indexes**: A=4, B=4

---

## Resolution Suggestions

### Summary

- **Total Suggestions**: 3
- **High Confidence** (≥0.85): 3
- **Medium Confidence** (0.60-0.84): 0
- **Auto-accepted**: 3

### Conflict #1 - Suggestions

#### Suggestion: sug_0

- **Kind**: NoSuggestion { reason: "Type incompatibility between String and Int64 cannot be automatically resolved" }
- **Confidence**: 0.90 (High)
- **Impact**: Critical
- **Explanation**: No automatic suggestion available - manual resolution required
- **Evidence**:
  - Reason: Type incompatibility between String and Int64 cannot be automatically resolved
- **Status**: ✅ Accepted

### Conflict #2 - Suggestions

#### Suggestion: sug_1

- **Kind**: NoSuggestion { reason: "Type incompatibility between String and Int64 cannot be automatically resolved" }
- **Confidence**: 0.90 (High)
- **Impact**: Critical
- **Explanation**: No automatic suggestion available - manual resolution required
- **Evidence**:
  - Reason: Type incompatibility between String and Int64 cannot be automatically resolved
- **Status**: ✅ Accepted

### Conflict #3 - Suggestions

#### Suggestion: sug_2

- **Kind**: NoSuggestion { reason: "Type incompatibility between String and Boolean cannot be automatically resolved" }
- **Confidence**: 0.90 (High)
- **Impact**: Critical
- **Explanation**: No automatic suggestion available - manual resolution required
- **Evidence**:
  - Reason: Type incompatibility between String and Boolean cannot be automatically resolved
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

- **Suggestion**: sug_0 (users::id)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: [TIMESTAMP]

#### Decision: auto_dec_2

- **Suggestion**: sug_1 (users::age)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: [TIMESTAMP]

#### Decision: auto_dec_3

- **Suggestion**: sug_2 (users::active)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: [TIMESTAMP]

---

## Recommendations

- ⚠️ **HIGH RISK**: Careful review of all conflicts recommended
- 🔥 Address 3 high-severity conflict(s) before auto-unification
- 📄 See `decision_log.json` for complete decision history and programmatic access


---

*Report generated by AUDD v0.1.0*
