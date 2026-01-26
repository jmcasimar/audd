# Contributing to AUDD

**🌐 Language / Idioma:**  
📘 [Español](../Contributing.md) | 📗 **English**

---

Thank you for your interest in contributing to AUDD! This document provides guidelines and best practices for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Environment Setup](#development-environment-setup)
- [Development Workflow](#development-workflow)
- [Running Tests](#running-tests)
- [Adding New Adapters](#adding-new-adapters)
- [Documentation Standards](#documentation-standards)
- [Code Review Process](#code-review-process)
- [Commit Guidelines](#commit-guidelines)
- [Pull Requests](#pull-requests)
- [Code Style](#code-style)

---

## Code of Conduct

This project and all participants are governed by the [AUDD Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior through GitHub Issues.

---

## Getting Started

### 1. Fork and Clone the Repository

```bash
# 1. Fork the repository on GitHub
# (Click the "Fork" button on https://github.com/jmcasimar/AUDD)

# 2. Clone your fork
git clone https://github.com/YOUR_USERNAME/AUDD.git
cd AUDD

# 3. Add upstream remote
git remote add upstream https://github.com/jmcasimar/AUDD.git

# 4. Verify remotes
git remote -v
# origin    https://github.com/YOUR_USERNAME/AUDD.git (fetch)
# origin    https://github.com/YOUR_USERNAME/AUDD.git (push)
# upstream  https://github.com/jmcasimar/AUDD.git (fetch)
# upstream  https://github.com/jmcasimar/AUDD.git (push)
```

### 2. Create a Feature Branch

```bash
# Update main
git checkout main
git pull upstream main

# Create feature branch
git checkout -b feature/my-new-feature

# Or for bugfixes
git checkout -b fix/bug-description
```

---

## Development Environment Setup

### Prerequisites

**Required:**
- **Rust 1.70+** - Install with [rustup](https://rustup.rs/)
- **Cargo** - Included with Rust
- **Git** - Version control

**Optional (for DB adapters):**
- **SQLite** - Included by default
- **MySQL client libraries** - For MySQL support
  ```bash
  # Ubuntu/Debian
  sudo apt-get install libmysqlclient-dev
  
  # macOS
  brew install mysql-client
  ```
- **PostgreSQL client libraries** - For PostgreSQL support
  ```bash
  # Ubuntu/Debian
  sudo apt-get install libpq-dev
  
  # macOS
  brew install postgresql
  ```

### Installing Rust

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Activate in current session
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Install Additional Components

```bash
# Rustfmt (code formatter)
rustup component add rustfmt

# Clippy (linter)
rustup component add clippy

# Rust Analyzer (optional, for IDEs)
rustup component add rust-analyzer
```

### Build the Project

```bash
# Build in development mode
cargo build

# Build in release mode (optimized)
cargo build --release

# Binary will be at:
# - Development: ./target/debug/audd
# - Release: ./target/release/audd
```

### Verify Setup

```bash
# Run tests
cargo test

# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# If everything passes, your environment is ready ✅
```

---

## Development Workflow

### Typical Development Cycle

```bash
# 1. Update your branch with latest changes
git checkout main
git pull upstream main
git checkout feature/my-feature
git rebase main

# 2. Make code changes
# ... edit files ...

# 3. Run tests while developing
cargo test

# 4. Format code
cargo fmt

# 5. Check with clippy
cargo clippy --all-targets --all-features

# 6. Commit changes
git add .
git commit -m "feat: Add new functionality X"

# 7. Push to your fork
git push origin feature/my-feature

# 8. Create Pull Request on GitHub
```

### Useful Commands During Development

```bash
# Build and run in one command
cargo run -- --help
cargo run -- inspect --source fixtures/adapters/users.csv

# View detailed warnings
cargo build --verbose

# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Verify project builds without warnings
cargo build --all-targets --all-features 2>&1 | tee build.log
```

---

## Running Tests

AUDD has different testing levels to ensure code quality.

### Unit Tests

```bash
# Run all tests
cargo test

# Tests for a specific crate
cargo test -p audd_ir
cargo test -p audd_compare
cargo test -p audd_adapters_file
cargo test -p audd_adapters_db
cargo test -p audd_resolution
cargo test -p audd-cli

# Specific test by name
cargo test test_csv_adapter
cargo test test_comparison_engine

# Run tests with detailed output
cargo test -- --nocapture

# Run tests with logging
RUST_LOG=debug cargo test -- --nocapture
```

### Integration Tests

```bash
# Integration tests are in each crate's tests/
# Run adapter integration tests
cargo test -p audd_adapters_file --test integration_test

# CLI tests
cargo test -p audd-cli --test cli_tests
cargo test -p audd-cli --test report_tests
```

### Coverage Tests (Optional)

```bash
# Install tarpaulin (coverage tool)
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --all-features --workspace --timeout 120 --out Html

# View report in tarpaulin-report.html
```

### Running Tests in Parallel

```bash
# By default cargo runs tests in parallel
cargo test

# Run tests sequentially (useful for debugging)
cargo test -- --test-threads=1
```

### Category-Specific Tests

```bash
# Only tests containing "csv" in the name
cargo test csv

# Only tests containing "compare"
cargo test compare

# Exclude slow tests (if marked with #[ignore])
cargo test -- --ignored
```

### Benchmarks (Future)

```bash
# When benchmarks are implemented
cargo bench
```

---

## Adding New Adapters

AUDD is designed to be extensible. Here's how to add support for a new format or database.

### Adding a File Adapter

**Example: Adding YAML format support**

#### Step 1: Create Adapter File

```bash
# Navigate to file adapters crate
cd crates/audd_adapters_file/src/

# Create new adapter file
touch yaml_adapter.rs
```

#### Step 2: Implement the `SchemaAdapter` Trait

```rust
// crates/audd_adapters_file/src/yaml_adapter.rs

use audd_ir::{SourceSchema, EntitySchema, FieldSchema, CanonicalType};
use crate::adapter::SchemaAdapter;
use crate::error::{AdapterResult, AdapterError};
use std::path::Path;
use std::fs;

pub struct YamlAdapter;

impl SchemaAdapter for YamlAdapter {
    fn load(&self, path: &Path) -> AdapterResult<SourceSchema> {
        // Read file
        let content = fs::read_to_string(path)
            .map_err(|e| AdapterError::IoError(e))?;
        
        // Parse YAML (you'll need to add serde_yaml to dependencies)
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;
        
        // Convert to IR
        let entities = self.parse_yaml_to_entities(&yaml_value)?;
        
        // Build SourceSchema
        Ok(SourceSchema::builder()
            .source_name(path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown"))
            .source_type("yaml")
            .entities(entities)
            .build())
    }
}

impl YamlAdapter {
    fn parse_yaml_to_entities(&self, yaml: &serde_yaml::Value) 
        -> AdapterResult<Vec<EntitySchema>> {
        // Implement parsing logic
        // This depends on YAML structure
        todo!("Implement YAML to EntitySchema parsing")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_yaml_adapter_load() {
        let adapter = YamlAdapter;
        let path = Path::new("fixtures/test.yaml");
        // Add assertions
    }
}
```

#### Step 3: Register in Factory

```rust
// crates/audd_adapters_file/src/factory.rs

use crate::yaml_adapter::YamlAdapter;

impl AdapterFactory {
    pub fn from_extension(ext: &str) -> AdapterResult<Box<dyn SchemaAdapter>> {
        match ext.to_lowercase().as_str() {
            "csv" => Ok(Box::new(CsvAdapter)),
            "json" => Ok(Box::new(JsonAdapter)),
            "xml" => Ok(Box::new(XmlAdapter)),
            "sql" => Ok(Box::new(SqlAdapter)),
            "yaml" | "yml" => Ok(Box::new(YamlAdapter)), // ← Add this
            _ => Err(AdapterError::UnsupportedFormat(ext.to_string())),
        }
    }
}
```

#### Step 4: Export Module

```rust
// crates/audd_adapters_file/src/lib.rs

pub mod yaml_adapter;
```

#### Step 5: Add Dependency (if needed)

```toml
# crates/audd_adapters_file/Cargo.toml

[dependencies]
serde_yaml = "0.9"  # Add YAML dependency
```

#### Step 6: Create Tests

```bash
# Create test fixture file
mkdir -p fixtures/adapters
cat > fixtures/adapters/test.yaml << EOF
users:
  - id: 1
    name: "Alice"
    email: "alice@example.com"
  - id: 2
    name: "Bob"
    email: "bob@example.com"
EOF
```

```rust
// crates/audd_adapters_file/tests/yaml_test.rs

#[test]
fn test_yaml_adapter_integration() {
    use audd_adapters_file::yaml_adapter::YamlAdapter;
    use audd_adapters_file::adapter::SchemaAdapter;
    use std::path::Path;
    
    let adapter = YamlAdapter;
    let path = Path::new("../../fixtures/adapters/test.yaml");
    
    let result = adapter.load(path);
    assert!(result.is_ok());
    
    let schema = result.unwrap();
    assert_eq!(schema.source_type, "yaml");
    assert!(!schema.entities.is_empty());
}
```

#### Step 7: Update Documentation

```markdown
# docs/adapters_files.md

## Supported Formats

| Format | Extension | Auto-detection | Type Inference |
|---------|-----------|----------------|----------------|
| CSV     | `.csv`    | ✓             | Basic          |
| JSON    | `.json`   | ✓             | ✓              |
| XML     | `.xml`    | ✓             | Basic          |
| SQL DDL | `.sql`    | ✓             | ✓              |
| YAML    | `.yaml`, `.yml` | ✓        | ✓              |  ← Add

### YAML Adapter

The YAML adapter supports structured YAML files...
```

### Adding a Database Adapter

The process is similar but in the `audd_adapters_db` crate:

```bash
cd crates/audd_adapters_db/src/
touch oracle_adapter.rs  # Example
```

Implement the `DatabaseAdapter` trait:

```rust
pub trait DatabaseAdapter {
    fn connect(&self, connection_string: &str) -> AdapterResult<()>;
    fn load_schema(&self) -> AdapterResult<SourceSchema>;
}
```

---

## Documentation Standards

### Code Documentation

All public items must have documentation:

```rust
/// Loads a schema from a CSV file.
///
/// # Arguments
///
/// * `path` - Path to the CSV file
///
/// # Returns
///
/// A `SourceSchema` representing the CSV structure
///
/// # Errors
///
/// Returns an error if:
/// - The file does not exist
/// - The file cannot be parsed
/// - The CSV has no headers
///
/// # Examples
///
/// ```
/// use audd_adapters_file::csv_adapter::CsvAdapter;
/// use std::path::Path;
///
/// let adapter = CsvAdapter;
/// let schema = adapter.load(Path::new("data.csv"))?;
/// ```
pub fn load(&self, path: &Path) -> AdapterResult<SourceSchema> {
    // implementation
}
```

### Markdown Documentation

- Keep Spanish as base language
- Provide English translation in `docs/en/`
- Include language header in each file:

```markdown
**🌐 Language / Idioma:**  
📘 [Español](../FILENAME.md) | 📗 **English**
```

### Documentation Structure

```
docs/
├── README.md                    # General index (Spanish)
├── Getting-Started.md           # Getting started guide (Spanish)
├── FAQ.md                       # FAQ (Spanish)
├── Usage-Examples.md            # Usage examples (Spanish)
├── Contributing.md              # Contributing guide (Spanish)
├── Architecture.md              # Architecture (Spanish)
├── CONFIG.md                    # Configuration (Spanish)
└── en/                          # English translations
    ├── README.md
    ├── Getting-Started.md
    ├── FAQ.md
    ├── Usage-Examples.md
    ├── Contributing.md
    ├── Architecture.md
    └── CONFIG.md
```

### Update CHANGELOG (when implemented)

```markdown
# Changelog

## [Unreleased]

### Added
- Support for YAML files (#123)
- New `audd validate` command (#124)

### Changed
- Improved comparison performance by 40% (#125)

### Fixed
- Fixed bug in NULL type detection (#126)
```

---

## Code Review Process

### Before Requesting Review

**Checklist:**

- [ ] Code compiles without warnings: `cargo build --all-targets --all-features`
- [ ] All tests pass: `cargo test`
- [ ] Code formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy --all-targets --all-features`
- [ ] Documentation updated (if applicable)
- [ ] Tests added for new functionality
- [ ] Commits follow conventions
- [ ] PR has clear description

### During Review

1. **Respond to comments** constructively
2. **Make additional commits** for requested changes (don't force push)
3. **Mark conversations as resolved** when you've applied changes
4. **Ask for clarification** if a comment isn't clear

### After Review

```bash
# Once approved, the PR will be merged by a maintainer
# Update your fork after merge
git checkout main
git pull upstream main
git push origin main
```

---

## Commit Guidelines

We use [Conventional Commits](https://www.conventionalcommits.org/) for clear and consistent commit messages.

### Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting changes (no code changes)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or modifying tests
- `chore`: Build, CI, or tooling changes
- `revert`: Revert a previous commit

### Common Scopes

- `cli`: CLI changes
- `ir`: Intermediate Representation changes
- `adapters`: Adapter changes (file or db)
- `compare`: Comparison engine changes
- `resolution`: Resolution engine changes
- `docs`: Documentation
- `ci`: CI/CD configuration

### Examples

```bash
# New feature
git commit -m "feat(adapters): Add support for YAML files"

# Bug fix
git commit -m "fix(compare): Fix NULL type detection in comparison"

# Documentation
git commit -m "docs: Update contributing guide with adapters section"

# Refactoring
git commit -m "refactor(ir): Simplify SourceSchema builder logic"

# Tests
git commit -m "test(adapters): Add integration tests for YamlAdapter"

# With body and footer
git commit -m "feat(cli): Add validate command

Adds new command to validate configuration files.

Closes #123"
```

### Atomic Commits

- One commit = one logical change
- If doing multiple things, separate into multiple commits

```bash
# Bad ✗
git commit -m "feat: add YAML, fix CSV bug, update docs"

# Good ✓
git commit -m "feat(adapters): Add YAML support"
git commit -m "fix(adapters): Fix CSV parsing with quotes"
git commit -m "docs(adapters): Update list of supported formats"
```

---

## Pull Requests

### Creating a Pull Request

1. **Push your branch** to your fork
   ```bash
   git push origin feature/my-feature
   ```

2. **Go to GitHub** and create Pull Request

3. **Fill out the PR template**:

```markdown
## Description

Brief description of changes.

## Type of change

- [ ] Bug fix (change that fixes an issue)
- [ ] New feature (change that adds functionality)
- [ ] Breaking change (change that breaks compatibility)
- [ ] Documentation

## How has this been tested?

Describe the tests performed.

## Checklist

- [ ] My code follows the project's style
- [ ] I have performed a self-review of my code
- [ ] I have commented my code in difficult to understand areas
- [ ] I have updated the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix works or my feature works
- [ ] New and existing unit tests pass locally
- [ ] I have run `cargo fmt` and `cargo clippy`
```

### PR Size

- **Ideal**: < 400 lines of change
- **Maximum recommended**: < 1000 lines
- If larger, consider splitting into multiple PRs

### PR Title

Follow conventional commits format:

```
feat(adapters): Add support for YAML files
fix(compare): Fix conflict detection in NULL types
docs: Update contributing guide
```

---

## Code Style

### Rust Style Guide

We follow the official [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

### Rustfmt Configuration

The project uses this configuration in `rustfmt.toml`:

```toml
edition = "2021"
max_width = 100
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
```

### Apply Formatting

```bash
# Format all files
cargo fmt

# Check without modifying
cargo fmt --all -- --check
```

### Clippy Lints

```bash
# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# This fails if there are warnings, perfect for CI
```

### Naming Conventions

```rust
// Types: PascalCase
struct SourceSchema { }
enum CanonicalType { }

// Functions and variables: snake_case
fn load_schema() { }
let field_name = "id";

// Constants: SCREAMING_SNAKE_CASE
const MAX_FIELDS: usize = 1000;

// Lifetimes: single lowercase letter
fn compare<'a>(schema_a: &'a Schema) { }
```

### Error Handling

```rust
// Prefer Result<T, E> over panic!
pub fn load(&self, path: &Path) -> AdapterResult<SourceSchema> {
    // Don't use unwrap() in production code
    let content = fs::read_to_string(path)?; // ✓ Use ?
    
    // In tests, unwrap() is fine
    #[cfg(test)]
    let schema = adapter.load(path).unwrap(); // ✓ Ok in tests
}
```

### Comments

```rust
// Comments in Spanish or English, both are acceptable
// Prefer documentation over comments

// Bad ✗ - obvious comment
// Increment counter
counter += 1;

// Good ✓ - explains the "why"
// We use Jaro-Winkler because it handles names with different conventions better
let similarity = jaro_winkler(&name_a, &name_b);
```

---

## CI/CD

The project uses GitHub Actions for CI/CD. Workflows run automatically on:

- Push to `main` or `develop`
- Pull Requests to `main` or `develop`

### Workflows

```yaml
# .github/workflows/ci.yml

jobs:
  fmt:      # Check formatting
  clippy:   # Run linter
  test:     # Run tests on Linux, Windows, macOS
  build:    # Verify builds on all platforms
```

### Passing CI Locally

```bash
# Run all CI checks locally
./scripts/check-ci.sh  # If it exists

# Or manually:
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --release --all-features
```

---

## Questions and Support

### Where to Ask for Help?

- **GitHub Discussions**: For general questions about contributing
- **GitHub Issues**: To report bugs or request features
- **Pull Request comments**: For questions about specific code

### Useful Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Cargo documentation
- [AUDD Architecture](Architecture.md) - Project architecture

---

## Recognition

All contributors are valued and recognized!

- Contributors are automatically listed on GitHub
- Significant contributions are mentioned in release notes

---

**Thank you for contributing to AUDD!** 🎉

Your time and effort help make AUDD better for everyone.

---

**Last updated:** 2026-01-26
