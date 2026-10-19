# AUDD Comparison Report

**Generated:** 1970-01-01 00:00:00 UTC  
**Schema A:** a  
**Schema B:** b  
**Report Version:** 1.0.0

---

## Executive Summary

### Compatibility Overview

- **Matches**: 535
- **Exclusives**: 37 (19 from A, 18 from B)
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

- **Total**: 535
- **Exact name matches**: 0
- **Average confidence**: 1.00

### Exclusives

- **From Schema A**: 19 (19 safe to add)
- **From Schema B**: 18 (18 safe to add)
- **Safe Addition Rate**: 100.0%

### Conflicts

- **Total**: 0

---

## Technical Details

### Matches

| Entity | Field | Match Type | Score | Index A | Index B |
|--------|-------|------------|-------|---------|---------|
| Cash_movements | (entity) | semantic_match | 1.00 | 0 | 0 |
| Cash_movements | object_id | semantic_match | 1.00 | 0 | 0 |
| Cash_movements | created_at | semantic_match | 1.00 | 1 | 1 |
| Cash_movements | updated_at | semantic_match | 1.00 | 2 | 2 |
| Cash_movements | box_id | semantic_match | 1.00 | 3 | 3 |
| Cash_movements | user_id | semantic_match | 1.00 | 4 | 4 |
| Cash_movements | type | semantic_match | 1.00 | 5 | 5 |
| Cash_movements | amount | semantic_match | 1.00 | 6 | 6 |
| Cash_movements | currency | semantic_match | 1.00 | 7 | 7 |
| Cash_movements | note | semantic_match | 1.00 | 8 | 8 |
| Cash_movements | metadata | semantic_match | 1.00 | 9 | 9 |
| Cash_movements | tip | semantic_match | 1.00 | 10 | 10 |
| Cashcut | (entity) | semantic_match | 1.00 | 1 | 1 |
| Cashcut | object_id | semantic_match | 1.00 | 0 | 0 |
| Cashcut | created_at | semantic_match | 1.00 | 1 | 1 |
| Cashcut | updated_at | semantic_match | 1.00 | 2 | 2 |
| Cashcut | quantity | semantic_match | 1.00 | 3 | 3 |
| Cashcut | cash | semantic_match | 1.00 | 4 | 4 |
| Cashcut | msg | semantic_match | 1.00 | 5 | 5 |
| Cashcut | open | semantic_match | 1.00 | 6 | 6 |
| Cashcut | box_id | semantic_match | 1.00 | 7 | 7 |
| Cashcut | user_id | semantic_match | 1.00 | 8 | 8 |
| CashregisterLog | (entity) | semantic_match | 1.00 | 2 | 2 |
| CashregisterLog | object_id | semantic_match | 1.00 | 0 | 0 |
| CashregisterLog | created_at | semantic_match | 1.00 | 1 | 1 |
| CashregisterLog | updated_at | semantic_match | 1.00 | 2 | 2 |
| CashregisterLog | quantity | semantic_match | 1.00 | 3 | 3 |
| CashregisterLog | cash | semantic_match | 1.00 | 4 | 4 |
| CashregisterLog | user_id | semantic_match | 1.00 | 5 | 5 |
| CatalogProduct | (entity) | semantic_match | 1.00 | 3 | 3 |
| CatalogProduct | object_id | semantic_match | 1.00 | 0 | 0 |
| CatalogProduct | created_at | semantic_match | 1.00 | 1 | 1 |
| CatalogProduct | updated_at | semantic_match | 1.00 | 2 | 2 |
| CatalogProduct | code | semantic_match | 1.00 | 3 | 3 |
| CatalogProduct | name | semantic_match | 1.00 | 4 | 4 |
| CatalogProduct | department | semantic_match | 1.00 | 5 | 5 |
| CatalogProduct | unit | semantic_match | 1.00 | 6 | 6 |
| CatalogProduct | image_url | semantic_match | 1.00 | 7 | 7 |
| Customer | (entity) | semantic_match | 1.00 | 4 | 4 |
| Customer | object_id | semantic_match | 1.00 | 0 | 0 |
| Customer | created_at | semantic_match | 1.00 | 1 | 1 |
| Customer | updated_at | semantic_match | 1.00 | 2 | 2 |
| Customer | name | semantic_match | 1.00 | 3 | 3 |
| Customer | phone | semantic_match | 1.00 | 4 | 4 |
| Customer | email | semantic_match | 1.00 | 5 | 5 |
| Customer | discount | semantic_match | 1.00 | 6 | 6 |
| Customer | credit | semantic_match | 1.00 | 7 | 7 |
| Customer | points | semantic_match | 1.00 | 8 | 8 |
| Customer | street | semantic_match | 1.00 | 9 | 9 |
| Customer | exterior_number | semantic_match | 1.00 | 10 | 10 |
| Customer | interior_number | semantic_match | 1.00 | 11 | 11 |
| Customer | city | semantic_match | 1.00 | 12 | 12 |
| Customer | state | semantic_match | 1.00 | 13 | 13 |
| Customer | country | semantic_match | 1.00 | 14 | 14 |
| Customer | postal_code | semantic_match | 1.00 | 15 | 15 |
| Department | (entity) | semantic_match | 1.00 | 5 | 5 |
| Department | object_id | semantic_match | 1.00 | 0 | 0 |
| Department | created_at | semantic_match | 1.00 | 1 | 1 |
| Department | updated_at | semantic_match | 1.00 | 2 | 2 |
| Department | name | semantic_match | 1.00 | 3 | 3 |
| Department | description | semantic_match | 1.00 | 4 | 4 |
| Department_Printer | (entity) | semantic_match | 1.00 | 6 | 6 |
| Department_Printer | department_id | semantic_match | 1.00 | 0 | 0 |
| Department_Printer | printer_id | semantic_match | 1.00 | 1 | 1 |
| Department_Printer | created_at | semantic_match | 1.00 | 2 | 2 |
| Department_Printer | updated_at | semantic_match | 1.00 | 3 | 3 |
| Expense | (entity) | semantic_match | 1.00 | 7 | 7 |
| Expense | object_id | semantic_match | 1.00 | 0 | 0 |
| Expense | created_at | semantic_match | 1.00 | 1 | 1 |
| Expense | updated_at | semantic_match | 1.00 | 2 | 2 |
| Expense | description | semantic_match | 1.00 | 3 | 3 |
| Expense | total | semantic_match | 1.00 | 4 | 4 |
| Expense | payment_method | semantic_match | 1.00 | 5 | 5 |
| Expense | box_id | semantic_match | 1.00 | 6 | 6 |
| Expense | user_id | semantic_match | 1.00 | 7 | 7 |
| Inventory | (entity) | semantic_match | 1.00 | 8 | 8 |
| Inventory | object_id | semantic_match | 1.00 | 0 | 0 |
| Inventory | created_at | semantic_match | 1.00 | 1 | 1 |
| Inventory | updated_at | semantic_match | 1.00 | 2 | 2 |
| Inventory | diff | semantic_match | 1.00 | 3 | 3 |
| Inventory | quantity | semantic_match | 1.00 | 4 | 4 |
| Inventory | min_quantity | semantic_match | 1.00 | 5 | 5 |
| Inventory | user_id | semantic_match | 1.00 | 6 | 6 |
| Inventory | warehouse_id | semantic_match | 1.00 | 7 | 7 |
| Inventory | product_id | semantic_match | 1.00 | 8 | 8 |
| InventoryLog | (entity) | semantic_match | 1.00 | 9 | 9 |
| InventoryLog | object_id | semantic_match | 1.00 | 0 | 0 |
| InventoryLog | created_at | semantic_match | 1.00 | 1 | 1 |
| InventoryLog | updated_at | semantic_match | 1.00 | 2 | 2 |
| InventoryLog | diff | semantic_match | 1.00 | 3 | 3 |
| InventoryLog | quantity | semantic_match | 1.00 | 4 | 4 |
| InventoryLog | user_id | semantic_match | 1.00 | 5 | 5 |
| InventoryLog | warehouse_id | semantic_match | 1.00 | 6 | 6 |
| InventoryLog | product_id | semantic_match | 1.00 | 7 | 7 |
| Inventory_movements | (entity) | semantic_match | 1.00 | 10 | 10 |
| Inventory_movements | object_id | semantic_match | 1.00 | 0 | 0 |
| Inventory_movements | created_at | semantic_match | 1.00 | 1 | 1 |
| Inventory_movements | updated_at | semantic_match | 1.00 | 2 | 2 |
| Inventory_movements | inventory_id | semantic_match | 1.00 | 3 | 3 |
| Inventory_movements | product_id | semantic_match | 1.00 | 4 | 4 |
| Inventory_movements | warehouse_id | semantic_match | 1.00 | 5 | 5 |
| Inventory_movements | user_id | semantic_match | 1.00 | 6 | 6 |
| Inventory_movements | qty | semantic_match | 1.00 | 7 | 7 |
| Inventory_movements | qty_before | semantic_match | 1.00 | 8 | 8 |
| Inventory_movements | qty_after | semantic_match | 1.00 | 9 | 9 |
| Inventory_movements | movement_type | semantic_match | 1.00 | 10 | 10 |
| Inventory_movements | movement_direction | semantic_match | 1.00 | 11 | 11 |
| Inventory_movements | reference_type | semantic_match | 1.00 | 12 | 12 |
| Inventory_movements | reference_id | semantic_match | 1.00 | 13 | 13 |
| Inventory_movements | source | semantic_match | 1.00 | 14 | 14 |
| Inventory_movements | notes | semantic_match | 1.00 | 15 | 15 |
| License | (entity) | semantic_match | 1.00 | 11 | 11 |
| License | object_id | semantic_match | 1.00 | 0 | 0 |
| License | created_at | semantic_match | 1.00 | 1 | 1 |
| License | updated_at | semantic_match | 1.00 | 2 | 2 |
| License | demo | semantic_match | 1.00 | 3 | 3 |
| License | expired_at | semantic_match | 1.00 | 4 | 4 |
| License | license_hash | semantic_match | 1.00 | 5 | 5 |
| License | first_license_hash | semantic_match | 1.00 | 6 | 6 |
| License | license_id | semantic_match | 1.00 | 7 | 7 |
| License | machine_id | semantic_match | 1.00 | 8 | 8 |
| License | issued_at | semantic_match | 1.00 | 9 | 9 |
| Multibox | (entity) | semantic_match | 1.00 | 12 | 12 |
| Multibox | object_id | semantic_match | 1.00 | 0 | 0 |
| Multibox | created_at | semantic_match | 1.00 | 1 | 1 |
| Multibox | updated_at | semantic_match | 1.00 | 2 | 2 |
| Multibox | type | semantic_match | 1.00 | 3 | 3 |
| Multibox | current_ip | semantic_match | 1.00 | 4 | 4 |
| Multibox | port | semantic_match | 1.00 | 5 | 5 |
| Multibox | principal_ip | semantic_match | 1.00 | 6 | 6 |
| Multibox | connected | semantic_match | 1.00 | 7 | 7 |
| Multibox | last_connection | semantic_match | 1.00 | 8 | 8 |
| Multibox | machine_id | semantic_match | 1.00 | 9 | 9 |
| Multibox | settings_id | semantic_match | 1.00 | 10 | 10 |
| Order_detail | (entity) | semantic_match | 1.00 | 13 | 13 |
| Order_detail | object_id | semantic_match | 1.00 | 0 | 0 |
| Order_detail | created_at | semantic_match | 1.00 | 1 | 1 |
| Order_detail | updated_at | semantic_match | 1.00 | 2 | 2 |
| Order_detail | code | semantic_match | 1.00 | 3 | 3 |
| Order_detail | quantity | semantic_match | 1.00 | 5 | 5 |
| Order_detail | price | semantic_match | 1.00 | 6 | 6 |
| Order_detail | discount | semantic_match | 1.00 | 7 | 7 |
| Order_detail | total | semantic_match | 1.00 | 8 | 8 |
| Order_detail | order_id | semantic_match | 1.00 | 9 | 9 |
| Order_detail | product_id | semantic_match | 1.00 | 10 | 10 |
| Order_detail | department_id | semantic_match | 1.00 | 11 | 11 |
| Order_detail | cost | semantic_match | 1.00 | 12 | 12 |
| Order_detail | unit | semantic_match | 1.00 | 13 | 13 |
| Order_detail | comments | semantic_match | 1.00 | 14 | 14 |
| Order_detail | is_prepared | semantic_match | 1.00 | 15 | 15 |
| Order_detail_options | (entity) | semantic_match | 1.00 | 14 | 14 |
| Order_detail_options | object_id | semantic_match | 1.00 | 0 | 0 |
| Order_detail_options | created_at | semantic_match | 1.00 | 1 | 1 |
| Order_detail_options | updated_at | semantic_match | 1.00 | 2 | 2 |
| Order_detail_options | order_detail_id | semantic_match | 1.00 | 3 | 3 |
| Order_detail_options | product_option_id | semantic_match | 1.00 | 4 | 4 |
| Order_detail_options | product_option_detail_id | semantic_match | 1.00 | 5 | 5 |
| Order_detail_options | name_snapshot | semantic_match | 1.00 | 6 | 6 |
| Order_detail_options | price_delta_snapshot | semantic_match | 1.00 | 7 | 7 |
| Order_payment | (entity) | semantic_match | 1.00 | 15 | 15 |
| Order_payment | object_id | semantic_match | 1.00 | 0 | 0 |
| Order_payment | created_at | semantic_match | 1.00 | 1 | 1 |
| Order_payment | updated_at | semantic_match | 1.00 | 2 | 2 |
| Order_payment | payment_method | semantic_match | 1.00 | 3 | 3 |
| Order_payment | reference | semantic_match | 1.00 | 4 | 4 |
| Order_payment | total | semantic_match | 1.00 | 5 | 5 |
| Order_payment | currency | semantic_match | 1.00 | 6 | 6 |
| Order_payment | currency_amount | semantic_match | 1.00 | 7 | 7 |
| Order_payment | exchange_rate | semantic_match | 1.00 | 8 | 8 |
| Order_payment | box_id | semantic_match | 1.00 | 9 | 9 |
| Order_payment | order_id | semantic_match | 1.00 | 10 | 10 |
| Order_payment | tip | semantic_match | 1.00 | 11 | 11 |
| Order_tax | (entity) | semantic_match | 1.00 | 16 | 16 |
| Order_tax | object_id | semantic_match | 1.00 | 0 | 0 |
| Order_tax | created_at | semantic_match | 1.00 | 1 | 1 |
| Order_tax | updated_at | semantic_match | 1.00 | 2 | 2 |
| Order_tax | total | semantic_match | 1.00 | 3 | 3 |
| Order_tax | order_id | semantic_match | 1.00 | 4 | 4 |
| Order_tax | product_id | semantic_match | 1.00 | 5 | 5 |
| Order_tax | order_array_id | semantic_match | 1.00 | 6 | 6 |
| Orders | (entity) | semantic_match | 1.00 | 17 | 17 |
| Orders | object_id | semantic_match | 1.00 | 0 | 0 |
| Orders | created_at | semantic_match | 1.00 | 1 | 1 |
| Orders | updated_at | semantic_match | 1.00 | 2 | 2 |
| Orders | total | semantic_match | 1.00 | 3 | 3 |
| Orders | toPay | semantic_match | 1.00 | 4 | 4 |
| Orders | discount | semantic_match | 1.00 | 5 | 5 |
| Orders | paid | semantic_match | 1.00 | 6 | 6 |
| Orders | layaway | semantic_match | 1.00 | 7 | 7 |
| Orders | credit | semantic_match | 1.00 | 8 | 8 |
| Orders | currency | semantic_match | 1.00 | 9 | 9 |
| Orders | box_id | semantic_match | 1.00 | 10 | 10 |
| Orders | user_id | semantic_match | 1.00 | 11 | 11 |
| Orders | customer_id | semantic_match | 1.00 | 12 | 12 |
| Orders | cost | semantic_match | 1.00 | 13 | 13 |
| Orders | table_id | semantic_match | 1.00 | 14 | 14 |
| Orders | tip | semantic_match | 1.00 | 15 | 15 |
| Orders | type | semantic_match | 1.00 | 16 | 16 |
| Orders | comments | semantic_match | 1.00 | 17 | 17 |
| Orders | points | semantic_match | 1.00 | 18 | 18 |
| Orders | identifier | semantic_match | 1.00 | 19 | 19 |
| Orders | is_cancellable | semantic_match | 1.00 | 20 | 20 |
| Payment_methods | (entity) | semantic_match | 1.00 | 18 | 18 |
| Payment_methods | object_id | semantic_match | 1.00 | 0 | 0 |
| Payment_methods | created_at | semantic_match | 1.00 | 1 | 1 |
| Payment_methods | updated_at | semantic_match | 1.00 | 2 | 2 |
| Payment_methods | label | semantic_match | 1.00 | 3 | 3 |
| Payment_methods | value | semantic_match | 1.00 | 4 | 4 |
| Payment_methods | is_active | semantic_match | 1.00 | 5 | 5 |
| Printer | (entity) | semantic_match | 1.00 | 19 | 19 |
| Printer | object_id | semantic_match | 1.00 | 0 | 0 |
| Printer | created_at | semantic_match | 1.00 | 1 | 1 |
| Printer | updated_at | semantic_match | 1.00 | 2 | 2 |
| Printer | name | semantic_match | 1.00 | 3 | 3 |
| Printer | role | semantic_match | 1.00 | 4 | 4 |
| Printer | is_default | semantic_match | 1.00 | 5 | 5 |
| Printer | is_enabled | semantic_match | 1.00 | 6 | 6 |
| Printer | connection_type | semantic_match | 1.00 | 7 | 7 |
| Printer | usb_mode | semantic_match | 1.00 | 8 | 8 |
| Printer | usb_vid | semantic_match | 1.00 | 9 | 9 |
| Printer | usb_pid | semantic_match | 1.00 | 10 | 10 |
| Printer | os_printer_name | semantic_match | 1.00 | 11 | 11 |
| Printer | net_host | semantic_match | 1.00 | 12 | 12 |
| Printer | net_port | semantic_match | 1.00 | 13 | 13 |
| Printer | ticket_copy | semantic_match | 1.00 | 14 | 14 |
| Printer | settings | semantic_match | 1.00 | 15 | 15 |
| Product | (entity) | semantic_match | 1.00 | 20 | 20 |
| Product | object_id | semantic_match | 1.00 | 0 | 0 |
| Product | created_at | semantic_match | 1.00 | 1 | 1 |
| Product | updated_at | semantic_match | 1.00 | 2 | 2 |
| Product | code | semantic_match | 1.00 | 3 | 3 |
| Product | name | semantic_match | 1.00 | 4 | 4 |
| Product | amount | semantic_match | 1.00 | 5 | 5 |
| Product | price | semantic_match | 1.00 | 6 | 6 |
| Product | cost | semantic_match | 1.00 | 7 | 7 |
| Product | user_id | semantic_match | 1.00 | 8 | 8 |
| Product | department_id | semantic_match | 1.00 | 9 | 9 |
| Product | unit | semantic_match | 1.00 | 10 | 10 |
| Product | url_image | semantic_match | 1.00 | 11 | 11 |
| Product | is_supply | semantic_match | 1.00 | 12 | 12 |
| Product | unit_purchase | semantic_match | 1.00 | 13 | 13 |
| Product | unit_quantity | semantic_match | 1.00 | 14 | 14 |
| Product | track_inventory | semantic_match | 1.00 | 15 | 15 |
| Product | (entity) | semantic_match | 0.89 | 20 | 22 |
| Product | object_id | semantic_match | 1.00 | 0 | 0 |
| Product | created_at | semantic_match | 1.00 | 1 | 1 |
| Product | updated_at | semantic_match | 1.00 | 2 | 2 |
| ProductLog | (entity) | semantic_match | 1.00 | 21 | 21 |
| ProductLog | object_id | semantic_match | 1.00 | 0 | 0 |
| ProductLog | created_at | semantic_match | 1.00 | 1 | 1 |
| ProductLog | updated_at | semantic_match | 1.00 | 2 | 2 |
| ProductLog | user_id | semantic_match | 1.00 | 3 | 3 |
| ProductLog | product_id | semantic_match | 1.00 | 4 | 4 |
| ProductLog | price | semantic_match | 1.00 | 5 | 5 |
| ProductLog | cost | semantic_match | 1.00 | 6 | 6 |
| Product_Product | (entity) | semantic_match | 0.89 | 22 | 20 |
| Product_Product | object_id | semantic_match | 1.00 | 0 | 0 |
| Product_Product | created_at | semantic_match | 1.00 | 1 | 1 |
| Product_Product | updated_at | semantic_match | 1.00 | 2 | 2 |
| Product_Product | (entity) | semantic_match | 1.00 | 22 | 22 |
| Product_Product | object_id | semantic_match | 1.00 | 0 | 0 |
| Product_Product | created_at | semantic_match | 1.00 | 1 | 1 |
| Product_Product | updated_at | semantic_match | 1.00 | 2 | 2 |
| Product_Product | quantity | semantic_match | 1.00 | 3 | 3 |
| Product_Product | product_id | semantic_match | 1.00 | 4 | 4 |
| Product_Product | includes_product | semantic_match | 1.00 | 5 | 5 |
| Product_Tax | (entity) | semantic_match | 1.00 | 23 | 23 |
| Product_Tax | object_id | semantic_match | 1.00 | 0 | 0 |
| Product_Tax | created_at | semantic_match | 1.00 | 1 | 1 |
| Product_Tax | updated_at | semantic_match | 1.00 | 2 | 2 |
| Product_Tax | tax_id | semantic_match | 1.00 | 3 | 3 |
| Product_Tax | product_id | semantic_match | 1.00 | 4 | 4 |
| Product_cost | (entity) | semantic_match | 1.00 | 24 | 24 |
| Product_cost | object_id | semantic_match | 1.00 | 0 | 0 |
| Product_cost | created_at | semantic_match | 1.00 | 1 | 1 |
| Product_cost | updated_at | semantic_match | 1.00 | 2 | 2 |
| Product_cost | product_id | semantic_match | 1.00 | 3 | 3 |
| Product_cost | source_type | semantic_match | 1.00 | 4 | 4 |
| Product_cost | source_id | semantic_match | 1.00 | 5 | 5 |
| Product_cost | old_cost | semantic_match | 1.00 | 6 | 6 |
| Product_cost | new_cost | semantic_match | 1.00 | 7 | 7 |
| Product_cost | currency | semantic_match | 1.00 | 8 | 8 |
| Product_cost | note | semantic_match | 1.00 | 9 | 9 |
| Product_option_assignment | (entity) | semantic_match | 1.00 | 25 | 25 |
| Product_option_assignment | object_id | semantic_match | 1.00 | 0 | 0 |
| Product_option_assignment | created_at | semantic_match | 1.00 | 1 | 1 |
| Product_option_assignment | product_id | semantic_match | 1.00 | 2 | 2 |
| Product_option_assignment | product_option_id | semantic_match | 1.00 | 3 | 3 |
| Product_option_assignment | sort_order | semantic_match | 1.00 | 4 | 4 |
| Product_option_detail | (entity) | semantic_match | 1.00 | 26 | 26 |
| Product_option_detail | object_id | semantic_match | 1.00 | 0 | 0 |
| Product_option_detail | created_at | semantic_match | 1.00 | 1 | 1 |
| Product_option_detail | updated_at | semantic_match | 1.00 | 2 | 2 |
| Product_option_detail | product_option_id | semantic_match | 1.00 | 3 | 3 |
| Product_option_detail | name | semantic_match | 1.00 | 4 | 4 |
| Product_option_detail | price_delta | semantic_match | 1.00 | 5 | 5 |
| Product_option_detail | sku_suffix | semantic_match | 1.00 | 6 | 6 |
| Product_option_detail | sort_order | semantic_match | 1.00 | 7 | 7 |
| Product_option_detail | is_active | semantic_match | 1.00 | 8 | 8 |
| Product_options | (entity) | semantic_match | 1.00 | 27 | 27 |
| Product_options | object_id | semantic_match | 1.00 | 0 | 0 |
| Product_options | created_at | semantic_match | 1.00 | 1 | 1 |
| Product_options | updated_at | semantic_match | 1.00 | 2 | 2 |
| Product_options | product_id | semantic_match | 1.00 | 3 | 3 |
| Product_options | name | semantic_match | 1.00 | 4 | 4 |
| Product_options | selection_mode | semantic_match | 1.00 | 5 | 5 |
| Product_options | is_required | semantic_match | 1.00 | 6 | 6 |
| Product_options | sort_order | semantic_match | 1.00 | 7 | 7 |
| Product_options | is_active | semantic_match | 1.00 | 8 | 8 |
| Purchase_order_details | (entity) | semantic_match | 1.00 | 28 | 28 |
| Purchase_order_details | object_id | semantic_match | 1.00 | 0 | 0 |
| Purchase_order_details | created_at | semantic_match | 1.00 | 1 | 1 |
| Purchase_order_details | updated_at | semantic_match | 1.00 | 2 | 2 |
| Purchase_order_details | purchase_order_id | semantic_match | 1.00 | 3 | 3 |
| Purchase_order_details | product_id | semantic_match | 1.00 | 4 | 4 |
| Purchase_order_details | quantity | semantic_match | 1.00 | 5 | 5 |
| Purchase_order_details | unit_cost | semantic_match | 1.00 | 6 | 6 |
| Purchase_order_details | unit_cost_mxn | semantic_match | 1.00 | 7 | 7 |
| Purchase_order_details | applied_unit_cost_mxn | semantic_match | 1.00 | 8 | 8 |
| Purchase_order_details | applied_total_mxn | semantic_match | 1.00 | 9 | 9 |
| Purchase_order_details | unit_purchase | semantic_match | 1.00 | 10 | 10 |
| Purchase_order_details | unit_quantity | semantic_match | 1.00 | 11 | 11 |
| Purchase_order_details | unit_sale | semantic_match | 1.00 | 12 | 12 |
| Purchase_orders | (entity) | semantic_match | 1.00 | 29 | 29 |
| Purchase_orders | object_id | semantic_match | 1.00 | 0 | 0 |
| Purchase_orders | created_at | semantic_match | 1.00 | 1 | 1 |
| Purchase_orders | updated_at | semantic_match | 1.00 | 2 | 2 |
| Purchase_orders | supplier_id | semantic_match | 1.00 | 3 | 3 |
| Purchase_orders | status | semantic_match | 1.00 | 4 | 4 |
| Purchase_orders | currency | semantic_match | 1.00 | 5 | 5 |
| Purchase_orders | currency_exchange | semantic_match | 1.00 | 6 | 6 |
| Purchase_orders | additionals | semantic_match | 1.00 | 7 | 7 |
| Purchase_orders | notes | semantic_match | 1.00 | 8 | 8 |
| Purchase_orders | paid_from_cash | semantic_match | 1.00 | 9 | 9 |
| Purchase_orders | cash_movement_id | semantic_match | 1.00 | 10 | 10 |
| Quotation | (entity) | semantic_match | 1.00 | 30 | 30 |
| Quotation | object_id | semantic_match | 1.00 | 0 | 0 |
| Quotation | created_at | semantic_match | 1.00 | 1 | 1 |
| Quotation | updated_at | semantic_match | 1.00 | 2 | 2 |
| Quotation | folio | semantic_match | 1.00 | 3 | 3 |
| Quotation | total | semantic_match | 1.00 | 4 | 4 |
| Quotation | toPay | semantic_match | 1.00 | 5 | 5 |
| Quotation | discount | semantic_match | 1.00 | 6 | 6 |
| Quotation | currency | semantic_match | 1.00 | 7 | 7 |
| Quotation | is_converted | semantic_match | 1.00 | 8 | 8 |
| Quotation | converted_order_id | semantic_match | 1.00 | 9 | 9 |
| Quotation | terms_quote | semantic_match | 1.00 | 10 | 10 |
| Quotation | transfer_comment | semantic_match | 1.00 | 11 | 11 |
| Quotation | box_id | semantic_match | 1.00 | 12 | 12 |
| Quotation | user_id | semantic_match | 1.00 | 13 | 13 |
| Quotation | customer_id | semantic_match | 1.00 | 14 | 14 |
| Quotation_detail | (entity) | semantic_match | 1.00 | 31 | 31 |
| Quotation_detail | object_id | semantic_match | 1.00 | 0 | 0 |
| Quotation_detail | created_at | semantic_match | 1.00 | 1 | 1 |
| Quotation_detail | updated_at | semantic_match | 1.00 | 2 | 2 |
| Quotation_detail | code | semantic_match | 1.00 | 3 | 3 |
| Quotation_detail | quantity | semantic_match | 1.00 | 5 | 5 |
| Quotation_detail | price | semantic_match | 1.00 | 6 | 6 |
| Quotation_detail | discount | semantic_match | 1.00 | 7 | 7 |
| Quotation_detail | total | semantic_match | 1.00 | 8 | 8 |
| Quotation_detail | quotation_id | semantic_match | 1.00 | 9 | 9 |
| Quotation_detail | product_id | semantic_match | 1.00 | 10 | 10 |
| Quotation_detail | department_id | semantic_match | 1.00 | 11 | 11 |
| Quotation_detail | unit | semantic_match | 1.00 | 12 | 12 |
| Quotation_detail | comments | semantic_match | 1.00 | 13 | 13 |
| Quotation_detail_options | (entity) | semantic_match | 1.00 | 32 | 32 |
| Quotation_detail_options | object_id | semantic_match | 1.00 | 0 | 0 |
| Quotation_detail_options | created_at | semantic_match | 1.00 | 1 | 1 |
| Quotation_detail_options | updated_at | semantic_match | 1.00 | 2 | 2 |
| Quotation_detail_options | quotation_detail_id | semantic_match | 1.00 | 3 | 3 |
| Quotation_detail_options | product_option_id | semantic_match | 1.00 | 4 | 4 |
| Quotation_detail_options | product_option_detail_id | semantic_match | 1.00 | 5 | 5 |
| Quotation_detail_options | name_snapshot | semantic_match | 1.00 | 6 | 6 |
| Quotation_detail_options | price_delta_snapshot | semantic_match | 1.00 | 7 | 7 |
| Recipe_items | (entity) | semantic_match | 1.00 | 33 | 33 |
| Recipe_items | object_id | semantic_match | 1.00 | 0 | 0 |
| Recipe_items | created_at | semantic_match | 1.00 | 1 | 1 |
| Recipe_items | updated_at | semantic_match | 1.00 | 2 | 2 |
| Recipe_items | recipe_id | semantic_match | 1.00 | 3 | 3 |
| Recipe_items | food_supply_id | semantic_match | 1.00 | 4 | 4 |
| Recipe_items | quantity | semantic_match | 1.00 | 5 | 5 |
| Restaurant_tables | (entity) | semantic_match | 1.00 | 34 | 34 |
| Restaurant_tables | object_id | semantic_match | 1.00 | 0 | 0 |
| Restaurant_tables | created_at | semantic_match | 1.00 | 1 | 1 |
| Restaurant_tables | updated_at | semantic_match | 1.00 | 2 | 2 |
| Restaurant_tables | name | semantic_match | 1.00 | 3 | 3 |
| Restaurant_tables | area | semantic_match | 1.00 | 4 | 4 |
| Restaurant_tables | is_active | semantic_match | 1.00 | 5 | 5 |
| Restaurant_tables | priority | semantic_match | 1.00 | 6 | 6 |
| Setting | (entity) | semantic_match | 1.00 | 35 | 35 |
| Setting | object_id | semantic_match | 1.00 | 0 | 0 |
| Setting | created_at | semantic_match | 1.00 | 1 | 1 |
| Setting | updated_at | semantic_match | 1.00 | 2 | 2 |
| Setting | small_logo | semantic_match | 1.00 | 3 | 3 |
| Setting | big_logo | semantic_match | 1.00 | 4 | 4 |
| Setting | print_logo | semantic_match | 1.00 | 5 | 5 |
| Setting | principal_color | semantic_match | 1.00 | 6 | 6 |
| Setting | secondary_color | semantic_match | 1.00 | 7 | 7 |
| Setting | name | semantic_match | 1.00 | 8 | 8 |
| Setting | phone | semantic_match | 1.00 | 9 | 9 |
| Setting | address | semantic_match | 1.00 | 10 | 10 |
| Setting | bank_account | semantic_match | 1.00 | 11 | 11 |
| Setting | bank_name | semantic_match | 1.00 | 12 | 12 |
| Setting | bank_clabe | semantic_match | 1.00 | 13 | 13 |
| Setting | name_quote | semantic_match | 1.00 | 14 | 14 |
| Setting | terms_quote | semantic_match | 1.00 | 15 | 15 |
| Setting | transfer_comment | semantic_match | 1.00 | 16 | 16 |
| Setting | max_credit | semantic_match | 1.00 | 17 | 17 |
| Setting | discount_customer | semantic_match | 1.00 | 18 | 18 |
| Setting | discount_order | semantic_match | 1.00 | 19 | 19 |
| Setting | enable_discount | semantic_match | 1.00 | 20 | 20 |
| Setting | mandatory_reference | semantic_match | 1.00 | 21 | 21 |
| Setting | usd_price | semantic_match | 1.00 | 22 | 22 |
| Setting | enable_bailment | semantic_match | 1.00 | 23 | 23 |
| Setting | tax_included | semantic_match | 1.00 | 24 | 24 |
| Setting | is_restaurant | semantic_match | 1.00 | 25 | 25 |
| Setting | currency | semantic_match | 1.00 | 26 | 26 |
| Setting | second_currency | semantic_match | 1.00 | 27 | 27 |
| Setting | costing_strategy | semantic_match | 1.00 | 28 | 28 |
| Setting | costing_last_n | semantic_match | 1.00 | 29 | 29 |
| Setting | enable_batches | semantic_match | 1.00 | 30 | 30 |
| Setting | points_perc | semantic_match | 1.00 | 31 | 31 |
| Setting | quotation_folio_prefix | semantic_match | 1.00 | 32 | 32 |
| Setting | quotation_folio_min_length | semantic_match | 1.00 | 33 | 33 |
| Setting_discount | (entity) | semantic_match | 1.00 | 36 | 36 |
| Setting_discount | object_id | semantic_match | 1.00 | 0 | 0 |
| Setting_discount | created_at | semantic_match | 1.00 | 1 | 1 |
| Setting_discount | updated_at | semantic_match | 1.00 | 2 | 2 |
| Setting_discount | min | semantic_match | 1.00 | 3 | 3 |
| Setting_discount | discount | semantic_match | 1.00 | 4 | 4 |
| Setting_discount | setting_id | semantic_match | 1.00 | 5 | 5 |
| Setting_local | (entity) | semantic_match | 1.00 | 37 | 37 |
| Setting_local | object_id | semantic_match | 1.00 | 0 | 0 |
| Setting_local | created_at | semantic_match | 1.00 | 1 | 1 |
| Setting_local | updated_at | semantic_match | 1.00 | 2 | 2 |
| Setting_local | setting_id | semantic_match | 1.00 | 3 | 3 |
| Setting_local | machine_id | semantic_match | 1.00 | 4 | 4 |
| Setting_local | backup_path | semantic_match | 1.00 | 5 | 5 |
| Setting_local | scale_vid | semantic_match | 1.00 | 6 | 6 |
| Setting_local | scale_pid | semantic_match | 1.00 | 7 | 7 |
| Setting_local | cash | semantic_match | 1.00 | 8 | 8 |
| Setting_local | enable_scale | semantic_match | 1.00 | 9 | 9 |
| Setting_local | user_id | semantic_match | 1.00 | 10 | 10 |
| Setting_local | version | semantic_match | 1.00 | 11 | 11 |
| Setting_local | channel | semantic_match | 1.00 | 12 | 12 |
| Setting_local | tour_login | semantic_match | 1.00 | 13 | 13 |
| Setting_local | touch_mode | semantic_match | 1.00 | 14 | 14 |
| Setting_local | is_commander | semantic_match | 1.00 | 15 | 15 |
| Setting_local | is_kitchen_display | semantic_match | 1.00 | 16 | 16 |
| Setting_local | auto_updates_enabled | semantic_match | 1.00 | 17 | 17 |
| Setting_local | allow_unstable_channel | semantic_match | 1.00 | 18 | 18 |
| Setting_local | product_grid_view | semantic_match | 1.00 | 19 | 19 |
| Setting_local | is_price_checker | semantic_match | 1.00 | 20 | 20 |
| Setting_local | uuid_baseboard | semantic_match | 1.00 | 21 | 21 |
| Setting_local | uuid_ram | semantic_match | 1.00 | 22 | 22 |
| Setting_local | uuid_disk | semantic_match | 1.00 | 23 | 23 |
| Setting_local | uuid_system | semantic_match | 1.00 | 24 | 24 |
| Setting_local | platform | semantic_match | 1.00 | 25 | 25 |
| Suppliers | (entity) | semantic_match | 1.00 | 38 | 38 |
| Suppliers | object_id | semantic_match | 1.00 | 0 | 0 |
| Suppliers | created_at | semantic_match | 1.00 | 1 | 1 |
| Suppliers | updated_at | semantic_match | 1.00 | 2 | 2 |
| Suppliers | name | semantic_match | 1.00 | 3 | 3 |
| Suppliers | phone | semantic_match | 1.00 | 4 | 4 |
| Suppliers | email | semantic_match | 1.00 | 5 | 5 |
| Suppliers | address | semantic_match | 1.00 | 6 | 6 |
| Suppliers | is_active | semantic_match | 1.00 | 7 | 7 |
| Tax | (entity) | semantic_match | 1.00 | 39 | 39 |
| Tax | object_id | semantic_match | 1.00 | 0 | 0 |
| Tax | created_at | semantic_match | 1.00 | 1 | 1 |
| Tax | updated_at | semantic_match | 1.00 | 2 | 2 |
| Tax | name | semantic_match | 1.00 | 3 | 3 |
| Tax | tax_name | semantic_match | 1.00 | 4 | 4 |
| Tax | value | semantic_match | 1.00 | 5 | 5 |
| Tax | type | semantic_match | 1.00 | 6 | 6 |
| Tax | withholding | semantic_match | 1.00 | 7 | 7 |
| Tax | transfer | semantic_match | 1.00 | 8 | 8 |
| Tax | apply_default | semantic_match | 1.00 | 9 | 9 |
| Tour | (entity) | semantic_match | 1.00 | 40 | 40 |
| Tour | object_id | semantic_match | 1.00 | 0 | 0 |
| Tour | user_id | semantic_match | 1.00 | 1 | 1 |
| Tour | dashboard_tour | semantic_match | 1.00 | 2 | 2 |
| Tour | customers_tour | semantic_match | 1.00 | 3 | 3 |
| Tour | products_tour | semantic_match | 1.00 | 4 | 4 |
| Tour | departments_tour | semantic_match | 1.00 | 5 | 5 |
| Tour | open_cash_tour | semantic_match | 1.00 | 6 | 6 |
| Tour | cash_register_tour | semantic_match | 1.00 | 7 | 7 |
| Tour | orders_tour | semantic_match | 1.00 | 8 | 8 |
| Tour | inventory_tour | semantic_match | 1.00 | 9 | 9 |
| Tour | expenses_tour | semantic_match | 1.00 | 10 | 10 |
| Tour | cash_drawer_tour | semantic_match | 1.00 | 11 | 11 |
| Tour | close_cash_tour | semantic_match | 1.00 | 12 | 12 |
| Tour | reports_tour | semantic_match | 1.00 | 13 | 13 |
| Tour | configuration_tour | semantic_match | 1.00 | 14 | 14 |
| Tour | daily_sales_report_tour | semantic_match | 1.00 | 15 | 15 |
| Tour | weekly_sales_report_tour | semantic_match | 1.00 | 16 | 16 |
| Tour | monthly_sales_report_tour | semantic_match | 1.00 | 17 | 17 |
| Tour | best_customers_report_tour | semantic_match | 1.00 | 18 | 18 |
| Tour | best_products_report_tour | semantic_match | 1.00 | 19 | 19 |
| Tour | product_details_report_tour | semantic_match | 1.00 | 20 | 20 |
| Tour | product_analysis_report_tour | semantic_match | 1.00 | 21 | 21 |
| Tour | payments_report_tour | semantic_match | 1.00 | 22 | 22 |
| Tour | credit_statement_report_tour | semantic_match | 1.00 | 23 | 23 |
| Tour | credit_general_report_tour | semantic_match | 1.00 | 24 | 24 |
| Tour | cashcuts_report_tour | semantic_match | 1.00 | 25 | 25 |
| Tour | options_config_tour | semantic_match | 1.00 | 26 | 26 |
| Tour | business_config_tour | semantic_match | 1.00 | 27 | 27 |
| Tour | users_config_tour | semantic_match | 1.00 | 28 | 28 |
| Tour | taxes_config_tour | semantic_match | 1.00 | 29 | 29 |
| Tour | database_config_tour | semantic_match | 1.00 | 30 | 30 |
| Tour | printer_config_tour | semantic_match | 1.00 | 31 | 31 |
| Tour | scale_config_tour | semantic_match | 1.00 | 32 | 32 |
| Tour | tpv_config_tour | semantic_match | 1.00 | 33 | 33 |
| Tour | multibox_config_tour | semantic_match | 1.00 | 34 | 34 |
| Unit | (entity) | semantic_match | 1.00 | 41 | 41 |
| Unit | object_id | semantic_match | 1.00 | 0 | 0 |
| Unit | created_at | semantic_match | 1.00 | 1 | 1 |
| Unit | updated_at | semantic_match | 1.00 | 2 | 2 |
| Unit | name | semantic_match | 1.00 | 3 | 3 |
| Unit | is_enabled | semantic_match | 1.00 | 4 | 4 |
| User | (entity) | semantic_match | 1.00 | 42 | 42 |
| User | object_id | semantic_match | 1.00 | 0 | 0 |
| User | created_at | semantic_match | 1.00 | 1 | 1 |
| User | updated_at | semantic_match | 1.00 | 2 | 2 |
| User | username | semantic_match | 1.00 | 3 | 3 |
| User | password | semantic_match | 1.00 | 4 | 4 |
| User | email | semantic_match | 1.00 | 5 | 5 |
| User | admin_permission | semantic_match | 1.00 | 6 | 6 |
| User | pin | semantic_match | 1.00 | 7 | 7 |
| Warehouse | (entity) | semantic_match | 1.00 | 43 | 43 |
| Warehouse | object_id | semantic_match | 1.00 | 0 | 0 |
| Warehouse | created_at | semantic_match | 1.00 | 1 | 1 |
| Warehouse | name | semantic_match | 1.00 | 3 | 2 |
| Warehouse | description | semantic_match | 1.00 | 4 | 3 |
| Warehouse | customer_id | semantic_match | 1.00 | 5 | 4 |

