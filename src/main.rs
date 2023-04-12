use clap::Parser;
use rand::Rng;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// String to prepend to front of identifier, included when considering max length
    #[arg(short, long, default_value = "")]
    prefix: String,

    /// String to append to end of identifier, included when considering max length
    #[arg(short, long, default_value = "")]
    suffix: String,

    /// Maximum length of the final identifier
    #[arg(short, long, default_value_t = 16)]
    max_length: usize,
}

fn main() {
    let args = Args::parse();

    let adjectives_file = "words-adj.txt";
    let adjs = read_file(adjectives_file);

    let nouns_file = "words-nouns.txt";
    let nouns = read_file(nouns_file);

    let mut max_length = args.max_length - args.prefix.len() - args.suffix.len();
    let random_adj = choose_word(adjs, max_length);

    max_length -= random_adj.len();
    let random_noun = choose_word(nouns, max_length);

    let adjective = capitalize_first_char(&random_adj);
    let noun = capitalize_first_char(&random_noun);
    let new_id = format!("{}{}{}{}", args.prefix, adjective, noun, args.suffix);

    if new_id.len() <= args.max_length {
        println!("{}", new_id);
    } else {
        // TODO: add something along the lines of "We are unable to generate an id with prefix
        // `stv` and suffix `123` and max length 16
        panic!("Failed to generate correct length ID: {}", new_id);
    }
}

/// Given a vector of words, choose a random word that is less than or equal to
/// the given max length.
fn choose_word(words: Vec<String>, max_length: usize) -> String {
    let filtered_words: Vec<String> = words
        .iter()
        .filter(|word| word.len() <= max_length)
        .cloned()
        .collect();

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..=filtered_words.len() - 1);

    filtered_words[random_index].clone()
}

fn read_file(filename: &str) -> Vec<String> {
    let contents =
        fs::read_to_string(filename).unwrap_or_else(|_| panic!("Failed to read file {}", filename));

    contents.split_whitespace().map(|s| s.to_string()).collect()
}

fn capitalize_first_char(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
