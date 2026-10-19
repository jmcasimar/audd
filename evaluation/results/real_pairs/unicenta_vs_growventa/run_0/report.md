# AUDD Comparison Report

**Generated:** 1970-01-01 00:00:00 UTC  
**Schema A:** MySQL-create  
**Schema B:** growventa  
**Report Version:** 1.0.0

---

## Executive Summary

### Compatibility Overview

- **Matches**: 12
- **Exclusives**: 183 (103 from A, 80 from B)
- **Conflicts**: 0
- **Compatibility Score**: 100.0%
- **Safe Addition Rate**: 100.0%
- **Conflict Rate**: 0.0%

### Risk Assessment

**Overall Risk Level**: ✅ **Low**

- Low conflict rate and no high-severity issues

---

## Detailed Breakdown

### Matches

- **Total**: 12
- **Exact name matches**: 0
- **Average confidence**: 0.98

### Exclusives

- **From Schema A**: 103 (103 safe to add)
- **From Schema B**: 80 (80 safe to add)
- **Safe Addition Rate**: 100.0%

### Conflicts

- **Total**: 0

---

## Technical Details

### Matches

| Entity | Field | Match Type | Score | Index A | Index B |
|--------|-------|------------|-------|---------|---------|
| CUSTOMERS | (entity) | semantic_match | 0.98 | 5 | 4 |
| CUSTOMERS | NAME | semantic_match | 1.00 | 3 | 3 |
| CUSTOMERS | CITY | semantic_match | 1.00 | 10 | 12 |
| CUSTOMERS | COUNTRY | semantic_match | 1.00 | 12 | 14 |
| CUSTOMERS | EMAIL | semantic_match | 1.00 | 15 | 5 |
| CUSTOMERS | PHONE | semantic_match | 1.00 | 16 | 4 |
| TAXES | (entity) | semantic_match | 0.92 | 8 | 39 |
| TAXES | NAME | semantic_match | 1.00 | 1 | 3 |
| PRODUCTS | (entity) | semantic_match | 0.98 | 15 | 20 |
| PRODUCTS | CODE | semantic_match | 1.00 | 2 | 3 |
| PRODUCTS | NAME | semantic_match | 1.00 | 4 | 4 |
| PRODUCTS | (entity) | semantic_match | 0.87 | 15 | 22 |

### Exclusives

