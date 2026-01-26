# AUDD Report Structure

**Version:** 1.0.0  
**Status:** Stable

## Overview

This document defines the structure and format of AUDD comparison reports. Reports are generated in Markdown format (with optional JSON export) and consist of two main sections:

1. **Executive Report** - High-level summary for quick decision-making
2. **Technical Report** - Detailed evidence and traceability for auditing

## Report Sections

### 1. Executive Summary

Quick overview of the comparison results with key metrics and indicators.

#### 1.1 Header
- Report title
- Generation timestamp
- Schema identifiers (A and B)
- Report version

#### 1.2 Compatibility Overview
High-level metrics:
- **Total Matches**: Number of compatible fields/entities
- **Safe Exclusives**: Fields that can be safely added to unified schema
- **Total Conflicts**: Number of incompatibilities requiring resolution
- **Compatibility Score**: Percentage of successful matches

**Formula:**
```
Compatibility Score = (Matches / (Matches + Conflicts)) × 100%
Safe Addition Rate = (Safe Exclusives / Total Exclusives) × 100%
Conflict Rate = (Conflicts / (Matches + Conflicts)) × 100%
```

#### 1.3 Risk Assessment
- **Overall Risk Level**: Low | Medium | High | Critical
  - Low: < 10% conflict rate, no critical severity conflicts
  - Medium: 10-25% conflict rate OR has high severity conflicts
  - High: 25-50% conflict rate OR has critical severity conflicts
  - Critical: > 50% conflict rate OR multiple critical conflicts

#### 1.4 Top Conflicts by Entity
Table showing entities with most conflicts (top 5 or all if fewer):
- Entity name
- Number of conflicts
- Highest severity level
- Resolution status (if suggestions exist)

### 2. Detailed Breakdown

Statistics broken down by category.

#### 2.1 Matches
- Total count
- Breakdown by match reason (exact, normalized, similarity)
- Average match confidence score

#### 2.2 Exclusives
- Total count from Schema A
- Total count from Schema B
- Count marked as safe to add
- Count requiring review

#### 2.3 Conflicts
- Total count
- Breakdown by conflict type (type_incompatible, nullability_mismatch, etc.)
- Breakdown by severity (low, medium, high, critical)

### 3. Technical Details

Complete listing for audit trail and traceability.

#### 3.1 Matches Listing
Table format:
| Entity | Field | Match Type | Score | Index A | Index B |
|--------|-------|------------|-------|---------|---------|

#### 3.2 Exclusives Listing
Table format:
| Entity | Field | Side | Safe to Add | Index |
|--------|-------|------|-------------|-------|

#### 3.3 Conflicts Listing
For each conflict:
- **Conflict ID**: Sequential number for reference
- **Entity**: Entity name
- **Field**: Field name (if applicable)
- **Type**: Conflict type
- **Severity**: Severity level
- **Evidence**:
  - From Schema A: Description
  - From Schema B: Description
  - Rule: Rule that was violated
- **Indexes**: Location in schemas (A, B)

### 4. Resolution Suggestions (if available)

For each conflict with suggestions:

#### 4.1 Suggestions Table
| Conflict ID | Suggestion | Confidence | Impact | Status |
|-------------|------------|------------|--------|--------|

#### 4.2 Detailed Suggestions
For each suggestion:
- **Suggestion ID**: Unique identifier
- **For Conflict**: Reference to conflict ID
- **Kind**: Type of suggestion (cast_safe, cast_risky, rename_field, etc.)
- **Confidence**: Score (0.0-1.0)
- **Impact**: Minimal | Low | Medium | High | Critical
- **Explanation**: Human-readable description
- **Evidence**: Supporting facts

### 5. Decision Log (if available)

Summary of resolution decisions:
- **Total Decisions**: Count
- **Accepted**: Count and percentage
- **Rejected**: Count and percentage
- **Auto-accepted**: Decisions made by system (high confidence)
- **Manual**: Decisions made by users

For each decision:
- **Decision ID**: Unique identifier
- **Suggestion**: Reference to suggestion
- **Status**: Accepted/Rejected
- **Source**: System or User
- **Rationale**: Reason for decision
- **Timestamp**: When decision was made

### 6. Recommendations

Actionable next steps based on analysis:
- If conflicts exist: Suggest reviewing high-severity conflicts first
- If many exclusives: Recommend reviewing safe_to_add flags
- If low compatibility: Suggest manual schema alignment
- Reference to decision_log.json for programmatic access

## Output Formats

### Markdown (Primary)
- File: `report.md`
- Human-readable with tables and formatting
- Includes internal anchors for navigation
- Self-contained (understandable without JSON files)

### JSON (Optional)
- File: `report.json`
- Structured data for programmatic consumption
- Same sections as Markdown
- Includes all raw data

## Conventions

