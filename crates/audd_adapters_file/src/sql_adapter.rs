//! SQL/DDL adapter implementation

use crate::adapter::SchemaAdapter;
use crate::error::{AdapterError, AdapterResult};
use audd_ir::{CanonicalType, Constraint, EntitySchema, FieldSchema, Key, KeyType, SourceSchema};
use std::fs;
use std::path::Path;

/// Adapter for SQL DDL files
///
/// Supports the subset of DDL found in typical MySQL dumps:
/// - Column definitions with types (CHAR/VARCHAR lengths become `MaxLength`,
///   DECIMAL keeps precision and scale, TINYINT(1) is Boolean)
/// - NOT NULL, inline PRIMARY KEY and UNIQUE
/// - Table-level PRIMARY KEY, UNIQUE and FOREIGN KEY clauses (other KEY,
///   INDEX and CHECK clauses are skipped)
/// - `CREATE UNIQUE INDEX` and `ALTER TABLE ... ADD CONSTRAINT` keys
/// - Quoted identifiers and SQL comments
///
/// Example:
/// ```sql
/// CREATE TABLE users (
///     id INT PRIMARY KEY,
///     name VARCHAR(255) NOT NULL,
///     email VARCHAR(255) UNIQUE
/// );
/// ```
pub struct SqlAdapter;

impl SqlAdapter {
    /// Create a new SQL adapter
    pub fn new() -> Self {
        SqlAdapter
    }

    /// Parse a simple CREATE TABLE statement
    fn parse_create_table(sql: &str) -> Result<(String, Vec<FieldSchema>, Vec<Key>), String> {
        let sql = sql.trim();

        // Extract table name
        let table_name = Self::extract_table_name(sql)?;

        // Extract column definitions
        let columns_section = Self::extract_columns_section(sql)?;
        let (fields, keys) = Self::parse_columns(&columns_section)?;

        Ok((table_name, fields, keys))
    }

    fn extract_table_name(sql: &str) -> Result<String, String> {
        let upper = sql.to_uppercase();
        if let Some(start) = upper.find("CREATE TABLE") {
            // Calculate position after "CREATE TABLE" in original string
            let pos_after_create = start + 12;
            if pos_after_create >= sql.len() {
                return Err("Invalid SQL after CREATE TABLE".to_string());
            }

            let after_create = sql
                .get(pos_after_create..)
                .ok_or("Invalid SQL structure")?
                .trim_start();

            // Handle optional IF NOT EXISTS
            let after_if = if after_create.to_uppercase().starts_with("IF NOT EXISTS") {
                let skip_len = "IF NOT EXISTS".len();
                if skip_len >= after_create.len() {
                    return Err("Invalid SQL after IF NOT EXISTS".to_string());
                }
                after_create
                    .get(skip_len..)
                    .ok_or("Invalid SQL after IF NOT EXISTS")?
                    .trim_start()
            } else {
                after_create
            };

            // Find table name (before opening parenthesis)
            if let Some(paren_pos) = after_if.find('(') {
                let name = after_if
                    .get(..paren_pos)
                    .ok_or("Invalid table name")?
                    .trim();
                // Remove quotes if present
                let name = name.trim_matches('`').trim_matches('"').trim_matches('\'');
                return Ok(name.to_string());
            }
        }
        Err("Invalid CREATE TABLE syntax".to_string())
    }

    fn extract_columns_section(sql: &str) -> Result<String, String> {
        if let Some(start) = sql.find('(') {
            if let Some(end) = sql.rfind(')') {
                if end > start {
                    return sql
                        .get(start + 1..end)
                        .map(|s| s.to_string())
                        .ok_or_else(|| "Could not extract column definitions".to_string());
                }
            }
        }
        Err("Could not find column definitions".to_string())
    }

