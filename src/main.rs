use anyhow::{Result, anyhow};
use clap::Parser;
use clap_num::number_range;
use std::collections::HashSet;
use tendermule::{
    MAX_ID_LENGTH, MAX_IDS_COUNT, MIN_ID_LENGTH, count_available_default, generate_ids_default,
};

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

    let config = tendermule::Config {
        prefix: args.prefix.clone(),
        suffix: args.suffix.clone(),
        count: args.count,
        max_length: args.max_length,
        alliterate: args.alliterate,
    };

    if args.available {
        let n = count_available_default(&config);
        println!("{n}");
        return;
    }

    match generate_ids_default(&config).and_then(non_empty_results) {
        Ok(ids) => print_results(ids),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

fn non_empty_results(results: HashSet<String>) -> Result<HashSet<String>> {
    if results.is_empty() {
        return Err(anyhow!(
            "No unique IDs available for the given constraints."
        ));
    }

    Ok(results)
}

fn print_results(results: HashSet<String>) {
    for result in results {
        println!("{result}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tendermule::Config;

    /// These values are documented in README.md. If this test fails because the word lists
    /// changed, update the `--available` example in README.md to match the new counts.
    #[test]
    fn test_available_counts_match_readme() {
        for (max_length, expected) in [(8, 48_473_usize), (12, 467_972), (16, 647_520)] {
            let config = Config {
                max_length,
                ..Config::default()
            };
            let actual = count_available_default(&config);
            assert_eq!(
                actual, expected,
                "Available count for --max-length {max_length} is now {actual} but README documents \
                 {expected}. Update the --available example in README.md."
            );
        }
    }

    #[test]
    fn test_empty_success_is_classified_as_error() {
        let result = non_empty_results(HashSet::new()).unwrap_err();
        assert_eq!(
            result.to_string(),
            "No unique IDs available for the given constraints."
        );
    }
}
