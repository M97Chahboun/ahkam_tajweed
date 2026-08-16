# Tajweed Rules

A comprehensive, high-performance Rust library for detecting and processing Islamic Quranic recitation rules (Tajweed) with full support for the Warsh ('an Nafi') and Hafs ('an 'Asim) narrations.

## Features

- **35+ Tajweed Rules**: Complete coverage of all major Quranic recitation rules
- **Dual Narration Support**: Warsh (via Al-Azraq) with narration-specific rules (Naql, Tasheel, Badal, Tarqeeq Ra, Idgham Naqis) and Hafs standard rules
- **Accurate Diacritic & Script Handling**: Proper processing of Arabic diacritics, Uthmani script nuances, and Zero-Width Joiner (ZWJ) shaping
- **Well-Structured & Fast**: Precomputed `VerseIndex` for zero-overhead character navigations and single-pass detection
- **Production Ready**: 220+ automated unit, integration, and alignment tests

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
    let verse = "الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ ۝";
    let rules = processor.process_verse(verse);

    for rule_match in rules {
        println!("Rule: {} ({})", rule_match.rule.english_name, rule_match.rule.arabic_name);
        println!("Position: {}-{}", rule_match.start_index, rule_match.end_index);
        println!("Description: {}", rule_match.rule.description_ar);
    }
}
```

## Usage

### Basic Processing

```rust
use tajweed_rules::{TajweedProcessor, RecitationStyle};

// Create a processor for Hafs recitation
let processor = TajweedProcessor::new(RecitationStyle::Hafs);

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

### Comparing Warsh and Hafs Narrations

```rust
use tajweed_rules::{TajweedProcessor, RecitationStyle};

let warsh = TajweedProcessor::new(RecitationStyle::Warsh);
let hafs = TajweedProcessor::new(RecitationStyle::Hafs);

// Naql in Warsh vs standard in Hafs
let verse = "قَدْ أَفْلَحَ";

println!("Warsh rules:");
for m in warsh.process_verse(verse) {
    println!("  {} ({})", m.rule.english_name, m.rule.arabic_name);
}

println!("Hafs rules:");
for m in hafs.process_verse(verse) {
    println!("  {} ({})", m.rule.english_name, m.rule.arabic_name);
}
```

## Supported Rules

### 1. Noon Sakinah, Tanwin & Ghunnah (أحكام النون الساكنة والتنوين والغنة)
- **Al-Izhar Al-Halqi** (الإظهار الحلقي) — Clear pronunciation before 6 throat letters (ء, هـ, ع, ح, غ, خ)
- **Al-Izhar Al-Mutlaq** (الإظهار المطلق) — Clear pronunciation within the same word (e.g., دنيا, صنوان)
- **Idgham with Ghunnah** (الإدغام بغنة) — Merging with nasalization (ي, ن, م, و)
- **Idgham without Ghunnah** (الإدغام بغير غنة) — Complete merging without nasal sound (ل, ر)
- **Idgham Naqis** (الإدغام الناقص) — Incomplete merging (Warsh specific)
- **Al-Iqlab** (الإقلاب) — Conversion of Noon/Tanwin to Meem before Ba (ب)
- **Al-Ikhfaa Al-Haqiqi** (الإخفاء الحقيقي) — Concealed pronunciation before 15 letters
- **Ghunnah Mushadda** (الغنة في المشدد) — Mandatory 2-harakat nasal sound on Noon and Meem with Shaddah (نّ / مّ)

### 2. Mim Sakinah (أحكام الميم الساكنة)
- **Al-Ikhfaa Al-Shafawi** (الإخفاء الشفوي) — Labial concealment before Ba (ب)
- **Al-Idgham Al-Shafawi / Idgham Mithlayn** (الإدغام الشفوي / إدغام المتماثلين) — Merging with another Meem (م)
- **Al-Izhar Al-Shafawi** (الإظهار الشفوي) — Clear pronunciation before all remaining letters

