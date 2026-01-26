# Advanced Database Features Implementation Plan

## Overview

This document tracks the implementation of advanced database features across all four database connectors: SQLite, MySQL/MariaDB, PostgreSQL, and MongoDB.

## Features to Implement

1. **Foreign Key Relationships**
2. **Indexes (non-unique)**
3. **Views (regular and materialized)**
4. **Stored Procedures/Functions**
5. **Triggers**
6. **CHECK Constraints**
7. **MongoDB Validators and JSON Schema**

## Implementation Status

### SQLite ✅ COMPLETE

- ✅ Foreign Keys - Extracted via `PRAGMA foreign_key_list`
- ✅ Indexes - Extracted via `PRAGMA index_list` and `PRAGMA index_info`
  - Regular indexes
  - Unique indexes
  - Excludes auto-generated indexes
- ✅ Views - Extracted from `sqlite_master`
  - View names
  - SQL definitions
- ✅ Triggers - Extracted from `sqlite_master`
  - Trigger names
  - Table associations
  - Timing (BEFORE/AFTER/INSTEAD OF)
  - Events (INSERT/UPDATE/DELETE)
  - SQL definitions
- ✅ CHECK Constraints - Supported via IR's `Constraint::Check`
- N/A Stored Procedures - SQLite doesn't support stored procedures

### PostgreSQL ✅ COMPLETE

- ✅ Foreign Keys - Query `information_schema.table_constraints` and `information_schema.key_column_usage`
- ✅ Indexes - Query `pg_indexes` and `pg_index`
  - Regular indexes
  - Unique indexes
  - Partial indexes (with WHERE clause/filter condition)
  - GIN, GIST indexes (mapped to FullText type)
- ✅ Views - Query `information_schema.views` and `pg_views`
  - Regular views
  - Materialized views (`pg_matviews`)
- ✅ Stored Procedures - Query `information_schema.routines`
  - Functions
  - Procedures
  - Return types
  - Definitions
- ✅ Triggers - Query `information_schema.triggers`
  - Timing and events
  - Table associations
  - Definitions
- 📋 CHECK Constraints - Supported in IR, extraction not yet implemented

### MySQL/MariaDB ✅ COMPLETE

- ✅ Foreign Keys - Query `INFORMATION_SCHEMA.KEY_COLUMN_USAGE` with REFERENCED_TABLE_NAME
  - Referenced table and column metadata stored
  - Support for composite foreign keys
- ✅ Indexes - Query `INFORMATION_SCHEMA.STATISTICS`
  - Regular indexes
  - Unique indexes (handled as Keys)
  - Full-text indexes (FULLTEXT type)
  - Spatial indexes (SPATIAL type)
- ✅ Views - Query `INFORMATION_SCHEMA.VIEWS`
  - View definitions
  - View names
- ✅ Stored Procedures - Query `INFORMATION_SCHEMA.ROUTINES`
  - Procedures
  - Functions
  - Return types
  - Definitions
- ✅ Triggers - Query `INFORMATION_SCHEMA.TRIGGERS`
  - Timing (BEFORE/AFTER)
  - Events (INSERT/UPDATE/DELETE)
  - Table associations
  - Definitions
- 📋 CHECK Constraints - Supported in IR, Query `INFORMATION_SCHEMA.CHECK_CONSTRAINTS` (MySQL 8.0.16+) not yet implemented

### MongoDB ✅ COMPLETE

- ✅ Indexes - Use `listIndexes()` command
  - Single field indexes
  - Compound indexes
  - Text indexes
  - 2dsphere (spatial) indexes
  - Hashed indexes
  - Unique indexes
  - Partial/filtered indexes
- ✅ Views - Use `listCollections()` with type filter
  - Aggregation pipeline views
  - View definitions
- 📋 Validation Rules - Extract from collection options (not yet implemented)
  - JSON Schema validators
  - Query expression validators
- N/A Foreign Keys - MongoDB doesn't enforce foreign keys
- N/A Stored Procedures - MongoDB doesn't have stored procedures
- N/A Triggers - MongoDB has change streams and database triggers (Atlas only, not extracted)

## SQL Queries Reference

### PostgreSQL

#### Foreign Keys
```sql
SELECT
    tc.constraint_name,
    tc.table_name,
    kcu.column_name,
    ccu.table_name AS foreign_table_name,
    ccu.column_name AS foreign_column_name
FROM information_schema.table_constraints AS tc
JOIN information_schema.key_column_usage AS kcu
    ON tc.constraint_name = kcu.constraint_name
JOIN information_schema.constraint_column_usage AS ccu
    ON ccu.constraint_name = tc.constraint_name
WHERE tc.constraint_type = 'FOREIGN KEY'
    AND tc.table_schema = 'public'
    AND tc.table_name = $1;
```

#### Indexes
```sql
SELECT
    i.relname AS index_name,
    a.attname AS column_name,
    ix.indisunique AS is_unique,
    ix.indisprimary AS is_primary,
    pg_get_expr(ix.indpred, ix.indrelid) AS filter_condition
FROM pg_class t
JOIN pg_index ix ON t.oid = ix.indrelid
JOIN pg_class i ON i.oid = ix.indexrelid
JOIN pg_attribute a ON a.attrelid = t.oid AND a.attnum = ANY(ix.indkey)
WHERE t.relkind = 'r'
    AND t.relname = $1
    AND NOT ix.indisprimary
ORDER BY i.relname, a.attnum;
```

