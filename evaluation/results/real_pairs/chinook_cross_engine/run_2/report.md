# AUDD Comparison Report

**Generated:** 1970-01-01 00:00:00 UTC  
**Schema A:** Chinook_MySql  
**Schema B:** Chinook_Sqlite  
**Report Version:** 1.0.0

---

## Executive Summary

### Compatibility Overview

- **Matches**: 71
- **Exclusives**: 2 (1 from A, 1 from B)
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

- **Total**: 71
- **Exact name matches**: 0
- **Average confidence**: 1.00

### Exclusives

- **From Schema A**: 1 (1 safe to add)
- **From Schema B**: 1 (1 safe to add)
- **Safe Addition Rate**: 100.0%

### Conflicts

- **Total**: 0

---

## Technical Details

### Matches

| Entity | Field | Match Type | Score | Index A | Index B |
|--------|-------|------------|-------|---------|---------|
| Artist | (entity) | semantic_match | 1.00 | 1 | 1 |
| Artist | ArtistId | semantic_match | 1.00 | 0 | 0 |
| Artist | Name | semantic_match | 1.00 | 1 | 1 |
| Customer | (entity) | semantic_match | 1.00 | 2 | 2 |
| Customer | CustomerId | semantic_match | 1.00 | 0 | 0 |
| Customer | FirstName | semantic_match | 1.00 | 1 | 1 |
| Customer | LastName | semantic_match | 1.00 | 2 | 2 |
| Customer | Company | semantic_match | 1.00 | 3 | 3 |
| Customer | Address | semantic_match | 1.00 | 4 | 4 |
| Customer | City | semantic_match | 1.00 | 5 | 5 |
| Customer | State | semantic_match | 1.00 | 6 | 6 |
| Customer | Country | semantic_match | 1.00 | 7 | 7 |
| Customer | PostalCode | semantic_match | 1.00 | 8 | 8 |
| Customer | Phone | semantic_match | 1.00 | 9 | 9 |
| Customer | Fax | semantic_match | 1.00 | 10 | 10 |
| Customer | Email | semantic_match | 1.00 | 11 | 11 |
| Customer | SupportRepId | semantic_match | 1.00 | 12 | 12 |
| Employee | (entity) | semantic_match | 1.00 | 3 | 3 |
| Employee | EmployeeId | semantic_match | 1.00 | 0 | 0 |
| Employee | LastName | semantic_match | 1.00 | 1 | 1 |
| Employee | FirstName | semantic_match | 1.00 | 2 | 2 |
| Employee | Title | semantic_match | 1.00 | 3 | 3 |
| Employee | ReportsTo | semantic_match | 1.00 | 4 | 4 |
| Employee | BirthDate | semantic_match | 1.00 | 5 | 5 |
| Employee | HireDate | semantic_match | 1.00 | 6 | 6 |
| Employee | Address | semantic_match | 1.00 | 7 | 7 |
| Employee | City | semantic_match | 1.00 | 8 | 8 |
| Employee | State | semantic_match | 1.00 | 9 | 9 |
| Employee | Country | semantic_match | 1.00 | 10 | 10 |
| Employee | PostalCode | semantic_match | 1.00 | 11 | 11 |
| Employee | Phone | semantic_match | 1.00 | 12 | 12 |
| Employee | Fax | semantic_match | 1.00 | 13 | 13 |
| Employee | Email | semantic_match | 1.00 | 14 | 14 |
| Genre | (entity) | semantic_match | 1.00 | 4 | 4 |
| Genre | GenreId | semantic_match | 1.00 | 0 | 0 |
| Genre | Name | semantic_match | 1.00 | 1 | 1 |
| Invoice | (entity) | semantic_match | 1.00 | 5 | 5 |
| Invoice | InvoiceId | semantic_match | 1.00 | 0 | 0 |
| Invoice | CustomerId | semantic_match | 1.00 | 1 | 1 |
| Invoice | InvoiceDate | semantic_match | 1.00 | 2 | 2 |
| Invoice | BillingAddress | semantic_match | 1.00 | 3 | 3 |
| Invoice | BillingCity | semantic_match | 1.00 | 4 | 4 |
| Invoice | BillingState | semantic_match | 1.00 | 5 | 5 |
| Invoice | BillingCountry | semantic_match | 1.00 | 6 | 6 |
| Invoice | BillingPostalCode | semantic_match | 1.00 | 7 | 7 |
| Invoice | Total | semantic_match | 1.00 | 8 | 8 |
| InvoiceLine | (entity) | semantic_match | 1.00 | 6 | 6 |
| InvoiceLine | InvoiceLineId | semantic_match | 1.00 | 0 | 0 |
| InvoiceLine | InvoiceId | semantic_match | 1.00 | 1 | 1 |
| InvoiceLine | TrackId | semantic_match | 1.00 | 2 | 2 |
| InvoiceLine | UnitPrice | semantic_match | 1.00 | 3 | 3 |
| InvoiceLine | Quantity | semantic_match | 1.00 | 4 | 4 |
| MediaType | (entity) | semantic_match | 1.00 | 7 | 7 |
| MediaType | MediaTypeId | semantic_match | 1.00 | 0 | 0 |
| MediaType | Name | semantic_match | 1.00 | 1 | 1 |
| Playlist | (entity) | semantic_match | 1.00 | 8 | 8 |
| Playlist | PlaylistId | semantic_match | 1.00 | 0 | 0 |
| Playlist | Name | semantic_match | 1.00 | 1 | 1 |
| PlaylistTrack | (entity) | semantic_match | 1.00 | 9 | 9 |
| PlaylistTrack | PlaylistId | semantic_match | 1.00 | 0 | 0 |
| PlaylistTrack | TrackId | semantic_match | 1.00 | 1 | 1 |
| Track | (entity) | semantic_match | 1.00 | 10 | 10 |
| Track | TrackId | semantic_match | 1.00 | 0 | 0 |
| Track | Name | semantic_match | 1.00 | 1 | 1 |
| Track | AlbumId | semantic_match | 1.00 | 2 | 2 |
| Track | MediaTypeId | semantic_match | 1.00 | 3 | 3 |
| Track | GenreId | semantic_match | 1.00 | 4 | 4 |
| Track | Composer | semantic_match | 1.00 | 5 | 5 |
| Track | Milliseconds | semantic_match | 1.00 | 6 | 6 |
| Track | Bytes | semantic_match | 1.00 | 7 | 7 |
| Track | UnitPrice | semantic_match | 1.00 | 8 | 8 |

### Exclusives

| Entity | Field | Side | Safe to Add | Index |
|--------|-------|------|-------------|-------|
| s
********************************************************************************/
CREATE TABLE `Album | (entity) | A | ✅ Yes | 0 |
| Album | (entity) | B | ✅ Yes | 0 |

---

## Recommendations

- ✅ Low risk detected - schemas are mostly compatible
- ✅ High compatibility score - schemas align well
- 📄 See `decision_log.json` for complete decision history and programmatic access


---

*Report generated by AUDD v0.1.0*