| Entity | Field | Side | Safe to Add | Index |
|--------|-------|------|-------------|-------|
| CUSTOMERS | ID | A | ✅ Yes | 0 |
| CUSTOMERS | SEARCHKEY | A | ✅ Yes | 1 |
| CUSTOMERS | TAXID | A | ✅ Yes | 2 |
| CUSTOMERS | TAXCATEGORY | A | ✅ Yes | 4 |
| CUSTOMERS | CARD | A | ✅ Yes | 5 |
| CUSTOMERS | MAXDEBT | A | ✅ Yes | 6 |
| CUSTOMERS | ADDRESS | A | ✅ Yes | 7 |
| CUSTOMERS | ADDRESS2 | A | ✅ Yes | 8 |
| CUSTOMERS | POSTAL | A | ✅ Yes | 9 |
| CUSTOMERS | REGION | A | ✅ Yes | 11 |
| CUSTOMERS | FIRSTNAME | A | ✅ Yes | 13 |
| CUSTOMERS | LASTNAME | A | ✅ Yes | 14 |
| CUSTOMERS | PHONE2 | A | ✅ Yes | 17 |
| CUSTOMERS | FAX | A | ✅ Yes | 18 |
| CUSTOMERS | NOTES | A | ✅ Yes | 19 |
| CUSTOMERS | VISIBLE | A | ✅ Yes | 20 |
| CUSTOMERS | CURDATE | A | ✅ Yes | 21 |
| CUSTOMERS | CURDEBT | A | ✅ Yes | 22 |
| CUSTOMERS | object_id | B | ✅ Yes | 0 |
| CUSTOMERS | created_at | B | ✅ Yes | 1 |
| CUSTOMERS | updated_at | B | ✅ Yes | 2 |
| CUSTOMERS | discount | B | ✅ Yes | 6 |
| CUSTOMERS | credit | B | ✅ Yes | 7 |
| CUSTOMERS | points | B | ✅ Yes | 8 |
| CUSTOMERS | street | B | ✅ Yes | 9 |
| CUSTOMERS | exterior_number | B | ✅ Yes | 10 |
| CUSTOMERS | interior_number | B | ✅ Yes | 11 |
| CUSTOMERS | state | B | ✅ Yes | 13 |
| CUSTOMERS | postal_code | B | ✅ Yes | 15 |
| TAXES | ID | A | ✅ Yes | 0 |
| TAXES | CATEGORY | A | ✅ Yes | 2 |
| TAXES | CUSTCATEGORY | A | ✅ Yes | 3 |
| TAXES | PARENTID | A | ✅ Yes | 4 |
| TAXES | RATE | A | ✅ Yes | 5 |
| TAXES | RATECASCADE | A | ✅ Yes | 6 |
| TAXES | RATEORDER | A | ✅ Yes | 7 |
| TAXES | object_id | B | ✅ Yes | 0 |
| TAXES | created_at | B | ✅ Yes | 1 |
| TAXES | updated_at | B | ✅ Yes | 2 |
| TAXES | tax_name | B | ✅ Yes | 4 |
| TAXES | value | B | ✅ Yes | 5 |
| TAXES | type | B | ✅ Yes | 6 |
| TAXES | withholding | B | ✅ Yes | 7 |
| TAXES | transfer | B | ✅ Yes | 8 |
| TAXES | apply_default | B | ✅ Yes | 9 |
| PRODUCTS | ID | A | ✅ Yes | 0 |
| PRODUCTS | REFERENCE | A | ✅ Yes | 1 |
| PRODUCTS | CODETYPE | A | ✅ Yes | 3 |
| PRODUCTS | PRICEBUY | A | ✅ Yes | 5 |
| PRODUCTS | PRICESELL | A | ✅ Yes | 6 |
| PRODUCTS | CATEGORY | A | ✅ Yes | 7 |
| PRODUCTS | TAXCAT | A | ✅ Yes | 8 |
| PRODUCTS | ATTRIBUTESET_ID | A | ✅ Yes | 9 |
| PRODUCTS | STOCKCOST | A | ✅ Yes | 10 |
| PRODUCTS | STOCKVOLUME | A | ✅ Yes | 11 |
| PRODUCTS | IMAGE | A | ✅ Yes | 12 |
| PRODUCTS | ISCOM | A | ✅ Yes | 13 |
| PRODUCTS | ISSCALE | A | ✅ Yes | 14 |
| PRODUCTS | ISKITCHEN | A | ✅ Yes | 15 |
| PRODUCTS | PRINTKB | A | ✅ Yes | 16 |
| PRODUCTS | SENDSTATUS | A | ✅ Yes | 17 |
| PRODUCTS | ISSERVICE | A | ✅ Yes | 18 |
| PRODUCTS | ATTRIBUTES | A | ✅ Yes | 19 |
| PRODUCTS | DISPLAY | A | ✅ Yes | 20 |
| PRODUCTS | object_id | B | ✅ Yes | 0 |
| PRODUCTS | created_at | B | ✅ Yes | 1 |
| PRODUCTS | updated_at | B | ✅ Yes | 2 |
| PRODUCTS | amount | B | ✅ Yes | 5 |
| PRODUCTS | price | B | ✅ Yes | 6 |
| PRODUCTS | cost | B | ✅ Yes | 7 |
| PRODUCTS | user_id | B | ✅ Yes | 8 |
| PRODUCTS | department_id | B | ✅ Yes | 9 |
| PRODUCTS | unit | B | ✅ Yes | 10 |
| PRODUCTS | url_image | B | ✅ Yes | 11 |
| PRODUCTS | is_supply | B | ✅ Yes | 12 |
| PRODUCTS | unit_purchase | B | ✅ Yes | 13 |
| PRODUCTS | unit_quantity | B | ✅ Yes | 14 |
| PRODUCTS | track_inventory | B | ✅ Yes | 15 |
| PRODUCTS | ID | A | ✅ Yes | 0 |
| PRODUCTS | REFERENCE | A | ✅ Yes | 1 |
| PRODUCTS | CODE | A | ✅ Yes | 2 |
| PRODUCTS | CODETYPE | A | ✅ Yes | 3 |
| PRODUCTS | NAME | A | ✅ Yes | 4 |
| PRODUCTS | PRICEBUY | A | ✅ Yes | 5 |
| PRODUCTS | PRICESELL | A | ✅ Yes | 6 |
| PRODUCTS | CATEGORY | A | ✅ Yes | 7 |
| PRODUCTS | TAXCAT | A | ✅ Yes | 8 |
| PRODUCTS | ATTRIBUTESET_ID | A | ✅ Yes | 9 |
| PRODUCTS | STOCKCOST | A | ✅ Yes | 10 |
| PRODUCTS | STOCKVOLUME | A | ✅ Yes | 11 |
| PRODUCTS | IMAGE | A | ✅ Yes | 12 |
| PRODUCTS | ISCOM | A | ✅ Yes | 13 |
| PRODUCTS | ISSCALE | A | ✅ Yes | 14 |
| PRODUCTS | ISKITCHEN | A | ✅ Yes | 15 |
| PRODUCTS | PRINTKB | A | ✅ Yes | 16 |
| PRODUCTS | SENDSTATUS | A | ✅ Yes | 17 |
| PRODUCTS | ISSERVICE | A | ✅ Yes | 18 |
| PRODUCTS | ATTRIBUTES | A | ✅ Yes | 19 |
| PRODUCTS | DISPLAY | A | ✅ Yes | 20 |
| PRODUCTS | object_id | B | ✅ Yes | 0 |
| PRODUCTS | created_at | B | ✅ Yes | 1 |
| PRODUCTS | updated_at | B | ✅ Yes | 2 |
| PRODUCTS | quantity | B | ✅ Yes | 3 |
| PRODUCTS | product_id | B | ✅ Yes | 4 |
| PRODUCTS | includes_product | B | ✅ Yes | 5 |
| APPLICATIONS | (entity) | A | ✅ Yes | 0 |
| ROLES | (entity) | A | ✅ Yes | 1 |
| PEOPLE | (entity) | A | ✅ Yes | 2 |
| RESOURCES | (entity) | A | ✅ Yes | 3 |
| TAXCUSTCATEGORIES | (entity) | A | ✅ Yes | 4 |
| CATEGORIES | (entity) | A | ✅ Yes | 6 |
| TAXCATEGORIES | (entity) | A | ✅ Yes | 7 |
| ATTRIBUTE | (entity) | A | ✅ Yes | 9 |
| ATTRIBUTEVALUE | (entity) | A | ✅ Yes | 10 |
| ATTRIBUTESET | (entity) | A | ✅ Yes | 11 |
| ATTRIBUTEUSE | (entity) | A | ✅ Yes | 12 |
| ATTRIBUTESETINSTANCE | (entity) | A | ✅ Yes | 13 |
| ATTRIBUTEINSTANCE | (entity) | A | ✅ Yes | 14 |
| PRODUCTS_CAT | (entity) | A | ✅ Yes | 16 |
| PRODUCTS_COM | (entity) | A | ✅ Yes | 17 |
| LOCATIONS | (entity) | A | ✅ Yes | 18 |
| STOCKDIARY | (entity) | A | ✅ Yes | 19 |
| STOCKLEVEL | (entity) | A | ✅ Yes | 20 |
| STOCKCURRENT | (entity) | A | ✅ Yes | 21 |
| CLOSEDCASH | (entity) | A | ✅ Yes | 22 |
| RECEIPTS | (entity) | A | ✅ Yes | 23 |
| TICKETS | (entity) | A | ✅ Yes | 24 |
| TICKETSNUM | (entity) | A | ✅ Yes | 25 |
| TICKETSNUM_REFUND | (entity) | A | ✅ Yes | 26 |
| TICKETSNUM_PAYMENT | (entity) | A | ✅ Yes | 27 |
| TICKETLINES | (entity) | A | ✅ Yes | 28 |
| PAYMENTS | (entity) | A | ✅ Yes | 29 |
| TAXLINES | (entity) | A | ✅ Yes | 30 |
| FLOORS | (entity) | A | ✅ Yes | 31 |
| PLACES | (entity) | A | ✅ Yes | 32 |
| RESERVATIONS | (entity) | A | ✅ Yes | 33 |
| RESERVATION_CUSTOMERS | (entity) | A | ✅ Yes | 34 |
| THIRDPARTIES | (entity) | A | ✅ Yes | 35 |
| SHAREDTICKETS | (entity) | A | ✅ Yes | 36 |
| SHIFTS | (entity) | A | ✅ Yes | 37 |
| LEAVES | (entity) | A | ✅ Yes | 38 |
| BREAKS | (entity) | A | ✅ Yes | 39 |
| SHIFT_BREAKS | (entity) | A | ✅ Yes | 40 |
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
| ProductLog | (entity) | B | ✅ Yes | 21 |
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
| Setting | (entity) | B | ✅ Yes | 35 |
| Setting_discount | (entity) | B | ✅ Yes | 36 |
| Setting_local | (entity) | B | ✅ Yes | 37 |
| Suppliers | (entity) | B | ✅ Yes | 38 |
| Tour | (entity) | B | ✅ Yes | 40 |
| Unit | (entity) | B | ✅ Yes | 41 |
| User | (entity) | B | ✅ Yes | 42 |
| Warehouse | (entity) | B | ✅ Yes | 43 |

---

## Recommendations

- ✅ Low risk detected - schemas are mostly compatible
- ✅ High compatibility score - schemas align well
- 📄 See `decision_log.json` for complete decision history and programmatic access


---

*Report generated by AUDD v0.1.0*
