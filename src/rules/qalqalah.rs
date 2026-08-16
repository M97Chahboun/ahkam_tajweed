//! Qalqalah (bouncing) rule detection refined

use crate::{
    types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType},
    utils::{get_context, is_punctuation, is_tajweed_ignorable, VerseIndex},
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
                let mut end_idx = i + 1;
                while end_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[end_idx]) {
                    end_idx += 1;
                }
                matches.push(RuleMatch {
                    start_index: i,
                    end_index: end_idx,
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
    // 1. Explicit Sukun (Sughra or Kubra depending on position)
    if index.has_sukun_after(idx) {
        if index.is_word_end(idx) || is_verse_end(verse_chars, idx + 1) {
            // Check if the letter also has a Shadda before/on the letter
            let has_shadda = idx > 0 && verse_chars[idx.saturating_sub(1)] == '\u{0651}';
            if has_shadda {
                return Some(TajweedRuleType::QalqalahAkbar);
            }
            return Some(TajweedRuleType::QalqalahKubra);
        }
        return Some(TajweedRuleType::QalqalahSughra);
    }

    // 2. Implicit Sukun due to stopping (Waqf)
    // If it's the end of the verse, any Harakah (Fatha/Damma/Kasra) becomes a Sukun
    if is_verse_end(verse_chars, idx) {
        // Qalqalah Akbar: letter has Shadda and is at Waqf (strongest echo)
        // Shadda appears as U+0651 after the letter
        let has_shadda = verse_chars[idx + 1..].iter().any(|&c| c == '\u{0651}');
        if has_shadda {
            return Some(TajweedRuleType::QalqalahAkbar);
        }
        return Some(TajweedRuleType::QalqalahKubra);
    }

    None
}

fn is_verse_end(chars: &[char], idx: usize) -> bool {
    idx + 1 == chars.len() || chars[idx + 1..].iter().all(|&c| is_tajweed_ignorable(c) || is_punctuation(c))
}
