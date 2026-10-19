# AUDD Comparison Report

**Generated:** 1970-01-01 00:00:00 UTC  
**Schema A:** growvtravel  
**Schema B:** growventa  
**Report Version:** 1.0.0

---

## Executive Summary

### Compatibility Overview

- **Matches**: 17
- **Exclusives**: 180 (95 from A, 85 from B)
- **Conflicts**: 4
- **Compatibility Score**: 81.0%
- **Safe Addition Rate**: 100.0%
- **Conflict Rate**: 19.0%

### Risk Assessment

**Overall Risk Level**: ⚠️ **Medium**

- Conflict rate is 19.0% (above 10% threshold)

### Top Conflicts by Entity

| Entity | Conflicts | Highest Severity |
|--------|-----------|------------------|
| settings | 3 | ⚠️ Medium |
| users | 1 | ⚠️ Medium |

---

## Detailed Breakdown

### Matches

- **Total**: 17
- **Exact name matches**: 0
- **Average confidence**: 1.00

### Exclusives

- **From Schema A**: 95 (95 safe to add)
- **From Schema B**: 85 (85 safe to add)
- **Safe Addition Rate**: 100.0%

### Conflicts

- **Total**: 4
- **By Type**:
  - NullabilityMismatch: 4
- **By Severity**:
  - Medium: 4

---

## Technical Details

### Matches

| Entity | Field | Match Type | Score | Index A | Index B |
|--------|-------|------------|-------|---------|---------|
| customers | (entity) | semantic_match | 0.98 | 11 | 4 |
| customers | name | semantic_match | 1.00 | 2 | 3 |
| customers | phone | semantic_match | 1.00 | 3 | 4 |
| customers | email | semantic_match | 1.00 | 4 | 5 |
| customers | created_at | semantic_match | 1.00 | 7 | 1 |
| customers | updated_at | semantic_match | 1.00 | 8 | 2 |
| settings | (entity) | semantic_match | 0.98 | 57 | 35 |
| settings | name | semantic_match | 1.00 | 1 | 8 |
| settings | address | semantic_match | 1.00 | 2 | 10 |
| settings | phone | semantic_match | 1.00 | 3 | 9 |
| settings | created_at | semantic_match | 1.00 | 12 | 1 |
| settings | updated_at | semantic_match | 1.00 | 13 | 2 |
| users | (entity) | semantic_match | 0.96 | 67 | 42 |
| users | email | semantic_match | 1.00 | 2 | 5 |
| users | password | semantic_match | 1.00 | 4 | 4 |
| users | created_at | semantic_match | 1.00 | 12 | 1 |
| users | updated_at | semantic_match | 1.00 | 13 | 2 |

### Exclusives

