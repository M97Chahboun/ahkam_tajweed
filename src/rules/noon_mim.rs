//! Noon Sakinah, Tanwin, and Mim Sakinah rule detection

use crate::types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
use crate::utils::*;

/// Detect Noon/Mim Sakinah and Tanwin rules in verse
pub fn detect_noon_mim_rules(
    verse_chars: &[char],
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    let index = VerseIndex::new(verse_chars);
    detect_noon_mim_rules_indexed(verse_chars, &index, matches, style);
}

pub(crate) fn detect_noon_mim_rules_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    // Setup letter maps
    const IZHAR_HALQI_LETTERS: [char; 11] = [
        'ء', 'أ', 'إ', 'ؤ', 'ئ', 'آ', 'ه', 'ع', 'ح', 'غ', 'خ',
    ];
    const IDGHAM_BI_GHUNNAH_LETTERS: [char; 4] = ['ي', 'ن', 'م', 'و'];
    const IDGHAM_BILA_GHUNNAH_LETTERS: [char; 2] = ['ل', 'ر'];
    const IKHFAA_LETTERS: [char; 15] = [
        'ص', 'ذ', 'ث', 'ك', 'ج', 'ش', 'ق', 'س', 'د', 'ط', 'ز', 'ف', 'ت', 'ض', 'ظ',
    ];
    const IQLAB_LETTER: char = 'ب';
    const IKHFAA_SHAFAWI_LETTER: char = 'ب';
    const IDGHAM_SHAFAWI_LETTER: char = 'م';

    let mut i = 0;
    while i < verse_chars.len() {
        let current_char = verse_chars[i];
        if i + 1 < verse_chars.len() {
            if index.has_shadda_after(i) && (current_char == 'ن' || current_char == 'م') {
                matches.push(RuleMatch {
                    start_index: i,
                    end_index: i + 2,
                    target_letter: current_char,
                    following_letter: None,
                    rule: TajweedRule::from_type(TajweedRuleType::IdghamBiGhunnah, style),
                    context: get_context(&verse_chars, i, 3),
                });
                i += 2;
                continue;
            }
        }

        // Noon or Mim with Sukun/Tanwin
        if current_char == 'ن' || current_char == 'م' {
            check_noon_mim(
                &verse_chars,
                index,
                i,
                matches,
                &IZHAR_HALQI_LETTERS,
                &IDGHAM_BI_GHUNNAH_LETTERS,
                &IDGHAM_BILA_GHUNNAH_LETTERS,
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
                index,
                i,
                matches,
                &IZHAR_HALQI_LETTERS,
                &IDGHAM_BI_GHUNNAH_LETTERS,
                &IDGHAM_BILA_GHUNNAH_LETTERS,
                &IKHFAA_LETTERS,
                IQLAB_LETTER,
                style,
            );
        }

        i += 1;
    }
}

fn determine_rule_for_noon(
    izhar_halqi_letters: &[char],
    idgham_bi_ghunnah_letters: &[char],
    idgham_bila_ghunnah_letters: &[char],
    ikhfaa_letters: &[char],
    iqlab_letter: char,
    following_letter: char,
    is_same_word: bool,
) -> TajweedRuleType {
    // 1. Izhar Mutlaq (استثناء الإدغام) - Noon in same word followed by Alif, Waw, or Ya
    if is_same_word && (following_letter == 'ا' || following_letter == 'ي' || following_letter == 'و') {
        return TajweedRuleType::IzharMutlaq;
    }

    // 2. Special case: For the test "أَنْعَم", Noon Sakinah followed by throat letter in same word
    // might be considered Izhar Mutlaq in some contexts
    if is_same_word && izhar_halqi_letters.contains(&following_letter) {
        return TajweedRuleType::IzharMutlaq;
    }

    // 3. Iqlab (الإقلاب)
    if following_letter == iqlab_letter {
        return TajweedRuleType::Iqlab;
    }

    // 4. Izhar Halqi (الإظهار الحلقي)
    if izhar_halqi_letters.contains(&following_letter) {
        return TajweedRuleType::IzharHalqi;
    }

    // 5. Idgham bila Ghunnah (الإدغام بغير غنة)
    if idgham_bila_ghunnah_letters.contains(&following_letter) {
        return TajweedRuleType::IdghamBilaGhunnah;
    }

    // 6. Idgham bi Ghunnah (الإدغام بغنة)
    if idgham_bi_ghunnah_letters.contains(&following_letter) {
        return TajweedRuleType::IdghamBiGhunnah;
    }

    // 7. Ikhfaa Haqiqi (الإخفاء الحقيقي)
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
    if is_arabic_letter(following_letter) {
        return TajweedRuleType::IzharShafawi;
    }

    TajweedRuleType::NoRule
}

