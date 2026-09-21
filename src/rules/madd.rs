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
    // The superscript Alef (ٰ) is a Madd Alif in its own right: it is what is
    // actually read in رَحْمَٰن، ٱلصَّلَوٰة، عَلَىٰ, where the letter it sits on is
    // either a plain consonant or a silent seat.
    const MADD_LETTERS: [char; 5] = ['ا', 'و', 'ي', '\u{06CC}', '\u{0670}'];

    let mut i = 0;
    while i < verse_chars.len() {
        let current_char = verse_chars[i];

        // The Alif of the definite article carries no Madd. Only a *genuine*
        // article counts: the Alif of ٱلْأَمْثَالَ or قَالَ لَهُ is an ordinary Madd
        // Alif that happens to sit before a Lam.
        if current_char == 'ا' || current_char == '\u{0671}' {
            if let Some(next_idx) = index.next_letter_after(i) {
                if verse_chars[next_idx] == 'ل'
                    && crate::rules::lam_al_tarif::article_start(verse_chars, index, next_idx)
                        == Some(i)
                {
                    i += 1;
                    continue;
                }
            }
        }

        // A letter carrying a silence mark is a spelling artefact — the Alif
        // of أَنَا۠ or of قَالُوا۟ is written but not read, so it can neither be
        // lengthened nor separate a Madd from a following Hamza.
        if is_silent_letter(verse_chars, i) {
            i += 1;
            continue;
        }

        if MADD_LETTERS.contains(&current_char) || current_char == 'آ' {
            // A Madd letter carries no vowel of its own — it is lengthened by
            // the vowel on the letter before it. A Waw or Ya that carries a
            // Fatha/Damma/Kasra/Tanwin or a Shadda is a consonant
            // (وُسْعَهَا، إِيَّاكَ), never a Madd letter.
            const OWN_VOWEL: u8 = DIAC_FATHA | DIAC_DAMMA | DIAC_KASRA | DIAC_TANWIN | DIAC_SHADDA;
            if index.diacritic_mask_at(i) & OWN_VOWEL != 0 && current_char != 'آ' {
                i += 1;
                continue;
            }
            let vowel = index.vowel_on_previous_letter(i);
            let has_basic_madd = if current_char == 'آ' {
                true // Alif Madd is always considered valid for madd
            } else {
                match (current_char, vowel) {
                    // The superscript Alef is written *only* as a Madd Alif —
                    // it never appears as anything else, and the letter it sits
                    // on may be a silent seat carrying no vowel of its own
                    // (ٱلصَّلَوٰة، أَدْرَىٰكَ), so there is nothing to check.
                    ('\u{0670}', _) => true,
                    ('ا', Some('\u{064E}')) => true, // Alif needs Fatha for basic madd
                    ('و', Some('\u{064F}')) => true, // Waw needs Damma for basic madd
                    ('ي' | '\u{06CC}', Some('\u{0650}')) => true, // Ya needs Kasra for basic madd
                    _ => false,
                }
            };

            let has_lin_candidate =
                matches!(current_char, 'و' | 'ي' | '\u{06CC}') && vowel == Some('\u{064E}');

            if has_basic_madd || has_lin_candidate || current_char == 'آ' {
                if let Some(madd_type) = detect_madd(current_char, verse_chars, index, i) {
                    // Calculate end index to include diacritics
                    let mut end_idx = i + 1;
                    while end_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[end_idx])
                    {
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

/// True when the Madd letter at `idx` ends a *grammatical* word that the
/// script writes joined to the next one.
///
/// The vocative يَا and the ها of التنبيه are written attached to what they
/// introduce — `يَٰٓأَيُّهَا`, `يَٰٓأَهْلَ`, `هَٰٓأَنتُمْ` — but they are separate words,
/// so the Hamza that follows opens a new word and the Madd is Munfasil, not
/// Muttasil. Both are written as a bare ي / ه with a superscript Alef.
pub(crate) fn is_detached_particle(verse_chars: &[char], index: &VerseIndex, idx: usize) -> bool {
    if verse_chars.get(idx) != Some(&'\u{0670}') {
        return false;
    }
    let Some(prev) = index.prev_letter_before(idx) else {
        return false;
    };
    let opens_word = match index.prev_letter_before(prev) {
        None => true,
        Some(p) => index.has_boundary_between(p + 1, prev),
    };
    if !opens_word {
        return false;
    }
    match verse_chars[prev] {
        'ي' => true,
        // ها التنبيه only separates before a word that itself opens with a
        // Hamza on an Alif seat (هَٰٓأَنتُمْ). هَٰٓؤُلَآءِ has fused into one word
        // and is read as a single Muttasil.
        'ه' => index
            .next_pronounced_letter(idx)
            .is_some_and(|n| matches!(verse_chars[n], 'أ' | 'إ' | 'ا' | 'آ')),
        _ => false,
    }
}

/// True when the letter at `idx` carries no diacritic of its own.
fn is_bare(index: &VerseIndex, idx: usize) -> bool {
    index.diacritic_mask_at(idx) & crate::utils::DIAC_ANY == 0
}

/// True when the recitation stops on the letter at `idx` — nothing but a Waqf
/// sign follows it, or it is the last letter of the verse.
///
/// A verse is a stopping place in its own right, which is why the final Madd
/// letter of nearly every ayah is a Madd 'Arid li-Sukun.
fn stops_after(verse_chars: &[char], index: &VerseIndex, idx: usize) -> bool {
    match index.next_letter_after(idx) {
        None => true,
        Some(next) => verse_chars[idx + 1..next]
            .iter()
            .any(|&c| matches!(c, '\u{06D6}'..='\u{06DD}' | '\u{FD3E}' | '\u{FD3F}')),
    }
}

fn detect_madd(
    madd_letter: char,
    verse_chars: &[char],
    index: &VerseIndex,
    current_index: usize,
) -> Option<TajweedRuleType> {
    let preceding_vowel = index.vowel_on_previous_letter(current_index);

    // A Waw/Ya carrying a Sukun after a Fatha is a *lin* letter (خَوْف، بَيْت).
    // It is only *lengthened* when the reciter stops on the letter after it,
    // whose Sukun is then temporary — مد اللين العارض للسكون. Read on, a lin
    // letter is simply pronounced softly, with no madd.
    if matches!(madd_letter, 'و' | 'ي' | '\u{06CC}') && preceding_vowel == Some('\u{064E}') {
        if !index.has_sukun_after(current_index) && !is_bare(index, current_index) {
            return None;
        }
        let next = index.next_pronounced_letter(current_index)?;
        if !index.has_boundary_between(current_index + 1, next)
            && stops_after(verse_chars, index, next)
        {
            return Some(TajweedRuleType::MaddLin);
        }
        return None;
    }

    // 1. Madd Lazim Kalimi — the Madd letter is followed, *in the same word*,
    // by a letter carrying a permanent Shadda (muthaqqal: ٱلضَّآلِّينَ, دَآبَّة)
    // or Sukun (mukhaffaf: ءَآلْـَٰٔنَ).
    //
    // Both halves matter. A Shadda opening the *next* word (وَمِمَّا رَّزَقْنَٰهُمْ)
    // leaves the Madd natural, and a Shadda on the Madd letter itself
    // (إِيَّاكَ) is not a Sukun after it at all.
    if let Some(next_idx) = index.next_pronounced_letter(current_index) {
        if !index.has_boundary_between(current_index + 1, next_idx)
            && (index.has_shadda_after(next_idx) || index.has_sukun_after(next_idx))
        {
            return Some(TajweedRuleType::MaddLazim);
        }
    }

    // 2. Check for Madd Muttasil/Munfasil: madd letter followed by hamza
    if let Some(next_idx) = index.next_letter_after(current_index) {
        if is_hamza(verse_chars[next_idx]) {
            // Check if there's a word boundary between madd letter and hamza
            let has_word_boundary = index.has_boundary_between(current_index + 1, next_idx)
                || is_detached_particle(verse_chars, index, current_index);

            return if has_word_boundary {
                Some(TajweedRuleType::MaddMunfasil)
            } else {
                Some(TajweedRuleType::MaddMuttasil)
            };
        }
    }

    // 3. Madd Arid li-Sukun — the Madd letter is followed, in the same word,
    //    by the letter the reciter stops on. The Sukun is *temporary*: it only
    //    exists because the recitation halts there, so the letter has to be
    //    the last one before a Waqf sign or the end of the verse.
    //    Source: quranica.com — "Only occurs at Waqf; if continuing, reverts
    //    to MaddTabeei."
    if let Some(next_idx) = index.next_pronounced_letter(current_index) {
        if !index.has_boundary_between(current_index + 1, next_idx)
            && stops_after(verse_chars, index, next_idx)
        {
            return Some(TajweedRuleType::MaddArid);
        }
    }

    // 4. Madd Badal — a Hamza *before* the Madd letter, in the same word
    //    (ءَامَنَ، أُوتُوا۟). It ranks below the stop: at a Waqf the temporary
    //    Sukun is what sets the length.
    if let Some(prev_idx) = index.prev_letter_before(current_index) {
        if is_hamza(verse_chars[prev_idx])
            && !index.has_boundary_between(prev_idx + 1, current_index)
        {
            return Some(TajweedRuleType::MaddBadal);
        }
    }

    // 5. Check if Madd letter is dropped in continuous reading (Wasl) before a Saakin letter / Hamzat Wasl
    // (حذف حرف المد لفظاً عند التقاء الساكنين في الوصل — مثل: في الجحيم، قالوا ابنوا، إذا الشمس)
    if is_madd_dropped_before_sakin(verse_chars, index, current_index) {
        return None;
    }

    // 6. Default: Natural madd (Tabee'i) - if conditions are met
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
                        return is_word_starting_with_wasl_or_sakin(
                            verse_chars,
                            index,
                            after_alif_idx,
                        );
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
        && !index.has_diacritic_after_mask(first_letter_idx, DIAC_FATHA | DIAC_DAMMA | DIAC_KASRA)
    {
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
    // When the text spells the Silah out with the small Waw / small Ya, those
    // marks are the authority on where it occurs, and the word-shape heuristic
    // below would only add false positives — a word-final voweled Haa is very
    // often part of the root (ٱللَّهِ، وَجْهِ) and not a pronoun at all.
    // A script that marks its own orthography — Alif Wasla, the superscript
    // Alef, the silence marks, the Silah marks — states where the Silah is.
    // Where it does, the *absence* of a Silah mark means there is none, and the
    // word-shape heuristic below would only add false positives: a word-final
    // voweled Haa is very often part of the root (ٱللَّهِ، تَنتَهِ) or a pronoun
    // that simply is not lengthened (أَنزَلْنَٰهُ).
    let silah_is_written = verse_chars.iter().any(|c| {
        matches!(
            c,
            '\u{06E5}' | '\u{06E6}' | '\u{0670}' | '\u{0671}' | '\u{06DF}' | '\u{06E0}'
        )
    });

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

        // 2. Haa Al-Kinayah (ـهُ / ـهِ) at word end between two voweled letters
        if ch == 'ه' && !silah_is_written {
            if let Some(next_letter_idx) = index.next_letter_after(i) {
                if index.has_boundary_between(i + 1, next_letter_idx) {
                    if let Some(prev_letter_idx) = index.prev_letter_before(i) {
                        if !index.has_boundary_between(prev_letter_idx + 1, i) {
                            // The Haa of the name of Allah (ٱللَّهِ، لِلَّهِ) is
                            // part of the word, not a pronoun — no Silah.
                            let is_lafz_al_jalalah = verse_chars[prev_letter_idx] == 'ل'
                                && index.has_shadda_after(prev_letter_idx)
                                && index
                                    .prev_letter_before(prev_letter_idx)
                                    .is_some_and(|p| verse_chars[p] == 'ل');
                            if is_lafz_al_jalalah {
                                i += 1;
                                continue;
                            }
                            let has_prev_vowel = index.has_diacritic_after_mask(
                                prev_letter_idx,
                                DIAC_FATHA | DIAC_DAMMA | DIAC_KASRA,
                            );
                            let has_ha_vowel =
                                index.has_diacritic_after_mask(i, DIAC_DAMMA | DIAC_KASRA);
                            let has_next_vowel = index.has_diacritic_after_mask(
                                next_letter_idx,
                                DIAC_FATHA | DIAC_DAMMA | DIAC_KASRA | DIAC_TANWIN,
                            );
                            let is_next_wasl = verse_chars[next_letter_idx] == '\u{0671}'
                                || (verse_chars[next_letter_idx] == 'ا'
                                    && !index.has_diacritic_after_mask(
                                        next_letter_idx,
                                        DIAC_FATHA | DIAC_DAMMA | DIAC_KASRA,
                                    ));

                            if has_prev_vowel
                                && has_ha_vowel
                                && has_next_vowel
                                && !is_next_wasl
                                && !matches.iter().any(|m| {
                                    (m.rule.rule_type == TajweedRuleType::MaddSilah
                                        || m.rule.rule_type == TajweedRuleType::MaddMunfasil)
                                        && m.start_index >= i
                                        && m.start_index <= i + 2
                                })
                            {
                                let is_kubra = is_hamza(verse_chars[next_letter_idx]);
                                let rule_type = if is_kubra {
                                    TajweedRuleType::MaddMunfasil
                                } else {
                                    TajweedRuleType::MaddSilah
                                };
                                let mut end_idx = i + 1;
                                while end_idx < verse_chars.len()
                                    && is_tajweed_ignorable(verse_chars[end_idx])
                                {
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
        let chars: Vec<char> = "قُولُ لَهُمْ".chars().collect();
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
