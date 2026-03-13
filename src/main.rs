use clap::Parser;
use clap_num::number_range;
use std::collections::HashSet;
use tendermule::{MAX_ID_LENGTH, MAX_IDS_COUNT, MIN_ID_LENGTH, count_available, generate_ids};

// TODO: Currently our static adjs and nouns are read via main.rs and not the library code.
// Consider moving the adjs and nouns to this library.
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

    /// Should we only consider adjective-noun pairs that start with the same letter
    #[arg(short, long, env = "TMULE_ALLITERATE", default_value_t = false)]
    alliterate: bool,

    /// Print the number of unique identifiers available for the current configuration and exit
    #[arg(long, env = "TMULE_AVAILABLE", conflicts_with = "count")]
    available: bool,
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
        alliterate: args.alliterate,
    };

    if args.available {
        let n = count_available(adjs, nouns, &config);
        println!("{n}");
        return;
    }

    let results = generate_ids(adjs, nouns, &config);
    match results {
        Ok(results) => {
            print_results(results);
        }
        _ => {
            if let Err(e) = results {
                eprintln!("Error: {e}");
            }
        }
    }
}

fn print_results(results: HashSet<String>) {
    for result in results {
        println!("{result}");
    }
}
