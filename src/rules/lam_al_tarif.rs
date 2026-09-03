//! Lam Al-Ta'rif (definite article) rule detection

use crate::rules::letters;
use crate::types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
use crate::utils::*;

/// Detect Lam Al-Ta'rif rules in verse
pub fn detect_lam_al_tarif_rules(
    verse_chars: &[char],
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    let index = VerseIndex::new(verse_chars);
    detect_lam_al_tarif_rules_indexed(verse_chars, &index, matches, style);
}

pub(crate) fn detect_lam_al_tarif_rules_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    for lam_idx in 0..verse_chars.len() {
        if verse_chars[lam_idx] != 'ل' {
            continue;
        }
        if let Some(start) = article_start(verse_chars, index, lam_idx) {
            check_lam_al_tarif(
                verse_chars,
                index,
                start,
                lam_idx,
                matches,
                letters::IZHAR_QAMARI,
                letters::IDGHAM_SHAMSI,
                style,
            );
        }
    }
}

/// The letters that may be prefixed to the definite article (وَ، فَ، بِ، كَ، لِ).
const ARTICLE_PREFIXES: [char; 5] = ['و', 'ف', 'ب', 'ك', 'ل'];

/// True when the letter at `idx` opens its word, allowing for one prefix
/// letter before it.
fn opens_word(verse_chars: &[char], index: &VerseIndex, idx: usize, allow_prefix: bool) -> bool {
    match index.prev_letter_before(idx) {
        None => true,
        Some(prev) => {
            index.has_boundary_between(prev + 1, idx)
                || (allow_prefix && ARTICLE_PREFIXES.contains(&verse_chars[prev]))
        }
    }
}

/// If the Lam at `lam_idx` is the Lam of the definite article, return the
/// index where the article starts — its Alif, or the Lam prefix that swallowed
/// it.
///
/// Two spellings occur:
///
/// * **الـ** — Alif (ا, U+0627) or Alif Wasla (ٱ, U+0671) then the Lam, either
///   opening the word or after one of وَ فَ بِ كَ لِ (وَٱلضُّحَىٰ).
/// * **لِلـ** — after the Lam prefix the article's Alif is not written at all,
///   so the word opens with a kasra-bearing Lam followed by the article's Lam
///   (لِلنَّاسِ = لِ + النَّاس، لِّلْمُتَّقِينَ).
pub(crate) fn article_start(
    verse_chars: &[char],
    index: &VerseIndex,
    lam_idx: usize,
) -> Option<usize> {
    if verse_chars.get(lam_idx) != Some(&'ل') {
        return None;
    }
    let prev = index.prev_letter_before(lam_idx)?;
    if index.has_boundary_between(prev + 1, lam_idx) {
        return None;
    }
    match verse_chars[prev] {
        'ا' | '\u{0671}' if opens_word(verse_chars, index, prev, true) => Some(prev),
        'ل' if index.has_diacritic_after_mask(prev, DIAC_KASRA)
            && opens_word(verse_chars, index, prev, true) =>
        {
            Some(prev)
        }
        _ => None,
    }
}

fn determine_rule_for_lam_al(
    izhar_qamari_letters: &[char],
    idgham_shamsi_letters: &[char],
    following_letter: char,
) -> TajweedRuleType {
    // 1. Idgham Shamsi (الإدغام الشمسي) - Sun letters
    if idgham_shamsi_letters.contains(&following_letter) {
        return TajweedRuleType::IdghamShamsi;
    }

    // 2. Izhar Qamari (الإظهار القمري) - Moon letters
    if izhar_qamari_letters.contains(&following_letter) {
        return TajweedRuleType::IzharQamari;
    }

    TajweedRuleType::NoRule
}

#[allow(clippy::too_many_arguments)]
fn check_lam_al_tarif(
    verse_chars: &[char],
    index: &VerseIndex,
    start: usize,
    lam_idx: usize,
    matches: &mut Vec<RuleMatch>,
    izhar_qamari_letters: &[char],
    idgham_shamsi_letters: &[char],
    style: RecitationStyle,
) {
    let Some(after_lam_idx) = index.next_letter_after(lam_idx) else {
        return;
    };
    if index.has_boundary_between(lam_idx + 1, after_lam_idx) {
        return;
    }
    let following_letter = verse_chars[after_lam_idx];
    let rule_type = determine_rule_for_lam_al(
        izhar_qamari_letters,
        idgham_shamsi_letters,
        following_letter,
    );
    if rule_type == TajweedRuleType::NoRule {
        return;
    }

    let mut end_idx = lam_idx + 1;
    while end_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[end_idx]) {
        end_idx += 1;
    }
    matches.push(RuleMatch {
        start_index: start,
        end_index: end_idx,
        target_letter: 'ل',
        following_letter: Some(following_letter),
        rule: TajweedRule::from_type(rule_type, style),
        context: get_context(verse_chars, start, 4),
    });
}