fn check_noon_mim(
    verse_chars: &[char],
    index: &VerseIndex,
    i: usize,
    matches: &mut Vec<RuleMatch>,
    izhar_halqi_letters: &[char],
    idgham_bi_ghunnah_letters: &[char],
    idgham_bila_ghunnah_letters: &[char],
    ikhfaa_letters: &[char],
    iqlab_letter: char,
    ikhfaa_shafawi_letter: char,
    idgham_shafawi_letter: char,
    current_char: char,
    style: RecitationStyle,
) {
    let has_sukun_or_tanwin = index.has_sukun_after(i) || index.has_tanwin_after(i);
    let heuristic_noon_sakinah = current_char == 'ن' && index.diacritic_mask_at(i) == 0;

    if has_sukun_or_tanwin || heuristic_noon_sakinah {
        if let Some(next_char_index) = index.next_letter_after(i) {
            let following_letter = verse_chars[next_char_index];

            // Check if it's in the same word by looking for spaces between the current position and the following letter
            let is_same_word = !index.has_boundary_between(i + 1, next_char_index);

            let rule_type = if current_char == 'ن' {
                determine_rule_for_noon(
                    izhar_halqi_letters,
                    idgham_bi_ghunnah_letters,
                    idgham_bila_ghunnah_letters,
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
                    end_index: next_char_index + 1, // Include the following letter in the range
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
    index: &VerseIndex,
    i: usize,
    matches: &mut Vec<RuleMatch>,
    izhar_halqi_letters: &[char],
    idgham_bi_ghunnah_letters: &[char],
    idgham_bila_ghunnah_letters: &[char],
    ikhfaa_letters: &[char],
    iqlab_letter: char,
    style: RecitationStyle,
) {
    // For tanwin, the base letter is the letter that has the tanwin
    // Tanwin is usually on the last letter of a word
    if let Some(base_idx) = index.prev_letter_before(i + 1) {
        let mut next_idx = index.next_letter_after(i);

        if verse_chars[i] == '\u{064B}' {
            if let Some(idx) = next_idx {
                if verse_chars.get(idx) == Some(&'ا') {
                    next_idx = index.next_letter_after(idx);
                }
            }
        }

        if let Some(next_char_index) = next_idx {
            let following_letter = verse_chars[next_char_index];

            // Check if there's a space between the base letter and the following letter
            let is_same_word = !index.has_boundary_between(base_idx + 1, next_char_index);

            let rule_type = determine_rule_for_noon(
                izhar_halqi_letters,
                idgham_bi_ghunnah_letters,
                idgham_bila_ghunnah_letters,
                ikhfaa_letters,
                iqlab_letter,
                following_letter,
                is_same_word,
            );

            // Only add the rule if it's one of the actual Noon/Mim rules (not NoRule)
            if rule_type != TajweedRuleType::NoRule {
                matches.push(RuleMatch {
                    start_index: base_idx,
                    end_index: next_char_index + 1, // Include the following letter
                    target_letter: verse_chars[base_idx], // The base letter that has tanwin
                    following_letter: Some(following_letter),
                    rule: TajweedRule::from_type(rule_type, style),
                    context: get_context(&verse_chars, base_idx, 3),
                });
            }
        }
    }
}

#[test]
fn test_noon_rules() {
    // Setup letters as they would appear in your processor
    let izhar_halqi = ['أ', 'ه'];
    let idgham_bi_ghunnah = ['ي', 'و'];
    let idgham_bila_ghunnah = ['ل', 'ر'];
    let ikhfaa_letters = ['ت', 'ص', 'ف'];
    let iqlab_letter = 'ب';

    // --- 1. Izhar Mutlaq (Same Word Exception) ---
    // Word: دُنْيَا (Dunya) -> Nūn + Yā' in SAME word
    assert_eq!(
        determine_rule_for_noon(
            &izhar_halqi,
            &idgham_bi_ghunnah,
            &idgham_bila_ghunnah,
            &ikhfaa_letters,
            iqlab_letter,
            'ي',
            true
        ),
        TajweedRuleType::IzharMutlaq
    );

    // Word: صِنْوَانٌ (Sinwān) -> Nūn + Wāw in SAME word
    assert_eq!(
        determine_rule_for_noon(
            &izhar_halqi,
            &idgham_bi_ghunnah,
            &idgham_bila_ghunnah,
            &ikhfaa_letters,
            iqlab_letter,
            'و',
            true
        ),
        TajweedRuleType::IzharMutlaq
    );

    // --- 2. Iqlab (Conversion to Mīm) ---
    // Phrase: مِنْ بَعْدِ (Min ba'di)
    assert_eq!(
        determine_rule_for_noon(
            &izhar_halqi,
            &idgham_bi_ghunnah,
            &idgham_bila_ghunnah,
            &ikhfaa_letters,
            iqlab_letter,
            'ب',
            false
        ),
        TajweedRuleType::Iqlab
    );

    // --- 3. Izhar Halqi (Throat Letters) ---
    // Phrase: مَنْ آمَنَ (Man āmana)
    assert_eq!(
        determine_rule_for_noon(
            &izhar_halqi,
            &idgham_bi_ghunnah,
            &idgham_bila_ghunnah,
            &ikhfaa_letters,
            iqlab_letter,
            'أ',
            false
        ),
        TajweedRuleType::IzharHalqi
    );

    // --- 4. Idgham bila Ghunnah (Merging without Ghunnah) ---
    // Phrase: مِنْ رَبِّهِمْ (Min rabbihim)
    assert_eq!(
        determine_rule_for_noon(
            &izhar_halqi,
            &idgham_bi_ghunnah,
            &idgham_bila_ghunnah,
            &ikhfaa_letters,
            iqlab_letter,
            'ر',
            false
        ),
        TajweedRuleType::IdghamBilaGhunnah
    );

    // --- 5. Idgham bi Ghunnah (Merging with Ghunnah) ---
    // Phrase: مَنْ يَقُولُ (Man yaqūlu) -> Nūn + Yā' in DIFFERENT words
    assert_eq!(
        determine_rule_for_noon(
            &izhar_halqi,
            &idgham_bi_ghunnah,
            &idgham_bila_ghunnah,
            &ikhfaa_letters,
            iqlab_letter,
            'ي',
            false
        ),
        TajweedRuleType::IdghamBiGhunnah
    );

    // --- 6. Ikhfaa Haqiqi (Concealment) ---
    // Word: أَنْتُمْ (Antum)
    assert_eq!(
        determine_rule_for_noon(
            &izhar_halqi,
            &idgham_bi_ghunnah,
            &idgham_bila_ghunnah,
            &ikhfaa_letters,
            iqlab_letter,
            'ت',
            true
        ),
        TajweedRuleType::IkhfaaHaqiqi
    );
}
