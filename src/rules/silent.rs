//! Silent letters (الحروف غير المنطوقة)
//!
//! Some letters are written but never read. The Uthmani script marks them, so
//! they can be reported exactly rather than guessed at:
//!
//! * **The small high rounded zero (U+06DF) and the small high upright
//!   rectangular zero (U+06E0)** sit on a letter that is dropped entirely — the
//!   Alif after a plural Waw (`قَالُوا۟`), the Alif of `أَنَا۠`, the Waw of
//!   `أُو۟لَٰٓئِكَ`.
//! * **A Waw or Alif acting as the seat of a superscript Alef** is likewise not
//!   read: in `ٱلصَّلَوٰةَ` and `ٱلْحَيَوٰةِ` the Waw is a relic of the older spelling
//!   and what is pronounced is the Alef written above it.
//!
//! Knowing which letters are silent matters beyond display: a silent letter can
//! neither carry a Madd nor separate a Madd letter from a following Hamza.

use crate::types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
use crate::utils::*;

/// True when the letter at `idx` carries `mark` among its diacritics.
fn has_mark(chars: &[char], idx: usize, mark: char) -> bool {
    chars[idx + 1..]
        .iter()
        .take_while(|c| is_tajweed_ignorable(**c) && !c.is_whitespace())
        .any(|c| *c == mark)
}

/// Detect the letters that are written but not pronounced.
pub fn detect_silent_letters(
    verse_chars: &[char],
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    let index = VerseIndex::new(verse_chars);
    detect_silent_letters_indexed(verse_chars, &index, matches, style);
}

pub(crate) fn detect_silent_letters_indexed(
    verse_chars: &[char],
    _index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    for (i, &ch) in verse_chars.iter().enumerate() {
        if !is_arabic_letter(ch) {
            continue;
        }

        // U+06DF marks a letter that is never read. U+06E0 marks one that is
        // dropped only while reading on and *is* pronounced at a stop
        // (أَنَا۠), so it is not reported as silent.
        let marked_silent = has_mark(verse_chars, i, '\u{06DF}');
        // A Waw or Alif carrying a superscript Alef is only its seat: the Alef
        // above is what is read (ٱلصَّلَوٰةَ، ٱلْحَيَوٰةِ، ٱلرِّبَوٰا۟).
        let is_seat = matches!(ch, 'و' | 'ا')
            && verse_chars.get(i + 1) == Some(&'\u{0670}');

        if !marked_silent && !is_seat {
            continue;
        }

        let mut end_idx = i + 1;
        while end_idx < verse_chars.len()
            && is_tajweed_ignorable(verse_chars[end_idx])
            && !verse_chars[end_idx].is_whitespace()
            && verse_chars[end_idx] != '\u{0670}'
        {
            end_idx += 1;
        }

        matches.push(RuleMatch {
            start_index: i,
            end_index: end_idx,
            target_letter: ch,
            following_letter: None,
            rule: TajweedRule::from_type(TajweedRuleType::Silent, style),
            context: get_context(verse_chars, i, 3),
        });
    }
}
