//! Lam Al-Ta'rif (definite article) rule detection

use crate::types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
use crate::utils::*;

/// Detect Lam Al-Ta'rif rules in verse
pub fn detect_lam_al_tarif_rules(
    verse_chars: &[char],
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    let index = VerseIndex::new(verse_chars);
    detect_lam_al_tarif_rules_indexed(verse_chars, &index, matches, style);
}

pub(crate) fn detect_lam_al_tarif_rules_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    // Setup letter maps
    const IZHAR_QAMARI_LETTERS: [char; 14] = [
        'ا', 'ب', 'غ', 'ح', 'ج', 'ك', 'و', 'خ', 'ف', 'ع', 'ق', 'ي', 'م', 'ه',
    ];

    const IDGHAM_SHAMSI_LETTERS: [char; 14] = [
        'ت', 'ث', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ل', 'ن',
    ];

    let mut i = 0;
    while i < verse_chars.len() {
        // Look for definite article "ال" at the START of a word
        if verse_chars[i] == 'ا' && i + 1 < verse_chars.len() {
            check_lam_al_tarif(
                verse_chars,
                index,
                i,
                matches,
                &IZHAR_QAMARI_LETTERS,
                &IDGHAM_SHAMSI_LETTERS,
                style,
            );
        }
        i += 1;
    }
}

fn determine_rule_for_lam_al(
    izhar_qamari_letters: &[char],
    idgham_shamsi_letters: &[char],
    following_letter: char,
) -> TajweedRuleType {
    // 1. Idgham Shamsi (الإدغام الشمسي) - Sun letters
    if idgham_shamsi_letters.contains(&following_letter) {
        return TajweedRuleType::IdghamShamsi;
    }

    // 2. Izhar Qamari (الإظهار القمري) - Moon letters
    if izhar_qamari_letters.contains(&following_letter) {
        return TajweedRuleType::IzharQamari;
    }

    TajweedRuleType::NoRule
}

fn check_lam_al_tarif(
    verse_chars: &[char],
    index: &VerseIndex,
    i: usize,
    matches: &mut Vec<RuleMatch>,
    izhar_qamari_letters: &[char],
    idgham_shamsi_letters: &[char],
    style: RecitationStyle,
) {
    // CRITICAL FIX: Check if this alif is at a word boundary
    // The definite article "ال" should be at the START of a word, not in the middle

    // Check if there's a letter before this alif without a word boundary.
    if let Some(prev_idx) = index.prev_letter_before(i) {
        if !index.has_boundary_between(prev_idx + 1, i) {
            return; // Not at word start
        }
    }

    // Now check for lam after alif
    if let Some(next_idx) = index.next_letter_after(i) {
        if verse_chars[next_idx] == 'ل' {
            if let Some(after_lam_idx) = index.next_letter_after(next_idx) {
                let following_letter = verse_chars[after_lam_idx];
                let rule_type = determine_rule_for_lam_al(
                    izhar_qamari_letters,
                    idgham_shamsi_letters,
                    following_letter,
                );

                if rule_type != TajweedRuleType::NoRule {
                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: next_idx,
                        target_letter: 'ل',
                        following_letter: Some(following_letter),
                        rule: TajweedRule::from_type(rule_type, style),
                        context: get_context(verse_chars, i, 3),
                    });
                }
            }
        }
    }
}
