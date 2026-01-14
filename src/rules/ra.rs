//! Ra emphasis and Allah name emphasis rule detection
//!
//! This module handles detection of rules related to:
//! - Tafkhim Ra (تفخيم الراء) - Emphasis/heaviness of Ra
//! - Tarqeeq Ra (ترقيق الراء) - Thinning/lightness of Ra (Warsh specific)
//! - Tafkhim Lafz Al-Jalalah (تفخيم لفظ الجلالة) - Emphasis of Allah's name

use crate::types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
use crate::utils::*;

/// Detect Tafkhim Ra (تفخيم الراء)
pub fn detect_tafkhim_ra(verse_chars: &[char], current_index: usize) -> Option<TajweedRuleType> {
    if let Some(vowel) = get_preceding_vowel(verse_chars, current_index) {
        match vowel {
            '\u{064E}' | '\u{064C}' => return Some(TajweedRuleType::TafkhimRa),
            '\u{0652}' => {
                if current_index >= 2 {
                    let mut back_idx = current_index - 1;
                    while back_idx > 0 && is_tajweed_ignorable(verse_chars[back_idx]) {
                        back_idx -= 1;
                    }
                    if back_idx < current_index {
                        if let Some(prev_vowel) = get_preceding_vowel(verse_chars, back_idx) {
                            if matches!(prev_vowel, '\u{064E}' | '\u{064C}') {
                                return Some(TajweedRuleType::TafkhimRa);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    None
}

/// Detect Tarqeeq Ra (ترقيق الراء)
pub fn detect_tarqeeq_ra(verse_chars: &[char], current_index: usize) -> Option<TajweedRuleType> {
    if let Some(vowel) = get_preceding_vowel(verse_chars, current_index) {
        match vowel {
            '\u{0650}' => return Some(TajweedRuleType::TarqeeqRa),
            '\u{0652}' => {
                if current_index >= 2 {
                    let mut back_idx = current_index - 1;
                    while back_idx > 0 && is_tajweed_ignorable(verse_chars[back_idx]) {
                        back_idx -= 1;
                    }
                    if back_idx < current_index {
                        if let Some(prev_vowel) = get_preceding_vowel(verse_chars, back_idx) {
                            if prev_vowel == '\u{0650}' {
                                return Some(TajweedRuleType::TarqeeqRa);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    None
}

/// Detect Tafkhim Lafz Al-Jalala (تفخيم لفظ الجلالة)
pub fn detect_tafkhim_lafuljalala(verse_chars: &[char], current_index: usize) -> bool {
    if current_index + 3 >= verse_chars.len() {
        return false;
    }

    if verse_chars[current_index] != 'ا' {
        return false;
    }

    let mut check_idx = current_index + 1;
    while check_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[check_idx]) {
        check_idx += 1;
    }

    if check_idx >= verse_chars.len() || verse_chars[check_idx] != 'ل' {
        return false;
    }

    check_idx += 1;
    while check_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[check_idx]) {
        check_idx += 1;
    }

    if check_idx >= verse_chars.len() {
        return false;
    }

    if verse_chars[check_idx] == 'ل' {
        check_idx += 1;
        while check_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[check_idx]) {
            check_idx += 1;
        }
        check_idx < verse_chars.len() && verse_chars[check_idx] == 'ه'
    } else {
        verse_chars[check_idx] == 'ه'
    }
}

/// Detect Ra emphasis and Allah name rules in verse
pub fn detect_ra_rules(verse_chars: &[char], matches: &mut Vec<RuleMatch>, style: RecitationStyle) {
    let mut i = 0;
    while i < verse_chars.len() {
        if verse_chars[i] == 'ر' {
            // Check for Tarqeeq Ra first (higher priority for Warsh)
            if style == RecitationStyle::Warsh {
                if let Some(tarqeeq_type) = detect_tarqeeq_ra(&verse_chars, i) {
                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: i,
                        target_letter: verse_chars[i],
                        following_letter: None,
                        rule: TajweedRule::from_type(tarqeeq_type, style),
                        context: get_context(&verse_chars, i, 3),
                    });
                    i += 1;
                    continue;
                }
            }

            // Check for Tafkhim Ra
            if let Some(tafkhim_type) = detect_tafkhim_ra(&verse_chars, i) {
                matches.push(RuleMatch {
                    start_index: i,
                    end_index: i,
                    target_letter: verse_chars[i],
                    following_letter: None,
                    rule: TajweedRule::from_type(tafkhim_type, style),
                    context: get_context(&verse_chars, i, 3),
                });
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
    let mut i = 0;
    while i < verse_chars.len() {
        if verse_chars[i] == 'ا' {
            if detect_tafkhim_lafuljalala(&verse_chars, i) {
                matches.push(RuleMatch {
                    start_index: i,
                    end_index: i,
                    target_letter: verse_chars[i],
                    following_letter: None,
                    rule: TajweedRule::from_type(TajweedRuleType::TafkhimLafuljalala, style),
                    context: get_context(&verse_chars, i, 3),
                });
                i += 3;
                continue;
            }
        }
        i += 1;
    }
}
