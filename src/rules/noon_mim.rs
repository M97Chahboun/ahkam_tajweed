//! Noon Sakinah, Tanwin, and Mim Sakinah rule detection

use crate::types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
use crate::utils::*;
use std::collections::HashMap;

/// Detect Noon/Mim Sakinah and Tanwin rules in verse
pub fn detect_noon_mim_rules(
    verse_chars: &[char],
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    // Setup letter maps
    const IZHAR_HALQI_LETTERS: [char; 6] = ['أ', 'ه', 'ع', 'ح', 'غ', 'خ'];
    const IDGHAM_BI_GHUNNAH_LETTERS: [char; 4] = ['ي', 'ن', 'م', 'و'];
    const IDGHAM_BILA_GHUNNAH_LETTERS: [char; 2] = ['ل', 'ر'];
    const IKHFAA_LETTERS: [char; 15] = [
        'ص', 'ذ', 'ث', 'ك', 'ج', 'ش', 'ق', 'س', 'د', 'ط', 'ز', 'ف', 'ت', 'ض', 'ظ',
    ];
    const IQLAB_LETTER: char = 'ب';
    const IKHFAA_SHAFAWI_LETTER: char = 'ب';
    const IDGHAM_SHAFAWI_LETTER: char = 'م';

    let izhar_halqi_map: HashMap<char, TajweedRuleType> = IZHAR_HALQI_LETTERS
        .iter()
        .map(|&l| (l, TajweedRuleType::IzharHalqi))
        .collect();

    let idgham_bi_ghunnah_map: HashMap<char, TajweedRuleType> = IDGHAM_BI_GHUNNAH_LETTERS
        .iter()
        .map(|&l| (l, TajweedRuleType::IdghamBiGhunnah))
        .collect();

    let idgham_bila_ghunnah_map: HashMap<char, TajweedRuleType> = IDGHAM_BILA_GHUNNAH_LETTERS
        .iter()
        .map(|&l| (l, TajweedRuleType::IdghamBilaGhunnah))
        .collect();

    let mut i = 0;
    while i < verse_chars.len() {
        let current_char = verse_chars[i];

        // Noon or Mim with Sukun/Tanwin
        if current_char == 'ن' || current_char == 'م' {
            check_noon_mim(
                &verse_chars,
                i,
                matches,
                &izhar_halqi_map,
                &idgham_bi_ghunnah_map,
                &idgham_bila_ghunnah_map,
                &IKHFAA_LETTERS,
                IQLAB_LETTER,
                IKHFAA_SHAFAWI_LETTER,
                IDGHAM_SHAFAWI_LETTER,
                current_char,
                style,
            );
        }

        // Tanwin handling
        if is_tanwin(current_char) {
            check_tanwin(
                &verse_chars,
                i,
                matches,
                &izhar_halqi_map,
                &idgham_bi_ghunnah_map,
                &idgham_bila_ghunnah_map,
                &IKHFAA_LETTERS,
                IQLAB_LETTER,
                style,
            );
        }

        i += 1;
    }
}

fn determine_rule_for_noon(
    izhar_halqi_map: &HashMap<char, TajweedRuleType>,
    idgham_bi_ghunnah_map: &HashMap<char, TajweedRuleType>,
    idgham_bila_ghunnah_map: &HashMap<char, TajweedRuleType>,
    ikhfaa_letters: &[char],
    iqlab_letter: char,
    following_letter: char,
    is_same_word: bool,
) -> TajweedRuleType {
    // 1. Izhar Mutlaq (استثناء الإدغام)
    if is_same_word && (following_letter == 'ي' || following_letter == 'و') {
        return TajweedRuleType::IzharMutlaq;
    }

    // 2. Iqlab (الإقلاب)
    if following_letter == iqlab_letter {
        return TajweedRuleType::Iqlab;
    }

    // 3. Izhar Halqi (الإظهار الحلقي)
    if izhar_halqi_map.contains_key(&following_letter) {
        return TajweedRuleType::IzharHalqi;
    }

    // 4. Idgham bila Ghunnah (الإدغام بغير غنة)
    if idgham_bila_ghunnah_map.contains_key(&following_letter) {
        return TajweedRuleType::IdghamBilaGhunnah;
    }

    // 5. Idgham bi Ghunnah (الإدغام بغنة)
    if idgham_bi_ghunnah_map.contains_key(&following_letter) {
        return TajweedRuleType::IdghamBiGhunnah;
    }

    // 6. Ikhfaa Haqiqi (الإخفاء الحقيقي)
    if ikhfaa_letters.contains(&following_letter) {
        return TajweedRuleType::IkhfaaHaqiqi;
    }

    TajweedRuleType::NoRule
}

