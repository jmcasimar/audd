//! Pluralization and simple stemming for entity names

use crate::config::{MatchLocale, PluralizationConfig, StemmingConfig};

/// Apply pluralization normalization to tokens
pub fn normalize_plural(
    tokens: &[String],
    config: &PluralizationConfig,
    locale: MatchLocale,
) -> Vec<String> {
    if !config.enabled {
        return tokens.to_vec();
    }

    tokens
        .iter()
        .map(|token| {
            if config.exceptions.contains(token) {
                token.clone()
            } else {
                to_singular(token, locale)
            }
        })
        .collect()
}

/// Apply simple stemming to tokens
pub fn apply_stemming(
    tokens: &[String],
    config: &StemmingConfig,
    locale: MatchLocale,
) -> Vec<String> {
    if !config.enabled {
        return tokens.to_vec();
    }

    tokens
        .iter()
        .map(|token| simple_stem(token, locale))
        .collect()
}

/// Convert a word to singular form (simple heuristic)
fn to_singular(word: &str, locale: MatchLocale) -> String {
    match locale {
        MatchLocale::En | MatchLocale::Mixed => to_singular_en(word),
        MatchLocale::Es => to_singular_es(word),
    }
}

/// Convert English word to singular
fn to_singular_en(word: &str) -> String {
    // Handle common irregular plurals
    let irregular = [
        ("children", "child"),
        ("people", "person"),
        ("men", "man"),
        ("women", "woman"),
        ("teeth", "tooth"),
        ("feet", "foot"),
        ("mice", "mouse"),
        ("geese", "goose"),
    ];

    for (plural, singular) in &irregular {
        if word == *plural {
            return singular.to_string();
        }
    }

    // Handle regular patterns
    if word.len() < 3 {
        return word.to_string();
    }

    // -ies -> -y (e.g., categories -> category)
    if word.ends_with("ies") && word.len() > 3 {
        let base = &word[..word.len() - 3];
        // Check if the letter before "ies" is not a vowel
        if let Some(prev_char) = base.chars().last() {
            if !matches!(prev_char, 'a' | 'e' | 'i' | 'o' | 'u') {
                return format!("{}y", base);
            }
        }
    }

    // -ves -> -fe or -f (e.g., knives -> knife)
    if word.ends_with("ves") && word.len() > 3 {
        let base = &word[..word.len() - 3];
        return format!("{}fe", base);
    }

    // -ses -> -s (e.g., addresses -> address, but not class -> clas)
    if word.ends_with("ses") && word.len() > 4 {
        let base = &word[..word.len() - 2];
        // Only apply if it ends with a double consonant like "ss"
        if base.ends_with("ss") {
            return base.to_string();
        }
    }

    // -xes -> -x (e.g., boxes -> box)
    if word.ends_with("xes") && word.len() > 3 {
        return word[..word.len() - 2].to_string();
    }

    // -ches, -shes -> -ch, -sh
    if (word.ends_with("ches") || word.ends_with("shes")) && word.len() > 4 {
        return word[..word.len() - 2].to_string();
    }

    // -es after o (e.g., potatoes -> potato)
    if word.ends_with("oes") && word.len() > 3 {
        return word[..word.len() - 2].to_string();
    }

    // -s (simple plural, most common)
    if word.ends_with('s') && word.len() > 1 {
        // Avoid removing 's' from words like "class", "bass", "less", "status", "analysis"
        if !word.ends_with("ss")
            && !word.ends_with("us")
            && !word.ends_with("is")
        {
            let without_s = &word[..word.len() - 1];
            return without_s.to_string();
        }
    }

    word.to_string()
}

/// Convert Spanish word to singular (basic rules)
fn to_singular_es(word: &str) -> String {
    if word.len() < 3 {
        return word.to_string();
    }

    // Basic Spanish pluralization: most words just add -s or -es
    // Note: This is a simplified implementation. Complex cases like
    // accent changes (nación/naciones) are not handled.
    
    // -s -> remove (e.g., usuarios -> usuario, clientes -> cliente)
    if word.ends_with('s') && word.len() > 1 {
        let without_s = &word[..word.len() - 1];
        // Don't remove 's' if it would result in an invalid word
        if !without_s.is_empty() {
            return without_s.to_string();
        }
    }

    word.to_string()
}

