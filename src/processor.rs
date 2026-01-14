//! Main Tajweed rule processor
//!
//! This module contains the core `TajweedProcessor` struct that analyzes Quranic verses
//! and detects applicable Tajweed rules by orchestrating the rule detection modules.

use crate::rules;
use crate::types::{RecitationStyle, RuleMatch};

/// The main Tajweed processor for analyzing Quranic verses
///
/// This processor detects and classifies Tajweed rules according to the specified
/// recitation style (Warsh or Hafs). It handles:
/// - Noon/Mim Sakinah rules
/// - Lam Al-Ta'rif rules
/// - Madd rules
/// - Qalqalah rules
/// - Ra emphasis rules
/// - Special rules like Tafkhim Lafz Al-Jalalah
pub struct TajweedProcessor {
    style: RecitationStyle,
}

impl TajweedProcessor {
    /// Create a new TajweedProcessor for the specified recitation style
    pub fn new(style: RecitationStyle) -> Self {
        TajweedProcessor { style }
    }

    /// Process a Quranic verse and detect all applicable Tajweed rules
    ///
    /// Returns a vector of `RuleMatch` objects containing all detected rules
    pub fn process_verse(&self, verse: &str) -> Vec<RuleMatch> {
        let mut matches: Vec<RuleMatch> = Vec::new();
        let verse_chars: Vec<char> = verse.chars().collect();

        // First pass: Noon/Mim Sakinah rules
        rules::noon_mim::detect_noon_mim_rules(&verse_chars, &mut matches, self.style);

        // Second pass: Lam Al-Ta'rif rules
        rules::lam_al_tarif::detect_lam_al_tarif_rules(&verse_chars, &mut matches, self.style);

        // Third pass: Madd rules
        rules::madd::detect_madd_rules(&verse_chars, &mut matches, self.style);

        // Fourth pass: Qalqalah rules
        rules::qalqalah::detect_qalqalah_rules(&verse_chars, &mut matches, self.style);

        // Fifth pass: Ra emphasis rules
        rules::ra::detect_ra_rules(&verse_chars, &mut matches, self.style);

        // Sixth pass: Tafkhim Lafz Al-Jalalah
        rules::ra::detect_allah_name_rules(&verse_chars, &mut matches, self.style);

        matches
    }

    /// Get the recitation style of this processor
    pub fn get_style(&self) -> RecitationStyle {
        self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_creation() {
        let processor = TajweedProcessor::new(RecitationStyle::Warsh);
        assert_eq!(processor.get_style(), RecitationStyle::Warsh);
    }

    #[test]
    fn test_basic_rule_detection() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        let verse = "الحَمْدُ";
        let matches = processor.process_verse(verse);
        assert!(!matches.is_empty());
    }
}
