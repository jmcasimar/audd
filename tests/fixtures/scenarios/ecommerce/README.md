# E-Commerce Test Scenario

## Overview

This scenario provides two semantically identical e-commerce databases with different naming conventions and minor type differences.

## Expected Test Results

### Matches
- users ↔ Users
- products ↔ Products  
- orders ↔ Orders

### Conflicts
- Type conflicts: DECIMAL vs REAL for prices
- Naming variations: quantity vs orderQuantity

## Setup

```bash
sqlite3 tests/fixtures/databases/ecommerce_a.db < tests/fixtures/databases/ecommerce_a.sql
sqlite3 tests/fixtures/databases/ecommerce_b.db < tests/fixtures/databases/ecommerce_b.sql
```