    fn parse_columns(columns: &str) -> Result<(Vec<FieldSchema>, Vec<Key>), String> {
        let mut fields = Vec::new();
        let mut primary_keys = Vec::new();
        let mut other_keys = Vec::new();

        // Split by comma, but be careful with nested parentheses
        let column_defs = Self::split_column_definitions(columns);

        for def in column_defs {
            let def = def.trim();
            if def.is_empty() || def.starts_with("--") {
                continue;
            }

            // Table-level clauses describe keys and constraints, not columns
            if let Some(clause) = Self::parse_table_clause(def) {
                match clause {
                    TableClause::PrimaryKey(cols) => primary_keys.extend(cols),
                    TableClause::Key(key) => other_keys.push(key),
                    TableClause::Ignored => {}
                }
                continue;
            }

            // Parse column definition
            if let Some(field) = Self::parse_column_definition(def)? {
                if field.0 {
                    // This column is a primary key
                    primary_keys.push(field.1.field_name.clone());
                }
                fields.push(field.1);
            }
        }

        let mut keys = Vec::new();
        if !primary_keys.is_empty() {
            keys.push(Key::primary(primary_keys));
        }
        keys.extend(other_keys);

        Ok((fields, keys))
    }

    /// Recognize table-level clauses such as `PRIMARY KEY (...)`, `UNIQUE KEY name (...)`,
    /// `FOREIGN KEY (...) REFERENCES ...`, `KEY idx (...)` or `CHECK (...)`.
    fn parse_table_clause(def: &str) -> Option<TableClause> {
        let upper = def.to_uppercase();
        // `CONSTRAINT name <clause>`: drop the constraint name
        let (upper, def) = if upper.starts_with("CONSTRAINT") {
            let rest = def["CONSTRAINT".len()..].trim_start();
            let (_, after_name) = Self::split_identifier(rest);
            let after_name = after_name.trim_start();
            (after_name.to_uppercase(), after_name)
        } else {
            (upper, def)
        };

        let starts = |kw: &str| {
            upper.starts_with(kw)
                && upper[kw.len()..]
                    .chars()
                    .next()
                    .is_none_or(|c| c.is_whitespace() || c == '(')
        };

        if starts("PRIMARY") {
            return Some(TableClause::PrimaryKey(Self::paren_columns(def)));
        }
        if starts("UNIQUE") {
            let cols = Self::paren_columns(def);
            return Some(if cols.is_empty() {
                TableClause::Ignored
            } else {
                TableClause::Key(Key::unique(cols))
            });
        }
        if starts("FOREIGN") {
            let cols = Self::paren_columns(def);
            return Some(if cols.is_empty() {
                TableClause::Ignored
            } else {
                TableClause::Key(Key::foreign(cols))
            });
        }
        if ["KEY", "INDEX", "FULLTEXT", "SPATIAL", "CHECK", "EXCLUDE"]
            .iter()
            .any(|kw| starts(kw))
        {
            return Some(TableClause::Ignored);
        }
        None
    }

    /// Column names inside the first pair of parentheses, e.g. `(a, b(10) DESC)` -> [a, b]
    fn paren_columns(def: &str) -> Vec<String> {
        let Some(start) = def.find('(') else {
            return Vec::new();
        };
        let mut depth = 0;
        let mut end = None;
        for (i, ch) in def[start..].char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(start + i);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(end) = end else {
            return Vec::new();
        };
        Self::split_column_definitions(&def[start + 1..end])
            .iter()
            .map(|c| Self::split_identifier(c.trim()).0)
            .filter(|c| !c.is_empty())
            .collect()
    }

    /// Split a leading identifier (bare or quoted with backticks, double quotes or
    /// brackets) from the rest of the definition
    fn split_identifier(s: &str) -> (String, &str) {
        let s = s.trim_start();
        let close = match s.chars().next() {
            Some('`') => Some('`'),
            Some('"') => Some('"'),
            Some('[') => Some(']'),
            _ => None,
        };
        if let Some(close) = close {
            if let Some(end) = s[1..].find(close) {
                return (s[1..end + 1].to_string(), &s[end + 2..]);
            }
        }
        let end = s
            .find(|c: char| c.is_whitespace() || c == '(')
            .unwrap_or(s.len());
        (s[..end].to_string(), &s[end..])
    }

