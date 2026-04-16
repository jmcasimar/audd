# Remote File Adapters

AUDD supports loading schemas from remote files via HTTP/HTTPS URLs and public Google Sheets.

## Supported Protocols

- **HTTP/HTTPS URLs**: Direct file downloads
- **Google Sheets**: Public spreadsheets (automatically exported as CSV)

## Supported Formats

Remote files support the same formats as local files:
- CSV (`.csv`)
- JSON (`.json`)
- XML (`.xml`)
- SQL/DDL (`.sql`, `.ddl`)

## Connection String Formats

### HTTP/HTTPS URLs

Use a standard URL pointing to the file:

```bash
audd load --source "https://example.com/data.csv"
audd load --source "https://api.example.com/schema.json"
audd load --source "https://storage.example.com/schema.sql"
```

### Google Sheets (Public)

Use the Google Sheets URL directly. The file will be automatically exported as CSV:

```bash
audd load --source "https://docs.google.com/spreadsheets/d/SHEET_ID/edit"
audd load --source "https://docs.google.com/spreadsheets/d/SHEET_ID/edit#gid=0"
```

**Note**: The Google Sheet must be publicly accessible (shared with "Anyone with the link can view").

## CLI Usage Examples

### Basic Load from URL

```bash
# Load from HTTP URL
audd load --source "https://example.com/employees.csv"

# Load from HTTPS URL with query parameters
audd load --source "https://api.example.com/data.json?version=latest"

# Load from Google Sheets
audd load --source "https://docs.google.com/spreadsheets/d/1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms/edit"
```

### Compare Remote and Local Sources

```bash
# Compare remote CSV with local database
audd compare \
  --source-a "https://example.com/prod-schema.csv" \
  --source-b "db:sqlite:///local.db"

# Compare two Google Sheets
audd compare \
  --source-a "https://docs.google.com/spreadsheets/d/SHEET_A/edit" \
  --source-b "https://docs.google.com/spreadsheets/d/SHEET_B/edit"
```

### Explicit Format Specification

If the URL doesn't have a clear file extension, you can use the programmatic API with an explicit format hint:

```rust
use audd_adapters_file::RemoteAdapter;

let adapter = RemoteAdapter::with_format(
    "https://api.example.com/schema",
    "json"
);
let schema = adapter.load_schema()?;
```

## Programmatic API

### Basic Usage

```rust
use audd_adapters_file::{load_schema_from_url, AdapterError};

fn main() -> Result<(), AdapterError> {
    // Load from HTTP URL
    let schema = load_schema_from_url("https://example.com/data.csv")?;
    println!("Loaded {} entities", schema.entities.len());
    
    // Load from Google Sheets
    let sheet_url = "https://docs.google.com/spreadsheets/d/SHEET_ID/edit";
    let schema = load_schema_from_url(sheet_url)?;
    println!("Loaded from Google Sheets: {}", schema.source_name);
    
    Ok(())
}
```

### With Format Hint

```rust
use audd_adapters_file::load_schema_from_url_with_format;

// Load from URL without clear extension
let schema = load_schema_from_url_with_format(
    "https://api.example.com/data",
    "json"
)?;
```

### Custom Adapter

```rust
use audd_adapters_file::RemoteAdapter;

// Create adapter with auto-detection
let adapter = RemoteAdapter::new("https://example.com/data.csv");
let schema = adapter.load_schema()?;

// Create adapter with explicit format
let adapter = RemoteAdapter::with_format(
    "https://api.example.com/endpoint",
    "json"
);
let schema = adapter.load_schema()?;
```

## Format Detection

The adapter automatically detects the file format using:

1. **Explicit format hint** (if provided via `with_format()`)
2. **Google Sheets** detection (always treated as CSV)
3. **URL file extension** (e.g., `.csv`, `.json`, `.xml`, `.sql`)

If the format cannot be detected, an error is returned with a helpful message.

## Error Handling

Common errors and solutions:

### HTTP Errors

```
❌ Error loading remote schema: Failed to fetch URL https://example.com/data.csv: HTTP error 404
```

**Solution**: Verify the URL is correct and accessible.

### Format Detection Errors