| Entity | Field | Side | Safe to Add | Index |
|--------|-------|------|-------------|-------|
| customers | id | A | ✅ Yes | 0 |
| customers | public_token | A | ✅ Yes | 1 |
| customers | birthdate | A | ✅ Yes | 5 |
| customers | credit_days | A | ✅ Yes | 6 |
| customers | classification_id | A | ✅ Yes | 9 |
| customers | UNIQUE | A | ✅ Yes | 10 |
| customers | KEY | A | ✅ Yes | 11 |
| customers | object_id | B | ✅ Yes | 0 |
| customers | discount | B | ✅ Yes | 6 |
| customers | credit | B | ✅ Yes | 7 |
| customers | points | B | ✅ Yes | 8 |
| customers | street | B | ✅ Yes | 9 |
| customers | exterior_number | B | ✅ Yes | 10 |
| customers | interior_number | B | ✅ Yes | 11 |
| customers | city | B | ✅ Yes | 12 |
| customers | state | B | ✅ Yes | 13 |
| customers | country | B | ✅ Yes | 14 |
| customers | postal_code | B | ✅ Yes | 15 |
| settings | id | A | ✅ Yes | 0 |
| settings | email | A | ✅ Yes | 4 |
| settings | terms | A | ✅ Yes | 5 |
| settings | reservation_prefix | A | ✅ Yes | 6 |
| settings | reservation_format | A | ✅ Yes | 7 |
| settings | reservation_id_pad_length | A | ✅ Yes | 8 |
| settings | profit_percentage | A | ✅ Yes | 9 |
| settings | profit_include_in_total | A | ✅ Yes | 10 |
| settings | enable_birthdate | A | ✅ Yes | 11 |
| settings | UNIQUE | A | ✅ Yes | 14 |
| settings | object_id | B | ✅ Yes | 0 |
| settings | small_logo | B | ✅ Yes | 3 |
| settings | big_logo | B | ✅ Yes | 4 |
| settings | print_logo | B | ✅ Yes | 5 |
| settings | principal_color | B | ✅ Yes | 6 |
| settings | secondary_color | B | ✅ Yes | 7 |
| settings | bank_account | B | ✅ Yes | 11 |
| settings | bank_name | B | ✅ Yes | 12 |
| settings | bank_clabe | B | ✅ Yes | 13 |
| settings | name_quote | B | ✅ Yes | 14 |
| settings | terms_quote | B | ✅ Yes | 15 |
| settings | transfer_comment | B | ✅ Yes | 16 |
| settings | max_credit | B | ✅ Yes | 17 |
| settings | discount_customer | B | ✅ Yes | 18 |
| settings | discount_order | B | ✅ Yes | 19 |
| settings | enable_discount | B | ✅ Yes | 20 |
| settings | mandatory_reference | B | ✅ Yes | 21 |
| settings | usd_price | B | ✅ Yes | 22 |
| settings | enable_bailment | B | ✅ Yes | 23 |
| settings | tax_included | B | ✅ Yes | 24 |
| settings | is_restaurant | B | ✅ Yes | 25 |
| settings | currency | B | ✅ Yes | 26 |
| settings | second_currency | B | ✅ Yes | 27 |
| settings | costing_strategy | B | ✅ Yes | 28 |
| settings | costing_last_n | B | ✅ Yes | 29 |
| settings | enable_batches | B | ✅ Yes | 30 |
| settings | points_perc | B | ✅ Yes | 31 |
| settings | quotation_folio_prefix | B | ✅ Yes | 32 |
| settings | quotation_folio_min_length | B | ✅ Yes | 33 |
| users | id | A | ✅ Yes | 0 |
| users | name | A | ✅ Yes | 1 |
| users | email_verified_at | A | ✅ Yes | 3 |
| users | phone | A | ✅ Yes | 5 |
| users | commission | A | ✅ Yes | 6 |
| users | birthdate | A | ✅ Yes | 7 |
| users | start_date | A | ✅ Yes | 8 |
| users | position | A | ✅ Yes | 9 |
| users | weekly_salary | A | ✅ Yes | 10 |
| users | remember_token | A | ✅ Yes | 11 |
| users | UNIQUE | A | ✅ Yes | 14 |
| users | object_id | B | ✅ Yes | 0 |
| users | username | B | ✅ Yes | 3 |
| users | admin_permission | B | ✅ Yes | 6 |
| users | pin | B | ✅ Yes | 7 |
| airlines | (entity) | A | ✅ Yes | 0 |
| bank_accounts | (entity) | A | ✅ Yes | 1 |
| benefit_transaction_settings | (entity) | A | ✅ Yes | 2 |
| bonus_transaction_settings | (entity) | A | ✅ Yes | 3 |
| cache | (entity) | A | ✅ Yes | 4 |
| cache_locks | (entity) | A | ✅ Yes | 5 |
| channels | (entity) | A | ✅ Yes | 6 |
| classifications | (entity) | A | ✅ Yes | 7 |
| comments | (entity) | A | ✅ Yes | 8 |
| commission_transaction_settings | (entity) | A | ✅ Yes | 9 |
| currencies | (entity) | A | ✅ Yes | 10 |
| customer_classifications | (entity) | A | ✅ Yes | 12 |
| customer_megatravel_interests | (entity) | A | ✅ Yes | 13 |
| destinations | (entity) | A | ✅ Yes | 14 |
| events | (entity) | A | ✅ Yes | 15 |
| failed_jobs | (entity) | A | ✅ Yes | 16 |
| files | (entity) | A | ✅ Yes | 17 |
| groups | (entity) | A | ✅ Yes | 18 |
| group_products | (entity) | A | ✅ Yes | 19 |
| group_services | (entity) | A | ✅ Yes | 20 |
| hotels | (entity) | A | ✅ Yes | 21 |
| jobs | (entity) | A | ✅ Yes | 22 |
| job_batches | (entity) | A | ✅ Yes | 23 |
| leads | (entity) | A | ✅ Yes | 24 |
| lead_groups | (entity) | A | ✅ Yes | 25 |
| lead_products | (entity) | A | ✅ Yes | 26 |
| lead_profits | (entity) | A | ✅ Yes | 27 |
| lead_services | (entity) | A | ✅ Yes | 28 |
| megatravel_categories | (entity) | A | ✅ Yes | 29 |
| megatravel_offers | (entity) | A | ✅ Yes | 30 |
| megatravel_offer_category | (entity) | A | ✅ Yes | 31 |
| migrations | (entity) | A | ✅ Yes | 32 |
| occupations | (entity) | A | ✅ Yes | 33 |
| password_reset_tokens | (entity) | A | ✅ Yes | 34 |
| payments | (entity) | A | ✅ Yes | 35 |
| payment_attempts | (entity) | A | ✅ Yes | 36 |
| payment_installments | (entity) | A | ✅ Yes | 37 |
| payment_requests | (entity) | A | ✅ Yes | 38 |
| payment_settings | (entity) | A | ✅ Yes | 39 |
| payment_supplier_settings | (entity) | A | ✅ Yes | 40 |
| payroll_payments | (entity) | A | ✅ Yes | 41 |
| payroll_transaction_settings | (entity) | A | ✅ Yes | 42 |
| periods | (entity) | A | ✅ Yes | 43 |
| personal_access_tokens | (entity) | A | ✅ Yes | 44 |
| pos_custom_field_definitions | (entity) | A | ✅ Yes | 45 |
| pos_custom_field_values | (entity) | A | ✅ Yes | 46 |
| preleads | (entity) | A | ✅ Yes | 47 |
| profit_distributions | (entity) | A | ✅ Yes | 48 |
| reminders | (entity) | A | ✅ Yes | 49 |
| retained_profit_settings | (entity) | A | ✅ Yes | 50 |
| roles | (entity) | A | ✅ Yes | 51 |
| role_screens | (entity) | A | ✅ Yes | 52 |
| services | (entity) | A | ✅ Yes | 53 |
| service_categories | (entity) | A | ✅ Yes | 54 |
| service_types | (entity) | A | ✅ Yes | 55 |
| sessions | (entity) | A | ✅ Yes | 56 |
| stages | (entity) | A | ✅ Yes | 58 |
| subsidiaries | (entity) | A | ✅ Yes | 59 |
| supplier_group_payments | (entity) | A | ✅ Yes | 60 |
| supplier_payments | (entity) | A | ✅ Yes | 61 |
| surveys | (entity) | A | ✅ Yes | 62 |
| survey_answers | (entity) | A | ✅ Yes | 63 |
| survey_questions | (entity) | A | ✅ Yes | 64 |
| tags | (entity) | A | ✅ Yes | 65 |
| transactions | (entity) | A | ✅ Yes | 66 |
| user_roles | (entity) | A | ✅ Yes | 68 |
| vacations | (entity) | A | ✅ Yes | 69 |
| Cash_movements | (entity) | B | ✅ Yes | 0 |
| Cashcut | (entity) | B | ✅ Yes | 1 |
| CashregisterLog | (entity) | B | ✅ Yes | 2 |
| CatalogProduct | (entity) | B | ✅ Yes | 3 |
| Department | (entity) | B | ✅ Yes | 5 |
| Department_Printer | (entity) | B | ✅ Yes | 6 |
| Expense | (entity) | B | ✅ Yes | 7 |
| Inventory | (entity) | B | ✅ Yes | 8 |
| InventoryLog | (entity) | B | ✅ Yes | 9 |
| Inventory_movements | (entity) | B | ✅ Yes | 10 |
| License | (entity) | B | ✅ Yes | 11 |
| Multibox | (entity) | B | ✅ Yes | 12 |
| Order_detail | (entity) | B | ✅ Yes | 13 |
| Order_detail_options | (entity) | B | ✅ Yes | 14 |
| Order_payment | (entity) | B | ✅ Yes | 15 |
| Order_tax | (entity) | B | ✅ Yes | 16 |
| Orders | (entity) | B | ✅ Yes | 17 |
| Payment_methods | (entity) | B | ✅ Yes | 18 |
| Printer | (entity) | B | ✅ Yes | 19 |
| Product | (entity) | B | ✅ Yes | 20 |
| ProductLog | (entity) | B | ✅ Yes | 21 |
| Product_Product | (entity) | B | ✅ Yes | 22 |
| Product_Tax | (entity) | B | ✅ Yes | 23 |
| Product_cost | (entity) | B | ✅ Yes | 24 |
| Product_option_assignment | (entity) | B | ✅ Yes | 25 |
| Product_option_detail | (entity) | B | ✅ Yes | 26 |
| Product_options | (entity) | B | ✅ Yes | 27 |
| Purchase_order_details | (entity) | B | ✅ Yes | 28 |
| Purchase_orders | (entity) | B | ✅ Yes | 29 |
| Quotation | (entity) | B | ✅ Yes | 30 |
| Quotation_detail | (entity) | B | ✅ Yes | 31 |
| Quotation_detail_options | (entity) | B | ✅ Yes | 32 |
| Recipe_items | (entity) | B | ✅ Yes | 33 |
| Restaurant_tables | (entity) | B | ✅ Yes | 34 |
| Setting_discount | (entity) | B | ✅ Yes | 36 |
| Setting_local | (entity) | B | ✅ Yes | 37 |
| Suppliers | (entity) | B | ✅ Yes | 38 |
| Tax | (entity) | B | ✅ Yes | 39 |
| Tour | (entity) | B | ✅ Yes | 40 |
| Unit | (entity) | B | ✅ Yes | 41 |
| Warehouse | (entity) | B | ✅ Yes | 43 |

