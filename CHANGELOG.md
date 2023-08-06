# Changelog

## v0.2.0 2023-08-06

### Added
  * Honor environment variables for options. [4110e41]
    - Provided option will take precedence over environment variables.
    - For example, `TMULE_PREFIX will work in place of `--prefix=stv`.

### Fixed
  * Fixed bug that incorrectly limited prefix and suffix to 4, not 5 characters. [7097f2a]
  * Library now returns `Err` (via anyhow) rather than string. This is a breaking change,
    but I am the only user at this moment. [c3375b4]

### Other
  * Fixed deprecated CI/CD dependencies. 
  * Updated documentation with minor fixes.

## v0.1.0 2023-06-04

* Initial release.
