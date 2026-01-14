#![warn(missing_docs)]

//! # Tajweed Warsh Rules
//!
//! A comprehensive Rust library for detecting and processing Islamic Quranic recitation rules
//! (Tajweed) with full support for the Warsh and Hafs narrations.
//!
//! ## Features
//!
//! - **25+ Tajweed Rules**: Comprehensive coverage of all major Quranic recitation rules
//! - **Dual Narration Support**: Warsh with 40+ narration-specific variants and Hafs standard rules
//! - **Accurate Diacritic Handling**: Proper processing of Arabic diacritical marks
//! - **Well-Structured API**: Clean, modular architecture for easy integration
//! - **Production Ready**: Thoroughly tested and documented
//!
//! ## Quick Start
//!
//! ```rust
//! use tajweed_warsh_rules::{TajweedProcessor, RecitationStyle};
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
//! ### Noon Sakinah & Tanwin (أحكام النون الساكنة والتنوين)
//! - Al-Izhar Al-Halqi (الإظهار الحلقي)
//! - Al-Izhar Al-Mutlaq (الإظهار المطلق)
//! - Idgham with Ghunnah (الإدغام بغنة)
//! - Idgham without Ghunnah (الإدغام بغير غنة)
//! - Idgham Naqis (الإدغام الناقص) - Warsh specific
//! - Al-Iqlab (الإقلاب)
//! - Al-Ikhfaa Al-Haqiqi (الإخفاء الحقيقي)
//!
//! ### Mim Sakinah (أحكام الميم الساكنة)
//! - Al-Ikhfaa Al-Shafawi (الإخفاء الشفوي)
//! - Al-Idgham Al-Shafawi (الإدغام الشفوي)
//! - Al-Izhar Al-Shafawi (الإظهار الشفوي)
//!
//! ### Lam Al-Ta'rif (أحكام لام أل التعريف)
//! - Al-Izhar Al-Qamari (الإظهار القمري)
//! - Al-Idgham Al-Shamsi (الإدغام الشمسي)
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
//! - Qalqalah Kubra (القلقلة الكبرى)
//! - Qalqalah Sughra (القلقلة الصغرى)
//!
//! ### Ra Emphasis (أحكام الراء)
//! - Tafkhim Ra (تفخيم الراء)
//! - Tarqeeq Ra (ترقيق الراء) - Warsh specific
//!
//! ### Special Rules
//! - Tafkhim Lafz Al-Jalalah (تفخيم لفظ الجلالة)
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

// Re-export main types and processor for convenient access
pub use processor::TajweedProcessor;
pub use types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod doctests {
    //! These module exists just for doctest examples that don't need to be executed
    //! The actual implementation is in the main modules
}
