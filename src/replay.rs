use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct FuzzData {
    prefix: String,
    suffix: String,
    count: u8,
    max_length: u8,
}

fn read_json_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_json<T>(json_str: &str) -> Result<T, serde_json::Error>
where
    T: serde::de::DeserializeOwned,
{
    let value: T = serde_json::from_str(json_str)?;
    Ok(value)
}

fn main() {
    let file_path = "test.json";
    match read_json_file(file_path) {
        Ok(contents) => match parse_json::<FuzzData>(&contents) {
            Ok(fuzz_data) => {
                println!("Parsed JSON: {:?}", fuzz_data);
                let args = vec![
                    "--prefix".to_string(),
                    fuzz_data.prefix,
                    "--suffix".to_string(),
                    fuzz_data.suffix,
                    "--count".to_string(),
                    fuzz_data.count.to_string(),
                    "--max-length".to_string(),
                    fuzz_data.max_length.to_string(),
                ];

                println!("Running command: {:?}", args);
                let output = Command::new("target/debug/tendermule")
                    .args(&args)
                    .output()
                    .expect("Failed to execute command");

                if output.status.success() {
                    println!("The program ran successfully!");
                    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                } else {
                    println!("The program failed!");
                    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(err) => {
                eprintln!("Failed to parse JSON: {}", err);
            }
        },
        Err(err) => {
            eprintln!("Failed to read file: {}", err);
        }
    }
}
