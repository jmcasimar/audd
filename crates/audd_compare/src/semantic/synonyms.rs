//! Synonym dictionary for controlled equivalences

use crate::config::SynonymConfig;

/// Check if two tokens are synonyms according to the configured groups
pub fn are_synonyms(token_a: &str, token_b: &str, config: &SynonymConfig) -> Option<usize> {
    if !config.enabled || token_a == token_b {
        return None;
    }

    for (group_idx, group) in config.groups.iter().enumerate() {
        let has_a = group.iter().any(|s| s == token_a);
        let has_b = group.iter().any(|s| s == token_b);

        if has_a && has_b {
            return Some(group_idx);
        }
    }

    None
}

/// Check if two token lists have synonym matches
pub fn check_synonym_match(
    tokens_a: &[String],
    tokens_b: &[String],
    config: &SynonymConfig,
) -> Option<Vec<(String, String, usize)>> {
    if !config.enabled {
        return None;
    }

    let mut matches = Vec::new();

    for token_a in tokens_a {
        for token_b in tokens_b {
            if let Some(group_idx) = are_synonyms(token_a, token_b, config) {
                matches.push((token_a.clone(), token_b.clone(), group_idx));
            }
        }
    }

    if matches.is_empty() {
        None
    } else {
        Some(matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_config() -> SynonymConfig {
        SynonymConfig {
            enabled: true,
            groups: vec![
                vec!["id".to_string(), "identifier".to_string(), "key".to_string()],
                vec![
                    "user".to_string(),
                    "customer".to_string(),
                    "client".to_string(),
                ],
                vec!["num".to_string(), "number".to_string(), "no".to_string()],
            ],
        }
    }

    #[test]
    fn test_are_synonyms_match() {
        let config = create_config();
        assert!(are_synonyms("id", "identifier", &config).is_some());
        assert!(are_synonyms("user", "customer", &config).is_some());
        assert!(are_synonyms("num", "number", &config).is_some());
    }

    #[test]
    fn test_are_synonyms_no_match() {
        let config = create_config();
        assert!(are_synonyms("id", "user", &config).is_none());
        assert!(are_synonyms("foo", "bar", &config).is_none());
    }

    #[test]
    fn test_are_synonyms_same_word() {
        let config = create_config();
        assert!(are_synonyms("user", "user", &config).is_none());
    }

    #[test]
    fn test_check_synonym_match() {
        let config = create_config();
        let tokens_a = vec!["user".to_string(), "id".to_string()];
        let tokens_b = vec!["customer".to_string(), "key".to_string()];

        let result = check_synonym_match(&tokens_a, &tokens_b, &config);
        assert!(result.is_some());

        let matches = result.unwrap();
        assert_eq!(matches.len(), 2);
        assert!(matches.iter().any(|(a, b, _)| a == "user" && b == "customer"));
        assert!(matches.iter().any(|(a, b, _)| a == "id" && b == "key"));
    }

    #[test]
    fn test_check_synonym_match_no_match() {
        let config = create_config();
        let tokens_a = vec!["foo".to_string()];
        let tokens_b = vec!["bar".to_string()];

        assert!(check_synonym_match(&tokens_a, &tokens_b, &config).is_none());
    }

    #[test]
    fn test_disabled_synonyms() {
        let config = SynonymConfig {
            enabled: false,
            groups: vec![vec!["user".to_string(), "customer".to_string()]],
        };

        assert!(are_synonyms("user", "customer", &config).is_none());
    }

    #[test]
    fn test_bidirectional_synonyms() {
        let config = create_config();
        assert!(are_synonyms("id", "identifier", &config).is_some());
        assert!(are_synonyms("identifier", "id", &config).is_some());
    }
}
