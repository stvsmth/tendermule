# CLAUDE.md

This file provides guidance to LLM Agents when working with code in this repository.

## Project Overview

**tendermule** is a Rust CLI utility and library that generates unique, memorable random
identifiers in `AdjectiveNoun` format (e.g., "WastefulGuppy", "CurvyDancer"). Identifiers are under
16 characters by default, with optional prefix/suffix and alliteration support.

## Commands

```bash
# Build
cargo build --release

# Run all tests
cargo test --locked

# Lint
cargo clippy --locked --all-targets --all-features

# Format check
cargo fmt --all -- --check

# Auto-format
cargo fmt --all

# Run a specific test
cargo test <test_name>

# Run the binary
./target/release/tendermule --prefix stv --count 5
```

## Architecture

The project is split into a library crate and two binaries:

- **`src/lib.rs`** — Core logic. `Config` struct holds generation parameters; `generate_ids()`
  validates constraints, builds all valid adjective-noun pairs, filters by length/alliteration, and
  randomly samples `count` unique IDs. Returns `HashSet<String>`.
- **`src/main.rs`** — CLI entry point using clap derive macros. All options also accept env vars
  with `TMULE_` prefix (e.g., `TMULE_PREFIX`).
- **`src/replay.rs`** — Fuzzing utility that reads a JSON `Config` and invokes the binary; used for
  edge case testing.
- **`src/words/adjs.rs`** and **`src/words/nouns.rs`** — Word lists, one word per line (for clean
  diffs), with `#[rustfmt::skip]`.

**Generation algorithm:** pre-compute all valid combinations → filter by `max_length` (accounts for
prefix/suffix) → optionally filter for alliteration → randomly sample without replacement.

## CI

The CI pipeline (`.github/workflows/ci.yaml`) runs:
1. `code_checks`: fmt, clippy, tests on ubuntu
2. `build`: cross-compile for `aarch64-apple-darwin`, `aarch64-unknown-linux-gnu`,
   `x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`

Version tags matching `v*` trigger GitHub Releases with packaged binaries.

MSRV: **1.85.0** (Rust 2024 edition)
