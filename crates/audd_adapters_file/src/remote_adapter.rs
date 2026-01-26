//! Remote file adapter for fetching schemas from URLs
//!
//! This adapter supports:
//! - HTTP/HTTPS URLs for CSV, JSON, XML, and SQL files
//! - Google Sheets public URLs (exported as CSV)

use crate::adapter::SchemaAdapter;
use crate::csv_adapter::CsvAdapter;
use crate::error::{AdapterError, AdapterResult};
use crate::json_adapter::JsonAdapter;
use crate::sql_adapter::SqlAdapter;
use crate::xml_adapter::XmlAdapter;
use audd_ir::SourceSchema;
use std::io::{Write, Read};
use std::path::Path;
use tempfile::NamedTempFile;

/// Maximum allowed file size for remote downloads (50MB)
const MAX_FILE_SIZE: u64 = 50 * 1024 * 1024;

/// Validate URL for security concerns
fn validate_url_safety(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parsed = url::Url::parse(url)?;
    
    // Block localhost and private IPs
    if let Some(host) = parsed.host_str() {
        if host == "localhost" || host == "127.0.0.1" || host.starts_with("192.168.") 
            || host.starts_with("10.") || host.starts_with("172.16.") 
            || host.starts_with("172.17.") || host.starts_with("172.18.")
            || host.starts_with("172.19.") || host.starts_with("172.20.")
            || host.starts_with("172.21.") || host.starts_with("172.22.")
            || host.starts_with("172.23.") || host.starts_with("172.24.")
            || host.starts_with("172.25.") || host.starts_with("172.26.")
            || host.starts_with("172.27.") || host.starts_with("172.28.")
            || host.starts_with("172.29.") || host.starts_with("172.30.")
            || host.starts_with("172.31.") || host == "::1" 
            || host.starts_with("169.254.") {
            return Err("Private/localhost URLs not allowed".into());
        }
    }
    
    // Only allow HTTP/HTTPS
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err("Only HTTP/HTTPS URLs allowed".into());
    }
    
    Ok(())
}

/// Adapter for loading schemas from remote URLs
pub struct RemoteAdapter {
    url: String,
    format_hint: Option<String>,
}

impl RemoteAdapter {
    /// Create a new remote adapter for the given URL
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to fetch the file from
    ///
    /// # Example
    ///
    /// ```
    /// use audd_adapters_file::RemoteAdapter;
    ///
    /// let adapter = RemoteAdapter::new("https://example.com/data.csv");
    /// ```
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            format_hint: None,
        }
    }

    /// Create a new remote adapter with an explicit format hint
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to fetch the file from
    /// * `format` - The format hint (csv, json, xml, sql)
    ///
    /// # Example
    ///
    /// ```
    /// use audd_adapters_file::RemoteAdapter;
    ///
    /// let adapter = RemoteAdapter::with_format("https://example.com/data", "csv");
    /// ```
    pub fn with_format(url: impl Into<String>, format: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            format_hint: Some(format.into()),
        }
    }

    /// Detect if a URL is a Google Sheets URL
    fn is_google_sheets_url(&self) -> bool {
        self.url.contains("docs.google.com/spreadsheets")
    }

    /// Convert a Google Sheets URL to a CSV export URL
    fn convert_google_sheets_url(&self) -> String {
        if let Some(sheet_id) = self.extract_google_sheets_id() {
            format!(
                "https://docs.google.com/spreadsheets/d/{}/export?format=csv",
                sheet_id
            )
        } else {
            self.url.clone()
        }
    }

    /// Extract the Google Sheets ID from a URL
    fn extract_google_sheets_id(&self) -> Option<String> {
        // Handle URLs like:
        // https://docs.google.com/spreadsheets/d/SHEET_ID/edit...
        // https://docs.google.com/spreadsheets/d/SHEET_ID/...
        if let Some(start) = self.url.find("/spreadsheets/d/") {
            let id_start = start + "/spreadsheets/d/".len();
            let remaining = &self.url[id_start..];
            if let Some(end) = remaining.find('/') {
                Some(remaining[..end].to_string())
            } else {
                Some(remaining.to_string())
            }
        } else {
            None
        }
    }

    /// Detect the file format from URL or format hint
    fn detect_format(&self) -> AdapterResult<String> {
        // Use format hint if provided
        if let Some(ref format) = self.format_hint {
            return Ok(format.clone());
        }

        // Google Sheets are always CSV
        if self.is_google_sheets_url() {
            return Ok("csv".to_string());
        }

        // Try to detect from URL extension
        let url_path = self
            .url
            .split('?')
            .next()
            .unwrap_or(&self.url)
            .to_lowercase();

        if url_path.ends_with(".csv") {
            Ok("csv".to_string())
        } else if url_path.ends_with(".json") {
            Ok("json".to_string())
        } else if url_path.ends_with(".xml") {
            Ok("xml".to_string())
        } else if url_path.ends_with(".sql") || url_path.ends_with(".ddl") {
            Ok("sql".to_string())
        } else {
            Err(AdapterError::UnsupportedFormat(format!(
                "Cannot detect format from URL: {}. Please use with_format() to specify the format explicitly.",
                self.url
            )))
        }
    }

    /// Fetch the content from the URL
    fn fetch_content(&self) -> AdapterResult<Vec<u8>> {
        let url = if self.is_google_sheets_url() {
            self.convert_google_sheets_url()
        } else {
            self.url.clone()
        };

        // Validate URL for security
        validate_url_safety(&url).map_err(|e| {
            AdapterError::IoError(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("URL validation failed: {}", e),
            ))
        })?;

        // Use ureq for HTTP requests (synchronous, simple)
        let response = ureq::get(&url)
            .call()
            .map_err(|e| AdapterError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to fetch URL {}: {}", url, e),
            )))?;

        // Check status code
        if response.status() != 200 {
            return Err(AdapterError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("HTTP error {}: {}", response.status(), response.status_text()),
            )));
        }

        // Check content-length header for file size limit
        if let Some(content_length) = response.header("content-length") {
            if let Ok(size) = content_length.parse::<u64>() {
                if size > MAX_FILE_SIZE {
                    return Err(AdapterError::IoError(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("File too large: {} bytes (max: {} bytes)", size, MAX_FILE_SIZE),
                    )));
                }
            }
        }

        // Read the response body with size limit
        let reader = response.into_reader();
        let mut buffer = Vec::new();
        let bytes_read = std::io::copy(&mut reader.take(MAX_FILE_SIZE), &mut buffer)
            .map_err(|e| AdapterError::IoError(e))?;

        // Enforce size limit even if content-length header was missing
        if bytes_read >= MAX_FILE_SIZE {
            return Err(AdapterError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("File too large: reached {} byte limit", MAX_FILE_SIZE),
            )));
        }

        Ok(buffer)
    }

    /// Load schema from the remote URL
    pub fn load_schema(&self) -> AdapterResult<SourceSchema> {
        // Detect format
        let format = self.detect_format()?;

        // Fetch content
        let content = self.fetch_content()?;

        // Create a temporary file with the appropriate extension
        let suffix = format!(".{}", format);
        let mut temp_file = NamedTempFile::with_suffix(&suffix)
            .map_err(|e| AdapterError::IoError(e))?;
        
        temp_file
            .write_all(&content)
            .map_err(|e| AdapterError::IoError(e))?;
        temp_file
            .flush()
            .map_err(|e| AdapterError::IoError(e))?;

        // Use the appropriate adapter based on format
        let mut schema = match format.as_str() {
            "csv" => CsvAdapter::new().load(temp_file.path())?,
            "json" => JsonAdapter::new().load(temp_file.path())?,
            "xml" => XmlAdapter::new().load(temp_file.path())?,
            "sql" | "ddl" => SqlAdapter::new().load(temp_file.path())?,
            _ => {
                return Err(AdapterError::UnsupportedFormat(format!(
                    "Unsupported format: {}",
                    format
                )))
            }
        };

        // Update source name to reflect it's from a remote URL
        schema.source_name = if self.is_google_sheets_url() {
            format!("google_sheets:{}", self.url)
        } else {
            format!("remote:{}", self.url)
        };
        schema.source_type = format!("remote_{}", format);

        Ok(schema)
    }
}

