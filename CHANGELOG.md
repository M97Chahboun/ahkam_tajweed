# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-01-14

### Changed

- **Major Refactoring**: Reorganized codebase into modular structure for better maintainability
  - Separated concerns into individual modules: `types`, `processor`, `utils`
  - Created `lib.rs` as the library root for crate publishing
  - Simplified `main.rs` to use library API for CLI

- **Improved Project Structure**: Now ready for publication on crates.io
  - Added comprehensive README with usage examples
  - Updated Cargo.toml with proper metadata
  - Added library and binary targets
  - Added unit tests

- **Code Quality Improvements**
  - Better separation of concerns
  - Improved code organization
  - Added inline documentation
  - Removed code duplication

### Fixed

- Fixed compiler warnings
- Improved error handling in module structure

### Added

- Comprehensive README.md with API documentation
- CHANGELOG.md
- Unit tests for utility functions and processor
- Support for building as a publishable Rust crate
- Proper module organization with `pub mod` declarations

## [0.1.0] - 2024-01-14

### Added

- Initial implementation of Tajweed rule processor
- Support for 25+ Tajweed rules
- Dual recitation style support (Warsh and Hafs)
- Interactive CLI for verse analysis
- Comprehensive rule detection including:
  - Noon/Mim Sakinah rules
  - Lam Al-Ta'rif rules
  - Madd rules with Warsh-specific variants
  - Qalqalah rules
  - Ra emphasis rules
  - Tafkhim Lafz Al-Jalalah
