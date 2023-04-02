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

    // TODO: add something along the lines of "with prefix `stv` and suffix `123` and max length 16"
    let mut id = format!("No ID works with max-length of {}", args.max_length);

    // Naive version for now; we'll add more complex logic that will index into a
    // word array sorted by length and then pick a random word from that array.
    let mut rng = rand::thread_rng();
    let remaining = 1000;
    for _ in 0..remaining {
        let adjective = capitalize_first_char(&adjs[rng.gen_range(0..adjs.len())]);
        let noun = capitalize_first_char(&nouns[rng.gen_range(0..nouns.len())]);
        let new_id = format!("{}{}{}{}", args.prefix, adjective, noun, args.suffix);

        if new_id.len() <= args.max_length {
            id = new_id;
            break;
        }
    }

    if remaining > 0 {
        println!("{}", id);
    } else {
        panic!("Failed to generate ID after {} attempts", remaining);
    }
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
