# Tajweed Rules

A comprehensive Rust library for detecting and processing Islamic Quranic recitation rules (Tajweed) with full support for the Warsh and Hafs narrations.

## Features

- **25+ Tajweed Rules**: Comprehensive coverage of all major Quranic recitation rules
- **Dual Narration Support**: Warsh with 40+ narration-specific variants and Hafs standard rules
- **Accurate Diacritic Handling**: Proper processing of Arabic diacritical marks
- **Well-Structured API**: Clean, modular architecture for easy integration
- **Production Ready**: Thoroughly tested and documented

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tajweed_rules = "0.2"
```

## Quick Start

```rust
use tajweed_rules::{TajweedProcessor, RecitationStyle};

fn main() {
    let processor = TajweedProcessor::new(RecitationStyle::Warsh);
    let verse = "الحَمْدُ للهِ رَبِّ العالمين";
    let rules = processor.process_verse(verse);

    for rule_match in rules {
        println!("Rule: {}", rule_match.rule.english_name);
        println!("Position: {}-{}", rule_match.start_index, rule_match.end_index);
        println!("Description: {}", rule_match.rule.description_ar);
    }
}
```

## Usage

### Basic Processing

```rust
use tajweed_rules::{TajweedProcessor, RecitationStyle};

// Create a processor for Warsh recitation
let processor = TajweedProcessor::new(RecitationStyle::Warsh);

// Process a verse
let verse = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ";
let matches = processor.process_verse(verse);

// Examine results
for m in matches {
    println!("Rule: {} ({})", m.rule.arabic_name, m.rule.english_name);
    println!("Position: {} to {}", m.start_index, m.end_index);
    if let Some(following) = m.following_letter {
        println!("Following letter: {}", following);
    }
    println!("Context: {}", m.context);
}
```

### Comparing Styles

```rust
use tajweed_rules::{TajweedProcessor, RecitationStyle};

let warsh = TajweedProcessor::new(RecitationStyle::Warsh);
let hafs = TajweedProcessor::new(RecitationStyle::Hafs);

let verse = "آمَنَ";

println!("Warsh rules:");
for m in warsh.process_verse(verse) {
    println!("  {}", m.rule.english_name);
}

println!("Hafs rules:");
for m in hafs.process_verse(verse) {
    println!("  {}", m.rule.english_name);
}
```

## Supported Rules

### Noon Sakinah & Tanwin (أحكام النون الساكنة والتنوين)

- **Al-Izhar Al-Halqi** (الإظهار الحلقي) - Clear pronunciation before throat letters
- **Al-Izhar Al-Mutlaq** (الإظهار المطلق) - Clear pronunciation in same word
- **Idgham with Ghunnah** (الإدغام بغنة) - Merging with nasal sound
- **Idgham without Ghunnah** (الإدغام بغير غنة) - Merging without nasal sound
- **Idgham Naqis** (الإدغام الناقص) - Incomplete merging (Warsh specific)
- **Al-Iqlab** (الإقلاب) - Converting to Meem before Ba
- **Al-Ikhfaa Al-Haqiqi** (الإخفاء الحقيقي) - Hidden pronunciation before specific letters

### Mim Sakinah (أحكام الميم الساكنة)

- **Al-Ikhfaa Al-Shafawi** (الإخفاء الشفوي) - Hidden labial pronunciation
- **Al-Idgham Al-Shafawi** (الإدغام الشفوي) - Labial merging
- **Al-Izhar Al-Shafawi** (الإظهار الشفوي) - Clear labial pronunciation

### Lam Al-Ta'rif (أحكام لام أل التعريف)

- **Al-Izhar Al-Qamari** (الإظهار القمري) - Clear pronunciation before lunar letters
- **Al-Idgham Al-Shamsi** (الإدغام الشمسي) - Merging with solar letters

### Madd Rules (أحكام المدود)

- **Madd Tabeei** (المد الطبيعي) - 2 harakaat
- **Madd Muttasil** (المد المتصل) - 4-5 harakaat (Warsh: 4-6)
- **Madd Munfasil** (المد المنفصل) - 2-4-5 harakaat (Warsh: 4-6)
- **Madd Lazim** (المد اللازم) - 6 harakaat
- **Madd Arid** (المد العارض للسكون) - 2-4-6 harakaat
- **Madd Lin** (المد اللين) - 2-4-6 harakaat
- **Madd Badal** (مد البدل) - 2 harakaat (Warsh: 4-6)
- **Madd Silah** (صلة الهاء) - Warsh specific

### Qalqalah (القلقلة)

- **Qalqalah Kubra** (القلقلة الكبرى) - Major bouncing at word end
- **Qalqalah Sughra** (القلقلة الصغرى) - Minor bouncing in connected position

### Ra Emphasis (أحكام الراء)

- **Tafkhim Ra** (تفخيم الراء) - Emphasis/heaviness
- **Tarqeeq Ra** (ترقيق الراء) - Thinning/lightness (Warsh specific)

### Special Rules

- **Tafkhim Lafz Al-Jalalah** (تفخيم لفظ الجلالة) - Emphasis on Allah's name

## Recitation Styles

### Hafs (حفص عن عاصم)

The most commonly used recitation worldwide. This is the baseline implementation with standard madd lengths and standard rule applications.

### Warsh (ورش عن نافع)

The primary recitation in North Africa and parts of the Middle East. Includes 40+ style-specific features:

- Extended Madd lengths for connected and separated madd types
- Ra thinning (Tarqeeq Ra) in specific contexts
- Ha prolongation (Madd Silah)
- Hamza simplification (Madd Badal) with extended lengths
- Incomplete merging (Idgham Naqis) variants

## API Reference

### `TajweedProcessor`

Main processor struct for analyzing verses.

```rust
impl TajweedProcessor {
    /// Create a new processor for the specified recitation style
    pub fn new(style: RecitationStyle) -> Self

