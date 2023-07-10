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

    // Make the multiplier (100) an arg?
    let mut max_attempts = config.count * 100;
    let mut results = HashSet::new();
    while max_attempts > 0 {
        max_attempts -= 1;

        let mut length = config.max_length;

        // Make this a constant
        if length < 3 {
            return Err(anyhow!(
                "Prefix, suffix, and max_length must leave 3 characters for the generated id",
            ));
        }
        let random_adj = choose_word(adjs, length);
        length -= random_adj.len();
        let adjective = capitalize_first_char(&random_adj);
        let mut new_id = format!("{}{}{}", config.prefix, adjective, config.suffix);
        if length < 3 && new_id.len() <= config.max_length {
            results.insert(new_id);
            if results.len() == config.count {
                break;
            }
        }

        let random_noun = choose_word(nouns, length);

        let noun = capitalize_first_char(&random_noun);
        new_id = format!("{}{}{}{}", config.prefix, adjective, noun, config.suffix);
        if new_id == format!("{}{}", config.prefix, config.suffix) {
            continue;
        }

        let new_id_len = new_id.len();
        let id_fits = new_id_len <= config.max_length;
        if id_fits {
            results.insert(new_id);
            if results.len() == config.count {
                break;
            }
        }
    }

    if max_attempts == 0 {
        let suggestion = "Perhaps your max_length is to small or your prefix/suffix are too large.";
        match results.len() {
            0 => {
                return Err(anyhow!(
                    "Unable to generate any unique identifiers. {}",
                    suggestion
                ));
            }
            _ => {
                return Err(anyhow!(
                    "Only generated {} of {} unique identifiers. {}",
                    results.len(),
                    config.count,
                    suggestion,
                ));
            }
        }
    }

    Ok(results)
}

/// Given a vector of words, choose a random word that is less than or equal to
/// the given max length.
fn choose_word(words: &[&str], max_length: usize) -> String {
    let filtered_words: Vec<&str> = words
        .iter()
        .filter(|word| word.len() <= max_length)
        .cloned()
        .collect();

    let mut rng = rand::thread_rng();
    let max_index = filtered_words.len().saturating_sub(1);
    let random_index = rng.gen_range(0..=max_index);
    filtered_words.get(random_index).unwrap_or(&"").to_string()
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
    fn test_choose_word_any() {
        let words = vec!["hello", "world", "four", "foo", "bar"];
        let mut set = HashSet::new();
        let mut i = 0;
        let max_runs = 200;
        while set.len() != 5 && i < max_runs {
            i += 1;
            let word = choose_word(&words, 5);
            set.insert(word);
        }
        assert!(set.contains("bar"));
        assert!(set.contains("foo"));
        assert!(set.contains("four"));
        assert!(set.contains("hello"));
        assert!(set.contains("world"));
    }

    #[test]
    fn test_choose_word_with_limit() {
        let words = vec!["hello", "world", "foo", "bar"];
        let mut set = HashSet::new();
        for _ in 0..24 {
            let word = choose_word(&words, 3);
            set.insert(word);
        }
        assert!(set.contains("bar"));
        assert!(set.contains("foo"));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_count_generates_unique_values() {
        let adjs = vec!["blue", "gray", "red", "green"];
        let nouns = vec!["cat", "dog", "bird", "fish"];
        let config = Config {
            prefix: String::from(""),
            suffix: String::from(""),
            count: 2,
            max_length: 10,
        };
        let ids = generate_ids(&adjs, &nouns, &config).unwrap();
        let results = ids.into_iter().collect::<Vec<String>>();
        assert_eq!(results.len(), 2);
        let id_1 = results[0].clone();
        let id_2 = results[1].clone();
        assert!(id_1 != id_2);
    }

    #[test]
    fn test_default_config_values() {
        let adjs = vec!["astronomical"];
        let nouns = vec!["goat"];
        let config = Config::default();
        let ids = generate_ids(&adjs, &nouns, &config).unwrap();
        let results = ids.into_iter().collect::<Vec<String>>();
        assert_eq!(results.len(), 1);
        let id = results[0].clone();
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