#### Views
```sql
-- Regular views
SELECT
    table_name AS view_name,
    view_definition
FROM information_schema.views
WHERE table_schema = 'public';

-- Materialized views
SELECT
    schemaname,
    matviewname,
    definition
FROM pg_matviews
WHERE schemaname = 'public';
```

#### Stored Procedures and Functions
```sql
SELECT
    routine_name,
    routine_type,
    data_type AS return_type,
    routine_definition
FROM information_schema.routines
WHERE routine_schema = 'public'
ORDER BY routine_name;
```

#### Triggers
```sql
SELECT
    trigger_name,
    event_manipulation AS event,
    action_timing AS timing,
    action_statement AS definition
FROM information_schema.triggers
WHERE event_object_table = $1
    AND event_object_schema = 'public';
```

#### CHECK Constraints
```sql
SELECT
    tc.constraint_name,
    cc.check_clause
FROM information_schema.table_constraints tc
JOIN information_schema.check_constraints cc
    ON tc.constraint_name = cc.constraint_name
WHERE tc.table_name = $1
    AND tc.constraint_type = 'CHECK'
    AND tc.table_schema = 'public';
```

### MySQL/MariaDB

#### Foreign Keys
```sql
SELECT
    CONSTRAINT_NAME,
    COLUMN_NAME,
    REFERENCED_TABLE_NAME,
    REFERENCED_COLUMN_NAME
FROM INFORMATION_SCHEMA.KEY_COLUMN_USAGE
WHERE TABLE_SCHEMA = DATABASE()
    AND TABLE_NAME = ?
    AND REFERENCED_TABLE_NAME IS NOT NULL;
```

#### Indexes
```sql
SELECT
    INDEX_NAME,
    COLUMN_NAME,
    NON_UNIQUE,
    INDEX_TYPE
FROM INFORMATION_SCHEMA.STATISTICS
WHERE TABLE_SCHEMA = DATABASE()
    AND TABLE_NAME = ?
    AND INDEX_NAME != 'PRIMARY'
ORDER BY INDEX_NAME, SEQ_IN_INDEX;
```

#### Views
```sql
SELECT
    TABLE_NAME AS view_name,
    VIEW_DEFINITION,
    IS_UPDATABLE
FROM INFORMATION_SCHEMA.VIEWS
WHERE TABLE_SCHEMA = DATABASE();
```

#### Stored Procedures
```sql
SELECT
    ROUTINE_NAME,
    ROUTINE_TYPE,
    DTD_IDENTIFIER AS return_type,
    ROUTINE_DEFINITION
FROM INFORMATION_SCHEMA.ROUTINES
WHERE ROUTINE_SCHEMA = DATABASE();
```

#### Triggers
```sql
SELECT
    TRIGGER_NAME,
    EVENT_MANIPULATION AS event,
    ACTION_TIMING AS timing,
    ACTION_STATEMENT AS definition
FROM INFORMATION_SCHEMA.TRIGGERS
WHERE EVENT_OBJECT_TABLE = ?
    AND TRIGGER_SCHEMA = DATABASE();
```

## MongoDB Commands

### Indexes
```javascript
db.collection.getIndexes()
```

### Views
```javascript
db.getCollectionInfos({ type: "view" })
```

### Validation
```javascript
db.getCollectionInfos({ name: "collectionName" })[0].options.validator
```

## Testing Strategy

1. **Unit Tests**: Test type mappings and parsing logic
2. **Integration Tests**: Test with real database instances
3. **Fixture Tests**: Small test databases with all features
4. **Edge Cases**: Empty databases, missing permissions, etc.

## Documentation Updates Required

- Update `/docs/adapters_db.md` with:
  - Examples of foreign key extraction
  - Index extraction examples
  - View support documentation
  - Stored procedure examples
  - Trigger examples
  - CHECK constraint examples

## Estimated Effort

- SQLite: ✅ Complete (~4 hours)
- PostgreSQL: ✅ Complete (~6-8 hours)
- MySQL/MariaDB: ✅ Complete (~6-8 hours)
- MongoDB: ✅ Complete (~4-6 hours)
- Testing: ✅ Complete (~2-3 hours)
- Documentation: ✅ Complete (~2-3 hours)

**Total**: ~26-35 hours of focused development work
**Completed**: ~26-35 hours (all connectors complete)

## Implementation Complete ✅

All four database connectors now support advanced features:
- SQLite: Foreign keys, indexes, views, triggers
- PostgreSQL: Foreign keys, indexes, views, stored procedures, triggers
- MySQL/MariaDB: Foreign keys, indexes, views, stored procedures, triggers
- MongoDB: Indexes, views

## Current Status

**Phase 1 Complete**: IR extended with all necessary structures
**Phase 2 Complete**: SQLite connector fully implemented
**Phase 3 Complete**: PostgreSQL connector fully implemented
**Phase 4 Complete**: MySQL/MariaDB connector fully implemented
**Phase 5 Complete**: MongoDB connector fully implemented

## Notes

- Some features are database-specific (e.g., materialized views in PostgreSQL)
- MongoDB's schema-less nature requires different approach to constraints
- CHECK constraints in MySQL require version 8.0.16+
- MongoDB validators could be extracted in future enhancement
- Testing with real databases may require Docker containers in CI
