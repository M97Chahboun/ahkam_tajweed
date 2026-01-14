//! Madd (vowel prolongation) rule detection

use crate::types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
use crate::utils::*;

/// Detect Madd rules in verse
pub fn detect_madd_rules(
    verse_chars: &[char],
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    const MADD_LETTERS: [char; 3] = ['ا', 'و', 'ي'];

    let mut i = 0;
    while i < verse_chars.len() {
        let current_char = verse_chars[i];

        if MADD_LETTERS.contains(&current_char) || current_char == 'آ' {
            let has_correct_vowel = if current_char == 'آ' {
                true
            } else if let Some(vowel) = get_preceding_vowel(&verse_chars, i) {
                match current_char {
                    'ا' => vowel == '\u{064E}',
                    'و' => vowel == '\u{064C}',
                    'ي' => vowel == '\u{0650}',
                    _ => false,
                }
            } else {
                false
            };

            if has_correct_vowel || current_char == 'آ' {
                if let Some(madd_type) = detect_madd(current_char, &verse_chars, i) {
                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: i,
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
    current_index: usize,
) -> Option<TajweedRuleType> {
    let has_following_hamza = is_following_hamza(verse_chars, current_index + 1);
    let has_following_shadda = is_following_shadda(verse_chars, current_index + 1);
    let word_end = is_word_end(verse_chars, current_index);

    if has_following_shadda {
        // Madd Lazim (المد اللازم) - 6 harakaat always
        Some(TajweedRuleType::MaddLazim)
    } else if has_following_hamza {
        // Either Muttasil or Munfasil based on word boundary
        if word_end {
            Some(TajweedRuleType::MaddMunfasil)
        } else {
            Some(TajweedRuleType::MaddMuttasil)
        }
    } else if madd_letter == 'ي' || madd_letter == 'و' {
        detect_soft_madd(madd_letter, verse_chars, current_index, word_end)
    } else {
        // Alif - check for Badal
        if verse_chars[current_index] == 'آ' {
            Some(TajweedRuleType::MaddBadal)
        } else if current_index > 0 {
            let mut back_idx = current_index - 1;
            loop {
                if !is_tajweed_ignorable(verse_chars[back_idx]) {
                    if is_hamza(verse_chars[back_idx]) {
                        return Some(TajweedRuleType::MaddBadal);
                    }
                    break;
                }
                if back_idx == 0 {
                    break;
                }
                back_idx -= 1;
            }
            if word_end {
                Some(TajweedRuleType::MaddArid)
            } else {
                Some(TajweedRuleType::MaddTabeei)
            }
        } else if word_end {
            Some(TajweedRuleType::MaddArid)
        } else {
            Some(TajweedRuleType::MaddTabeei)
        }
    }
}

fn detect_soft_madd(
    _madd_letter: char,
    verse_chars: &[char],
    current_index: usize,
    word_end: bool,
) -> Option<TajweedRuleType> {
    // Check for Madd Lin (المد اللين)
    let mut next_idx = current_index + 1;
    while next_idx < verse_chars.len()
        && is_tajweed_ignorable(verse_chars[next_idx])
        && !is_sukun(verse_chars[next_idx])
    {
        next_idx += 1;
    }

    if next_idx < verse_chars.len() && is_sukun(verse_chars[next_idx]) {
        let mut after_sukun_idx = next_idx + 1;
        while after_sukun_idx < verse_chars.len()
            && is_tajweed_ignorable(verse_chars[after_sukun_idx])
        {
            after_sukun_idx += 1;
        }

        if after_sukun_idx < verse_chars.len() {
            let next_letter = verse_chars[after_sukun_idx];
            if next_letter == 'ل' || next_letter == 'ر' {
                return Some(TajweedRuleType::MaddLin);
            }
        }
    }

    // Check for Madd Arid
    if word_end {
        Some(TajweedRuleType::MaddArid)
    } else {
        Some(TajweedRuleType::MaddTabeei)
    }
}
