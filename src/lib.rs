use anyhow::{Result, anyhow};
use rand::seq::SliceRandom;
use std::collections::HashSet;

pub const MAX_IDS_COUNT: usize = 999_999;
pub const MIN_ID_LENGTH: usize = 8;
pub const MAX_ID_LENGTH: usize = 255;

// TODO: Currently our static adjs and nouns are read via main.rs and not the library code.
// Consider moving the adjs and nouns to this library.

pub struct Config {
    pub count: usize,
    pub max_length: usize,
    pub prefix: String,
    pub suffix: String,
    pub alliterate: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prefix: String::new(),
            suffix: String::new(),
            count: 1,
            max_length: 16,
            alliterate: false,
        }
    }
}

/// Count how many unique IDs are possible for the given adjectives, nouns, and config,
/// without generating them. Respects `max_length`, `prefix`, `suffix`, and `alliterate`.
pub fn count_available(adjs: &[&str], nouns: &[&str], config: &Config) -> usize {
    let prefix_len = config.prefix.chars().count();
    let suffix_len = config.suffix.chars().count();
    let mut count = 0;
    for adj in adjs {
        let adj = capitalize_first_char(adj);
        let adj_first = adj.chars().next();
        let adj_len = adj.chars().count();
        for noun in nouns {
            let noun = capitalize_first_char(noun);
            if config.alliterate && adj_first != noun.chars().next() {
                continue;
            }
            if prefix_len + adj_len + noun.chars().count() + suffix_len <= config.max_length {
                count += 1;
            }
        }
    }
    count
}

/// Generate a set of unique IDs based on the given adjectives, nouns, and config.
/// # Arguments
/// * `adjs` - A slice of adjectives to use in the ID generation
/// * `nouns` - A slice of nouns to use in the ID generation
/// * `config` - A `Config` struct that contains the constraints for the ID generation
///
/// # Errors
/// * Returns an when passed parameters do not meet the constraints
/// * Returns an error when no unique IDs are available for the given constraints
///
pub fn generate_ids(adjs: &[&str], nouns: &[&str], config: &Config) -> Result<HashSet<String>> {
    // TODO: make this more dynamic
    if config.prefix.chars().count() > 5 {
        return Err(anyhow!("Prefix must be 5 characters or less."));
    }
    if config.suffix.chars().count() > 5 {
        return Err(anyhow!("Suffix must be 5 characters or less."));
    }
    if config.count > MAX_IDS_COUNT {
        return Err(anyhow!("Count must be {MAX_IDS_COUNT} or less."));
    }
    if config.max_length > MAX_ID_LENGTH {
        return Err(anyhow!("Max length must be {MAX_ID_LENGTH} or less."));
    }

    // Make sure we have some adjectives and nouns to work
    if adjs.is_empty() {
        return Err(anyhow!("No adjectives provided."));
    }
    if nouns.is_empty() {
        return Err(anyhow!("No nouns provided."));
    }

    let distinct_ids_avail = adjs.len() * nouns.len();
    let mut uniq_ids = Vec::with_capacity(distinct_ids_avail / 4);

    // Build a vec of all available ids for given constraints
    for adj in adjs {
        let adj = capitalize_first_char(adj);
        for noun in nouns {
            let noun = capitalize_first_char(noun);
            if config.alliterate && adj.chars().next() != noun.chars().next() {
                continue;
            }
            let id = format!("{}{}{}{}", config.prefix, adj, noun, config.suffix);
            if id.chars().count() <= config.max_length {
                uniq_ids.push(id);
            }
        }
    }

    // Return an error if there are no unique IDs available
    if uniq_ids.is_empty() {
        return Err(anyhow!(
            "No unique IDs available for the given constraints."
        ));
    }
    // ... or not enough unique IDs to satisfy the request
    if uniq_ids.len() < config.count {
        return Err(anyhow!(format!(
            "Only {} IDs available, cannot produce {}.",
            uniq_ids.len(),
            config.count
        )));
    }

    // Randomly choose config.count number of ids from the precomputed list
    let mut rng = rand::rng();
    uniq_ids.shuffle(&mut rng);
    let random_ids: HashSet<String> = uniq_ids.into_iter().take(config.count).collect();

    Ok(random_ids)
}

fn capitalize_first_char(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// ///////////////////////////////////////////////////////////////////////////////////////////////
// Tests
// ///////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first_char() {
        assert_eq!(capitalize_first_char("hello"), "Hello");
        assert_eq!(capitalize_first_char("i"), "I");
        assert_eq!(capitalize_first_char(""), "");
    }

    #[test]
    fn test_default_config_values() {
        let adjs = vec!["astronomical"];
        let nouns = vec!["goat"];
        let config = Config::default();
        let result = generate_ids(&adjs, &nouns, &config).unwrap();
        let ids = result.into_iter().collect::<Vec<String>>();
        assert_eq!(ids.len(), 1);
        let id = ids[0].clone();
        assert_eq!(id, "AstronomicalGoat");
    }

    #[test]
    fn test_capitalize_first_char_empty_string() {
        assert_eq!(capitalize_first_char(""), "");
    }

    #[test]
    fn test_capitalize_first_char_single_char() {
        assert_eq!(capitalize_first_char("a"), "A");
    }

    #[test]
    fn test_capitalize_first_char_multiple_chars() {
        assert_eq!(capitalize_first_char("hello"), "Hello");
    }

    #[test]
    fn test_capitalize_first_char_unicode() {
        assert_eq!(capitalize_first_char("éclair"), "Éclair");
    }

    #[test]
    fn test_capitalize_first_char_numbers() {
        assert_eq!(capitalize_first_char("123"), "123");
    }

    #[test]
    fn test_capitalize_first_char_emoji() {
        assert_eq!(
            capitalize_first_char("💻 Sing Me a tune"),
            "💻 Sing Me a tune"
        );
    }

    #[test]
    fn test_capitalize_first_char_special_chars() {
        assert_eq!(capitalize_first_char("!@#$"), "!@#$");
    }

    #[test]
    fn test_empty_adjectives_passed() {
        assert_eq!(
            generate_ids(&[], &["mule"], &Config::default())
                .unwrap_err()
                .to_string(),
            "No adjectives provided."
        );
    }

    #[test]
    fn test_empty_nouns_passed() {
        assert_eq!(
            generate_ids(&["tender"], &[], &Config::default())
                .unwrap_err()
                .to_string(),
            "No nouns provided."
        );
    }
}
