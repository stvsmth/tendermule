use clap::Parser;
use std::process::exit;
use tendermule::generate_ids;
mod words;
use words::adjs;
use words::nouns;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// String to prepend to front of identifier, included when considering max length
    #[arg(short, long, default_value = "")]
    prefix: String,

    /// String to append to end of identifier, included when considering max length
    #[arg(short, long, default_value = "")]
    suffix: String,

    /// Number of unique identifiers to generate
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Maximum length of the final identifier
    #[arg(short, long, default_value_t = 16)]
    max_length: usize,
}

fn main() {
    let args = Args::parse();

    if args.count == 0 {
        eprintln!("Error: count must be greater than 0");
        exit(128);
    }

    let adjs = adjs::ADJS;
    let nouns = nouns::NOUNS;
    let available_length = args.max_length - args.prefix.len() - args.suffix.len();

    let results = generate_ids(
        adjs,
        nouns,
        args.prefix,
        args.suffix,
        args.count,
        available_length,
    );

    for id in results {
        println!("{}", id);
    }
}
