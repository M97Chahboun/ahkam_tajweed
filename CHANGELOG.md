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

#### Found by differential testing against an external corpus

The whole Quran was run through the engine and compared, rule by rule, against
the [`cpfair/quran-tajweed`](https://github.com/cpfair/quran-tajweed) reference
corpus (CC-BY-4.0) over the Tanzil Uthmani text. Each discrepancy below was
confirmed against the tajweed before being fixed; regression tests for all of
them live in `src/corpus_diff_tests.rs`.

- **همزة الوصل** was never detected in the Uthmani script. Detection only looked
  at a plain Alif at the start of a word, so the Alif Wasla the script actually
  writes (ٱ, U+0671) — 13,252 occurrences — was skipped entirely. It is now
  recognised wherever it stands, including inside a word after a prefix
  (`بِٱسْمِ`). Recall 0% → 100%.
- **الميم الساكنة without a written sukun**: the Uthmani script leaves the sukun
  off a Mim Sakinah standing before م or ب, because the following mark already
  carries the information. The "unmarked letter is sakinah" rule was applied to
  the Noon only, so `قُلُوبِهِم مَّرَضٌ` and `هُم بِمُؤْمِنِينَ` produced nothing. Idgham
  Shafawi and Ikhfaa Shafawi both went from 0% to 100% recall (1,328 occurrences).
- **المد المنفصل across the silent Alif**: the Alif of `قَالُوٓا۟` is written but not
  read, and it was being taken for the letter that decides the Madd's class, so
  `قَالُوٓا۟ إِنَّمَا` was reported as Muttasil. Recall 77.4% → 93.9%.
- **المد اللازم الحرفي** (الحروف المقطعة): a Maddah over a letter that is not a
  Madd letter can only be a disjoined surah opener (`الٓمٓ`, `صٓ`, `نٓ`), whose
  spelled-out name holds a six-count Madd. These were reported as Muttasil.
- **المد اللازم over-detection**: it was enough for *any* neighbouring letter to
  carry a shadda, including one in the next word (`وَمِمَّا رَّزَقْنَٰهُمْ`) or one on the
  Madd letter itself (`إِيَّاكَ`). It now requires the shadda or sukun on the letter
  *after* the Madd letter, in the same word. Recall 66.2% → 97.3%, false
  positives 2,675 → 2.
- **A voweled Waw/Ya was treated as a Madd letter**: a Madd letter carries no
  vowel of its own, so `وُسْعَهَا` and `ٱلْوُثْقَىٰ` were producing madd rules on a
  plain consonant.
- **المد العارض للسكون was effectively missing** (recall 0.2%): it fired wherever
  a waqf sign appeared anywhere later in the verse, and never at the end of the
  verse — which is where nearly every one of them is. The stop is now the letter
  the reciter actually halts on: the last letter before a waqf sign, or the last
  letter of the verse. Recall 0.2% → 97.9% (4,438 occurrences).
- **صلة الهاء on words that have no pronoun**: the fallback heuristic (a
  word-final voweled Haa between two voweled letters) reported a Silah on the
  name of Allah (`ٱللَّهِ إِنَّ`). It now skips the لفظ الجلالة, and steps aside
  entirely in texts that write the Silah out with the small Waw / small Ya.
- **لام التعريف after the لـ prefix**: when the article's Alif is dropped in
  writing (`لِلنَّاسِ` = لِ + النَّاس، `لِّلْمُتَّقِينَ`) the article was not recognised at
  all. Recall 95.2% → 99.9%.
- **إدغام المتقاربين on the article's Lam**: `ٱلرَّحْمَٰنِ` was reported as Idgham
  Mutaqaribayn instead of Lam Shamsiyyah. The ل+ر pair now only counts where the
  Lam is not the article's (`قُل رَّبِّ`). False positives 542 → 1.
- **Duplicate matches**: when both the explicit-symbol scan and a rule module see
  the same occurrence their spans rarely coincide exactly, so the exact-key
  dedup let the duplicate through — 1,680 duplicated Munfasil and 399 duplicated
  Iqlab matches over the Quran. Dedup now collapses overlapping spans of the same
  rule.
- Three stale test expectations were corrected along the way: `أَمَّا` is not Madd
  Lazim (the shadda precedes the Madd letter) and `وَقْفٌ` is not Madd Lin (the Waw
  carries the fatha rather than a sukun).

### Added

- `src/corpus_diff_tests.rs` — 24 regression tests, one per discrepancy found by
  the whole-Quran comparison described above.
- `examples/dump_rules.rs` — dumps every detected rule as TSV, so the engine can
  be diffed against an external corpus.
- `VerseIndex::next_pronounced_letter` / `VerseIndex::vowel_on_previous_letter`,
  `utils::is_silent_letter`, `letters::MADD_CARRIERS_ALL`.
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
