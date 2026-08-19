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
/// 3. In Warsh: when it has fatha/damma and preceded by Kasra / Saakin Ya, unless an exception applies.
pub fn detect_tarqeeq_ra(
    verse_chars: &[char],
    current_index: usize,
) -> Option<TajweedRuleType> {
    let index = VerseIndex::new(verse_chars);
    detect_tarqeeq_ra_styled(verse_chars, &index, current_index, RecitationStyle::Hafs)
}

fn detect_tarqeeq_ra_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    current_index: usize,
) -> Option<TajweedRuleType> {
    detect_tarqeeq_ra_styled(verse_chars, index, current_index, RecitationStyle::Hafs)
}

fn detect_tarqeeq_ra_styled(
    verse_chars: &[char],
    index: &VerseIndex,
    current_index: usize,
    style: RecitationStyle,
) -> Option<TajweedRuleType> {
    if current_index >= verse_chars.len() {
        return None;
    }

    // 1. Ra with Kasra (رِ / رٍ) is always Tarqeeq in both Hafs and Warsh
    if index.has_diacritic_after_mask(current_index, DIAC_KASRA) {
        return Some(TajweedRuleType::TarqeeqRa);
    }

    // 2. Ra with Sukun (رْ)
    if index.has_sukun_after(current_index) {
        if let Some(prev_idx) = index.prev_letter_before(current_index) {
            // Preceded by Kasra in same word
            if index.has_diacritic_after_mask(prev_idx, DIAC_KASRA)
                && !index.has_boundary_between(prev_idx + 1, current_index)
            {
                // Check if followed by an Isti'la letter with Fatha/Damma in same word (e.g. قرطاس, فرقة, مرصاد, إرصادا)
                if let Some(next_idx) = index.next_letter_after(current_index) {
                    let next_ch = verse_chars[next_idx];
                    if matches!(next_ch, 'ص' | 'ض' | 'ط' | 'ظ' | 'ق' | 'غ' | 'خ')
                        && !index.has_boundary_between(current_index + 1, next_idx)
                        && index.has_diacritic_after_mask(next_idx, DIAC_FATHA | DIAC_DAMMA | DIAC_TANWIN)
                    {
                        return None; // Must be Tafkhim
                    }
                }
                return Some(TajweedRuleType::TarqeeqRa);
            }
            // Preceded by Saakin Ya (e.g. خَيْرْ, قَدِيرْ at stop)
            if (verse_chars[prev_idx] == 'ي' || verse_chars[prev_idx] == '\u{06CC}')
                && (index.has_sukun_after(prev_idx) || index.diacritic_mask_at(prev_idx) == 0)
            {
                return Some(TajweedRuleType::TarqeeqRa);
            }
        }
    }

    // 3. Warsh-specific rules for Ra with Fatha or Damma (رَ / رُ / رًا / رٌ)
    if style == RecitationStyle::Warsh {
        if index.has_diacritic_after_mask(current_index, DIAC_FATHA | DIAC_DAMMA | DIAC_TANWIN) {
            // Check for mandatory Tafkhim exceptions in Warsh
            if is_warsh_ra_tafkhim_exception(verse_chars, index, current_index) {
                return None; // Exception -> Tafkhim
            }

            if let Some(prev_idx) = index.prev_letter_before(current_index) {
                if !index.has_boundary_between(prev_idx + 1, current_index) {
                    // Case A: Preceded by Saakin Ya (e.g. خَيْرًا, طَيْرًا, نَذِيرٌ, خَبِيرًا, بَصِيرٌ, غَيْرَ)
                    if verse_chars[prev_idx] == 'ي' || verse_chars[prev_idx] == '\u{06CC}' {
                        if index.has_sukun_after(prev_idx) || index.diacritic_mask_at(prev_idx) == 0 {
                            return Some(TajweedRuleType::TarqeeqRa);
                        }
                    }

                    // Case B: Preceded by direct original Kasra (e.g. نَاصِرًا, قَادِرُونَ, سِرَاجًا)
                    if index.has_diacritic_after_mask(prev_idx, DIAC_KASRA) {
                        return Some(TajweedRuleType::TarqeeqRa);
                    }

                    // Case C: Preceded by Kasra separated by a single non-Isti'la Saakin letter (e.g. عِبْرَةً, سِحْرٌ, مِحْرَاب, ذِكْرَا, إِكْرَاه)
                    let prev_ch = verse_chars[prev_idx];
                    let is_sakin_prev = index.has_sukun_after(prev_idx) || index.diacritic_mask_at(prev_idx) == 0;
                    if is_sakin_prev && !matches!(prev_ch, 'ص' | 'ط' | 'ق' | 'ض' | 'ظ' | 'غ') {
                        if let Some(prev_prev_idx) = index.prev_letter_before(prev_idx) {
                            if !index.has_boundary_between(prev_prev_idx + 1, current_index) {
                                if index.has_diacritic_after_mask(prev_prev_idx, DIAC_KASRA) {
                                    return Some(TajweedRuleType::TarqeeqRa);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// Check if a Ra in Warsh falls under one of the mandatory Tafkhim exceptions:
/// 1. Isti'la separator: ص, ط, ق (e.g. مِصْرًا, قِطْرًا, وِقْرًا)
/// 2. Followed by an Isti'la letter with Fatha/Damma in same word: صراط, فراق, إعراض, إشراق
/// 3. Repeated Ra in same word: ضراراً, فراراً, مراراً, إسراراً, مدراراً
/// 4. Foreign names: إبراهيم, إسرائيل, إسرائيل, عمران
fn is_warsh_ra_tafkhim_exception(
    verse_chars: &[char],
    index: &VerseIndex,
    current_index: usize,
) -> bool {
    // 1. Check if separated from Kasra by an Isti'la letter (ص, ط, ق)
    if let Some(prev_idx) = index.prev_letter_before(current_index) {
        if !index.has_boundary_between(prev_idx + 1, current_index) {
            let prev_ch = verse_chars[prev_idx];
            if matches!(prev_ch, 'ص' | 'ط' | 'ق') {
                return true; // مِصْراً, قِطْراً, وِقْراً -> Tafkhim!
            }
        }
    }

    // 2. Check if followed by an Isti'la letter with Fatha/Damma in the same word (صِرَاط, فِرَاق, إِعْرَاض, إِشْرَاق)
    if let Some(next_idx) = index.next_letter_after(current_index) {
        if !index.has_boundary_between(current_index + 1, next_idx) {
            let next_ch = verse_chars[next_idx];
            if matches!(next_ch, 'ص' | 'ض' | 'ط' | 'ظ' | 'ق' | 'غ') {
                return true;
            }
        }
    }

    // 3. Check for repeated Ra in the same word (ضِرَارًا, فِرَارًا, مِرَارًا, إِسْرَارًا, مِدْرَارًا)
    if let Some(next_idx) = index.next_letter_after(current_index) {
        if !index.has_boundary_between(current_index + 1, next_idx) && verse_chars[next_idx] == 'ر' {
            return true;
        }
        if let Some(after_next_idx) = index.next_letter_after(next_idx) {
            if !index.has_boundary_between(current_index + 1, after_next_idx) && verse_chars[after_next_idx] == 'ر' {
                return true;
            }
        }
    }
    if let Some(prev_idx) = index.prev_letter_before(current_index) {
        if !index.has_boundary_between(prev_idx + 1, current_index) && verse_chars[prev_idx] == 'ر' {
            return true;
        }
    }

    // 4. Check for foreign names (إبراهيم, إسرائيل, عمران, إرم)
    let word = get_current_word(verse_chars, index, current_index);
    let foreign_stems = [
        "إبرهم", "إبرهيم", "إبراهيم",
        "إسرءيل", "إسراءيل", "إسرائيل", "إسرافيل",
        "عمرن", "عمران",
        "إرم",
    ];
    for stem in foreign_stems {
        if word.contains(stem) {
            return true;
        }
    }

    false
}

fn get_current_word(verse_chars: &[char], index: &VerseIndex, idx: usize) -> String {
    let mut start = idx;
    while let Some(prev) = index.prev_letter_before(start) {
        if index.has_boundary_between(prev + 1, start) {
            break;
        }
        start = prev;
    }
    let mut end = idx;
    while let Some(next) = index.next_letter_after(end) {
        if index.has_boundary_between(end + 1, next) {
            break;
        }
        end = next;
    }
    let word_slice = &verse_chars[start..=end.min(verse_chars.len() - 1)];
    word_slice.iter().filter(|&&c| is_arabic_letter(c)).collect()
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

            // Check for Tarqeeq Ra (applicable in both styles, with full Warsh rules and exceptions)
            if let Some(tarqeeq_type) = detect_tarqeeq_ra_styled(verse_chars, index, i, style) {
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

pub(crate) fn detect_istiila_rules_indexed(
    verse_chars: &[char],
    _index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    const ISTIILA_LETTERS: [char; 7] = ['خ', 'ص', 'ض', 'غ', 'ط', 'ق', 'ظ'];
    let mut i = 0;
    while i < verse_chars.len() {
        let ch = verse_chars[i];
        if ISTIILA_LETTERS.contains(&ch) {
            let mut end_idx = i + 1;
            while end_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[end_idx]) {
                end_idx += 1;
            }
            matches.push(RuleMatch {
                start_index: i,
                end_index: end_idx,
                target_letter: ch,
                following_letter: None,
                rule: TajweedRule::from_type(TajweedRuleType::TafkhimHuruf, style),
                context: get_context(verse_chars, i, 3),
            });
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