### 3. Lam Al-Ta'rif & Hamzat Wasl (أحكام لام أل التعريف وهمزة الوصل)
- **Al-Izhar Al-Qamari** (الإظهار القمري) — Clear pronunciation before 14 lunar letters (ابغ حجك وخف عقيمه)
- **Al-Idgham Al-Shamsi** (الإدغام الشمسي) — Merging into 14 solar letters
- **Hamzat Al-Wasl** (همزة الوصل) — Connecting Hamza (in definite article, Form I imperatives, and standard Wasl nouns)

### 4. Madd Rules (أحكام المدود)
- **Madd Tabeei** (المد الطبيعي) — Natural prolongation (2 counts)
- **Madd Muttasil** (المد المتصل) — Connected prolongation before Hamza in the same word (4-5 counts, Warsh: 4-6)
- **Madd Munfasil** (المد المنفصل) — Separated prolongation before Hamza across word boundary (2-4-5 counts, Warsh: 4-6)
- **Madd Lazim** (المد اللازم) — Compulsory prolongation before permanent Sukun/Shaddah (6 counts)
- **Madd Arid** (المد العارض للسكون) — Incidental prolongation at Waqf/pause (2-4-6 counts)
- **Madd Lin** (المد اللين) — Soft prolongation on Waw/Ya Saakin preceded by Fatha (2-4-6 counts)
- **Madd Badal** (مد البدل) — Hamza preceding Madd letter (2 counts, Warsh: 2/4/6 counts)
- **Madd Silah** (صلة الهاء) — Extension of Ha Kinayah (Warsh: includes Silah Kubra)

### 5. Qalqalah (القلقلة)
- **Qalqalah Akbar** (القلقلة الأكبر) — Strongest echo on Qalqalah letter with Shaddah at Waqf (e.g., الْحَجِّ)
- **Qalqalah Kubra** (القلقلة الكبرى) — Major echo at word end / verse end
- **Qalqalah Sughra** (القلقلة الصغرى) — Minor echo in connected/middle positions

### 6. Ra & Lafz Al-Jalalah (أحكام الراء ولفظ الجلالة)
- **Tafkhim Ra** (تفخيم الراء) — Emphasized heavy Ra (with Fatha/Damma, or Sukun after Fatha/Damma)
- **Tarqeeq Ra** (ترقيق الراء) — Light Ra (with Kasra, Sukun after Kasra, or after Saakin Ya e.g., خَيْرْ, قَدِيرْ)
- **Tafkhim Lafz Al-Jalalah** (تفخيم لفظ الجلالة) — Heavy "Allah" when preceded by Fatha/Damma (e.g., قَالَ اللَّهُ)
- **Tarqeeq Lafz Al-Jalalah** (ترقيق لفظ الجلالة) — Light "Allah" when preceded by Kasra (e.g., بِاللَّهِ, بِسْمِ اللَّهِ)

### 7. Idgham Mutajanisayn & Mutaqaribayn (إدغام المتجانسين والمتقاربين)
- **Idgham Mutajanisayn** (إدغام المتجانسين) — Same articulation point with different characteristics (`ط+ت`, `ذ+ظ`, `د+ت`)
- **Idgham Mutaqaribayn** (إدغام المتقاربين) — Adjacent/close articulation points (`ق+ك`, `ل+ر`)

### 8. Waqf, Wasl & Sakt Signs (علامات الوقف والوصل والسكت)
- **Waqf Lazim** (مـ) — Compulsory Stop
- **Waqf Mamnou** (لا) — Prohibited Stop
- **Waqf Jaiz** (ج) — Permissible Stop
- **Waqf Awla** (قلى) — Stop Preferred
- **Wasl Awla** (صلى) — Continue Preferred
- **Waqf Muanaqah** (∴) — Embracing Stop (stop at either, not both)
- **Sakt** (س) — Short pause without breathing