impl SchemaAdapter for RemoteAdapter {
    fn load(&self, _path: &Path) -> AdapterResult<SourceSchema> {
        // For remote adapter, we ignore the path parameter and use the URL instead
        self.load_schema()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_sheets_id_extraction() {
        let adapter = RemoteAdapter::new(
            "https://docs.google.com/spreadsheets/d/1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms/edit#gid=0"
        );
        assert_eq!(
            adapter.extract_google_sheets_id(),
            Some("1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms".to_string())
        );
    }

    #[test]
    fn test_google_sheets_url_conversion() {
        let adapter = RemoteAdapter::new(
            "https://docs.google.com/spreadsheets/d/SHEET_ID/edit#gid=0"
        );
        assert_eq!(
            adapter.convert_google_sheets_url(),
            "https://docs.google.com/spreadsheets/d/SHEET_ID/export?format=csv"
        );
    }

    #[test]
    fn test_is_google_sheets_url() {
        let adapter1 = RemoteAdapter::new("https://docs.google.com/spreadsheets/d/123/edit");
        assert!(adapter1.is_google_sheets_url());

        let adapter2 = RemoteAdapter::new("https://example.com/data.csv");
        assert!(!adapter2.is_google_sheets_url());
    }

    #[test]
    fn test_format_detection() {
        let adapter1 = RemoteAdapter::new("https://example.com/data.csv");
        assert_eq!(adapter1.detect_format().unwrap(), "csv");

        let adapter2 = RemoteAdapter::new("https://example.com/data.json?param=value");
        assert_eq!(adapter2.detect_format().unwrap(), "json");

        let adapter3 = RemoteAdapter::new("https://example.com/data.xml");
        assert_eq!(adapter3.detect_format().unwrap(), "xml");

        let adapter4 = RemoteAdapter::new("https://example.com/schema.sql");
        assert_eq!(adapter4.detect_format().unwrap(), "sql");

        let adapter5 = RemoteAdapter::with_format("https://example.com/data", "csv");
        assert_eq!(adapter5.detect_format().unwrap(), "csv");
    }

    #[test]
    fn test_google_sheets_format_detection() {
        let adapter = RemoteAdapter::new("https://docs.google.com/spreadsheets/d/123/edit");
        assert_eq!(adapter.detect_format().unwrap(), "csv");
    }

    #[test]
    fn test_unknown_format_error() {
        let adapter = RemoteAdapter::new("https://example.com/data");
        assert!(adapter.detect_format().is_err());
    }
}
