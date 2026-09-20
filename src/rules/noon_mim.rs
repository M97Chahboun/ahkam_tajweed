//! Noon Sakinah, Tanwin, and Mim Sakinah rule detection

use crate::rules::letters;
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

    let mut i = 0;
    while i < verse_chars.len() {
        let current_char = verse_chars[i];
        if i + 1 < verse_chars.len()
            && index.has_shadda_after(i)
            && (current_char == 'ن' || current_char == 'م')
            && !is_idgham_product(verse_chars, index, i)
        {
                let mut end_idx = i + 1;
                while end_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[end_idx]) {
                    end_idx += 1;
                }
                matches.push(RuleMatch {
                    start_index: i,
                    end_index: end_idx,
                    target_letter: current_char,
                    following_letter: None,
                    rule: TajweedRule::from_type(TajweedRuleType::GhunnahMushadda, style),
                    context: get_context(verse_chars, i, 3),
                });
                // Fall through: a Noon/Mim can carry a Shadda *and* a Tanwin
                // (صُمٌّ وَ، مُّسَمًّى، بِغَمٍّ), and the Tanwin has rules of its own.
            }

        // Noon or Mim with Sukun/Tanwin
        if current_char == 'ن' || current_char == 'م' {
            check_noon_mim(
                verse_chars,
                index,
                i,
                matches,
                letters::IZHAR_HALQI,
                letters::IDGHAM_BI_GHUNNAH,
                letters::IDGHAM_BILA_GHUNNAH,
                letters::IKHFAA,
                letters::IQLAB,
                letters::IKHFAA_SHAFAWI,
                letters::IDGHAM_SHAFAWI,
                current_char,
                style,
            );
        }

        // Tanwin handling
        if is_tanwin(current_char) {
            check_tanwin(
                verse_chars,
                index,
                i,
                matches,
                letters::IZHAR_HALQI,
                letters::IDGHAM_BI_GHUNNAH,
                letters::IDGHAM_BILA_GHUNNAH,
                letters::IKHFAA,
                letters::IQLAB,
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
    // 1. Izhar Halqi (الإظهار الحلقي) - applies whether in same word or across words (e.g. أنعمت, منهم)
    if izhar_halqi_letters.contains(&following_letter) {
        return TajweedRuleType::IzharHalqi;
    }

    // 2. Izhar Mutlaq (الإظهار المطلق) - Noon Sakinah in same word followed by Waw or Ya (e.g. دنيا, قنوان, صنوان, بنيان)
    if is_same_word && (following_letter == 'ي' || following_letter == 'و' || following_letter == '\u{06CC}') {
        return TajweedRuleType::IzharMutlaq;
    }

    // 3. Iqlab (الإقلاب)
    if following_letter == iqlab_letter {
        return TajweedRuleType::Iqlab;
    }

    // 4. Idgham bila Ghunnah (الإدغام بغير غنة)
    if idgham_bila_ghunnah_letters.contains(&following_letter) {
        return TajweedRuleType::IdghamBilaGhunnah;
    }

    // 5. Idgham bi Ghunnah (الإدغام بغنة)
    if idgham_bi_ghunnah_letters.contains(&following_letter) {
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
    if is_arabic_letter(following_letter) {
        return TajweedRuleType::IzharShafawi;
    }

    TajweedRuleType::NoRule
}

/// True when the Shadda on the Noon/Mim at `idx` is the *product* of an
/// assimilation rather than part of the word.
///
/// `مِن مَّاءٍ` and `لَهُم مَّشَوْا۟` double their Mim only because the Noon/Mim
/// Sakinah before them merged into it, and `يَوْمَئِذٍ نَّاعِمَةٌ` doubles its Noon
/// only because of the Tanwin. The Ghunnah is real but it belongs to the Idgham
/// that produced it, which is reported in its own right — reporting a separate
/// Ghunnah Mushaddadah on top would count the same sound twice.
fn is_idgham_product(verse_chars: &[char], index: &VerseIndex, idx: usize) -> bool {
    let Some(mut prev) = index.prev_letter_before(idx) else {
        return false;
    };
    // Tanwin Fath is written on the letter *before* a bare seat Alif
    // (مَثَلًا, رِجْزًا), so step over that Alif to reach the letter that
    // carries the Tanwin.
    if matches!(verse_chars[prev], 'ا' | 'ى')
        && index.diacritic_mask_at(prev) & crate::utils::DIAC_ANY == 0
    {
        match index.prev_letter_before(prev) {
            Some(p) if !index.has_boundary_between(p + 1, prev) => prev = p,
            _ => {}
        }
    }
    // Tanwin on the previous letter always assimilates into a following ن or م.
    if index.has_tanwin_after(prev) {
        return true;
    }
    // A Noon Sakinah merges into ن and م; a Mim Sakinah merges into م only.
    let prev_char = verse_chars[prev];
    let prev_is_sakin = index.has_sukun_after(prev) || index.diacritic_mask_at(prev) == 0;
    if !prev_is_sakin {
        return false;
    }
    match (prev_char, verse_chars[idx]) {
        ('ن', 'ن') | ('ن', 'م') | ('م', 'م') => true,
        _ => false,
    }
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
    // Only a Sukun makes the letter itself sakinah. A Noon or Mim *carrying*
    // a Tanwin is the seat of the Tanwin, and the Tanwin's own rules apply:
    // قَوْمٌ مُّسْرِفُونَ is إدغام بغنة (Tanwin into Mim), not إدغام شفوي, and
    // عَلِيمٌۢ بِ is إقلاب, not إخفاء شفوي.
    let has_sukun_or_tanwin = index.has_sukun_after(i);
    // A Noon or Mim carrying no mark at all is Sakinah. The Uthmani script
    // relies on this: it leaves the Sukun off a Mim Sakinah standing before
    // م or ب (قُلُوبِهِم مَّرَضٌ, هُم بِمُؤْمِنِينَ), because the following
    // Shadda / Ikhfaa mark already carries the information.
    let unmarked_sakinah =
        matches!(current_char, 'ن' | 'م') && index.diacritic_mask_at(i) == 0;

    if has_sukun_or_tanwin || unmarked_sakinah {
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
                let mut end_idx = i + 1;
                while end_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[end_idx]) {
                    end_idx += 1;
                }
                matches.push(RuleMatch {
                    start_index: i,
                    end_index: end_idx,
                    target_letter: current_char,
                    following_letter: Some(following_letter),
                    rule: TajweedRule::from_type(rule_type, style),
                    context: get_context(verse_chars, i, 3),
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
    // Tanwin is on the last letter of a word
    if let Some(base_idx) = index.prev_letter_before(i + 1) {
        let mut next_idx = index.next_letter_after(i);

        // Skip any trailing Alif in the same word for Tanwin Fath (e.g. شيئاً, عملاً, مذكوراً)
        while let Some(idx) = next_idx {
            if (verse_chars[idx] == 'ا' || verse_chars[idx] == 'ى')
                && !index.has_boundary_between(base_idx + 1, idx)
            {
                next_idx = index.next_letter_after(idx);
            } else {
                break;
            }
        }

        if let Some(next_char_index) = next_idx {
            let following_letter = verse_chars[next_char_index];

            // Tanwin is always at word end — never triggers Izhar Mutlaq
            let is_same_word = false;

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
                let mut end_idx = i + 1;
                while end_idx < verse_chars.len()
                    && (is_tajweed_ignorable(verse_chars[end_idx]) || verse_chars[end_idx] == 'ا')
                    && (end_idx <= i + 2)
                {
                    end_idx += 1;
                }
                matches.push(RuleMatch {
                    start_index: base_idx,
                    end_index: end_idx.max(i + 1),
                    target_letter: verse_chars[base_idx], // The base letter that has tanwin
                    following_letter: Some(following_letter),
                    rule: TajweedRule::from_type(rule_type, style),
                    context: get_context(verse_chars, base_idx, 3),
                });
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// An-Naql (النقل) — Warsh only
// When a word ends with a non-Madd Sakin letter and the next word begins with
// Hamza al-Qat'a, transfer the Hamza's vowel to the Sakin letter and drop the Hamza.
// Source: Multiple Warsh authorities — mandatory rule in Warsh (Al-Azraq route).
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) fn detect_naql_rules_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    // Only applies to Warsh
    if style != RecitationStyle::Warsh {
        return;
    }

    let madd_letters = letters::MADD_LETTERS;
    let hamza_forms = letters::HAMZA_FORMS;

    detect_naql_lam_al_tarif_indexed(verse_chars, index, matches, style);

    let mut i = 0;
    while i < verse_chars.len() {
        // Find a word boundary (space)
        if verse_chars[i] == ' ' {
            // Look back: find the last non-diacritic letter before this space
            if let Some(prev_letter_idx) = index.prev_letter_before(i) {
                let prev_letter = verse_chars[prev_letter_idx];
                // Condition 1: preceding letter must be Sakin and NOT a Madd letter
                let is_sakin = index.has_sukun_after(prev_letter_idx);
                let is_madd_letter = madd_letters.contains(&prev_letter);

                if is_sakin && !is_madd_letter {
                    // Look forward: find the first letter of the next word
                    if let Some(next_letter_idx) = index.next_letter_after(i) {
                        let next_letter = verse_chars[next_letter_idx];
                        // Condition 2: next word must start with Hamza al-Qat'a
                        if hamza_forms.contains(&next_letter) {
                            matches.push(RuleMatch {
                                start_index: prev_letter_idx,
                                end_index: next_letter_idx + 1,
                                target_letter: prev_letter,
                                following_letter: Some(next_letter),
                                rule: TajweedRule::from_type(TajweedRuleType::Naql, style),
                                context: get_context(verse_chars, prev_letter_idx, 4),
                            });
                        }
                    }
                }
            }
        }
        i += 1;
    }
}

/// النقل في لام التعريف — the one position where Warsh transfers a vowel
/// *inside* a word rather than across a word boundary.
///
/// When the definite article meets a hamzat qat' (`الْإِيمَٰن`, `الْأَمْر`,
/// `الْأَرْض`), Warsh moves the hamza's vowel onto the sakin Lam and drops the
/// hamza: `لِايمَٰن`, `لَامْر`. Two spellings reach the analyser:
///
/// * **ordinary orthography** — `الْإِيمَٰن`: the Lam still carries its sukun and
///   the hamza is still written.
/// * **Warsh mushaf orthography** — `اُ۬لِايمَٰنَ`: the transfer is already part of
///   the script, so the Lam carries the hamza's vowel (a Lam of the article is
///   otherwise never voweled) and all that is left of the hamza is its silent
///   Alif seat — the one the printed mushaf marks with a dot underneath.
///
/// Reported as GitHub issue #8 (سورة الحجرات 49:7), where neither spelling was
/// detected because Naql was only ever looked for across a space.
fn detect_naql_lam_al_tarif_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    let vowels = crate::utils::DIAC_FATHA | crate::utils::DIAC_DAMMA | crate::utils::DIAC_KASRA;

    for (i, &ch) in verse_chars.iter().enumerate() {
        // The article starts with a plain Alif or with Hamzat Wasl.
        if ch != 'ا' && ch != '\u{0671}' {
            continue;
        }

        // Only an article — not an Alif sitting inside a word root. The article
        // may still be prefixed by و ف ب ك ل (وَالْأَمْر).
        if let Some(prev_idx) = index.prev_letter_before(i) {
            if !index.has_boundary_between(prev_idx + 1, i)
                && !matches!(verse_chars[prev_idx], 'و' | 'ف' | 'ب' | 'ك' | 'ل')
            {
                continue;
            }
        }

        let Some(lam_idx) = index.next_letter_after(i) else {
            continue;
        };
        if verse_chars[lam_idx] != 'ل' || index.has_boundary_between(i + 1, lam_idx) {
            continue;
        }

        let Some(after_idx) = index.next_letter_after(lam_idx) else {
            continue;
        };
        if index.has_boundary_between(lam_idx + 1, after_idx) {
            continue;
        }

        let after = verse_chars[after_idx];
        let is_written_hamza =
            index.has_sukun_after(lam_idx) && letters::HAMZA_FORMS.contains(&after);
        // A voweled article Lam followed by a bare Alif is the transfer already
        // spelled out; a Shadda would make it a sun letter, not a Naql.
        let is_transferred = index.has_diacritic_after_mask(lam_idx, vowels)
            && !index.has_shadda_after(lam_idx)
            && after == 'ا'
            && index.diacritic_mask_at(after_idx) & crate::utils::DIAC_ANY == 0;

        if is_written_hamza || is_transferred {
            matches.push(RuleMatch {
                start_index: lam_idx,
                end_index: after_idx + 1,
                target_letter: verse_chars[lam_idx],
                following_letter: Some(after),
                rule: TajweedRule::from_type(TajweedRuleType::Naql, style),
                context: get_context(verse_chars, lam_idx, 4),
            });
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tasheel Al-Hamza (تسهيل الهمزة) — Warsh only
// When two Hamzas appear consecutively in the same word (first with Fatha),
// the second is softened to between Hamza and the corresponding Madd letter.
// Source: Warsh Al-Azraq authorities.
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) fn detect_tasheel_rules_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    if style != RecitationStyle::Warsh {
        return;
    }

    let hamza_forms = letters::HAMZA_FORMS;

    let mut i = 0;
    while i < verse_chars.len() {
        let ch = verse_chars[i];
        if hamza_forms.contains(&ch) {
            // First Hamza must have Fatha
            if index.has_diacritic_after_mask(i, crate::utils::DIAC_FATHA) {
                // Look for a second Hamza in the same word (no boundary in between)
                if let Some(next_idx) = index.next_letter_after(i) {
                    if !index.has_boundary_between(i + 1, next_idx)
                        && hamza_forms.contains(&verse_chars[next_idx])
                    {
                        matches.push(RuleMatch {
                            start_index: i,
                            end_index: next_idx + 1,
                            target_letter: ch,
                            following_letter: Some(verse_chars[next_idx]),
                            rule: TajweedRule::from_type(TajweedRuleType::TasheelHamza, style),
                            context: get_context(verse_chars, i, 3),
                        });
                    }
                }
            }
        }
        i += 1;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Idgham Mutajanisayn (إدغام المتجانسين)
// Assimilation between letters from the same articulation point but with
// different characteristics.  Pairs verified by Al-Jazariyyah:
//   ط Sakin + ت  |  ذ Sakin + ظ  |  د Sakin + ت  |  ت Sakin + ط
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) fn detect_idgham_mutajanisayn_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    // Pairs: (sakin_letter, following_letter)
    const MUTAJANISAYN_PAIRS: [(char, char); 6] = [
        ('ط', 'ت'), // Ta marbuta group
        ('ت', 'ط'), // reversed
        ('ذ', 'ظ'), // Dhal + Dha
        ('ظ', 'ذ'), // reversed
        ('د', 'ت'), // Dal + Ta
        ('ت', 'د'), // Ta + Dal
    ];

    let mut i = 0;
    while i < verse_chars.len() {
        let ch = verse_chars[i];
        // Check if this letter has Sukun or has no short vowel (unvoweled in Uthmani script)
        let is_sakin = index.has_sukun_after(i)
            || !index.has_diacritic_after_mask(i, DIAC_FATHA | DIAC_DAMMA | DIAC_KASRA | DIAC_TANWIN);
        if is_sakin {
            if let Some(next_idx) = index.next_letter_after(i) {
                let next_ch = verse_chars[next_idx];
                if MUTAJANISAYN_PAIRS.contains(&(ch, next_ch)) {
                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: next_idx + 1,
                        target_letter: ch,
                        following_letter: Some(next_ch),
                        rule: TajweedRule::from_type(
                            TajweedRuleType::IdghamMutajanisayn,
                            style,
                        ),
                        context: get_context(verse_chars, i, 3),
                    });
                }
            }
        }
        i += 1;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Idgham Mutaqaribayn (إدغام المتقاربين)
// Assimilation between letters from adjacent/close articulation points.
// Most common pairs: ق Sakin + ك  (only pair found in the Quran per Al-Jazariyyah)
// Also: ل Sakin + ر in specific contexts
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) fn detect_idgham_mutaqaribayn_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    // Pairs occurring in the Quran (Al-Jazariyyah authority)
    const MUTAQARIBAYN_PAIRS: [(char, char); 2] = [
        ('ق', 'ك'), // Qaf Sakin + Kaf — e.g. ألمْ نَخْلُقْكُمْ
        ('ل', 'ر'), // Lam Sakin + Ra — "Bal ran" بَلْ رَان
    ];

    let mut i = 0;
    while i < verse_chars.len() {
        let ch = verse_chars[i];
        let is_sakin = index.has_sukun_after(i)
            || !index.has_diacritic_after_mask(i, DIAC_FATHA | DIAC_DAMMA | DIAC_KASRA | DIAC_TANWIN);
        if is_sakin {
            if let Some(next_idx) = index.next_letter_after(i) {
                let next_ch = verse_chars[next_idx];
                // The Lam of the definite article before a Ra is Lam
                // Shamsiyyah (ٱلرَّحْمَٰنِ), not Idgham Mutaqaribayn — the pair
                // only counts where the Lam is a word of its own (قُل رَّبِّ).
                let is_article_lam = ch == 'ل'
                    && crate::rules::lam_al_tarif::article_start(verse_chars, index, i).is_some();
                if !is_article_lam && MUTAQARIBAYN_PAIRS.contains(&(ch, next_ch)) {
                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: next_idx + 1,
                        target_letter: ch,
                        following_letter: Some(next_ch),
                        rule: TajweedRule::from_type(
                            TajweedRuleType::IdghamMutaqaribayn,
                            style,
                        ),
                        context: get_context(verse_chars, i, 3),
                    });
                }
            }
        }
        i += 1;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Hamzat Al-Wasl (همزة الوصل)
// The connecting Hamza is pronounced at the start of recitation but dropped
// when connected to the previous word in continuous reading.
//
// Occurs in:
//  - Definite article الـ (Al-)
//  - Imperative verbs of Form I: افعل pattern
//  - Specific nouns: اسم، ابن، امرؤ، امرأة، اثنتان، اسم
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) fn detect_hamzat_wasl_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    // Specific nouns containing Hamzat Wasl (Arabic)
    const WASL_NOUNS: [&str; 5] = ["اسم", "ابن", "امرؤ", "امرأة", "اثنتان"];

    let mut i = 0;
    while i < verse_chars.len() {
        let ch = verse_chars[i];

        // A connecting Hamza that opens the recitation is *pronounced*: there
        // is nothing before it to connect to. The rule is about the Hamza
        // being dropped, so the first letter of the verse is not reported.
        if index.prev_letter_before(i).is_none() {
            i += 1;
            continue;
        }

        // The Uthmani script spells the connecting Hamza out as Alif Wasla
        // (ٱ, U+0671). Where it is written there is nothing left to infer —
        // it is a Hamzat Wasl wherever it stands, including inside a word
        // after a prefix (بِٱسْمِ, وَٱلضُّحَىٰ).
        if ch == '\u{0671}' {
            let following = index.next_letter_after(i).map(|n| verse_chars[n]);
            matches.push(RuleMatch {
                start_index: i,
                end_index: i + 1,
                target_letter: ch,
                following_letter: following,
                rule: TajweedRule::from_type(TajweedRuleType::HamzatWasl, style),
                context: get_context(verse_chars, i, 3),
            });
            i += 1;
            continue;
        }

        // Otherwise only a plain Alif (ا) can carry it, and then only at the
        // start of a word, where it has to be told apart from Hamzat Qat'.
        if ch != 'ا' {
            i += 1;
            continue;
        }

        // Must be at the start of a word (preceded by space or at position 0)
        let at_word_start = i == 0 || verse_chars[i - 1] == ' ';
        if !at_word_start {
            i += 1;
            continue;
        }

        // Check for definite article: الـ
        if let Some(next_idx) = index.next_letter_after(i) {
            if verse_chars[next_idx] == 'ل' {
                // This is Hamzat Wasl of the definite article
                matches.push(RuleMatch {
                    start_index: i,
                    end_index: next_idx,
                    target_letter: ch,
                    following_letter: Some('ل'),
                    rule: TajweedRule::from_type(TajweedRuleType::HamzatWasl, style),
                    context: get_context(verse_chars, i, 3),
                });
                i += 1;
                continue;
            }

            // Check for imperative verb form I: starts with Alif then a letter with Kasra/Damma sukun
            // i.e. next letter has Kasra (Form I imperative has Kasrat al-wasl)
            if index.has_diacritic_after_mask(i, crate::utils::DIAC_KASRA)
                || (index.diacritic_mask_at(i) == 0 && is_arabic_letter(verse_chars[next_idx]))
            {
                // Check it's a clean word start with no Hamza diacritic above the Alif
                // (Hamzat Wasl has no hamza sign above it, unlike Hamzat Qat'a which has أ or إ)
                if ch == 'ا' {
                    // Check if any of the WASL_NOUNS match at this position
                    let remaining: String = verse_chars[i..].iter().collect();
                    let is_wasl_noun = WASL_NOUNS
                        .iter()
                        .any(|noun| remaining.starts_with(noun));

                    if is_wasl_noun {
                        matches.push(RuleMatch {
                            start_index: i,
                            end_index: i + 1,
                            target_letter: ch,
                            following_letter: Some(verse_chars[next_idx]),
                            rule: TajweedRule::from_type(TajweedRuleType::HamzatWasl, style),
                            context: get_context(verse_chars, i, 3),
                        });
                    }
                }
            }
        }

        i += 1;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Al-Ishmam (الإشمام) / Al-Ikhtilas (الاختلاس)
// Occurs uniquely in Surah Yusuf [12:11] in the word (تَأْمَنَّا / تَامَ۬نَّا).
// Marked in Uthmani script with U+06EC (filled dot) or U+06EB (open diamond)
// above/below the Noon/Meem.
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) fn detect_ishmam_rules_indexed(
    verse_chars: &[char],
    index: &VerseIndex,
    matches: &mut Vec<RuleMatch>,
    style: RecitationStyle,
) {
    let mut i = 0;
    while i < verse_chars.len() {
        let ch = verse_chars[i];

        // 1. Direct detection via Uthmani Ishmam/Tashil mark U+06EC or U+06EB on/adjacent to Noon
        if ch == '\u{06EC}' || ch == '\u{06EB}' {
            if let Some(target_idx) = index.next_letter_after(i).or_else(|| index.prev_letter_before(i)) {
                let target_ch = verse_chars[target_idx];
                if target_ch == 'ن' {
                    let start_idx = target_idx;
                    let mut end_idx = target_idx.max(i) + 1;
                    while end_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[end_idx]) {
                        end_idx += 1;
                    }
                    if !matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::Ishmam && m.start_index == start_idx) {
                        matches.push(RuleMatch {
                            start_index: start_idx,
                            end_index: end_idx,
                            target_letter: 'ن',
                            following_letter: None,
                            rule: TajweedRule::from_type(TajweedRuleType::Ishmam, style),
                            context: get_context(verse_chars, target_idx, 3),
                        });
                    }
                }
            }
        }

        // 2. Word-level detection for تأمنا / تامنا (تَأْمَنَّا / تَامَنَّا)
        if ch == 'ت' && i + 3 < verse_chars.len() {
            if let Some(second_idx) = index.next_letter_after(i) {
                let second_ch = verse_chars[second_idx];
                if matches!(second_ch, 'ا' | 'أ' | 'ء' | 'ـ' | '\u{0670}' | '\u{06E4}') {
                    if let Some(mim_idx) = index.next_letter_after(second_idx) {
                        if verse_chars[mim_idx] == 'م' {
                            if let Some(noon_idx) = index.next_letter_after(mim_idx) {
                                if verse_chars[noon_idx] == 'ن' && index.has_shadda_after(noon_idx) {
                                    let mut end_idx = noon_idx + 1;
                                    while end_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[end_idx]) {
                                        end_idx += 1;
                                    }
                                    let start_idx = noon_idx;
                                    if !matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::Ishmam && (m.start_index == start_idx || m.start_index <= mim_idx)) {
                                        matches.push(RuleMatch {
                                            start_index: start_idx,
                                            end_index: end_idx,
                                            target_letter: 'ن',
                                            following_letter: None,
                                            rule: TajweedRule::from_type(TajweedRuleType::Ishmam, style),
                                            context: get_context(verse_chars, noon_idx, 3),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        i += 1;
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