### 9. Warsh-Specific Rules (أحكام ورش الخاصة)
- **An-Naql** (النقل) — Transfer of Hamza vowel to the preceding Saakin consonant across word boundaries (e.g., `قَدْ أَفْلَحَ` → `قَدَفْلَحَ`, `الْأَرْضِ` → `الَارْضِ`)
- **Tasheel Al-Hamza** (تسهيل الهمزة) — Softening of consecutive Hamzas in the same word (e.g., `أَأَنذَرْتَهُمْ`)
- **Madd Badal Extension** — 2, 4, or 6 harakaat options (Al-Qasr, At-Tawassut, At-Tool)
- **Special Ra Tarqeeq & Idgham Naqis**

## Recitation Styles

| Feature | Hafs ('an 'Asim) | Warsh ('an Nafi' via Al-Azraq) |
|---|---|---|
| **Madd Muttasil / Munfasil** | 4-5 harakaat | 4-6 harakaat |
| **Madd Badal** | 2 harakaat | 2, 4, or 6 harakaat |
| **An-Naql (النقل)** | No | ✅ Yes (vowel transfer to Saakin) |
| **Tasheel Al-Hamza (تسهيل الهمزة)** | Limited | ✅ Yes (two Hamzas in word) |
| **Tarqeeq Ra (ترقيق الراء)** | Standard conditions | Extended (after Kasra, Saakin Ya) |
| **Idgham Naqis (الإدغام الناقص)** | Standard | ✅ Narration-specific variants |

## API Reference

### `TajweedProcessor`

Main processor struct for analyzing verses.

```rust
impl TajweedProcessor {
    /// Create a new processor for the specified recitation style
    pub fn new(style: RecitationStyle) -> Self;

    /// Process a verse and return all detected rules
    pub fn process_verse(&self, verse: &str) -> Vec<RuleMatch>;

    /// Get the recitation style of this processor
    pub fn get_style(&self) -> RecitationStyle;
}
```

### `RuleMatch`

Represents a detected rule in a verse.

```rust
pub struct RuleMatch {
    pub start_index: usize,             // Character offset where the rule starts
    pub end_index: usize,               // Character offset where the rule ends
    pub target_letter: char,            // The main letter
    pub following_letter: Option<char>, // The following letter if relevant
    pub rule: TajweedRule,              // The detected rule metadata
    pub context: String,                // Surrounding text context
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

# Inside the interactive CLI:
> بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ
> :style warsh
> :style both
> :q
```

## Testing

Run the full automated test suite:

```bash
cargo test
```

All 220+ unit, integration, and alignment tests are verified against classical Tajweed references (Al-Jazariyyah, Tuhfat Al-Atfal, and Al-Azraq rules for Warsh).

## Project Structure

```
src/
├── lib.rs                      # Library root, documentation and public re-exports
├── main.rs                     # Interactive CLI application
├── types.rs                    # Core types, RecitationStyle and TajweedRuleType enum
├── processor.rs                # Main high-performance rule detection pipeline
├── utils.rs                    # Arabic character utilities, VerseIndex, diacritics
├── zwj_handler.rs              # Zero-Width Joiner (ZWJ) Arabic script connectivity
├── tajweed_alignment_tests.rs  # Comprehensive classical alignment test suite
└── rules/                      # Modular rule detection logic
    ├── mod.rs                  # Rules module root
    ├── noon_mim.rs             # Noon/Mim Sakinah, Tanwin, Ghunnah, Naql, Tasheel, Idgham
    ├── lam_al_tarif.rs         # Lam Al-Ta'rif (Izhar Qamari & Idgham Shamsi)
    ├── madd.rs                 # Madd rules (Tabeei, Muttasil, Munfasil, Lazim, Arid, Lin, Badal)
    ├── qalqalah.rs             # Qalqalah (Sughra, Kubra, Akbar)
    └── ra.rs                   # Ra rules (Tafkhim/Tarqeeq) & Allah Name rules
```

## License

Licensed under either of Apache License Version 2.0 or MIT license at your option.

## Acknowledgments

This library implements rules based on established Islamic scholarship in Tajweed (Quranic recitation rules) according to both the Warsh and Hafs narrations, as documented in classical Arabic Islamic texts (*Al-Muqaddimah Al-Jazariyyah*, *Tuhfat Al-Atfal*, and the path of Al-Azraq).
