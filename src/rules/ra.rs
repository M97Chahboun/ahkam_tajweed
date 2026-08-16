//! Ra emphasis and Allah name emphasis rule detection
//!
//! This module handles detection of rules related to:
//! - Tafkhim Ra (تفخيم الراء) - Emphasis/heaviness of Ra
//! - Tarqeeq Ra (ترقيق الراء) - Thinning/lightness of Ra (Warsh specific)
//! - Tafkhim Lafz Al-Jalalah (تفخيم لفظ الجلالة) - Emphasis of Allah's name

use crate::types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
use crate::utils::*;

/// Detect Tafkhim Ra (تفخيم الراء)
/// Ra is emphasized (heavy) when:
/// 1. It has a fatha or damma
/// 2. It has a sukoon and the letter before has fatha or damma
/// Detect Tafkhim Ra (تفخيم الراء)
pub fn detect_tafkhim_ra(
    verse_chars: &[char],
    current_index: usize,
) -> Option<TajweedRuleType> {
    let index = VerseIndex::new(verse_chars);
    detect_tafkhim_ra_indexed(verse_chars, &index, current_index)
}

fn detect_tafkhim_ra_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    current_index: usize,
) -> Option<TajweedRuleType> {
    if current_index >= verse_chars.len() {
        return None;
    }

    // Check if current letter has fatha or damma
    if index.has_diacritic_after_mask(current_index, DIAC_FATHA | DIAC_DAMMA | DIAC_TANWIN) {
        return Some(TajweedRuleType::TafkhimRa);
    }

    // Check for sukoon case - look at preceding vowel
    if index.has_sukun_after(current_index) {
        if let Some(prev_idx) = index.prev_letter_before(current_index) {
            if index.has_diacritic_after_mask(prev_idx, DIAC_FATHA | DIAC_DAMMA | DIAC_TANWIN) {
                return Some(TajweedRuleType::TafkhimRa);
            }
        }
    }

    None
}

/// Detect Tarqeeq Ra (ترقيق الراء)
/// Ra is thinned (light) when:
/// 1. It has a kasra
/// 2. It has a sukoon and the letter before has kasra
/// Detect Tarqeeq Ra (ترقيق الراء)
pub fn detect_tarqeeq_ra(
    verse_chars: &[char],
    current_index: usize,
) -> Option<TajweedRuleType> {
    let index = VerseIndex::new(verse_chars);
    detect_tarqeeq_ra_indexed(verse_chars, &index, current_index)
}

fn detect_tarqeeq_ra_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    current_index: usize,
) -> Option<TajweedRuleType> {
    if current_index >= verse_chars.len() {
        return None;
    }

    // Check if current letter has kasra
    if index.has_diacritic_after_mask(current_index, DIAC_KASRA | DIAC_TANWIN) {
        return Some(TajweedRuleType::TarqeeqRa);
    }

    // Check for sukoon case - Ra with sukoon and preceded by kasra OR saakin Ya
    if index.has_sukun_after(current_index) {
        if let Some(prev_idx) = index.prev_letter_before(current_index) {
            // Condition A: Preceded by original Kasra
            if index.has_diacritic_after_mask(prev_idx, DIAC_KASRA | DIAC_TANWIN) {
                return Some(TajweedRuleType::TarqeeqRa);
            }
            // Condition B: Preceded by Saakin Ya (e.g. خَيْر, قَدِير)
            // Source: quranica.com — "Ra Saakin after Saakin Ya → Tarqeeq when stopping"
            if verse_chars[prev_idx] == 'ي' && index.has_sukun_after(prev_idx) {
                return Some(TajweedRuleType::TarqeeqRa);
            }
        }
    }

    None
}


/// Detect Tafkhim Lafz Al-Jalala (تفخيم لفظ الجلالة)
/// Allah's name is emphasized when preceded by fatha or damma
/// Detect Tafkhim Lafz Al-Jalala (تفخيم لفظ الجلالة)
pub fn detect_tafkhim_lafuljalala(verse_chars: &[char], current_index: usize) -> Option<usize> {
    let index = VerseIndex::new(verse_chars);
    detect_tafkhim_lafuljalala_indexed(verse_chars, &index, current_index).map(|(idx, _)| idx)
}

