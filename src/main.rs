use clap::Parser;
use clap_num::number_range;
use std::collections::HashSet;
use tendermule::{generate_ids, MAX_IDS_COUNT, MAX_ID_LENGTH, MIN_ID_LENGTH};

mod words;
use words::adjs;
use words::nouns;

fn valid_max_count(s: &str) -> Result<usize, String> {
    number_range(s, 1, MAX_IDS_COUNT)
}

fn valid_id_len(s: &str) -> Result<usize, String> {
    number_range(s, MIN_ID_LENGTH, MAX_ID_LENGTH)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// String to prepend to front of identifier, included when considering max length
    #[arg(short, long, env = "TMULE_PREFIX", default_value = "")]
    prefix: String,

    /// String to append to end of identifier, included when considering max length
    #[arg(short, long, env = "TMULE_SUFFIX", default_value = "")]
    suffix: String,

    /// Number of unique identifiers to generate
    #[arg(short, long, env = "TMULE_COUNT", default_value_t = 1, value_parser=valid_max_count)]
    count: usize,

    /// Maximum length of the final identifier
    #[arg(short, long, env = "TMULE_MAX_LENGTH", default_value_t = 16, value_parser=valid_id_len)]
    max_length: usize,
}

fn main() {
    let args = Args::parse();

    let adjs = adjs::ADJS;
    let nouns = nouns::NOUNS;
    let config = tendermule::Config {
        prefix: args.prefix.clone(),
        suffix: args.suffix.clone(),
        count: args.count,
        max_length: args.max_length,
    };

    let results = generate_ids(adjs, nouns, &config);
    if let Ok(results) = results {
        print_results(results);
    } else if let Err(e) = results {
        eprintln!("Error: {}", e);
    }
}

fn print_results(results: HashSet<String>) {
    for result in results {
        println!("{}", result);
    }
}
