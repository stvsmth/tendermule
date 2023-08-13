use anyhow::{anyhow, Result};
use rand::Rng;
use std::collections::HashSet;

pub const MAX_IDS_COUNT: usize = 1_000_000;
pub const MIN_ID_LENGTH: usize = 8;
pub const MAX_ID_LENGTH: usize = 256;

pub struct Config {
    pub prefix: String,
    pub suffix: String,
    pub count: usize,
    pub max_length: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prefix: String::from(""),
            suffix: String::from(""),
            count: 1,
            max_length: 16,
        }
    }
}

pub fn generate_ids(adjs: &[&str], nouns: &[&str], config: &Config) -> Result<HashSet<String>> {
    // TODO: make this more dynamic
    if config.prefix.chars().count() > 5 {
        return Err(anyhow!("Prefix must be less than or equal to 5 characters"));
    }
    if config.suffix.chars().count() > 5 {
        return Err(anyhow!("Suffix must be less than or equal to 5 characters"));
    }
    if config.count > MAX_IDS_COUNT {
        return Err(anyhow!(
            "Count must be less than or equal to {}",
            MAX_IDS_COUNT
        ));
    }
    if config.max_length > MAX_ID_LENGTH {
        return Err(anyhow!(
            "Max length must be less than or equal to {}",
            MAX_ID_LENGTH
        ));
    }

    let distinct_ids_avail = adjs.len() * nouns.len();
    let mut uniq_ids = Vec::with_capacity(distinct_ids_avail / 4);

    // Build a vec of all available ids for given constraints
    for adj in adjs {
        let adj = capitalize_first_char(adj);
        for noun in nouns {
            let noun = capitalize_first_char(noun);
            let id = format!("{}{}{}{}", config.prefix, adj, noun, config.suffix);
            if id.len() <= config.max_length {
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
        return Err(anyhow!(
            "Not enough unique IDs available for the given count. Only {} IDs available.",
            uniq_ids.len()
        ));
    }

    // Randomly choose config.count number of ids from the precomputed list
    let mut random_ids = HashSet::with_capacity(config.count);
    let mut rng = rand::thread_rng();

    while random_ids.len() != config.count {
        let random_index = rng.gen_range(0..uniq_ids.len());
        if let Some(id) = uniq_ids.get(random_index) {
            random_ids.insert(id.clone());
        }
    }

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
}
