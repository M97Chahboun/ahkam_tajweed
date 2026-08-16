#![warn(missing_docs)]

//! # Tajweed Warsh Rules
//!
//! A comprehensive Rust library for detecting and processing Islamic Quranic recitation rules
//! (Tajweed) with full support for the Warsh and Hafs narrations.
//!
//! ## Features
//!
//! - **35+ Tajweed Rules**: Comprehensive coverage of all major Quranic recitation rules
//! - **Dual Narration Support**: Warsh with 40+ narration-specific variants (Naql, Tasheel, Badal, Tarqeeq Ra) and Hafs standard rules
//! - **Accurate Diacritic Handling**: Proper processing of Arabic diacritical marks
//! - **Well-Structured API**: Clean, modular architecture for easy integration
//! - **Production Ready**: Thoroughly tested (220+ automated tests) and documented
//!
//! ## Quick Start
//!
//! ```rust
//! use tajweed_rules::{TajweedProcessor, RecitationStyle};
//!
//! let processor = TajweedProcessor::new(RecitationStyle::Warsh);
//! let verse = "الحَمْدُ للهِ رَبِّ العالمين";
//! let rules = processor.process_verse(verse);
//!
//! for rule_match in rules {
//!     println!("Rule: {}", rule_match.rule.english_name);
//!     println!("Position: {}-{}", rule_match.start_index, rule_match.end_index);
//! }
//! ```
//!
//! ## Supported Rules
//!
//! ### Noon Sakinah, Tanwin & Ghunnah (أحكام النون الساكنة والتنوين والغنة)
//! - Al-Izhar Al-Halqi (الإظهار الحلقي)
//! - Al-Izhar Al-Mutlaq (الإظهار المطلق)
//! - Idgham with Ghunnah (الإدغام بغنة)
//! - Idgham without Ghunnah (الإدغام بغير غنة)
//! - Idgham Naqis (الإدغام الناقص) - Warsh specific
//! - Al-Iqlab (الإقلاب)
//! - Al-Ikhfaa Al-Haqiqi (الإخفاء الحقيقي)
//! - Ghunnah Mushadda (الغنة في المشدد) - Noon/Meem with Shaddah
//!
//! ### Mim Sakinah (أحكام الميم الساكنة)
//! - Al-Ikhfaa Al-Shafawi (الإخفاء الشفوي)
//! - Al-Idgham Al-Shafawi (الإدغام الشفوي)
//! - Al-Izhar Al-Shafawi (الإظهار الشفوي)
//!
//! ### Lam Al-Ta'rif & Hamzat Wasl (أحكام لام أل التعريف وهمزة الوصل)
//! - Al-Izhar Al-Qamari (الإظهار القمري)
//! - Al-Idgham Al-Shamsi (الإدغام الشمسي)
//! - Hamzat Al-Wasl (همزة الوصل)
//!
//! ### Madd Rules (أحكام المدود)
//! - Madd Tabeei (المد الطبيعي)
//! - Madd Muttasil (المد المتصل)
//! - Madd Munfasil (المد المنفصل)
//! - Madd Lazim (المد اللازم)
//! - Madd Arid (المد العارض للسكون)
//! - Madd Lin (المد اللين)
//! - Madd Badal (مد البدل) - Warsh variants: 2-6 harakaat
//! - Madd Silah (صلة الهاء) - Warsh specific
//!
//! ### Qalqalah (القلقلة)
//! - Qalqalah Akbar (القلقلة الأكبر) - Shaddah at Waqf
//! - Qalqalah Kubra (القلقلة الكبرى)
//! - Qalqalah Sughra (القلقلة الصغرى)
//!
//! ### Ra & Lafz Al-Jalalah (أحكام الراء ولفظ الجلالة)
//! - Tafkhim Ra (تفخيم الراء)
//! - Tarqeeq Ra (ترقيق الراء) - includes after Saakin Ya
//! - Tafkhim Lafz Al-Jalalah (تفخيم لفظ الجلالة) - after Fatha/Damma
//! - Tarqeeq Lafz Al-Jalalah (ترقيق لفظ الجلالة) - after Kasra
//!
//! ### Idgham Mutajanisayn & Mutaqaribayn (إدغام المتجانسين والمتقاربين)
//! - Idgham Mutajanisayn (إدغام المتجانسين) - (ط+ت, ذ+ظ, د+ت)
//! - Idgham Mutaqaribayn (إدغام المتقاربين) - (ق+ك, ل+ر)
//!
//! ### Warsh-Specific Rules (أحكام ورش الخاصة)
//! - An-Naql (النقل) - vowel transfer from Hamza Qat'a to preceding Saakin
//! - Tasheel Al-Hamza (تسهيل الهمزة) - softening consecutive Hamzas
//!
//! ## Recitation Styles
//!
//! This library supports two major recitation narrations:
//!
//! - **Hafs** (حفص عن عاصم): The most common recitation style used globally
//! - **Warsh** (ورش عن نافع): The primary recitation in North Africa and parts of the Middle East
//!
//! The Warsh variant includes 40+ style-specific features such as:
//! - Extended Madd lengths (2-6 vs 4-5 harakaat)
//! - Ra thinning (Tarqeeq Ra)
//! - Ha prolongation (Madd Silah)
//! - Hamza simplification (Madd Badal with extended lengths)

