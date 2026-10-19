# AUDD Comparison Report

**Generated:** 1970-01-01 00:00:00 UTC  
**Schema A:** a  
**Schema B:** b  
**Report Version:** 1.0.0

---

## Executive Summary

### Compatibility Overview

- **Matches**: 64
- **Exclusives**: 4 (2 from A, 2 from B)
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

- **Total**: 64
- **Exact name matches**: 0
- **Average confidence**: 1.00

### Exclusives

- **From Schema A**: 2 (2 safe to add)
- **From Schema B**: 2 (2 safe to add)
- **Safe Addition Rate**: 100.0%

### Conflicts

- **Total**: 0

---

## Technical Details

### Matches

| Entity | Field | Match Type | Score | Index A | Index B |
|--------|-------|------------|-------|---------|---------|
| Categories | (entity) | semantic_match | 1.00 | 0 | 0 |
| Categories | CategoryID | semantic_match | 1.00 | 0 | 0 |
| Categories | CategoryName | semantic_match | 1.00 | 1 | 1 |
| Customers | (entity) | semantic_match | 1.00 | 1 | 1 |
| Customers | CustomerID | semantic_match | 1.00 | 0 | 0 |
| Customers | CompanyName | semantic_match | 1.00 | 1 | 1 |
| Customers | ContactName | semantic_match | 1.00 | 2 | 2 |
| Customers | ContactTitle | semantic_match | 1.00 | 3 | 3 |
| Customers | Address | semantic_match | 1.00 | 4 | 4 |
| Customers | City | semantic_match | 1.00 | 5 | 5 |
| Customers | Region | semantic_match | 1.00 | 6 | 6 |
| Customers | PostalCode | semantic_match | 1.00 | 7 | 7 |
| Customers | Country | semantic_match | 1.00 | 8 | 8 |
| Customers | Phone | semantic_match | 1.00 | 9 | 9 |
| Customers | Fax | semantic_match | 1.00 | 10 | 10 |
| OrderDetails | (entity) | semantic_match | 1.00 | 3 | 2 |
| OrderDetails | OrderID | semantic_match | 1.00 | 0 | 0 |
| OrderDetails | ProductID | semantic_match | 1.00 | 1 | 1 |
| OrderDetails | UnitPrice | semantic_match | 1.00 | 2 | 2 |
| OrderDetails | Quantity | semantic_match | 1.00 | 3 | 3 |
| OrderDetails | Discount | semantic_match | 1.00 | 4 | 4 |
| Orders | (entity) | semantic_match | 1.00 | 4 | 3 |
| Orders | OrderID | semantic_match | 1.00 | 0 | 0 |
| Orders | CustomerID | semantic_match | 1.00 | 1 | 1 |
| Orders | EmployeeID | semantic_match | 1.00 | 2 | 2 |
| Orders | OrderDate | semantic_match | 1.00 | 3 | 3 |
| Orders | RequiredDate | semantic_match | 1.00 | 4 | 4 |
| Orders | ShippedDate | semantic_match | 1.00 | 5 | 5 |
| Orders | ShipVia | semantic_match | 1.00 | 6 | 6 |
| Orders | Freight | semantic_match | 1.00 | 7 | 7 |
| Orders | ShipName | semantic_match | 1.00 | 8 | 8 |
| Orders | ShipAddress | semantic_match | 1.00 | 9 | 9 |
| Orders | ShipCity | semantic_match | 1.00 | 10 | 10 |
| Orders | ShipRegion | semantic_match | 1.00 | 11 | 11 |
| Orders | ShipPostalCode | semantic_match | 1.00 | 12 | 12 |
| Orders | ShipCountry | semantic_match | 1.00 | 13 | 13 |
| Products | (entity) | semantic_match | 1.00 | 5 | 4 |
| Products | ProductID | semantic_match | 1.00 | 0 | 0 |
| Products | ProductName | semantic_match | 1.00 | 1 | 1 |
| Products | SupplierID | semantic_match | 1.00 | 2 | 2 |
| Products | CategoryID | semantic_match | 1.00 | 3 | 3 |
| Products | QuantityPerUnit | semantic_match | 1.00 | 4 | 4 |
| Products | UnitPrice | semantic_match | 1.00 | 5 | 5 |
| Products | UnitsInStock | semantic_match | 1.00 | 6 | 6 |
| Products | UnitsOnOrder | semantic_match | 1.00 | 7 | 7 |
| Products | ReorderLevel | semantic_match | 1.00 | 8 | 8 |
| Products | Discontinued | semantic_match | 1.00 | 9 | 9 |
| Shippers | (entity) | semantic_match | 1.00 | 6 | 5 |
| Shippers | ShipperID | semantic_match | 1.00 | 0 | 0 |
| Shippers | CompanyName | semantic_match | 1.00 | 1 | 1 |
| Shippers | Phone | semantic_match | 1.00 | 2 | 2 |
| Suppliers | (entity) | semantic_match | 1.00 | 7 | 6 |
| Suppliers | SupplierID | semantic_match | 1.00 | 0 | 0 |
| Suppliers | CompanyName | semantic_match | 1.00 | 1 | 1 |
| Suppliers | ContactName | semantic_match | 1.00 | 2 | 2 |
| Suppliers | ContactTitle | semantic_match | 1.00 | 3 | 3 |
| Suppliers | Address | semantic_match | 1.00 | 4 | 4 |
| Suppliers | City | semantic_match | 1.00 | 5 | 5 |
| Suppliers | Region | semantic_match | 1.00 | 6 | 6 |
| Suppliers | PostalCode | semantic_match | 1.00 | 7 | 7 |
| Suppliers | Country | semantic_match | 1.00 | 8 | 8 |
| Suppliers | Phone | semantic_match | 1.00 | 9 | 9 |
| Suppliers | Fax | semantic_match | 1.00 | 10 | 10 |
| Suppliers | HomePage | semantic_match | 1.00 | 11 | 11 |

### Exclusives

| Entity | Field | Side | Safe to Add | Index |
|--------|-------|------|-------------|-------|
| Categories | Description | A | ✅ Yes | 2 |
| Categories | extra_field_4361 | B | ✅ Yes | 2 |
| Employees | (entity) | A | ✅ Yes | 2 |
| tbl_3476 | (entity) | B | ✅ Yes | 7 |

---

## Recommendations

- ✅ Low risk detected - schemas are mostly compatible
- ✅ High compatibility score - schemas align well
- 📄 See `decision_log.json` for complete decision history and programmatic access


---

*Report generated by AUDD v0.1.0*
