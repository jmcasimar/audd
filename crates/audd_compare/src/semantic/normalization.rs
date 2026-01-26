//! Advanced normalization for semantic matching

use unicode_normalization::UnicodeNormalization as _;
use unidecode::unidecode;

use crate::config::{NormalizationConfig, UnicodeNormalization};

/// Result of normalization
#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedText {
    /// Normalized text
    pub text: String,

    /// Tokens derived from normalized text
    pub tokens: Vec<String>,
}

/// Normalize text for semantic comparison
pub fn normalize(text: &str, config: &NormalizationConfig) -> NormalizedText {
    let mut result = text.trim().to_string();

    // Apply Unicode normalization
    result = match config.unicode_normalization {
        UnicodeNormalization::NFC => result.chars().nfc().collect(),
        UnicodeNormalization::NFKC => result.chars().nfkc().collect(),
    };

    // Remove diacritics if configured
    if config.remove_diacritics {
        result = unidecode(&result);
    }

    // Split camelCase/PascalCase if configured
    if config.split_camel_case {
        result = split_camel_case(&result);
    }

    // Normalize separators: replace - _ . / with space
    result = result
        .chars()
        .map(|ch| match ch {
            '-' | '_' | '.' | '/' | '\\' => ' ',
            _ => ch,
        })
        .collect();

    // Convert to lowercase
    result = result.to_lowercase();

    // Collapse multiple spaces
    result = collapse_spaces(&result);

    // Tokenize
    let mut tokens: Vec<String> = result
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    // Remove stopwords
    if !config.stopwords.is_empty() {
        tokens.retain(|token| !config.stopwords.contains(token));
    }

    // Rebuild text from tokens
    let text = tokens.join(" ");

    NormalizedText { text, tokens }
}

/// Split camelCase and PascalCase into words
fn split_camel_case(text: &str) -> String {
    let mut result = String::with_capacity(text.len() + 10);
    let mut prev_was_lower = false;

    for ch in text.chars() {
        if ch.is_uppercase() && prev_was_lower {
            result.push(' ');
        }
        result.push(ch);
        prev_was_lower = ch.is_lowercase();
    }

    result
}

/// Collapse multiple spaces into single space
fn collapse_spaces(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut prev_was_space = false;

    for ch in text.chars() {
        if ch.is_whitespace() {
            if !prev_was_space {
                result.push(' ');
                prev_was_space = true;
            }
        } else {
            result.push(ch);
            prev_was_space = false;
        }
    }

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> NormalizationConfig {
        NormalizationConfig {
            remove_diacritics: true,
            unicode_normalization: UnicodeNormalization::NFC,
            split_camel_case: true,
            stopwords: vec!["tbl".to_string(), "table".to_string()],
        }
    }

    #[test]
    fn test_normalize_basic() {
        let config = default_config();
        let result = normalize("UserProfile", &config);
        assert_eq!(result.text, "user profile");
        assert_eq!(result.tokens, vec!["user", "profile"]);
    }

    #[test]
    fn test_normalize_with_underscores() {
        let config = default_config();
        let result = normalize("user_profile", &config);
        assert_eq!(result.text, "user profile");
        assert_eq!(result.tokens, vec!["user", "profile"]);
    }

    #[test]
    fn test_normalize_mixed_separators() {
        let config = default_config();
        let result = normalize("user-profile.name", &config);
        assert_eq!(result.text, "user profile name");
        assert_eq!(result.tokens, vec!["user", "profile", "name"]);
    }

    #[test]
    fn test_normalize_with_diacritics() {
        let config = default_config();
        let result = normalize("canción", &config);
        assert_eq!(result.text, "cancion");
    }

    #[test]
    fn test_normalize_with_spaces() {
        let config = default_config();
        let result = normalize("  User   Profile  ", &config);
        assert_eq!(result.text, "user profile");
        assert_eq!(result.tokens, vec!["user", "profile"]);
    }

    #[test]
    fn test_normalize_stopwords() {
        let config = default_config();
        let result = normalize("user_tbl", &config);
        assert_eq!(result.text, "user");
        assert_eq!(result.tokens, vec!["user"]);
    }

    #[test]
    fn test_split_camel_case() {
        assert_eq!(split_camel_case("UserProfile"), "User Profile");
        assert_eq!(split_camel_case("firstName"), "first Name");
        assert_eq!(split_camel_case("getUserById"), "get User By Id");
        assert_eq!(split_camel_case("HTML"), "HTML");
    }

    #[test]
    fn test_collapse_spaces() {
        assert_eq!(collapse_spaces("a  b   c"), "a b c");
        assert_eq!(collapse_spaces("  hello  world  "), "hello world");
        assert_eq!(collapse_spaces("nospaces"), "nospaces");
    }

    #[test]
    fn test_normalize_uppercase() {
        let config = default_config();
        let result = normalize("USERS", &config);
        assert_eq!(result.text, "users");
    }

    #[test]
    fn test_normalize_complex() {
        let config = default_config();
        let result = normalize("UserTable_2023", &config);
        assert_eq!(result.text, "user 2023");
        assert_eq!(result.tokens, vec!["user", "2023"]);
    }
}