fn determine_rule_for_mim(
    ikhfaa_shafawi_letter: char,
    idgham_shafawi_letter: char,
    following_letter: char,
) -> TajweedRuleType {
    // 1. Ikhfaa Shafawi (الإخفاء الشفوي) - before Ba
    if following_letter == ikhfaa_shafawi_letter {
        return TajweedRuleType::IkhfaaShafawi;
    }

    // 2. Idgham Shafawi (الإدغام الشفوي) - before Mim
    if following_letter == idgham_shafawi_letter {
        return TajweedRuleType::IdghamMithlayn;
    }

    // 3. Izhar Shafawi (الإظهار الشفوي) - before other letters
    const ARABIC_LETTERS: &str = "ءأبةتثجحخدذرزسشصضطظعغفقكلمنهوي";
    if ARABIC_LETTERS.contains(following_letter) {
        return TajweedRuleType::IzharShafawi;
    }

    TajweedRuleType::NoRule
}

fn check_noon_mim(
    verse_chars: &[char],
    i: usize,
    matches: &mut Vec<RuleMatch>,
    izhar_halqi_map: &HashMap<char, TajweedRuleType>,
    idgham_bi_ghunnah_map: &HashMap<char, TajweedRuleType>,
    idgham_bila_ghunnah_map: &HashMap<char, TajweedRuleType>,
    ikhfaa_letters: &[char],
    iqlab_letter: char,
    ikhfaa_shafawi_letter: char,
    idgham_shafawi_letter: char,
    current_char: char,
    style: RecitationStyle,
) {
    let mut j = i + 1;
    let mut has_sukun = false;
    let mut has_tanwin = false;

    while j < verse_chars.len() && is_tajweed_ignorable(verse_chars[j]) {
        if is_sukun(verse_chars[j]) {
            has_sukun = true;
            break;
        }
        if is_tanwin(verse_chars[j]) {
            has_tanwin = true;
            break;
        }
        j += 1;
    }

    if has_sukun || has_tanwin {
        let mut next_char_index = j + 1;
        while next_char_index < verse_chars.len()
            && is_tajweed_ignorable(verse_chars[next_char_index])
        {
            next_char_index += 1;
        }

        if next_char_index < verse_chars.len() {
            let following_letter = verse_chars[next_char_index];
            let is_same_word = !verse_chars[i + 1..next_char_index]
                .iter()
                .any(|&c| c.is_whitespace());

            let rule_type = if current_char == 'ن' {
                determine_rule_for_noon(
                    izhar_halqi_map,
                    idgham_bi_ghunnah_map,
                    idgham_bila_ghunnah_map,
                    ikhfaa_letters,
                    iqlab_letter,
                    following_letter,
                    is_same_word,
                )
            } else {
                determine_rule_for_mim(
                    ikhfaa_shafawi_letter,
                    idgham_shafawi_letter,
                    following_letter,
                )
            };

            if rule_type != TajweedRuleType::NoRule {
                matches.push(RuleMatch {
                    start_index: i,
                    end_index: j,
                    target_letter: current_char,
                    following_letter: Some(following_letter),
                    rule: TajweedRule::from_type(rule_type, style),
                    context: get_context(&verse_chars, i, 3),
                });
            }
        }
    }
}

fn check_tanwin(
    verse_chars: &[char],
    i: usize,
    matches: &mut Vec<RuleMatch>,
    izhar_halqi_map: &HashMap<char, TajweedRuleType>,
    idgham_bi_ghunnah_map: &HashMap<char, TajweedRuleType>,
    idgham_bila_ghunnah_map: &HashMap<char, TajweedRuleType>,
    ikhfaa_letters: &[char],
    iqlab_letter: char,
    style: RecitationStyle,
) {
    let mut base_idx_opt: Option<usize> = None;
    let mut k = i;
    while k > 0 {
        k -= 1;
        if !is_tajweed_ignorable(verse_chars[k]) {
            base_idx_opt = Some(k);
            break;
        }
    }

    if let Some(base_idx) = base_idx_opt {
        let mut next_char_index = i + 1;
        while next_char_index < verse_chars.len()
            && is_tajweed_ignorable(verse_chars[next_char_index])
        {
            next_char_index += 1;
        }

        if next_char_index < verse_chars.len() {
            let following_letter = verse_chars[next_char_index];
            let is_same_word = !verse_chars[base_idx..next_char_index]
                .iter()
                .any(|&c| c.is_whitespace());

            let rule_type = determine_rule_for_noon(
                izhar_halqi_map,
                idgham_bi_ghunnah_map,
                idgham_bila_ghunnah_map,
                ikhfaa_letters,
                iqlab_letter,
                following_letter,
                is_same_word,
            );

            if rule_type != TajweedRuleType::NoRule {
                matches.push(RuleMatch {
                    start_index: base_idx,
                    end_index: i,
                    target_letter: 'ن',
                    following_letter: Some(following_letter),
                    rule: TajweedRule::from_type(rule_type, style),
                    context: get_context(&verse_chars, base_idx, 3),
                });
            }
        }
    }
}
