//! Configuration file support for AUDD CLI

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::error::{CliError, CliResult};

/// Default confidence threshold for auto-accepting suggestions
pub const DEFAULT_CONFIDENCE_THRESHOLD: f64 = 0.9;

/// Default decision ID prefix
pub const DEFAULT_DECISION_ID_PREFIX: &str = "auto_dec";

/// Configuration for the AUDD CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Comparison configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare: Option<CompareConfig>,
    
    /// Resolution configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<ResolutionConfig>,
    
    /// Output configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<OutputConfig>,
}

/// Configuration for the compare command
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CompareConfig {
    /// Similarity threshold for matching entities/fields (0.0 to 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_threshold: Option<f64>,
    
    /// Default output directory for comparison results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_output_dir: Option<String>,
}

/// Configuration for the resolution engine
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ResolutionConfig {
    /// Confidence threshold for auto-accepting suggestions (0.0 to 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<f64>,
    
    /// Prefix for decision IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_id_prefix: Option<String>,
    
    /// Whether to allow risky suggestions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_risky_suggestions: Option<bool>,
}

/// Configuration for output generation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OutputConfig {
    /// Whether to generate the unified schema file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_unified_schema: Option<bool>,
    
    /// Whether to generate the diff file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_diff: Option<bool>,
    
    /// Whether to generate the decision log file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_decision_log: Option<bool>,
    
    /// Whether to generate the markdown report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_report: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            compare: None,
            resolution: None,
            output: None,
        }
    }
}

impl Default for CompareConfig {
    fn default() -> Self {
        Self {
            similarity_threshold: None,
            default_output_dir: None,
        }
    }
}

impl Default for ResolutionConfig {
    fn default() -> Self {
        Self {
            confidence_threshold: None,
            decision_id_prefix: None,
            allow_risky_suggestions: None,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            generate_unified_schema: None,
            generate_diff: None,
            generate_decision_log: None,
            generate_report: None,
        }
    }
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> CliResult<Self> {
        let contents = fs::read_to_string(&path).map_err(|e| CliError::IoError(e))?;
        let config: Config = toml::from_str(&contents)
            .map_err(|e| CliError::ConfigParseError(e.to_string()))?;
        Ok(config)
    }
    
    /// Try to load configuration from default locations
    /// Searches in order: ./audd.toml, ~/.audd.toml, ~/.config/audd/config.toml
    pub fn load_default() -> Self {
        // Try current directory
        if let Ok(config) = Self::from_file("audd.toml") {
            return config;
        }
        
        // Try home directory
        if let Some(home) = dirs::home_dir() {
            let home_config = home.join(".audd.toml");
            if let Ok(config) = Self::from_file(&home_config) {
                return config;
            }
            
            // Try XDG config directory
            let xdg_config = home.join(".config").join("audd").join("config.toml");
            if let Ok(config) = Self::from_file(&xdg_config) {
                return config;
            }
        }
        
        // Return default config if no file found
        Self::default()
    }
    
    /// Get the confidence threshold with precedence: override > config > default
    pub fn get_confidence_threshold(&self, override_value: Option<f64>) -> f64 {
        override_value
            .or_else(|| self.resolution.as_ref().and_then(|r| r.confidence_threshold))
            .unwrap_or(DEFAULT_CONFIDENCE_THRESHOLD)
    }
    
    /// Get the decision ID prefix with precedence: override > config > default
    pub fn get_decision_id_prefix(&self, override_value: Option<&str>) -> String {
        override_value
            .map(|s| s.to_string())
            .or_else(|| self.resolution.as_ref().and_then(|r| r.decision_id_prefix.clone()))
            .unwrap_or_else(|| DEFAULT_DECISION_ID_PREFIX.to_string())
    }
    
    /// Get the default output directory
    pub fn get_default_output_dir(&self) -> String {
        self.compare
            .as_ref()
            .and_then(|c| c.default_output_dir.clone())
            .unwrap_or_else(|| "output".to_string())
    }
    
    /// Check if unified schema generation is enabled
    pub fn should_generate_unified_schema(&self) -> bool {
        self.output
            .as_ref()
            .and_then(|o| o.generate_unified_schema)
            .unwrap_or(true)
    }
    
    /// Check if diff generation is enabled
    pub fn should_generate_diff(&self) -> bool {
        self.output
            .as_ref()
            .and_then(|o| o.generate_diff)
            .unwrap_or(true)
    }
    
    /// Check if decision log generation is enabled
    pub fn should_generate_decision_log(&self) -> bool {
        self.output
            .as_ref()
            .and_then(|o| o.generate_decision_log)
            .unwrap_or(true)
    }
    
    /// Check if report generation is enabled
    pub fn should_generate_report(&self) -> bool {
        self.output
            .as_ref()
            .and_then(|o| o.generate_report)
            .unwrap_or(true)
    }
    
    /// Generate a sample configuration file
    pub fn sample() -> String {
        let sample = Config {
            compare: Some(CompareConfig {
                similarity_threshold: Some(0.8),
                default_output_dir: Some("output".to_string()),
            }),
            resolution: Some(ResolutionConfig {
                confidence_threshold: Some(0.9),
                decision_id_prefix: Some("auto_dec".to_string()),
                allow_risky_suggestions: Some(false),
            }),
            output: Some(OutputConfig {
                generate_unified_schema: Some(true),
                generate_diff: Some(true),
                generate_decision_log: Some(true),
                generate_report: Some(true),
            }),
        };
        
        toml::to_string_pretty(&sample).unwrap_or_default()
    }
}

// Add dirs crate for home directory detection
mod dirs {
    use std::path::PathBuf;
    
    pub fn home_dir() -> Option<PathBuf> {
        std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.get_confidence_threshold(None), DEFAULT_CONFIDENCE_THRESHOLD);
        assert_eq!(config.get_decision_id_prefix(None), DEFAULT_DECISION_ID_PREFIX);
        assert!(config.should_generate_unified_schema());
        assert!(config.should_generate_diff());
    }
    
    #[test]
    fn test_precedence_confidence_threshold() {
        let config = Config {
            resolution: Some(ResolutionConfig {
                confidence_threshold: Some(0.8),
                decision_id_prefix: None,
                allow_risky_suggestions: None,
            }),
            ..Default::default()
        };
        
        // Config value
        assert_eq!(config.get_confidence_threshold(None), 0.8);
        
        // Override takes precedence
        assert_eq!(config.get_confidence_threshold(Some(0.95)), 0.95);
    }
    
    #[test]
    fn test_sample_config() {
        let sample = Config::sample();
        assert!(sample.contains("confidence_threshold"));
        assert!(sample.contains("similarity_threshold"));
        assert!(sample.contains("output"));
    }
}