    fn split_column_definitions(columns: &str) -> Vec<String> {
        let mut defs = Vec::new();
        let mut current = String::new();
        let mut paren_depth = 0;

        for ch in columns.chars() {
            match ch {
                '(' => {
                    paren_depth += 1;
                    current.push(ch);
                }
                ')' => {
                    paren_depth -= 1;
                    current.push(ch);
                }
                ',' if paren_depth == 0 => {
                    if !current.trim().is_empty() {
                        defs.push(current.trim().to_string());
                    }
                    current.clear();
                }
                _ => current.push(ch),
            }
        }

        if !current.trim().is_empty() {
            defs.push(current.trim().to_string());
        }

        defs
    }

    fn parse_column_definition(def: &str) -> Result<Option<(bool, FieldSchema)>, String> {
        let (field_name, rest) = Self::split_identifier(def);
        if field_name.is_empty() {
            return Ok(None);
        }

        let rest = rest.trim_start();
        if rest.is_empty() {
            return Err(format!("Invalid column definition: {}", def));
        }

        // The type runs until the first whitespace outside parentheses,
        // so `DECIMAL(10, 2)` and `ENUM('a', 'b')` stay in one piece
        let mut depth = 0;
        let mut type_end = rest.len();
        for (i, ch) in rest.char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => depth -= 1,
                c if c.is_whitespace() && depth == 0 => {
                    type_end = i;
                    break;
                }
                _ => {}
            }
        }
        let type_str = &rest[..type_end];
        let canonical_type = Self::map_sql_type(type_str);

        let def_upper = def.to_uppercase();
        let is_primary = def_upper.contains("PRIMARY KEY");
        let nullable = !def_upper.contains("NOT NULL") && !is_primary;

        let mut builder = FieldSchema::builder()
            .field_name(field_name)
            .canonical_type(canonical_type)
            .nullable(nullable);

        if let Some(len) = Self::declared_char_length(type_str) {
            builder = builder.add_constraint(Constraint::max_length(len));
        }
        if def_upper.contains("UNIQUE") {
            builder = builder.add_constraint(Constraint::unique());
        }

        Ok(Some((is_primary, builder.build())))
    }

    /// Numeric parameters of a type, e.g. `DECIMAL(8,3)` -> `[8, 3]`
    fn type_params(sql_type: &str) -> Vec<u16> {
        let (Some(start), Some(end)) = (sql_type.find('('), sql_type.rfind(')')) else {
            return Vec::new();
        };
        if end <= start {
            return Vec::new();
        }
        sql_type[start + 1..end]
            .split(',')
            .filter_map(|p| p.trim().parse().ok())
            .collect()
    }

    /// Declared length of CHAR/VARCHAR types, e.g. `VARCHAR(80)` -> 80
    fn declared_char_length(sql_type: &str) -> Option<usize> {
        let upper = sql_type.to_uppercase();
        let base = upper.split('(').next().unwrap_or(&upper);
        match base {
            "CHAR" | "VARCHAR" | "NCHAR" | "NVARCHAR" | "CHARACTER" => {
                match Self::type_params(&upper).as_slice() {
                    [len] => Some(*len as usize),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn map_sql_type(sql_type: &str) -> CanonicalType {
        let sql_type = sql_type.to_uppercase();
        let base_type = sql_type.split('(').next().unwrap_or(&sql_type);

        match base_type {
            // TINYINT(1) is the conventional MySQL boolean
            "TINYINT" if Self::type_params(&sql_type) == [1] => CanonicalType::Boolean,
            "TINYINT" | "INT" | "INTEGER" | "SMALLINT" | "MEDIUMINT" => CanonicalType::Int32,
            "BIGINT" | "LONG" => CanonicalType::Int64,
            "FLOAT" | "REAL" => CanonicalType::Float32,
            "DOUBLE" => CanonicalType::Float64,
            "DECIMAL" | "NUMERIC" => match Self::type_params(&sql_type).as_slice() {
                [p, s] => CanonicalType::Decimal {
                    precision: *p,
                    scale: *s,
                },
                [p] => CanonicalType::Decimal {
                    precision: *p,
                    scale: 0,
                },
                _ => CanonicalType::Decimal {
                    precision: 10,
                    scale: 0,
                },
            },
            "BOOLEAN" | "BOOL" | "BIT" => CanonicalType::Boolean,
            "CHAR" | "VARCHAR" | "NVARCHAR" | "NCHAR" | "CHARACTER" | "ENUM" | "SET" => {
                CanonicalType::String
            }
            "TEXT" | "TINYTEXT" | "CLOB" | "LONGTEXT" | "MEDIUMTEXT" => CanonicalType::Text,
            "BLOB" | "TINYBLOB" | "MEDIUMBLOB" | "LONGBLOB" | "BINARY" | "VARBINARY" => {
                CanonicalType::Binary
            }
            "DATE" => CanonicalType::Date,
            "TIME" => CanonicalType::Time,
            "DATETIME" => CanonicalType::DateTime,
            "TIMESTAMP" => CanonicalType::Timestamp,
            "YEAR" => CanonicalType::Int32,
            "JSON" => CanonicalType::Json,
            "UUID" => CanonicalType::Uuid,
            _ => CanonicalType::Unknown {
                original_type: sql_type.clone(),
            },
        }
    }
}

impl SqlAdapter {
    /// Keys declared in their own statement:
    /// `CREATE UNIQUE INDEX name ON table (cols)` and
    /// `ALTER TABLE table ADD [CONSTRAINT name] PRIMARY KEY | UNIQUE | FOREIGN KEY (cols) ...`
    fn parse_standalone_key(statement: &str) -> Option<(String, Key)> {
        let upper = statement.to_uppercase();
        let words: Vec<&str> = upper.split_whitespace().take(3).collect();

        if words.len() == 3 && words[..3] == ["CREATE", "UNIQUE", "INDEX"] {
            let on = upper.find(" ON ")?;
            let (table, rest) = Self::split_identifier(&statement[on + 4..]);
            let cols = Self::paren_columns(rest);
            return (!cols.is_empty()).then(|| (table, Key::unique(cols)));
        }

        if words.len() >= 2 && words[..2] == ["ALTER", "TABLE"] {
            let after =
                statement.trim_start()["ALTER".len()..].trim_start()["TABLE".len()..].trim_start();
            let (table, rest) = Self::split_identifier(after);
            let rest = rest.trim_start();
            if !rest.to_uppercase().starts_with("ADD ") {
                return None;
            }
            return match Self::parse_table_clause(rest[4..].trim_start())? {
                TableClause::PrimaryKey(cols) if !cols.is_empty() => {
                    Some((table, Key::primary(cols)))
                }
                TableClause::Key(key) => Some((table, key)),
                _ => None,
            };
        }
        None
    }
}

/// Table-level clause inside CREATE TABLE
enum TableClause {
    PrimaryKey(Vec<String>),
    Key(Key),
    Ignored,
}

/// Remove `-- ...`, `# ...` and `/* ... */` comments (including MySQL `/*!NNNNN ... */`
/// conditional comments), leaving string literals and quoted identifiers intact.
fn strip_sql_comments(sql: &str) -> String {
    let mut out = String::with_capacity(sql.len());
    let mut chars = sql.chars().peekable();
    let mut quote: Option<char> = None;

    while let Some(ch) = chars.next() {
        if let Some(q) = quote {
            out.push(ch);
            if ch == '\\' && q != '`' {
                if let Some(next) = chars.next() {
                    out.push(next);
                }
            } else if ch == q {
                quote = None;
            }
            continue;
        }
        match ch {
            '\'' | '"' | '`' => {
                quote = Some(ch);
                out.push(ch);
            }
            '-' if chars.peek() == Some(&'-') => {
                for c in chars.by_ref() {
                    if c == '\n' {
                        out.push('\n');
                        break;
                    }
                }
            }
            '#' => {
                for c in chars.by_ref() {
                    if c == '\n' {
                        out.push('\n');
                        break;
                    }
                }
            }
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                let mut prev = '\0';
                for c in chars.by_ref() {
                    if prev == '*' && c == '/' {
                        break;
                    }
                    prev = c;
                }
                out.push(' ');
            }
            _ => out.push(ch),
        }
    }
    out
}

impl Default for SqlAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaAdapter for SqlAdapter {
    fn load(&self, path: &Path) -> AdapterResult<SourceSchema> {
        let content = strip_sql_comments(&fs::read_to_string(path)?);

        // Support multiple CREATE TABLE statements
        let mut entities = Vec::new();

        for statement in content.split(';') {
            let statement = statement.trim();
            if statement.to_uppercase().contains("CREATE TABLE") {
                match Self::parse_create_table(statement) {
                    Ok((table_name, fields, keys)) => {
                        let entity = EntitySchema::builder()
                            .entity_name(table_name)
                            .entity_type("table")
                            .fields(fields)
                            .keys(keys)
                            .build();
                        entities.push(entity);
                    }
                    Err(e) => return Err(AdapterError::SqlError(e)),
                }
            }
        }

        if entities.is_empty() {
            return Err(AdapterError::EmptyData(
                "No CREATE TABLE statements found".to_string(),
            ));
        }

        // Keys declared outside CREATE TABLE
        for statement in content.split(';') {
            if let Some((table, key)) = SqlAdapter::parse_standalone_key(statement.trim()) {
                if let Some(entity) = entities
                    .iter_mut()
                    .find(|e| e.entity_name.eq_ignore_ascii_case(&table))
                {
                    if key.key_type == KeyType::Primary {
                        entity.keys.retain(|k| k.key_type != KeyType::Primary);
                    }
                    entity.keys.push(key);
                }
            }
        }

        // Derive source name from filename
        let source_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("sql_source")
            .to_string();

        Ok(SourceSchema::builder()
            .source_name(source_name)
            .source_type("sql")
            .entities(entities)
            .build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_sql_basic() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
CREATE TABLE users (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255)
);"#
        )
        .unwrap();

        let adapter = SqlAdapter::new();
        let schema = adapter.load(file.path()).unwrap();

        assert_eq!(schema.source_type, "sql");
        assert_eq!(schema.entities.len(), 1);

        let entity = &schema.entities[0];
        assert_eq!(entity.entity_name, "users");
        assert_eq!(entity.fields.len(), 3);

        let id_field = entity.fields.iter().find(|f| f.field_name == "id").unwrap();
        assert_eq!(id_field.canonical_type, CanonicalType::Int32);
        assert!(!id_field.nullable);

        let name_field = entity
            .fields
            .iter()
            .find(|f| f.field_name == "name")
            .unwrap();
        assert!(!name_field.nullable);

        assert_eq!(entity.keys.len(), 1);
    }

    #[test]
    fn test_sql_multiple_tables() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
CREATE TABLE users (
    id INT PRIMARY KEY,
    name VARCHAR(255)
);

CREATE TABLE posts (
    id INT PRIMARY KEY,
    title TEXT
);"#
        )
        .unwrap();

        let adapter = SqlAdapter::new();
        let schema = adapter.load(file.path()).unwrap();

        assert_eq!(schema.entities.len(), 2);
        assert_eq!(schema.entities[0].entity_name, "users");
        assert_eq!(schema.entities[1].entity_name, "posts");
    }

    #[test]
    fn test_sql_type_mapping() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
