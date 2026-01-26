# Configuration File Support

AUDD CLI supports configuration files to customize behavior without needing to pass flags every time.

## Configuration File Locations

AUDD will automatically search for configuration files in the following locations (in order):

1. `./audd.toml` - Current directory
2. `~/.audd.toml` - Home directory
3. `~/.config/audd/config.toml` - XDG config directory

Alternatively, you can specify a custom config file path:

```bash
audd --config /path/to/custom/config.toml compare ...
```

## Generating a Sample Configuration

To generate a sample configuration file:

```bash
# Generate in current directory
audd generate-config

# Generate to a custom location
audd generate-config --out ~/.audd.toml
```

This creates a TOML file with all available options and their default values.

## Configuration Options

### [compare] Section

```toml
[compare]
# Similarity threshold for matching entities/fields (0.0 to 1.0)
similarity_threshold = 0.8

# Default output directory for comparison results
default_output_dir = "output"
```

### [resolution] Section

```toml
[resolution]
# Confidence threshold for auto-accepting suggestions (0.0 to 1.0)
# Only suggestions with confidence >= this value will be auto-accepted
confidence_threshold = 0.9

# Prefix for decision IDs (useful for tracking different runs)
decision_id_prefix = "auto_dec"

# Whether to allow risky suggestions (e.g., lossy type casts)
allow_risky_suggestions = false
```

### [output] Section

```toml
[output]
# Control which output files are generated
generate_unified_schema = true
generate_diff = true
generate_decision_log = true
generate_report = true
```

## Precedence Rules

When the same setting is specified in multiple places, the following precedence applies:

1. **CLI flags** (highest priority)
2. **Configuration file** (specified with `--config` or auto-loaded)
3. **Default values** (lowest priority)

### Example

If you have a config file with:
```toml
[resolution]
confidence_threshold = 0.85
```

And run:
```bash
audd compare --confidence-threshold 0.95 ...
```

The CLI flag value `0.95` will be used, overriding the config file value.

## Complete Example

### 1. Generate configuration file

```bash
audd generate-config --out ~/.audd.toml
```

### 2. Edit the configuration

```toml
[compare]
default_output_dir = "/var/audd/output"

[resolution]
confidence_threshold = 0.85
decision_id_prefix = "prod_dec"
allow_risky_suggestions = false

[output]
generate_unified_schema = true
generate_diff = true
generate_decision_log = true
generate_report = true
```

### 3. Use the configuration

```bash
# Config is loaded automatically from ~/.audd.toml
audd compare --source-a data1.csv --source-b data2.json

# Output goes to /var/audd/output (from config)
# Decisions use "prod_dec_*" IDs (from config)
# Confidence threshold is 0.85 (from config)
```

### 4. Override specific settings

```bash
# Use different confidence threshold for this run
audd compare \
  --source-a data1.csv \
  --source-b data2.json \
  --confidence-threshold 0.95

# Override output directory for this run
audd compare \
  --source-a data1.csv \
  --source-b data2.json \
  --out /tmp/comparison
```

## Use Cases

### Development vs Production

**Development config** (`audd-dev.toml`):
```toml
[resolution]
confidence_threshold = 0.75  # More aggressive
allow_risky_suggestions = true

[output]
generate_report = false  # Skip reports in dev
```

**Production config** (`audd-prod.toml`):
```toml
[resolution]
confidence_threshold = 0.9  # Conservative
allow_risky_suggestions = false

[output]
generate_report = true  # Always generate reports
```

Usage:
```bash
# Development
audd --config audd-dev.toml compare ...

# Production
audd --config audd-prod.toml compare ...
```

### Team Standardization

Place a shared `audd.toml` in your project repository:

```toml
# Project standard settings
[compare]
default_output_dir = "schema_comparison"

[resolution]
confidence_threshold = 0.88
decision_id_prefix = "team_dec"
```

Everyone on the team gets consistent behavior without memorizing flags.

## Troubleshooting

### Config file not being loaded

Check that:
1. The file exists in one of the search locations
2. The file has valid TOML syntax
3. File permissions allow reading

To verify which config is being used, you can check the behavior:
```bash
# Without config: uses default output directory "output"
# With config: uses configured output directory
audd compare --source-a a.csv --source-b b.json
```

### Invalid configuration

If the config file has invalid TOML syntax, you'll see an error:
```
❌ Error loading config file: Failed to parse configuration file: ...
```

Fix the syntax error and try again.

### Testing configuration

Use the `generate-config` command to see the correct format:
```bash
audd generate-config --out /tmp/reference.toml
cat /tmp/reference.toml
```

## Schema Reference

For the complete TOML schema, see the generated sample configuration file or the source code in `crates/audd-cli/src/config.rs`.