```
❌ Error loading remote schema: Cannot detect format from URL: https://example.com/data
```

**Solution**: Use `load_schema_from_url_with_format()` with an explicit format hint.

### Access Denied (Google Sheets)

```
❌ Error loading remote schema: Failed to fetch URL: HTTP error 403
```

**Solution**: Ensure the Google Sheet is publicly accessible (shared with "Anyone with the link can view").

## Security Considerations

### HTTPS Recommended

Always use HTTPS URLs when possible to ensure data integrity and privacy:

```bash
# ✅ Good: HTTPS
audd load --source "https://example.com/data.csv"

# ⚠️  Warning: HTTP (insecure)
audd load --source "http://example.com/data.csv"
```

### Public Data Only

Remote file adapters are designed for publicly accessible data:

- ✅ Public Google Sheets
- ✅ Public HTTP(S) endpoints
- ✅ Open data portals
- ❌ Private/authenticated endpoints (not supported in current version)

### Data Privacy

When loading from remote URLs:
- Data is temporarily downloaded to a local temp file
- Temp files are automatically cleaned up after processing
- No credentials are stored or transmitted (except in URL itself)

## Implementation Details

### Download Process

1. **URL validation**: Checks if URL is Google Sheets or standard HTTP(S)
2. **Format detection**: Determines file format from URL or hint
3. **HTTP request**: Downloads file content using `ureq` library
4. **Temporary file**: Writes content to temp file with appropriate extension
5. **Adapter delegation**: Uses standard file adapter (CSV, JSON, XML, SQL) to parse
6. **Cleanup**: Temp file is automatically deleted

### Google Sheets Export

For Google Sheets URLs:
- Extracts the sheet ID from the URL
- Converts to export URL: `https://docs.google.com/spreadsheets/d/{SHEET_ID}/export?format=csv`
- Downloads as CSV
- Processes using CSV adapter

### Supported Sheet URL Formats

All standard Google Sheets URL formats are supported:
```
https://docs.google.com/spreadsheets/d/SHEET_ID/edit
https://docs.google.com/spreadsheets/d/SHEET_ID/edit#gid=0
https://docs.google.com/spreadsheets/d/SHEET_ID/edit?usp=sharing
```

## Comparison with Database Connectors

| Feature | Remote Files | Database Connectors |
|---------|-------------|---------------------|
| Network Access | HTTP/HTTPS | Database protocols |
| Authentication | URL-based | Connection string credentials |
| Data Format | Files (CSV, JSON, XML, SQL) | Database schemas |
| Connection | Stateless | Stateful |
| Performance | Download + parse | Direct schema queries |

## Future Enhancements

Potential future additions:
- Authentication support (API keys, OAuth)
- Private Google Sheets access
- Custom HTTP headers
- Download progress reporting
- Caching mechanism
- FTP/SFTP support

## Examples

### Real-World Use Cases

#### Public Data Portal

```bash
# Load schema from government open data
audd load --source "https://data.gov/catalog/dataset.csv"
```

#### Google Sheets Collaboration

```bash
# Team uses Google Sheets for schema documentation
audd compare \
  --source-a "https://docs.google.com/spreadsheets/d/PROD_SHEET/edit" \
  --source-b "db:postgresql://localhost/production"
```

#### API Schema Export

```bash
# API exports schema as JSON
audd load --source "https://api.example.com/v1/schema/export.json"
```

#### Cross-Format Comparison

```bash
# Compare remote JSON with local SQL file
audd compare \
  --source-a "https://example.com/schema.json" \
  --source-b "local-schema.sql"
```

## Testing

The remote adapter includes comprehensive tests:

```bash
# Run all file adapter tests (including remote)
cd crates/audd_adapters_file
cargo test

# Run specific remote adapter tests
cargo test remote_adapter::tests
```

Test coverage includes:
- Google Sheets URL detection and conversion
- Format detection from URLs
- Error handling for unsupported formats
- Edge cases (URLs with query parameters, etc.)

## Dependencies

Remote file support requires:
- `ureq`: HTTP client library (synchronous, lightweight)
- `tempfile`: Temporary file management

These are automatically included when using the `audd_adapters_file` crate.