### Conflicts

#### <a name="conflict-1"></a>Conflict #1

- **Entity**: settings
- **Field**: name
- **Type**: NullabilityMismatch
- **Severity**: ⚠️ Medium
- **Evidence**:
  - **From Schema A**: Nullable: false
  - **From Schema B**: Nullable: true
  - **Rule**: Nullability must match or be compatible
- **Indexes**: A=1, B=8

#### <a name="conflict-2"></a>Conflict #2

- **Entity**: settings
- **Field**: address
- **Type**: NullabilityMismatch
- **Severity**: ⚠️ Medium
- **Evidence**:
  - **From Schema A**: Nullable: false
  - **From Schema B**: Nullable: true
  - **Rule**: Nullability must match or be compatible
- **Indexes**: A=2, B=10

#### <a name="conflict-3"></a>Conflict #3

- **Entity**: settings
- **Field**: phone
- **Type**: NullabilityMismatch
- **Severity**: ⚠️ Medium
- **Evidence**:
  - **From Schema A**: Nullable: false
  - **From Schema B**: Nullable: true
  - **Rule**: Nullability must match or be compatible
- **Indexes**: A=3, B=9

#### <a name="conflict-4"></a>Conflict #4

- **Entity**: users
- **Field**: email
- **Type**: NullabilityMismatch
- **Severity**: ⚠️ Medium
- **Evidence**:
  - **From Schema A**: Nullable: false
  - **From Schema B**: Nullable: true
  - **Rule**: Nullability must match or be compatible