fn detect_tafkhim_lafuljalala_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    current_index: usize,
) -> Option<(usize, bool)> {
    // Returns Some((end_idx, is_tafkhim))
    // Returns None if not the word Allah

    // Ensure we have enough characters to check
    if current_index + 3 >= verse_chars.len() {
        return None;
    }

    // Check if we're at the start of "الله" (Allah)
    // Must start with Alif (ا)
    if verse_chars[current_index] != 'ا' {
        return None;
    }

    let mut check_idx = current_index + 1;

    // Skip any diacritics after alif (but not word boundaries)
    while check_idx < verse_chars.len() {
        let c = verse_chars[check_idx];
        if is_word_boundary(c) {
            return None;
        }
        if !is_tajweed_ignorable(c) {
            break;
        }
        check_idx += 1;
    }

    // Check for first lam (ل)
    if check_idx >= verse_chars.len() || verse_chars[check_idx] != 'ل' {
        return None;
    }
    check_idx += 1;

    // Skip diacritics after first lam (but not word boundaries)
    while check_idx < verse_chars.len() {
        let c = verse_chars[check_idx];
        if is_word_boundary(c) {
            return None;
        }
        if !is_tajweed_ignorable(c) {
            break;
        }
        check_idx += 1;
    }

    // CRITICAL FIX: Must have either:
    // 1. A second explicit lam (ل), OR
    // 2. Go back and verify there was a shadda on the first lam
    let has_second_lam = check_idx < verse_chars.len() && verse_chars[check_idx] == 'ل';

    if has_second_lam {
        // Explicit second lam found
        check_idx += 1;
        while check_idx < verse_chars.len() {
            let c = verse_chars[check_idx];
            if is_word_boundary(c) {
                return None;
            }
            if !is_tajweed_ignorable(c) {
                break;
            }
            check_idx += 1;
        }
    } else {
        // No explicit second lam - verify shadda was present on first lam
        let mut found_shadda = false;
        let first_lam_pos = current_index + 1;
        for i in (first_lam_pos + 1)..check_idx {
            if verse_chars[i] == '\u{0651}' {
                found_shadda = true;
                break;
            }
        }
        if !found_shadda {
            return None;
        }
    }

    // Check for ha (ه) at the end
    if check_idx >= verse_chars.len() || verse_chars[check_idx] != 'ه' {
        return None;
    }

    // ADDITIONAL VALIDATION: Check what comes after ha
    let mut after_ha_idx = check_idx + 1;
    let mut saw_boundary = false;
    while after_ha_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[after_ha_idx]) {
        if is_word_boundary(verse_chars[after_ha_idx]) {
            saw_boundary = true;
        }
        after_ha_idx += 1;
    }

    // If there's a letter immediately after without a boundary, it's not Allah
    if after_ha_idx < verse_chars.len()
        && is_arabic_letter(verse_chars[after_ha_idx])
        && !saw_boundary
    {
        return None;
    }

    // Determine Tafkhim vs Tarqeeq based on the preceding letter's vowel
    let is_tafkhim = if let Some(prev_idx) = index.prev_letter_before(current_index) {
        // Kasra → Tarqeeq (light)
        // Fatha / Damma / Tanwin / nothing → Tafkhim (heavy)
        !index.has_diacritic_after_mask(prev_idx, DIAC_KASRA)
    } else {
        true // At start of verse (Ibtida) → Tafkhim
    };

    Some((check_idx, is_tafkhim))
}


/// Detect Ra emphasis and Allah name rules in verse
pub fn detect_ra_rules(verse_chars: &[char], matches: &mut Vec<RuleMatch>, style: RecitationStyle) {
    let index = VerseIndex::new(verse_chars);
    detect_ra_rules_indexed(verse_chars, &index, matches, style);
}

