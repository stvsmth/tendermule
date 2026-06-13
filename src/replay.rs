//! Replay/fuzzing helper: reads a JSON `Config` from a file and invokes the `tendermule`
//! binary with the matching CLI flags. Used to reproduce edge cases found while fuzzing.
//!
//! Usage: `replay [path-to-json]` (defaults to `test.json` in the working directory).

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::{Command, ExitCode};

#[derive(Debug, Serialize, Deserialize)]
struct FuzzData {
    prefix: String,
    suffix: String,
    count: usize,
    max_length: usize,
}

/// Locate the sibling `tendermule` binary next to this executable, so the same build profile
/// (`debug`/`release`) is used without hardcoding a path.
fn tendermule_bin() -> Result<PathBuf> {
    let mut path = std::env::current_exe()
        .context("could not determine the current executable path")?
        .parent()
        .context("current executable has no parent directory")?
        .join("tendermule");
    if cfg!(windows) {
        path.set_extension("exe");
    }
    Ok(path)
}

fn run() -> Result<bool> {
    let json_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "test.json".into());
    let contents = std::fs::read_to_string(&json_path)
        .with_context(|| format!("failed to read {json_path}"))?;
    let fuzz: FuzzData =
        serde_json::from_str(&contents).with_context(|| format!("failed to parse {json_path}"))?;

    let bin = tendermule_bin()?;
    let output = Command::new(&bin)
        .args([
            "--prefix",
            &fuzz.prefix,
            "--suffix",
            &fuzz.suffix,
            "--count",
            &fuzz.count.to_string(),
            "--max-length",
            &fuzz.max_length.to_string(),
        ])
        .output()
        .with_context(|| format!("failed to execute {}", bin.display()))?;

    if output.status.success() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(output.status.success())
}

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("Error: {e:#}");
            ExitCode::FAILURE
        }
    }
}