CREATE TABLE types_test (
    int_col INT,
    bigint_col BIGINT,
    float_col FLOAT,
    double_col DOUBLE,
    bool_col BOOLEAN,
    text_col TEXT,
    date_col DATE,
    timestamp_col TIMESTAMP
);"#
        )
        .unwrap();

        let adapter = SqlAdapter::new();
        let schema = adapter.load(file.path()).unwrap();
        let entity = &schema.entities[0];

        assert_eq!(
            entity
                .fields
                .iter()
                .find(|f| f.field_name == "int_col")
                .unwrap()
                .canonical_type,
            CanonicalType::Int32
        );
        assert_eq!(
            entity
                .fields
                .iter()
                .find(|f| f.field_name == "bigint_col")
                .unwrap()
                .canonical_type,
            CanonicalType::Int64
        );
        assert_eq!(
            entity
                .fields
                .iter()
                .find(|f| f.field_name == "bool_col")
                .unwrap()
                .canonical_type,
            CanonicalType::Boolean
        );
    }

    #[test]
    fn test_sql_mysqldump_table() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
-- Dump header; a comment with a ; semicolon
CREATE TABLE `order details` (
  `id` int NOT NULL AUTO_INCREMENT, -- surrogate key (auto, aproximadamente)
  `order id` int NOT NULL,
  `code` varchar(20) NOT NULL,
  `price` decimal(8, 3) DEFAULT NULL,
  `active` tinyint(1) NOT NULL DEFAULT '1',
  `qty` tinyint unsigned NOT NULL,
  `created` datetime NOT NULL,
  `notes` text COMMENT 'free text -- not a comment',
  PRIMARY KEY (`id`),
  /*!50705 SPATIAL KEY `idx_location` (`notes`),*/
  UNIQUE KEY `uq_code` (`code`),
  KEY `idx_order` (`order id`),
  CONSTRAINT `fk_order` FOREIGN KEY (`order id`) REFERENCES `orders` (`id`),
  CONSTRAINT `chk_qty` CHECK ((`qty` > 0))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"#
        )
        .unwrap();

        let schema = SqlAdapter::new().load(file.path()).unwrap();
        let entity = &schema.entities[0];
        assert_eq!(entity.entity_name, "order details");

        // Only real columns, no fields made from table-level clauses
        let names: Vec<_> = entity
            .fields
            .iter()
            .map(|f| f.field_name.as_str())
            .collect();
        assert_eq!(
            names,
            vec!["id", "order id", "code", "price", "active", "qty", "created", "notes"]
        );

        let get = |n: &str| entity.fields.iter().find(|f| f.field_name == n).unwrap();
        assert_eq!(get("code").constraints, vec![Constraint::max_length(20)]);
        assert_eq!(
            get("price").canonical_type,
            CanonicalType::Decimal {
                precision: 8,
                scale: 3
            }
        );
        assert_eq!(get("active").canonical_type, CanonicalType::Boolean);
        assert_eq!(get("qty").canonical_type, CanonicalType::Int32);
        assert_eq!(get("created").canonical_type, CanonicalType::DateTime);
        assert_eq!(get("notes").canonical_type, CanonicalType::Text);
        assert!(!get("id").nullable);

        let key = |t: audd_ir::KeyType| {
            entity
                .keys
                .iter()
                .filter(|k| k.key_type == t)
                .map(|k| k.field_names.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(key(audd_ir::KeyType::Primary), vec![vec!["id".to_string()]]);
        assert_eq!(
            key(audd_ir::KeyType::Unique),
            vec![vec!["code".to_string()]]
        );
        assert_eq!(
            key(audd_ir::KeyType::Foreign),
            vec![vec!["order id".to_string()]]
        );
    }

    #[test]
    fn test_sql_standalone_keys() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
CREATE TABLE people (ID VARCHAR(255) NOT NULL, NAME VARCHAR(255) NOT NULL);
CREATE UNIQUE INDEX PEOPLE_NAME_INX ON PEOPLE(NAME);
ALTER TABLE `people` ADD CONSTRAINT `PK_people` PRIMARY KEY (`ID`);
CREATE TABLE tickets (ID INT NOT NULL, PERSON VARCHAR(255));
ALTER TABLE tickets ADD CONSTRAINT FK_person FOREIGN KEY (PERSON) REFERENCES people (ID);
CREATE INDEX IDX_person ON tickets (PERSON);"#
        )
        .unwrap();

        let schema = SqlAdapter::new().load(file.path()).unwrap();
        let keys = |name: &str| {
            schema
                .entities
                .iter()
                .find(|e| e.entity_name == name)
                .unwrap()
                .keys
                .iter()
                .map(|k| (k.key_type, k.field_names.clone()))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            keys("people"),
            vec![
                (KeyType::Unique, vec!["NAME".to_string()]),
                (KeyType::Primary, vec!["ID".to_string()]),
            ]
        );
        assert_eq!(
            keys("tickets"),
            vec![(KeyType::Foreign, vec!["PERSON".to_string()])]
        );
    }
}