pub(crate) fn detect_ra_rules_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    let mut i = 0;
    while i < verse_chars.len() {
        if verse_chars[i] == 'ر' {
            let mut found_rule = false;

            // Check for Tarqeeq Ra (applicable in both styles, but more common in Warsh)
            if let Some(tarqeeq_type) = detect_tarqeeq_ra_indexed(verse_chars, index, i) {
                // Calculate end index including diacritics
                let mut end_idx = i + 1;
                while end_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[end_idx]) {
                    end_idx += 1;
                }

                matches.push(RuleMatch {
                    start_index: i,
                    end_index: end_idx,
                    target_letter: verse_chars[i],
                    following_letter: None,
                    rule: TajweedRule::from_type(tarqeeq_type, style),
                    context: get_context(verse_chars, i, 3),
                });
                found_rule = true;
            }

            // Check for Tafkhim Ra (if not already found tarqeeq in Warsh)
            if !found_rule {
                if let Some(tafkhim_type) = detect_tafkhim_ra_indexed(verse_chars, index, i) {
                    // Calculate end index including diacritics
                    let mut end_idx = i + 1;
                    while end_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[end_idx])
                    {
                        end_idx += 1;
                    }

                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: end_idx,
                        target_letter: verse_chars[i],
                        following_letter: None,
                        rule: TajweedRule::from_type(tafkhim_type, style),
                        context: get_context(verse_chars, i, 3),
                    });
                }
            }
        }
        i += 1;
    }
}

/// Detect Allah name emphasis rules in verse
pub fn detect_allah_name_rules(
    verse_chars: &[char],
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    let index = VerseIndex::new(verse_chars);
    detect_allah_name_rules_indexed(verse_chars, &index, matches, style);
}

pub(crate) fn detect_allah_name_rules_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    let mut i = 0;
    while i < verse_chars.len() {
        if verse_chars[i] == 'ا' {
            if let Some((end_idx, is_tafkhim)) =
                detect_tafkhim_lafuljalala_indexed(verse_chars, index, i)
            {
                let rule_type = if is_tafkhim {
                    TajweedRuleType::TafkhimLafuljalala
                } else {
                    TajweedRuleType::TarqeeqLafuljalala
                };
                matches.push(RuleMatch {
                    start_index: i,
                    end_index: end_idx + 1, // Include the ha
                    target_letter: verse_chars[i],
                    following_letter: None,
                    rule: TajweedRule::from_type(rule_type, style),
                    context: get_context(verse_chars, i, 5),
                });
                // Skip past the entire Allah word
                i = end_idx + 1;
                continue;
            }
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tafkhim_ra_with_fatha() {
        let chars: Vec<char> = "رَ".chars().collect();
        let index = VerseIndex::new(&chars);
        assert_eq!(
            detect_tafkhim_ra_indexed(&chars, &index, 0),
            Some(TajweedRuleType::TafkhimRa)
        );
    }

    #[test]
    fn test_tafkhim_ra_with_sukun_prev_fatha() {
        let chars: Vec<char> = "بَرْ".chars().collect();
        let ra_idx = 2;
        let index = VerseIndex::new(&chars);
        assert_eq!(
            detect_tafkhim_ra_indexed(&chars, &index, ra_idx),
            Some(TajweedRuleType::TafkhimRa)
        );
    }

    #[test]
    fn test_tarqeeq_ra_with_kasra() {
        let chars: Vec<char> = "رِ".chars().collect();
        let index = VerseIndex::new(&chars);
        assert_eq!(
            detect_tarqeeq_ra_indexed(&chars, &index, 0),
            Some(TajweedRuleType::TarqeeqRa)
        );
    }

    #[test]
    fn test_tarqeeq_ra_with_sukun_prev_kasra() {
        let chars: Vec<char> = "بِرْ".chars().collect();
        let ra_idx = 2;
        let index = VerseIndex::new(&chars);
        assert_eq!(
            detect_tarqeeq_ra_indexed(&chars, &index, ra_idx),
            Some(TajweedRuleType::TarqeeqRa)
        );
    }
}