pub mod processor;
pub mod rules;
pub mod types;
pub mod utils;
pub mod zwj_handler;

#[cfg(test)]
mod tajweed_alignment_tests;

// Re-export main types and processor for convenient access
pub use processor::TajweedProcessor;
pub use types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
pub use zwj_handler::apply_zwj_to_text;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod debug_tests {
    use super::*;

    #[test]
    fn debug_tanwin_ikhfaa_case() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        let matches = processor.process_verse("كِتَابًا كَبِيرًا");

        println!("Debug: Found {} rules for 'كِتَابًا كَبِيرًا':", matches.len());
        for (i, rule_match) in matches.iter().enumerate() {
            println!("  {}: {} ({}-{}) - '{}' -> '{}' | Context: {}",
                i+1,
                rule_match.rule.english_name,
                rule_match.start_index,
                rule_match.end_index,
                rule_match.target_letter,
                rule_match.following_letter.map(|c| c.to_string()).unwrap_or("-".to_string()),
                rule_match.context
            );
        }

        // Check if any rule involves tanwin
        let has_ikhaa = matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::IkhfaaHaqiqi);
        println!("Has IkhfaaHaqiqi: {}", has_ikhaa);

        // Let's also check the individual characters
        let chars: Vec<char> = "كِتَابًا كَبِيرًا".chars().collect();
        for (i, c) in chars.iter().enumerate() {
            println!("Index {}: '{}' (U+{:04X})", i, c, *c as u32);
        }
    }

    #[test]
    fn debug_madd_lin_case() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        let matches = processor.process_verse("لَيْسَ");

        println!("Debug: Found {} rules for 'لَيْسَ':", matches.len());
        for (i, rule_match) in matches.iter().enumerate() {
            println!("  {}: {} ({}-{}) - '{}' -> '{}' | Context: {}",
                i+1,
                rule_match.rule.english_name,
                rule_match.start_index,
                rule_match.end_index,
                rule_match.target_letter,
                rule_match.following_letter.map(|c| c.to_string()).unwrap_or("-".to_string()),
                rule_match.context
            );
        }

        // Check if any rule involves MaddLin or MaddTabeei
        let has_maddlin = matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::MaddLin);
        let has_maddtabeei = matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::MaddTabeei);
        println!("Has MaddLin: {}, Has MaddTabeei: {}", has_maddlin, has_maddtabeei);

        // Let's also check the individual characters
        let chars: Vec<char> = "لَيْسَ".chars().collect();
        for (i, c) in chars.iter().enumerate() {
            println!("Index {}: '{}' (U+{:04X})", i, c, *c as u32);
        }
    }

    #[test]
    fn debug_madd_lin_waw_case() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        let matches = processor.process_verse("وَقْفٌ");

        println!("Debug: Found {} rules for 'وَقْفٌ':", matches.len());
        for (i, rule_match) in matches.iter().enumerate() {
            println!("  {}: {} ({}-{}) - '{}' -> '{}' | Context: {}",
                i+1,
                rule_match.rule.english_name,
                rule_match.start_index,
                rule_match.end_index,
                rule_match.target_letter,
                rule_match.following_letter.map(|c| c.to_string()).unwrap_or("-".to_string()),
                rule_match.context
            );
        }

        // Check if any rule involves MaddLin or MaddTabeei
        let has_maddlin = matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::MaddLin);
        let has_maddtabeei = matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::MaddTabeei);
        println!("Has MaddLin: {}, Has MaddTabeei: {}", has_maddlin, has_maddtabeei);

        // Let's also check the individual characters
        let chars: Vec<char> = "وَقْفٌ".chars().collect();
        for (i, c) in chars.iter().enumerate() {
            println!("Index {}: '{}' (U+{:04X})", i, c, *c as u32);
        }
    }

    #[test]
    fn debug_madd_muttasil_case() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        let matches = processor.process_verse("سُوْءٌ");

        println!("Debug: Found {} rules for 'سُوْءٌ':", matches.len());
        for (i, rule_match) in matches.iter().enumerate() {
            println!("  {}: {} ({}-{}) - '{}' -> '{}' | Context: {}",
                i+1,
                rule_match.rule.english_name,
                rule_match.start_index,
                rule_match.end_index,
                rule_match.target_letter,
                rule_match.following_letter.map(|c| c.to_string()).unwrap_or("-".to_string()),
                rule_match.context
            );
        }

        // Check if any rule involves MaddMuttasil
        let has_maddmuttasils = matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::MaddMuttasil);
        let has_maddtabeei = matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::MaddTabeei);
        println!("Has MaddMuttasil: {}, Has MaddTabeei: {}", has_maddmuttasils, has_maddtabeei);

        // Let's also check the individual characters
        let chars: Vec<char> = "سُوْءٌ".chars().collect();
        for (i, c) in chars.iter().enumerate() {
            println!("Index {}: '{}' (U+{:04X})", i, c, *c as u32);
        }
    }

    #[test]
    fn debug_tarqeeq_ra_case() {
        let processor = TajweedProcessor::new(RecitationStyle::Warsh); // Tarqeeq Ra is more common in Warsh
        let matches = processor.process_verse("رِيحٌ");

        println!("Debug: Found {} rules for 'رِيحٌ':", matches.len());
        for (i, rule_match) in matches.iter().enumerate() {
            println!("  {}: {} ({}-{}) - '{}' -> '{}' | Context: {}",
                i+1,
                rule_match.rule.english_name,
                rule_match.start_index,
                rule_match.end_index,
                rule_match.target_letter,
                rule_match.following_letter.map(|c| c.to_string()).unwrap_or("-".to_string()),
                rule_match.context
            );
        }

        // Check if any rule involves Tarqeeq Ra
        let has_tarqeeq = matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::TarqeeqRa);
        let has_tafkhim = matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::TafkhimRa);
        println!("Has TarqeeqRa: {}, Has TafkhimRa: {}", has_tarqeeq, has_tafkhim);

        // Let's also check the individual characters
        let chars: Vec<char> = "رِيحٌ".chars().collect();
        for (i, c) in chars.iter().enumerate() {
            println!("Index {}: '{}' (U+{:04X})", i, c, *c as u32);
        }
    }

    #[test]
    fn debug_waw_madd_with_damma_case() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        let matches = processor.process_verse("نُوحٌ");

        println!("Debug: Found {} rules for 'نُوحٌ':", matches.len());
        for (i, rule_match) in matches.iter().enumerate() {
            println!("  {}: {} ({}-{}) - '{}' -> '{}' | Context: {}",
                i+1,
                rule_match.rule.english_name,
                rule_match.start_index,
                rule_match.end_index,
                rule_match.target_letter,
                rule_match.following_letter.map(|c| c.to_string()).unwrap_or("-".to_string()),
                rule_match.context
            );
        }

        // Check if any rule involves MaddTabeei
        let has_maddtabeei = matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::MaddTabeei);
        println!("Has MaddTabeei: {}", has_maddtabeei);

        // Let's also check the individual characters
        let chars: Vec<char> = "نُوحٌ".chars().collect();
        for (i, c) in chars.iter().enumerate() {
            println!("Index {}: '{}' (U+{:04X})", i, c, *c as u32);
        }
    }
}