### Formatting
- Use tables for structured data
- Use badges/emojis for status indicators:
  - ✅ Success/Accepted
  - ⚠️ Warning/Medium severity
  - ❌ Error/Rejected
  - 🔍 Info/Review needed
  - ⚡ High priority
  - 🔥 Critical

### Naming
- Section headers use title case
- Field names use snake_case in technical sections
- Entity names preserve original casing

### Anchors
- Each major section has an anchor: `#executive-summary`, `#technical-details`, etc.
- Conflicts have anchors: `#conflict-1`, `#conflict-2`, etc.

## Example Output Mock

```markdown
# AUDD Comparison Report

**Generated:** 2024-01-15 14:30:00 UTC  
**Schema A:** users.csv  
**Schema B:** users.json  
**Report Version:** 1.0.0

---

## Executive Summary

### Compatibility Overview

- **Matches**: 6
- **Exclusives**: 4 (3 from A, 1 from B)
- **Conflicts**: 3
- **Compatibility Score**: 66.7%
- **Safe Addition Rate**: 75.0%
- **Conflict Rate**: 33.3%

### Risk Assessment

**Overall Risk Level**: ⚠️ **Medium**

- Conflict rate is 33.3% (above 25% threshold)
- 1 high-severity conflict detected
- 2 medium-severity conflicts detected

### Top Conflicts by Entity

| Entity | Conflicts | Highest Severity | Status |
|--------|-----------|------------------|--------|
| users  | 3         | High             | 2 suggestions available |

---

## Detailed Breakdown

### Matches
- **Total**: 6
- **Exact name matches**: 5
- **Normalized matches**: 1
- **Average confidence**: 0.95

### Exclusives
- **From Schema A**: 3 (2 safe to add)
- **From Schema B**: 1 (1 safe to add)
- **Safe Addition Rate**: 75.0%

### Conflicts
- **Total**: 3
- **By Type**:
  - Type incompatible: 2
  - Nullability mismatch: 1
- **By Severity**:
  - High: 1
  - Medium: 2

---

## Technical Details

### Matches

| Entity | Field | Match Type | Score | Index A | Index B |
|--------|-------|------------|-------|---------|---------|
| users  | id    | exact_name | 1.00  | 0       | 0       |
| users  | name  | exact_name | 1.00  | 1       | 1       |
...

### Exclusives

| Entity | Field    | Side | Safe to Add | Index |
|--------|----------|------|-------------|-------|
| users  | password | A    | ✅ Yes      | 5     |
...

### Conflicts

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

## Resolution Suggestions

### Summary
- **Total Suggestions**: 2
- **High Confidence**: 1
- **Medium Confidence**: 1
- **Auto-accepted**: 1

### Conflict #1 - Suggestions

#### Suggestion: sug_001
- **Kind**: prefer_type
- **Confidence**: 0.90 (High)
- **Impact**: Medium
- **Explanation**: Prefer Int32 type, cast String values during migration
- **Evidence**:
  - Preferred type: Int32
  - Alternative type: String
  - Rule: Prefer numeric types for age fields
- **Status**: ✅ Auto-accepted

...

---

## Decision Log

### Summary
- **Total Decisions**: 1
- **Accepted**: 1 (100%)
- **Rejected**: 0 (0%)
- **By Source**:
  - System (auto): 1
  - User (manual): 0

### Decisions

#### Decision: dec_001
- **Suggestion**: sug_001 (Conflict #1)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Confidence 0.90 exceeds threshold 0.85
- **Timestamp**: 2024-01-15 14:30:05 UTC

---

## Recommendations

- 🔍 Review remaining 2 conflicts without auto-accepted suggestions
- ⚠️ High-severity conflict in users.age requires manual review
- ✅ Most fields are compatible (66.7% compatibility)
- 📄 See decision_log.json for complete decision history

---

*Report generated by AUDD v0.1.0*
```

## Metrics and Indicators

### Core Metrics
1. **Compatibility Score**: Success rate of field matching
2. **Conflict Rate**: Percentage of fields with conflicts
3. **Safe Addition Rate**: Percentage of exclusives safe to add
4. **Resolution Rate**: Percentage of conflicts with accepted suggestions

### Quality Indicators
1. **Average Match Confidence**: Mean confidence of all matches
2. **Suggestion Coverage**: Percentage of conflicts with suggestions
3. **Auto-acceptance Rate**: Percentage of suggestions auto-accepted

### Risk Indicators
1. **Critical Conflict Count**: Number of critical severity conflicts
2. **High Severity Rate**: Percentage of high/critical conflicts
3. **Unresolved Conflict Count**: Conflicts without accepted suggestions

## Maintenance

This structure is versioned and should remain stable. Changes require:
1. Version increment
2. Backward compatibility considerations
3. Update to golden test files
4. Documentation of migration path

## References

- EPIC 03: Comparison engine
- EPIC 04: Suggestions and resolution
- EPIC 07: CLI outputs