### Exclusives

| Entity | Field | Side | Safe to Add | Index |
|--------|-------|------|-------------|-------|
| Order_detail | product | A | ✅ Yes | 4 |
| Order_detail | producto | B | ✅ Yes | 4 |
| Product | code | A | ✅ Yes | 3 |
| Product | name | A | ✅ Yes | 4 |
| Product | amount | A | ✅ Yes | 5 |
| Product | price | A | ✅ Yes | 6 |
| Product | cost | A | ✅ Yes | 7 |
| Product | user_id | A | ✅ Yes | 8 |
| Product | department_id | A | ✅ Yes | 9 |
| Product | unit | A | ✅ Yes | 10 |
| Product | url_image | A | ✅ Yes | 11 |
| Product | is_supply | A | ✅ Yes | 12 |
| Product | unit_purchase | A | ✅ Yes | 13 |
| Product | unit_quantity | A | ✅ Yes | 14 |
| Product | track_inventory | A | ✅ Yes | 15 |
| Product | quantity | B | ✅ Yes | 3 |
| Product | product_id | B | ✅ Yes | 4 |
| Product | includes_product | B | ✅ Yes | 5 |
| Product_Product | quantity | A | ✅ Yes | 3 |
| Product_Product | product_id | A | ✅ Yes | 4 |
| Product_Product | includes_product | A | ✅ Yes | 5 |
| Product_Product | code | B | ✅ Yes | 3 |
| Product_Product | name | B | ✅ Yes | 4 |
| Product_Product | amount | B | ✅ Yes | 5 |
| Product_Product | price | B | ✅ Yes | 6 |
| Product_Product | cost | B | ✅ Yes | 7 |
| Product_Product | user_id | B | ✅ Yes | 8 |
| Product_Product | department_id | B | ✅ Yes | 9 |
| Product_Product | unit | B | ✅ Yes | 10 |
| Product_Product | url_image | B | ✅ Yes | 11 |
| Product_Product | is_supply | B | ✅ Yes | 12 |
| Product_Product | unit_purchase | B | ✅ Yes | 13 |
| Product_Product | unit_quantity | B | ✅ Yes | 14 |
| Product_Product | track_inventory | B | ✅ Yes | 15 |
| Quotation_detail | product | A | ✅ Yes | 4 |
| Quotation_detail | producto | B | ✅ Yes | 4 |
| Warehouse | updated_at | A | ✅ Yes | 2 |

---

## Recommendations

- ✅ Low risk detected - schemas are mostly compatible
- ✅ High compatibility score - schemas align well
- 📄 See `decision_log.json` for complete decision history and programmatic access


---

*Report generated by AUDD v0.1.0*
