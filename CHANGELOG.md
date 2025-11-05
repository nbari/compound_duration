# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2025-11-05

### Breaking Changes
- Changed all internal constants from `usize` to `u64` to fix arithmetic overflow on 32-bit architectures
- Simplified function signatures to use `TryInto<u64>` trait bounds
- Updated to Rust 2024 edition (requires recent Rust compiler)

### Added
- Comprehensive CI/CD workflows (CI, Release, Security Audit)
- EditorConfig for consistent code style
- Improved documentation with more examples
- Added `no-std` category support

### Changed
- Modernized README with badges and better examples
- Improved inline documentation
- Removed deprecated Travis CI badges
- Updated license field in Cargo.toml to use SPDX identifier

### Fixed
- **Critical**: Fixed arithmetic overflow on 32-bit architectures (Debian build failure)
- Removed all `unwrap()` calls from public API (now use proper error messages)

## [1.2.1] - 2021-09-01

### Changed
- Clippy improvements: variables can be used directly in format strings

## [1.2.0] - 2021-08-15

### Changed
- Implemented trait to prevent using `as usize`

## [1.1.0] - 2021-07-20

### Added
- `format_ns` function for nanosecond formatting

## [1.0.3] - 2021-06-10

### Changed
- Updated README

## [1.0.2] - 2021-05-15

### Changed
- Clippy cleanup

## [1.0.1] - 2021-04-20

### Fixed
- Fixed Travis CI configuration

## [1.0.0] - 2019-06-15

### Added
- Initial release
- `format_dhms` function
- `format_wdhms` function