- **Indexes**: A=2, B=5

---

## Resolution Suggestions

### Summary

- **Total Suggestions**: 4
- **High Confidence** (≥0.85): 4
- **Medium Confidence** (0.60-0.84): 0
- **Auto-accepted**: 4

### Conflict #1 - Suggestions

#### Suggestion: sug_0

- **Kind**: PreferType { preferred_type: "Nullable", alternative_type: "alternative", rule: "R-NULL-001" }
- **Confidence**: 0.90 (High)
- **Impact**: Medium
- **Explanation**: Acepta los datos de ambas fuentes
- **Evidence**:
  - catalog:R-NULL-001
- **Status**: ✅ Accepted

### Conflict #2 - Suggestions

#### Suggestion: sug_1

- **Kind**: PreferType { preferred_type: "Nullable", alternative_type: "alternative", rule: "R-NULL-001" }
- **Confidence**: 0.90 (High)
- **Impact**: Medium
- **Explanation**: Acepta los datos de ambas fuentes
- **Evidence**:
  - catalog:R-NULL-001
- **Status**: ✅ Accepted

### Conflict #3 - Suggestions

#### Suggestion: sug_2

- **Kind**: PreferType { preferred_type: "Nullable", alternative_type: "alternative", rule: "R-NULL-001" }
- **Confidence**: 0.90 (High)
- **Impact**: Medium
- **Explanation**: Acepta los datos de ambas fuentes
- **Evidence**:
  - catalog:R-NULL-001
- **Status**: ✅ Accepted

### Conflict #4 - Suggestions

#### Suggestion: sug_3

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

- **Total Decisions**: 4
- **Accepted**: 4 (100.0%)
- **Rejected**: 0 (0.0%)
- **By Source**:
  - System (auto): 4
  - User (manual): 0

### Decisions

#### Decision: auto_dec_1

- **Suggestion**: sug_0 (settings::name)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: 1970-01-01 00:00:00 UTC

#### Decision: auto_dec_2

- **Suggestion**: sug_1 (settings::address)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: 1970-01-01 00:00:00 UTC

#### Decision: auto_dec_3

- **Suggestion**: sug_2 (settings::phone)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: 1970-01-01 00:00:00 UTC

#### Decision: auto_dec_4

- **Suggestion**: sug_3 (users::email)
- **Status**: ✅ Accepted
- **Source**: System (high_confidence_auto_accept)
- **Rationale**: Automated by rule: high_confidence_auto_accept
- **Timestamp**: 1970-01-01 00:00:00 UTC

---

## Recommendations

- ⚠️ Review medium and high-severity conflicts before unification
- ✅ High compatibility score - schemas align well
- 📄 See `decision_log.json` for complete decision history and programmatic access


---

*Report generated by AUDD v0.1.0*
