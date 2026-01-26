# Report Tests

This directory contains integration tests for the AUDD report generation functionality.

## Golden File Tests

The `report_tests.rs` file includes golden file tests that ensure the report format remains stable across changes.

### What are Golden Files?

Golden files are expected output files that are used to verify that the actual output matches the expected output. They serve as regression tests to ensure that changes to the code don't inadvertently change the report format.

### Running Tests

```bash
# Run all report tests
cargo test -p audd-cli report_

# Run a specific test
cargo test -p audd-cli test_report_generation_users
```

### Updating Golden Files

If you intentionally change the report format, you'll need to update the golden files:

```bash
UPDATE_GOLDEN=1 cargo test -p audd-cli test_report_generation_users
```

**Important:** Before updating golden files:
1. Verify that the changes to the report format are intentional
2. Review the diff to ensure it matches your expectations
3. Update the report structure documentation in `docs/reporting.md` if needed

### Golden Files Location

Golden files are stored in `tests/golden/`:
- `users_csv_vs_json.md` - Report for comparing users.csv and users.json fixtures

### Normalization

The test normalizes certain variable content before comparison:
- Timestamps are replaced with `[TIMESTAMP]` placeholder
- This ensures tests are deterministic and don't fail due to time differences

## Test Coverage

Current tests cover:
1. **Full Report Generation** - End-to-end test with real fixtures
2. **Report Structure** - Ensures all sections are present
3. **Risk Level Indicators** - Tests risk assessment logic
4. **Metrics Calculation** - Tests edge cases for metrics calculation

## Adding New Tests

When adding new report features:
1. Add unit tests in `src/report.rs` for the core logic
2. Add integration tests in `tests/report_tests.rs` for end-to-end scenarios
3. Create golden files for new report variants if applicable
