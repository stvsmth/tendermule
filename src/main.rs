use clap::Parser;
use clap_num::number_range;
use tendermule::generate_ids;
mod words;
use words::adjs;
use words::nouns;

fn valid_max_count(s: &str) -> Result<usize, String> {
    number_range(s, 1, 1_000_000)
}

fn valid_id_len(s: &str) -> Result<u8, String> {
    number_range(s, 8, 255)
}

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
    #[arg(short, long, default_value_t = 1, value_parser=valid_max_count)]
    count: usize,

    /// Maximum length of the final identifier
    #[arg(short, long, default_value_t = 16, value_parser=valid_id_len)]
    max_length: usize,
}

fn main() {
    let args = Args::parse();

    let adjs = adjs::ADJS;
    let nouns = nouns::NOUNS;

    let results = generate_ids(
        adjs,
        nouns,
        args.prefix,
        args.suffix,
        args.count,
        args.max_length,
    );

    for id in results {
        println!("{}", id);
    }
}
