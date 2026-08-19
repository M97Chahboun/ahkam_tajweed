//! Lam Al-Ta'rif (definite article) rule detection

use crate::rules::letters;
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
    let mut i = 0;
    while i < verse_chars.len() {
        let ch = verse_chars[i];
        // Definite article "ال" can start with regular Alif 'ا' (U+0627) or Hamzat Wasl 'ٱ' (U+0671)
        if (ch == 'ا' || ch == '\u{0671}') && i + 1 < verse_chars.len() {
            check_lam_al_tarif(
                verse_chars,
                index,
                i,
                matches,
                letters::IZHAR_QAMARI,
                letters::IDGHAM_SHAMSI,
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
    // Definite article "ال" can be at the start of a word, or preceded by prefix letters 'و', 'ف', 'ب', 'ك', 'ل'
    if let Some(prev_idx) = index.prev_letter_before(i) {
        if !index.has_boundary_between(prev_idx + 1, i) {
            let prev_char = verse_chars[prev_idx];
            if !matches!(prev_char, 'و' | 'ف' | 'ب' | 'ك' | 'ل') {
                return; // Inside a word root, not an article
            }
        }
    }

    // Now check for lam after alif (must be within the same word)
    if let Some(next_idx) = index.next_letter_after(i) {
        if verse_chars[next_idx] == 'ل' && !index.has_boundary_between(i + 1, next_idx) {
            if let Some(after_lam_idx) = index.next_letter_after(next_idx) {
                if !index.has_boundary_between(next_idx + 1, after_lam_idx) {
                    let following_letter = verse_chars[after_lam_idx];
                    let rule_type = determine_rule_for_lam_al(
                        izhar_qamari_letters,
                        idgham_shamsi_letters,
                        following_letter,
                    );

                    if rule_type != TajweedRuleType::NoRule {
                        let mut end_idx = next_idx + 1;
                        while end_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[end_idx]) {
                            end_idx += 1;
                        }
                        matches.push(RuleMatch {
                            start_index: i,
                            end_index: end_idx,
                            target_letter: verse_chars[next_idx],
                            following_letter: Some(following_letter),
                            rule: TajweedRule::from_type(rule_type, style),
                            context: get_context(verse_chars, i, 4),
                        });
                    }
                }
            }
        }
    }
}