/// Apply simple stemming (very basic, entity-focused)
fn simple_stem(word: &str, locale: MatchLocale) -> String {
    match locale {
        MatchLocale::En | MatchLocale::Mixed => simple_stem_en(word),
        MatchLocale::Es => simple_stem_es(word),
    }
}

/// Simple English stemming
fn simple_stem_en(word: &str) -> String {
    if word.len() < 4 {
        return word.to_string();
    }

    // Remove common suffixes for entity names
    let suffixes = ["ing", "ed", "er", "ly"];

    for suffix in &suffixes {
        if word.ends_with(suffix) && word.len() > suffix.len() + 2 {
            return word[..word.len() - suffix.len()].to_string();
        }
    }

    word.to_string()
}

/// Simple Spanish stemming
fn simple_stem_es(word: &str) -> String {
    if word.len() < 4 {
        return word.to_string();
    }

    // Remove common Spanish suffixes
    let suffixes = ["ción", "dad", "mente"];

    for suffix in &suffixes {
        if word.ends_with(suffix) && word.len() > suffix.len() + 2 {
            return word[..word.len() - suffix.len()].to_string();
        }
    }

    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> PluralizationConfig {
        PluralizationConfig {
            enabled: true,
            exceptions: vec![],
        }
    }

    fn stemming_config() -> StemmingConfig {
        StemmingConfig { enabled: true }
    }

    #[test]
    fn test_to_singular_en_simple() {
        assert_eq!(to_singular_en("users"), "user");
        assert_eq!(to_singular_en("customers"), "customer");
        assert_eq!(to_singular_en("products"), "product");
    }

    #[test]
    fn test_to_singular_en_ies() {
        assert_eq!(to_singular_en("categories"), "category");
        assert_eq!(to_singular_en("cities"), "city");
        assert_eq!(to_singular_en("libraries"), "library");
    }

    #[test]
    fn test_to_singular_en_es() {
        assert_eq!(to_singular_en("addresses"), "address");
        assert_eq!(to_singular_en("classes"), "class");
    }

    #[test]
    fn test_to_singular_en_irregular() {
        assert_eq!(to_singular_en("children"), "child");
        assert_eq!(to_singular_en("people"), "person");
        assert_eq!(to_singular_en("men"), "man");
    }

    #[test]
    fn test_to_singular_already_singular() {
        assert_eq!(to_singular_en("user"), "user");
        assert_eq!(to_singular_en("class"), "class");  // should not remove 's' from 'class'
        assert_eq!(to_singular_en("status"), "status");
    }

    #[test]
    fn test_to_singular_es() {
        assert_eq!(to_singular_es("usuarios"), "usuario");
        assert_eq!(to_singular_es("clientes"), "cliente");
        assert_eq!(to_singular_es("productos"), "producto");
    }

    #[test]
    fn test_normalize_plural() {
        let config = default_config();
        let tokens = vec!["users".to_string(), "profiles".to_string()];
        let result = normalize_plural(&tokens, &config, MatchLocale::En);
        assert_eq!(result, vec!["user", "profile"]);
    }

    #[test]
    fn test_normalize_plural_with_exception() {
        let config = PluralizationConfig {
            enabled: true,
            exceptions: vec!["data".to_string()],
        };
        let tokens = vec!["users".to_string(), "data".to_string()];
        let result = normalize_plural(&tokens, &config, MatchLocale::En);
        assert_eq!(result, vec!["user", "data"]);
    }

    #[test]
    fn test_simple_stem_en() {
        assert_eq!(simple_stem_en("running"), "runn");
        assert_eq!(simple_stem_en("created"), "creat");
        assert_eq!(simple_stem_en("user"), "user");
    }

    #[test]
    fn test_apply_stemming() {
        let config = stemming_config();
        let tokens = vec!["running".to_string(), "user".to_string()];
        let result = apply_stemming(&tokens, &config, MatchLocale::En);
        assert_eq!(result, vec!["runn", "user"]);
    }

    #[test]
    fn test_disabled_pluralization() {
        let config = PluralizationConfig {
            enabled: false,
            exceptions: vec![],
        };
        let tokens = vec!["users".to_string()];
        let result = normalize_plural(&tokens, &config, MatchLocale::En);
        assert_eq!(result, vec!["users"]);
    }
}
