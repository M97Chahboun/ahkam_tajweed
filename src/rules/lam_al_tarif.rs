//! Lam Al-Ta'rif (definite article) rule detection

use crate::types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
use crate::utils::*;
use std::collections::HashMap;

/// Detect Lam Al-Ta'rif rules in verse
pub fn detect_lam_al_tarif_rules(
    verse_chars: &[char],
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    // Setup letter maps
    const IZHAR_QAMARI_LETTERS: [char; 14] = [
        'ا', 'ب', 'غ', 'ح', 'ج', 'ك', 'و', 'خ', 'ف', 'ع', 'ق', 'ي', 'م', 'ه',
    ];
    const IDGHAM_SHAMSI_LETTERS: [char; 14] = [
        'ت', 'ث', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ل', 'ن',
    ];

    let izhar_qamari_map: HashMap<char, TajweedRuleType> = IZHAR_QAMARI_LETTERS
        .iter()
        .map(|&l| (l, TajweedRuleType::IzharQamari))
        .collect();

    let idgham_shamsi_map: HashMap<char, TajweedRuleType> = IDGHAM_SHAMSI_LETTERS
        .iter()
        .map(|&l| (l, TajweedRuleType::IdghamShamsi))
        .collect();

    let mut i = 0;
    while i < verse_chars.len() {
        // Lam Al-Ta'rif
        if verse_chars[i] == 'ا' && i + 1 < verse_chars.len() {
            check_lam_al_tarif(
                &verse_chars,
                i,
                matches,
                &izhar_qamari_map,
                &idgham_shamsi_map,
                style,
            );
        }
        i += 1;
    }
}

fn determine_rule_for_lam_al(
    izhar_qamari_map: &HashMap<char, TajweedRuleType>,
    idgham_shamsi_map: &HashMap<char, TajweedRuleType>,
    following_letter: char,
) -> TajweedRuleType {
    // 1. Izhar Qamari (الإظهار القمري)
    if izhar_qamari_map.contains_key(&following_letter) {
        return TajweedRuleType::IzharQamari;
    }

    // 2. Idgham Shamsi (الإدغام الشمسي)
    if idgham_shamsi_map.contains_key(&following_letter) {
        return TajweedRuleType::IdghamShamsi;
    }

    TajweedRuleType::NoRule
}

fn check_lam_al_tarif(
    verse_chars: &[char],
    i: usize,
    matches: &mut Vec<RuleMatch>,
    izhar_qamari_map: &HashMap<char, TajweedRuleType>,
    idgham_shamsi_map: &HashMap<char, TajweedRuleType>,
    style: RecitationStyle,
) {
    let mut next_idx = i + 1;
    while next_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[next_idx]) {
        next_idx += 1;
    }

    if next_idx < verse_chars.len() && verse_chars[next_idx] == 'ل' {
        let mut after_lam_idx = next_idx + 1;
        while after_lam_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[after_lam_idx])
        {
            after_lam_idx += 1;
        }

        if after_lam_idx < verse_chars.len() {
            let following_letter = verse_chars[after_lam_idx];
            let rule_type =
                determine_rule_for_lam_al(izhar_qamari_map, idgham_shamsi_map, following_letter);

            if rule_type != TajweedRuleType::NoRule {
                matches.push(RuleMatch {
                    start_index: i,
                    end_index: next_idx,
                    target_letter: 'ل',
                    following_letter: Some(following_letter),
                    rule: TajweedRule::from_type(rule_type, style),
                    context: get_context(&verse_chars, i, 3),
                });
            }
        }
    }
}
