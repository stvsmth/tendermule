## Project Overview

**tendermule** is a Rust CLI utility and library that generates unique, memorable random
identifiers in `AdjectiveNoun` format (e.g., "WastefulGuppy", "CurvyDancer"). Identifiers up to
16 characters by default, with optional prefix/suffix and alliteration support.

## Commands

```bash
# Build
cargo build --release

# Build the library alone, without CLI dependencies
cargo build --no-default-features

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

# Audit dependencies for vulnerabilities
cargo audit

# Vet dependencies for supply-chain review
cargo vet

# Run the binary
./target/release/tendermule --prefix stv --count 5
```

## Development Setup

After cloning, enable the pre-commit hook (runs fmt, clippy, tests, and vet):

```bash
git config core.hooksPath hooks
```

## Architecture

The project is split into a library crate and two binaries:

- **`src/lib.rs`** — Core logic. `Config` struct holds generation parameters; `generate_ids()`
  validates constraints, builds all valid adjective-noun pairs, filters by length/alliteration, and
  randomly samples `count` unique IDs. Returns `HashSet<String>`. `generate_ids_default()` and
  `count_available_default()` are convenience wrappers that use the crate's built-in word lists, so
  library consumers don't need to supply their own.
- **`src/words/`** — Word-list module (`pub mod words`), owned by the library. `adjs.rs` and
  `nouns.rs` hold the lists, one word per line (for clean diffs), with `#[rustfmt::skip]`.
- **`src/main.rs`** — CLI entry point using clap derive macros. All options also accept env vars
  with `TMULE_` prefix (e.g., `TMULE_PREFIX`).
- **`src/replay.rs`** — Fuzzing utility that reads a JSON `Config` and invokes the binary; used for
  edge case testing.

**Cargo features:** The `cli` feature (enabled by `default`) gates the CLI-only dependencies
(`clap`, `clap-num`, `serde`, `serde_json`); both binaries declare `required-features = ["cli"]`.
The library itself depends only on `anyhow` and `rand`, so `--no-default-features` builds the
library alone without the CLI stack. Keep CLI-only deps out of `src/lib.rs` and `src/words/` or the
library-only build breaks.

**Generation algorithm:** pre-compute all valid combinations → filter by `max_length` (accounts for
prefix/suffix) → optionally filter for alliteration → randomly sample without replacement.

## CI

The CI pipeline (`.github/workflows/ci.yaml`) runs:
1. `code_checks`: fmt, clippy, tests, vet, and a library-only `--no-default-features` build/test on
   ubuntu
2. `msrv`: verifies the crate builds on the minimum supported Rust version
3. `build`: cross-compile for `aarch64-apple-darwin`, `aarch64-unknown-linux-gnu`,
   `x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`

Version tags matching `v*` trigger GitHub Releases with packaged binaries.

A separate weekly workflow (`.github/workflows/audit.yaml`) runs `cargo audit` to check for
dependency vulnerabilities without blocking CI on unrelated PRs.

MSRV: **1.85.0** (Rust 2024 edition)

## Releasing

```bash
# 1. Tag the release and push the tag to trigger CI
git tag v0.X.Y
git push origin v0.X.Y

# 2. CI builds all targets and creates the GitHub Release automatically.
#    After CI completes, add release notes:
gh release edit v0.X.Y --notes "* ...release notes...

**Full Changelog**: https://github.com/stvsmth/tendermule/compare/vPREVIOUS...v0.X.Y"
```
