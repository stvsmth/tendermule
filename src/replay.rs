use std::io::Read;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct FuzzData {
    prefix: String,
    suffix: String,
    count: u8,
    max_length: u8,
}

fn main() {
    // Read the binary data from stdin
    let mut data = Vec::new();
    std::io::stdin().read_to_end(&mut data).unwrap();

    // Deserialize the data into our FuzzData structure
    let fuzz_data: Result<FuzzData, _> = bincode::deserialize(&data);

    if let Ok(fuzz_data) = fuzz_data {
        // Prepare the arguments
        let args = vec![
            "--prefix".to_string(),
            fuzz_data.prefix,
            "--suffix".to_string(),
            fuzz_data.suffix,
            "--count".to_string(),
            fuzz_data.count.to_string(),
            "--max_length".to_string(),
            fuzz_data.max_length.to_string(),
        ];

        // Run the target binary with the arguments
        let output = Command::new("path/to/target/binary")
            .args(&args)
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!("The program crashed!");
            // Handle crash...
        }
    }
}
