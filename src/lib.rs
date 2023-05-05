use rand::Rng;
use std::collections::HashSet;

pub fn generate_ids(
    adjs: &[&str],
    nouns: &[&str],
    prefix: String,
    suffix: String,
    count: usize,
    max_length: usize,
) -> HashSet<String> {
    // TODO: make this more dynamic
    if prefix.len() > 5 {
        eprintln!("Prefix must be less than or equal to 5 characters");
        std::process::exit(1);
    }
    if suffix.len() > 5 {
        eprintln!("Suffix must be less than or equal to 5 characters");
        std::process::exit(1);
    }

    // Make the multiplier (100) an arg?
    let mut max_attempts = count * 100;
    let mut results = HashSet::new();
    while max_attempts > 0 {
        max_attempts -= 1;

        let mut length = max_length;

        // Make this a constant
        if length < 3 {
            eprintln!(
                "Prefix, suffix, and max_length must leave 3 characters for the generated id"
            );
            std::process::exit(1);
        }
        let random_adj = choose_word(adjs, length);
        length -= random_adj.len();
        let adjective = capitalize_first_char(&random_adj);
        let mut new_id = format!("{}{}{}", prefix, adjective, suffix);
        if length < 3 && new_id.len() <= max_length {
            results.insert(new_id);
            if results.len() == count {
                break;
            }
        }

        let random_noun = choose_word(nouns, length);

        let noun = capitalize_first_char(&random_noun);
        new_id = format!("{}{}{}{}", prefix, adjective, noun, suffix);

        let new_id_len = new_id.len();
        let id_fits = new_id_len <= max_length;
        if id_fits {
            results.insert(new_id);
            if results.len() == count {
                break;
            }
        }
    }

    if max_attempts == 0 {
        panic!("Failed to generate {} unique identifiers", count);
    }
    results
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
    let random_index = rng.gen_range(0..=filtered_words.len() - 1);

    filtered_words[random_index].to_string()
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
        for _ in 0..24 {
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
        let count = 2;
        let max_length = 10;
        let ids = generate_ids(
            &adjs,
            &nouns,
            "".to_string(),
            "".to_string(),
            count,
            max_length,
        );
        let results = ids.into_iter().collect::<Vec<String>>();
        assert_eq!(results.len(), 2);
        let id_1 = results[0].clone();
        let id_2 = results[1].clone();
        assert!(id_1 != id_2);
    }
}
