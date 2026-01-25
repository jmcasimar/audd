//! Core adapter trait

use audd_ir::SourceSchema;
use crate::error::AdapterResult;
use std::path::Path;

/// Trait for loading schemas from different file formats
pub trait SchemaAdapter {
    /// Load a schema from the given file path
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to load
    ///
    /// # Returns
    ///
    /// A `SourceSchema` representing the file's structure
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed
    fn load(&self, path: &Path) -> AdapterResult<SourceSchema>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct TestAdapter;

    impl SchemaAdapter for TestAdapter {
        fn load(&self, _path: &Path) -> AdapterResult<SourceSchema> {
            Ok(SourceSchema::builder()
                .source_name("test")
                .source_type("test")
                .build())
        }
    }

    #[test]
    fn test_trait_implementation() {
        let adapter = TestAdapter;
        let result = adapter.load(&PathBuf::from("test.txt"));
        assert!(result.is_ok());
        let schema = result.unwrap();
        assert_eq!(schema.source_name, "test");
    }
}
