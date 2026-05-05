//! Qalqalah (bouncing) rule detection refined

use crate::{
    types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType},
    utils::{get_context, is_punctuation, VerseIndex},
};

/// Detect Qalqalah rules in verse
pub fn detect_qalqalah_rules(
    verse_chars: &[char],
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    let index = VerseIndex::new(verse_chars);
    detect_qalqalah_rules_indexed(verse_chars, &index, matches, style);
}

pub(crate) fn detect_qalqalah_rules_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    const QALQALAH_LETTERS: [char; 5] = ['ق', 'ط', 'ب', 'ج', 'د'];

    for (i, &ch) in verse_chars.iter().enumerate() {
        if QALQALAH_LETTERS.contains(&ch) {
            // Check if the letter is functionally "Sakin" (either written or via Waqf)
            if let Some(qalqalah_type) = check_qalqalah_type(verse_chars, index, i) {
                matches.push(RuleMatch {
                    start_index: i,
                    end_index: i, // Diacritics are usually treated as part of the letter's span
                    target_letter: ch,
                    following_letter: verse_chars.get(i + 1).cloned(),
                    rule: TajweedRule::from_type(qalqalah_type, style),
                    context: get_context(verse_chars, i, 3),
                });
            }
        }
    }
}

fn check_qalqalah_type(
    verse_chars: &[char],
    index: &VerseIndex,
    idx: usize,
) -> Option<TajweedRuleType> {
    let next_char = verse_chars.get(idx + 1);

    // 1. Explicit Sukun (Sughra or Kubra depending on position)
    if let Some(&nc) = next_char {
        if is_sukun(nc) {
            if index.is_word_end(idx) || is_verse_end(verse_chars, idx + 1) {
                return Some(TajweedRuleType::QalqalahKubra);
            }
            return Some(TajweedRuleType::QalqalahSughra);
        }
    }

    // 2. Implicit Sukun due to stopping (Waqf)
    // If it's the end of the verse, any Harakah (Fatha/Damma/Kasra) becomes a Sukun
    if is_verse_end(verse_chars, idx) {
        // Special case: If it has a Shadda at the end, it's often called 'Akbar'
        // but usually grouped under Kubra in basic implementations.
        return Some(TajweedRuleType::QalqalahKubra);
    }

    None
}
// Mock helpers for clarity
fn is_sukun(c: char) -> bool {
    c == '\u{0652}'
} // Arabic Sukun
fn is_verse_end(chars: &[char], idx: usize) -> bool {
    // Logic to check if there are no more letters after this index in the Ayah
    idx + 1 == chars.len() || chars[idx + 1..].iter().all(|&c| is_punctuation(c))
}
