# Changelog

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