    /// Process a verse and return all detected rules
    pub fn process_verse(&self, verse: &str) -> Vec<RuleMatch>

    /// Get the recitation style of this processor
    pub fn get_style(&self) -> RecitationStyle
}
```

### `RuleMatch`

Represents a detected rule in a verse.

```rust
pub struct RuleMatch {
    pub start_index: usize,           // Where the rule starts
    pub end_index: usize,             // Where the rule ends
    pub target_letter: char,          // The main letter
    pub following_letter: Option<char>, // The following letter if relevant
    pub rule: TajweedRule,            // The detected rule
    pub context: String,              // Surrounding context
}
```

### `TajweedRule`

Contains detailed information about a rule.

```rust
pub struct TajweedRule {
    pub rule_type: TajweedRuleType,
    pub arabic_name: &'static str,
    pub english_name: &'static str,
    pub description_ar: &'static str,
    pub warsh_specific: bool,
    pub madd_length_warsh: Option<(u8, u8)>, // (min, max) in harakaat
}
```

## CLI Tool

This package includes an interactive CLI tool for analyzing verses:

```bash
cargo run --bin tajweed

# In the CLI:
> بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ
> :style warsh
> :style both
> :q
```

## Testing

Run the test suite:

```bash
cargo test
```

## Building

Build the library:

```bash
cargo build --release
```

## Project Structure

```
src/
├── lib.rs                      # Library root and public API
├── main.rs                     # Interactive CLI application
├── types.rs                    # Core types and enumerations
├── processor.rs                # Main processor orchestrating rule detection
├── utils.rs                    # Utility functions for Arabic characters
└── rules/                      # Modular rule detection modules
    ├── mod.rs                  # Rules module root
    ├── noon_mim.rs             # Noon/Mim Sakinah and Tanwin rules (11 rules)
    ├── lam_al_tarif.rs         # Lam Al-Ta'rif (definite article) rules (2 rules)
    ├── madd.rs                 # Madd (vowel prolongation) rules (8 rules)
    ├── qalqalah.rs             # Qalqalah (bouncing) rules (2 rules)
    └── ra.rs                   # Ra emphasis and Allah name emphasis rules (3 rules)
```

### Module Descriptions

- **types.rs** (222 lines): Defines `TajweedRule`, `RuleMatch`, `RecitationStyle`, and `TajweedRuleType` enum with 30+ rule variants
- **utils.rs** (162 lines): Arabic character utilities including diacritic handling, vowel detection, and context extraction
- **processor.rs** (75 lines): Clean orchestrator that coordinates the 6-pass rule detection algorithm
- **rules/noon_mim.rs** (300 lines): Detects Noon/Mim/Tanwin rules with comprehensive letter mapping logic
- **rules/lam_al_tarif.rs** (95 lines): Identifies Lunar (Izhar Qamari) and Solar (Idgham Shamsi) letters
- **rules/madd.rs** (130 lines): Handles 8 Madd variants with proper shadda and hamza detection
- **rules/qalqalah.rs** (45 lines): Detects Qalqalah Kubra and Sughra based on sukun placement
- **rules/ra.rs** (130 lines): Processes Ra emphasis rules with Warsh-specific Tarqeeq detection

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Submit a pull request

## License

Licensed under either of Apache License Version 2.0 or MIT license at your option.

## Citation

If you use this library in academic or professional work, please cite:

```bibtex
@software{tajweed_rules,
  title = {Tajweed Rules: A Comprehensive Quranic Recitation Rule Processor},
  year = {2026},
  version = {0.2.0}
}
```

## Acknowledgments

This library implements rules based on established Islamic scholarship in Tajweed (Quranic recitation rules) according to both the Warsh and Hafs narrations, as documented in classical Arabic Islamic texts.

## Support

For issues, feature requests, or questions, please open an issue on the repository.
