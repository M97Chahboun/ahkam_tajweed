//! Madd (vowel prolongation) rule detection

use crate::types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
use crate::utils::*;

/// Detect Madd rules in verse
pub fn detect_madd_rules(
    verse_chars: &[char],
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    let index = VerseIndex::new(verse_chars);
    detect_madd_rules_indexed(verse_chars, &index, matches, style);
}

pub(crate) fn detect_madd_rules_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    const MADD_LETTERS: [char; 4] = ['ا', 'و', 'ي', '\u{06CC}'];

    let mut i = 0;
    while i < verse_chars.len() {
        let current_char = verse_chars[i];

        if MADD_LETTERS.contains(&current_char) || current_char == 'آ' {
            let vowel = index.preceding_vowel(i);
            let has_basic_madd = if current_char == 'آ' {
                true // Alif Madd is always considered valid for madd
            } else {
                match (current_char, vowel) {
                    ('ا', Some('\u{064E}')) => true, // Alif needs Fatha for basic madd
                    ('و', Some('\u{064F}')) => true, // Waw needs Damma for basic madd
                    ('ي' | '\u{06CC}', Some('\u{0650}')) => true, // Ya needs Kasra for basic madd
                    _ => false,
                }
            };

            let has_lin_candidate = matches!(current_char, 'و' | 'ي' | '\u{06CC}') && vowel == Some('\u{064E}');

            if has_basic_madd || has_lin_candidate || current_char == 'آ' {
                if let Some(madd_type) = detect_madd(current_char, verse_chars, index, i) {
                    // Calculate end index to include diacritics
                    let mut end_idx = i + 1;
                    while end_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[end_idx]) {
                        end_idx += 1;
                    }

                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: end_idx,
                        target_letter: current_char,
                        following_letter: None,
                        rule: TajweedRule::from_type(madd_type, style),
                        context: get_context(&verse_chars, i, 3),
                    });
                }
            }
        }

        i += 1;
    }
}

fn detect_madd(
    madd_letter: char,
    verse_chars: &[char],
    index: &VerseIndex,
    current_index: usize,
) -> Option<TajweedRuleType> {
    let preceding_vowel = index.preceding_vowel(current_index);

    // If Waw/Ya carries a Fatha, only Madd Lin is possible.
    if matches!(madd_letter, 'و' | 'ي' | '\u{06CC}') && preceding_vowel == Some('\u{064E}') {
        // Check for Madd Lin (sukun on the madd letter or the following letter)
        if index.has_sukun_after(current_index) {
            return Some(TajweedRuleType::MaddLin);
        }

        if let Some(next_idx) = index.next_letter_after(current_index) {
            if index.has_sukun_after(next_idx) {
                return Some(TajweedRuleType::MaddLin);
            }
        }

        return None;
    }

    // 1. Check for Madd Lazim: madd letter preceded by letter with shadda (like in "أَمَّا")
    if let Some(prev_idx) = index.prev_letter_before(current_index) {
        // Check if the letter at prev_idx has shadda following it
        if index.has_shadda_after(prev_idx) {
            return Some(TajweedRuleType::MaddLazim);
        }
    }

    // 2. Check for Madd Badal: hamza BEFORE madd letter (same word)
    if let Some(prev_idx) = index.prev_letter_before(current_index) {
        if is_hamza(verse_chars[prev_idx]) {
            if !index.has_boundary_between(prev_idx + 1, current_index) {
                return Some(TajweedRuleType::MaddBadal);
            }
        }
    }

    // 3. Check for Madd Muttasil/Munfasil: madd letter followed by hamza
    if let Some(next_idx) = index.next_letter_after(current_index) {
        if is_hamza(verse_chars[next_idx]) {
            // Check if there's a word boundary between madd letter and hamza
            let has_word_boundary = index.has_boundary_between(current_index + 1, next_idx);

            return if has_word_boundary {
                Some(TajweedRuleType::MaddMunfasil)
            } else {
                Some(TajweedRuleType::MaddMuttasil)
            };
        }
    }

    // 4. Check for Madd Lazim: madd letter followed by letter with shadda
    if let Some(next_idx) = index.next_letter_after(current_index) {
        if index.has_shadda_after(next_idx) {
            return Some(TajweedRuleType::MaddLazim);
        }
    }

    // 5. Madd Lin already handled above for Waw/Ya with Fatha

    // 6. Madd Arid li-Sukun: Madd letter followed by a letter with explicit Sukun,
    //    or followed by the final letter of a word that has a Waqf / verse end sign after it.
    //    Source: quranica.com — "Only occurs at Waqf; if continuing, reverts to MaddTabeei."
    if let Some(next_idx) = index.next_letter_after(current_index) {
        if index.has_sukun_after(next_idx) {
            return Some(TajweedRuleType::MaddArid);
        }
        let has_waqf_or_verse_end = verse_chars[next_idx..].iter().any(|&c| {
            matches!(
                c,
                '\u{06D5}'..='\u{06DC}'
                | '\u{06DD}'..='\u{06DF}'
                | '\u{FD3E}' | '\u{FD3F}'
            )
        });
        if has_waqf_or_verse_end && index.is_word_end(next_idx) {
            return Some(TajweedRuleType::MaddArid);
        }
    }

    // 7. Default: Natural madd (Tabee'i) - if conditions are met
    // Natural madd occurs when madd letter has its corresponding vowel and is not followed by hamza or shadda
    Some(TajweedRuleType::MaddTabeei)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn has_rule(matches: &[RuleMatch], rule: TajweedRuleType) -> bool {
        matches.iter().any(|m| m.rule.rule_type == rule)
    }

    #[test]
    fn test_madd_waw_with_damma() {
        let chars: Vec<char> = "قُولُ".chars().collect();
        let index = VerseIndex::new(&chars);
        let mut matches = Vec::new();
        detect_madd_rules_indexed(&chars, &index, &mut matches, RecitationStyle::Hafs);
        assert!(has_rule(&matches, TajweedRuleType::MaddTabeei));
    }

    #[test]
    fn test_madd_muttasil_and_munfasil() {
        let mut matches = Vec::new();

        let mut chars: Vec<char> = "جَاء".chars().collect();
        let index = VerseIndex::new(&chars);
        detect_madd_rules_indexed(&chars, &index, &mut matches, RecitationStyle::Hafs);
        assert!(has_rule(&matches, TajweedRuleType::MaddMuttasil));

        matches.clear();
        chars = "قَا أ".chars().collect();
        let index = VerseIndex::new(&chars);
        detect_madd_rules_indexed(&chars, &index, &mut matches, RecitationStyle::Hafs);
        assert!(has_rule(&matches, TajweedRuleType::MaddMunfasil));
    }
}
