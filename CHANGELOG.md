# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **المد المنفصل / المد المتصل في رواية ورش** (#4): via طريق الأزرق both are
  ishbaa' — six harakaat only — instead of the 4/5/6 range previously reported.
  Descriptions now name the ṭarīq (الأزرق vs الأصبهاني).
- **صلة الهاء** (#5): no longer flagged as Warsh-specific — it is read by all
  qurra' with differences in detail. The description no longer calls the Haa
  *sakinah*; it is the voweled Haa of the pronoun between two voweled letters
  (matching what detection already did).
- **ترقيق الراء** (#6): a Ra carrying a kasra (e.g. `لِنُرِيَهُۥ`, 17:1) is thinned by
  every reader and is no longer labelled Warsh-specific. `TarqeeqRa` now reports
  its narration scope **per occurrence**: agreed positions (kasra Ra, sakin Ra
  after a kasra or a sakin Ya) are unflagged, while Warsh's own positions
  (fatha/damma Ra after a kasra or a sakin Ya) keep `warsh_specific = true`.

- **النقل** (#8): Naql was only ever looked for across a word boundary, so the
  transfer onto the Lam of the definite article — the case the README already
  documented — was never reported. Both spellings are now detected: the ordinary
  one (`الْإِيمَٰن`, sakin Lam + written hamza) and the Warsh mushaf one
  (`اُ۬لِايمَٰنَ`, where the Lam already carries the transferred vowel and only the
  silent Alif of the hamza is left). Reported on سورة الحجرات (49:7).
- **`src/comprehensive_tests.rs` never ran**: the file was never declared in
  `lib.rs`, so its 17 tests had never been compiled. It is now part of the suite,
  and four assertions that contradicted the tajweed were corrected — tanwin
  before ع is إظهار حلقي and before ي is إدغام بغنة (both were asserted as
  إخفاء), لفظ الجلالة after a kasra (`بِسْمِ اللَّهِ`) is ترقيق not تفخيم, and a lone
  خ is a تفخيم trigger rather than an empty result.

### Added

- `TajweedRule::with_warsh_specific` — override the narration scope of a single
  occurrence, for rules that mix agreed-upon and Warsh-only positions.
- `rules::ra::TarqeeqScope` — `Agreed` / `WarshSpecific` classification produced
  by Ra tarqeeq detection.
- `src/reported_issues_tests.rs` — 36 regression tests covering the reported
  verses (17:1 and 49:7) and the surrounding cases in both narrations.

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
