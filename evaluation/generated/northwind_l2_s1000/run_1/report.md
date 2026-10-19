# AUDD Comparison Report

**Generated:** 1970-01-01 00:00:00 UTC  
**Schema A:** a  
**Schema B:** b  
**Report Version:** 1.0.0

---

## Executive Summary

### Compatibility Overview

- **Matches**: 80
- **Exclusives**: 3 (2 from A, 1 from B)
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

- **Total**: 80
- **Exact name matches**: 0
- **Average confidence**: 1.00

### Exclusives

- **From Schema A**: 2 (2 safe to add)
- **From Schema B**: 1 (1 safe to add)
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
| Categories | Description | semantic_match | 1.00 | 2 | 2 |
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
| Employees | (entity) | semantic_match | 1.00 | 2 | 2 |
| Employees | EmployeeID | semantic_match | 1.00 | 0 | 0 |
| Employees | LastName | semantic_match | 1.00 | 1 | 1 |
| Employees | FirstName | semantic_match | 1.00 | 2 | 2 |
| Employees | Title | semantic_match | 1.00 | 3 | 3 |
| Employees | TitleOfCourtesy | semantic_match | 1.00 | 4 | 4 |
| Employees | BirthDate | semantic_match | 1.00 | 5 | 5 |
| Employees | HireDate | semantic_match | 1.00 | 6 | 6 |
| Employees | Address | semantic_match | 1.00 | 7 | 7 |
| Employees | City | semantic_match | 1.00 | 8 | 8 |
| Employees | Region | semantic_match | 1.00 | 9 | 9 |
| Employees | PostalCode | semantic_match | 1.00 | 10 | 10 |
| Employees | Country | semantic_match | 1.00 | 11 | 11 |
| Employees | HomePhone | semantic_match | 1.00 | 12 | 12 |
| Employees | Extension | semantic_match | 1.00 | 13 | 13 |
| Employees | Notes | semantic_match | 1.00 | 14 | 14 |
| Employees | ReportsTo | semantic_match | 1.00 | 15 | 15 |
| OrderDetails | (entity) | semantic_match | 1.00 | 3 | 3 |
| OrderDetails | OrderID | semantic_match | 1.00 | 0 | 0 |
| OrderDetails | ProductID | semantic_match | 1.00 | 1 | 1 |
| OrderDetails | UnitPrice | semantic_match | 1.00 | 2 | 2 |
| OrderDetails | Quantity | semantic_match | 1.00 | 3 | 3 |
| OrderDetails | Discount | semantic_match | 1.00 | 4 | 4 |
| Orders | (entity) | semantic_match | 1.00 | 4 | 4 |
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
| Products | (entity) | semantic_match | 1.00 | 5 | 5 |
| Products | ProductID | semantic_match | 1.00 | 0 | 0 |
| Products | ProductName | semantic_match | 1.00 | 1 | 1 |
| Products | SupplierID | semantic_match | 1.00 | 2 | 2 |
| Products | CategoryID | semantic_match | 1.00 | 3 | 3 |
| Products | UnitPrice | semantic_match | 1.00 | 5 | 4 |
| Products | UnitsInStock | semantic_match | 1.00 | 6 | 5 |
| Products | UnitsOnOrder | semantic_match | 1.00 | 7 | 6 |
| Products | ReorderLevel | semantic_match | 1.00 | 8 | 7 |
| Products | Discontinued | semantic_match | 1.00 | 9 | 8 |
| Shippers | (entity) | semantic_match | 1.00 | 6 | 6 |
| Shippers | ShipperID | semantic_match | 1.00 | 0 | 0 |
| Shippers | Phone | semantic_match | 1.00 | 2 | 1 |
| Suppliers | (entity) | semantic_match | 1.00 | 7 | 7 |
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
| Products | QuantityPerUnit | A | ✅ Yes | 4 |
| Products | extra_field_9760 | B | ✅ Yes | 9 |
| Shippers | CompanyName | A | ✅ Yes | 1 |

---

## Recommendations

- ✅ Low risk detected - schemas are mostly compatible
- ✅ High compatibility score - schemas align well
- 📄 See `decision_log.json` for complete decision history and programmatic access


---

*Report generated by AUDD v0.1.0*
