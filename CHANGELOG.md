# Changelog

## v0.3.5 2026-06-12

### Added
  * Additional nouns in the built-in word list.

## v0.3.4 2026-06-06

### Fixed
  * Avoid a panic when no identifier can be generated for the given constraints; return an
    error instead.

## v0.3.3 2026-06-06

### Added
  * `generate_ids_default()` and `count_available_default()` wrappers, so library consumers can
    use the built-in word lists without supplying their own.

### Changed
  * Split into a library and CLI: the `cli` feature gates the CLI-only dependencies
    (`clap`, `clap-num`, `serde`, `serde_json`), so `--no-default-features` builds the library
    alone.

### Other
  * Removed duplicate tests.

## v0.3.2 2026-04-11

### Other
  * Added supply-chain checks (`cargo-audit`, `cargo-vet`).
  * Bumped dependencies and fixed CI.

## v0.3.1 2026-04-09

### Other
  * CI fix (`upload-artifact@v6` for Node 24).
  * Bumped dependencies, updated adjective/noun lists, tidied anyhow error messages.

## v0.3.0 2026-03-12

### Added
  * `--available` flag (and `TMULE_AVAILABLE` env var) prints the count of unique identifiers
    possible for the current configuration and exits. Respects `--prefix`, `--suffix`,
    `--max-length`, and `--alliterate`. Conflicts with `--count`.
  * `count_available()` library function exposes the same logic for programmatic use.

## v0.2.3 2025-02-02

### Other
  * Updated dependencies, fix some trivial issues from Clippy.

## v0.2.2 2024-11-24

### Other
  * Updated dependencies to latest versions.

## v0.2.1 2024-09-16

### Other
  * Updated dependencies to latest versions.

## v0.2.0 2023-08-06

### Added
  * Honor environment variables for options. (4110e41)
    - Provided option will take precedence over environment variables.
    - For example, `TMULE_PREFIX=stv` will work in place of `--prefix=stv`.

### Fixed
  * Fixed bug that incorrectly limited prefix and suffix to 4, not 5 characters. (7097f2a)
  * Library now returns `Err` (via anyhow) rather than string. This is a breaking change,
    but I am the only user at this moment. (c3375b4)

### Other
  * Fixed deprecated CI/CD dependencies. 
  * Updated documentation with minor fixes.

## v0.1.0 2023-06-04

* Initial release.
