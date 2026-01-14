//! Qalqalah (bouncing) rule detection

use crate::types::{RuleMatch, TajweedRule, TajweedRuleType, RecitationStyle};
use crate::utils::*;

/// Detect Qalqalah rules in verse
pub fn detect_qalqalah_rules(
    verse_chars: &[char],
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    const QALQALAH_LETTERS: [char; 5] = ['ق', 'ط', 'ب', 'ج', 'د'];

    let mut i = 0;
    while i < verse_chars.len() {
        if QALQALAH_LETTERS.contains(&verse_chars[i]) {
            if let Some(qalqalah_type) = detect_qalqalah(&verse_chars, i) {
                matches.push(RuleMatch {
                    start_index: i,
                    end_index: i,
                    target_letter: verse_chars[i],
                    following_letter: None,
                    rule: TajweedRule::from_type(qalqalah_type, style),
                    context: get_context(&verse_chars, i, 3),
                });
            }
        }
        i += 1;
    }
}

fn detect_qalqalah(verse_chars: &[char], current_index: usize) -> Option<TajweedRuleType> {
    let mut has_sukun = false;
    let mut sukun_idx = current_index + 1;

    while sukun_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[sukun_idx]) {
        if is_sukun(verse_chars[sukun_idx]) {
            has_sukun = true;
            break;
        }
        sukun_idx += 1;
    }

    if !has_sukun {
        return None;
    }

    if is_word_end(verse_chars, current_index) {
        Some(TajweedRuleType::QalqalahKubra)
    } else {
        Some(TajweedRuleType::QalqalahSughra)
    }
}
