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

        // An Alif followed immediately by Lam in the same token (definite article "الـ") is not Madd
        if current_char == 'ا' || current_char == '\u{0671}' {
            if let Some(next_idx) = index.next_letter_after(i) {
                if verse_chars[next_idx] == 'ل' && !index.has_boundary_between(i + 1, next_idx) {
                    i += 1;
                    continue;
                }
            }
        }

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
                        context: get_context(verse_chars, i, 3),
                    });
                }
            }
        }

        i += 1;
    }

    // Detect Madd Silah (هاء الضمير / هاء الكناية)
    detect_silah_rules_indexed(verse_chars, index, matches, style);
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

    // 1. Check for Madd Lazim:
    // (a) Madd letter followed by letter with shadda (e.g. الضالين, دابة, الحاقة)
    if let Some(next_idx) = index.next_letter_after(current_index) {
        if index.has_shadda_after(next_idx) {
            return Some(TajweedRuleType::MaddLazim);
        }
    }
    // (b) Alif preceded by letter with shadda (e.g. أَمَّا)
    if madd_letter == 'ا' {
        if let Some(prev_idx) = index.prev_letter_before(current_index) {
            if index.has_shadda_after(prev_idx) {
                return Some(TajweedRuleType::MaddLazim);
            }
        }
    }

    // 2. Check for Madd Badal: hamza BEFORE madd letter (same word)
    if let Some(prev_idx) = index.prev_letter_before(current_index) {
        if is_hamza(verse_chars[prev_idx])
            && !index.has_boundary_between(prev_idx + 1, current_index) {
                return Some(TajweedRuleType::MaddBadal);
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

    // 7. Check if Madd letter is dropped in continuous reading (Wasl) before a Saakin letter / Hamzat Wasl
    // (حذف حرف المد لفظاً عند التقاء الساكنين في الوصل — مثل: في الجحيم، قالوا ابنوا، إذا الشمس)
    if is_madd_dropped_before_sakin(verse_chars, index, current_index) {
        return None;
    }

    // 8. Default: Natural madd (Tabee'i) - if conditions are met
    // Natural madd occurs when madd letter has its corresponding vowel and is not followed by hamza or shadda
    Some(TajweedRuleType::MaddTabeei)
}

fn is_madd_dropped_before_sakin(
    verse_chars: &[char],
    index: &VerseIndex,
    current_index: usize,
) -> bool {
    if let Some(next_idx) = index.next_letter_after(current_index) {
        let is_same_word = !index.has_boundary_between(current_index + 1, next_idx);
        if is_same_word {
            // E.g. قَالُوا ٱبْنُوا — Waw is followed by silent trailing Alif in same word
            if verse_chars[next_idx] == 'ا' || verse_chars[next_idx] == 'ى' {
                if let Some(after_alif_idx) = index.next_letter_after(next_idx) {
                    if index.has_boundary_between(next_idx + 1, after_alif_idx) {
                        return is_word_starting_with_wasl_or_sakin(verse_chars, index, after_alif_idx);
                    }
                }
            }
            return false;
        } else {
            // Madd letter is directly at the word end (e.g. فِي ٱلْجَحِيمِ, إِذَا ٱلشَّمْسُ, يَمْحُ ٱللَّهُ)
            return is_word_starting_with_wasl_or_sakin(verse_chars, index, next_idx);
        }
    }
    false
}

fn is_word_starting_with_wasl_or_sakin(
    verse_chars: &[char],
    index: &VerseIndex,
    first_letter_idx: usize,
) -> bool {
    let first_ch = verse_chars[first_letter_idx];
    // 1. Hamzat Wasl ٱ (U+0671)
    if first_ch == '\u{0671}' {
        return true;
    }
    // 2. Regular Alif without vowels followed by Lam or Saakin/Shadda letter (e.g. الجحيم, ابنوا, اتقوا)
    if first_ch == 'ا'
        && !index.has_diacritic_after_mask(first_letter_idx, DIAC_FATHA | DIAC_DAMMA | DIAC_KASRA) {
            if let Some(second_idx) = index.next_letter_after(first_letter_idx) {
                if !index.has_boundary_between(first_letter_idx + 1, second_idx)
                    && (verse_chars[second_idx] == 'ل'
                        || index.has_sukun_after(second_idx)
                        || index.has_shadda_after(second_idx))
                    {
                        return true;
                    }
            }
        }
    // 3. Direct Saakin letter at word start
    if index.has_sukun_after(first_letter_idx) {
        return true;
    }
    false
}

// ─────────────────────────────────────────────────────────────────────────────
// Madd Silah (صلة هاء الكناية / هاء الضمير)
// Haa Al-Kinayah (ـهُ / ـهِ) at word end between two voweled letters:
// - Followed by Hamza: Madd Silah Kubra (صلة كبرى — treated as Madd Munfasil)
// - Followed by non-Hamza: Madd Silah Sughra (صلة صغرى — 2 harakaat)
// - If written with small waw ۥ (U+06E5) or small ya ۦ (U+06E6), match on the mark.
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) fn detect_silah_rules_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    let mut i = 0;
    while i < verse_chars.len() {
        let ch = verse_chars[i];

        // 1. Explicit Small Waw ۥ (U+06E5) or Small Ya ۦ (U+06E6)
        if ch == '\u{06E5}' || ch == '\u{06E6}' {
            let is_kubra = if let Some(next_idx) = index.next_letter_after(i) {
                is_hamza(verse_chars[next_idx])
            } else {
                false
            };
            let rule_type = if is_kubra {
                TajweedRuleType::MaddMunfasil
            } else {
                TajweedRuleType::MaddSilah
            };
            matches.push(RuleMatch {
                start_index: i,
                end_index: i + 1,
                target_letter: ch,
                following_letter: None,
                rule: TajweedRule::from_type(rule_type, style),
                context: get_context(verse_chars, i, 3),
            });
        }

        // 2. Haa Al-Kinayah (ـهُ / ـهِ / ه) at word end between two voweled letters
        if ch == 'ه' || ch == 'ة' {
            if let Some(next_letter_idx) = index.next_letter_after(i) {
                if index.has_boundary_between(i + 1, next_letter_idx) {
                    if let Some(prev_letter_idx) = index.prev_letter_before(i) {
                        if !index.has_boundary_between(prev_letter_idx + 1, i) {
                            let has_prev_vowel = index.has_diacritic_after_mask(prev_letter_idx, DIAC_FATHA | DIAC_DAMMA | DIAC_KASRA);
                            let has_ha_vowel = index.has_diacritic_after_mask(i, DIAC_DAMMA | DIAC_KASRA);
                            let has_next_vowel = index.has_diacritic_after_mask(next_letter_idx, DIAC_FATHA | DIAC_DAMMA | DIAC_KASRA | DIAC_TANWIN);
                            let is_next_wasl = verse_chars[next_letter_idx] == '\u{0671}'
                                || (verse_chars[next_letter_idx] == 'ا' && !index.has_diacritic_after_mask(next_letter_idx, DIAC_FATHA | DIAC_DAMMA | DIAC_KASRA));

                            if has_prev_vowel && has_ha_vowel && has_next_vowel && !is_next_wasl
                                && !matches.iter().any(|m| (m.rule.rule_type == TajweedRuleType::MaddSilah || m.rule.rule_type == TajweedRuleType::MaddMunfasil) && m.start_index >= i && m.start_index <= i + 2) {
                                    let is_kubra = is_hamza(verse_chars[next_letter_idx]);
                                    let rule_type = if is_kubra {
                                        TajweedRuleType::MaddMunfasil
                                    } else {
                                        TajweedRuleType::MaddSilah
                                    };
                                    let mut end_idx = i + 1;
                                    while end_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[end_idx]) {
                                        end_idx += 1;
                                    }
                                    matches.push(RuleMatch {
                                        start_index: i,
                                        end_index: end_idx,
                                        target_letter: 'ه',
                                        following_letter: Some(verse_chars[next_letter_idx]),
                                        rule: TajweedRule::from_type(rule_type, style),
                                        context: get_context(verse_chars, i, 3),
                                    });
                                }
                        }
                    }
                }
            }
        }

        i += 1;
    }
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
