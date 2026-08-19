//! Utility functions for Arabic character and diacritic handling
//!
//! This module provides helper functions for:
//! - Identifying diacritical marks and special characters
//! - Vowel detection and positioning
//! - Character classification
//! - Context extraction for display

/// Check if character should be ignored in Tajweed analysis
pub fn is_tajweed_ignorable(c: char) -> bool {
    matches!(
        c,
        '\u{064B}'..='\u{065F}' // Diacritics
            | '\u{0670}' // Alif Khanjareeya
            | '\u{0640}' // Tatweel
            | '\u{06D6}'..='\u{06DC}' // Additional marks
            | '\u{06DF}'..='\u{06E8}'
            | '\u{06EA}'..='\u{06ED}'
            | '\u{200C}' | '\u{200D}' // Zero-width characters
            | ' ' | '\t' | '\n' | '\r' // Whitespace
    )
}

/// Arabic letters used for rule detection (basic letters + common variants).
pub const ARABIC_LETTERS: &str =
    "ءأإآؤئٱابةتثجحخدذرزسشصضطظعغفقكلمنهويىةي\u{06CC}\u{06A9}";

/// Check if character is an Arabic letter relevant for Tajweed rules
pub fn is_arabic_letter(c: char) -> bool {
    ARABIC_LETTERS.contains(c)
}

/// Diacritic mask bit for Fatha.
pub const DIAC_FATHA: u8 = 1 << 0;
/// Diacritic mask bit for Damma.
pub const DIAC_DAMMA: u8 = 1 << 1;
/// Diacritic mask bit for Kasra.
pub const DIAC_KASRA: u8 = 1 << 2;
/// Diacritic mask bit for Sukun.
pub const DIAC_SUKUN: u8 = 1 << 3;
/// Diacritic mask bit for Shadda.
pub const DIAC_SHADDA: u8 = 1 << 4;
/// Diacritic mask bit for any Tanwin.
pub const DIAC_TANWIN: u8 = 1 << 5;
/// Diacritic mask for any recognized mark.
pub const DIAC_ANY: u8 =
    DIAC_FATHA | DIAC_DAMMA | DIAC_KASRA | DIAC_SUKUN | DIAC_SHADDA | DIAC_TANWIN;

fn diacritic_bit(c: char) -> u8 {
    match c {
        '\u{064E}' => DIAC_FATHA,
        '\u{064F}' => DIAC_DAMMA,
        '\u{0650}' => DIAC_KASRA,
        '\u{0652}' | '\u{06E1}' => DIAC_SUKUN,
        '\u{0651}' => DIAC_SHADDA,
        '\u{064B}' | '\u{064C}' | '\u{064D}' | '\u{0657}' | '\u{0658}' | '\u{065E}' | '\u{08F0}'..='\u{08F2}' => DIAC_TANWIN,
        _ => 0,
    }
}

/// Precomputed index over a verse for O(1) lookups during rule detection.
pub struct VerseIndex<'a> {
    chars: &'a [char],
    prev_letter: Vec<Option<usize>>,
    next_letter: Vec<Option<usize>>,
    boundary_prefix: Vec<usize>,
    diacritic_mask: Vec<u8>,
}

impl<'a> VerseIndex<'a> {
    /// Build an index for the given verse characters.
    pub fn new(chars: &'a [char]) -> Self {
        let len = chars.len();
        let mut prev_letter = vec![None; len];
        let mut next_letter = vec![None; len];
        let mut boundary_prefix = vec![0; len + 1];
        let mut diacritic_mask = vec![0; len];

        let mut last_letter: Option<usize> = None;
        let mut current_letter: Option<usize> = None;

        for (i, &c) in chars.iter().enumerate() {
            boundary_prefix[i + 1] = boundary_prefix[i] + if is_word_boundary(c) { 1 } else { 0 };

            if is_word_boundary(c) {
                current_letter = None;
            }

            if is_arabic_letter(c) {
                last_letter = Some(i);
                current_letter = Some(i);
            } else {
                let mask = diacritic_bit(c);
                if mask != 0 {
                    if let Some(letter_idx) = current_letter {
                        diacritic_mask[letter_idx] |= mask;
                    }
                } else if !is_tajweed_ignorable(c) {
                    current_letter = None;
                }
            }

            prev_letter[i] = last_letter;
        }

        let mut next_letter_idx: Option<usize> = None;
        for i in (0..len).rev() {
            if is_arabic_letter(chars[i]) {
                next_letter_idx = Some(i);
            }
            next_letter[i] = next_letter_idx;
        }

        VerseIndex {
            chars,
            prev_letter,
            next_letter,
            boundary_prefix,
            diacritic_mask,
        }
    }

    /// Return true if any word boundary exists between two indices.
    pub fn has_boundary_between(&self, start: usize, end: usize) -> bool {
        let start = start.min(self.chars.len());
        let end = end.min(self.chars.len());
        self.boundary_prefix[end] > self.boundary_prefix[start]
    }

    /// Return the next Arabic letter index after the given position.
    pub fn next_letter_after(&self, idx: usize) -> Option<usize> {
        if idx + 1 >= self.chars.len() {
            None
        } else {
            self.next_letter[idx + 1]
        }
    }

    /// Return the previous Arabic letter index before the given position.
    pub fn prev_letter_before(&self, idx: usize) -> Option<usize> {
        if idx == 0 {
            None
        } else {
            self.prev_letter[idx - 1]
        }
    }

    /// Return true if any diacritic in `mask` exists after the given letter.
    pub fn has_diacritic_after_mask(&self, idx: usize, mask: u8) -> bool {
        self.diacritic_mask
            .get(idx)
            .map(|m| m & mask != 0)
            .unwrap_or(false)
    }

    /// Return true if the letter has a following Sukun.
    pub fn has_sukun_after(&self, idx: usize) -> bool {
        self.has_diacritic_after_mask(idx, DIAC_SUKUN)
    }

    /// Return true if the letter has a following Shadda.
    pub fn has_shadda_after(&self, idx: usize) -> bool {
        self.has_diacritic_after_mask(idx, DIAC_SHADDA)
    }

    /// Return true if the letter has a following Tanwin.
    pub fn has_tanwin_after(&self, idx: usize) -> bool {
        self.has_diacritic_after_mask(idx, DIAC_TANWIN)
    }

    /// Return the diacritic mask for the given letter.
    pub fn diacritic_mask_at(&self, idx: usize) -> u8 {
        *self.diacritic_mask.get(idx).unwrap_or(&0)
    }

    /// Return the effective vowel for a letter, searching current then previous letter in word.
    pub fn preceding_vowel(&self, idx: usize) -> Option<char> {
        let mask = *self.diacritic_mask.get(idx).unwrap_or(&0);
        if mask & DIAC_FATHA != 0 {
            return Some('\u{064E}');
        }
        if mask & DIAC_DAMMA != 0 {
            return Some('\u{064F}');
        }
        if mask & DIAC_KASRA != 0 {
            return Some('\u{0650}');
        }

        let prev_idx = self.prev_letter_before(idx)?;
        if self.has_boundary_between(prev_idx + 1, idx) {
            return None;
        }

        let prev_mask = *self.diacritic_mask.get(prev_idx).unwrap_or(&0);
        if prev_mask & DIAC_FATHA != 0 {
            return Some('\u{064E}');
        }
        if prev_mask & DIAC_DAMMA != 0 {
            return Some('\u{064F}');
        }
        if prev_mask & DIAC_KASRA != 0 {
            return Some('\u{0650}');
        }

        None
    }

    /// Return true if the letter is at the end of a word.
    pub fn is_word_end(&self, idx: usize) -> bool {
        match self.next_letter_after(idx) {
            Some(next_idx) => self.has_boundary_between(idx + 1, next_idx),
            None => true,
        }
    }
}

/// Check if character is Sukun (سكون) - U+0652 or Uthmani U+06E1
pub fn is_sukun(c: char) -> bool {
    matches!(c, '\u{0652}' | '\u{06E1}')
}

/// Check if character is Tanwin (تنوين) - includes Fathatan, Dammatan, Kasratan
pub fn is_tanwin(c: char) -> bool {
    matches!(c, '\u{064B}' | '\u{064C}' | '\u{064D}' | '\u{0657}' | '\u{0658}' | '\u{065E}' | '\u{08F0}'..='\u{08F2}')
}

/// Check if character is Shadda (شدة) - U+0651
pub fn is_shadda(c: char) -> bool {
    c == '\u{0651}'
}

/// Check if character is a short vowel (Fatha, Damma, Kasra)
pub fn is_vowel(c: char) -> bool {
    matches!(c, '\u{064E}' | '\u{064F}' | '\u{0650}')
}

/// Check if character is a Hamza (همزة) in any form
pub fn is_hamza(c: char) -> bool {
    matches!(c, 'أ' | 'إ' | 'ؤ' | 'ئ' | 'ء' | 'آ')
}

/// Get the vowel that precedes or immediately follows a character
///
/// This function checks:
/// 1. First, if there's a vowel immediately following the character (in diacritics)
/// 2. If not, looks backward for the vowel before the character
pub fn get_preceding_vowel(verse_chars: &[char], index: usize) -> Option<char> {
    // First, check if there's a vowel immediately following the character (in diacritics)
    if index + 1 < verse_chars.len() && is_vowel(verse_chars[index + 1]) {
        return Some(verse_chars[index + 1]);
    }

    if index == 0 {
        return None;
    }

    // If not, look backwards for the vowel before the character
    let mut idx = index - 1;
    loop {
        if is_word_boundary(verse_chars[idx]) {
            return None;
        }
        if is_vowel(verse_chars[idx]) {
            return Some(verse_chars[idx]);
        }
        if !is_tajweed_ignorable(verse_chars[idx]) && verse_chars[idx] != '\u{0651}' {
            return None;
        }
        if idx == 0 {
            break;
        }
        idx -= 1;
    }
    None
}

/// Get context around a position in the verse
///
/// Returns a string of characters within `window` positions of the target
pub fn get_context(verse_chars: &[char], index: usize, window: usize) -> String {
    let start = index.saturating_sub(window);
    let end = (index + window + 1).min(verse_chars.len());
    verse_chars[start..end].iter().collect()
}

/// Check if a character is a following hamza in the verse
pub fn is_following_hamza(verse_chars: &[char], start_idx: usize) -> bool {
    let mut idx = start_idx;
    while idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[idx]) {
        idx += 1;
    }
    idx < verse_chars.len() && is_hamza(verse_chars[idx])
}

/// Check if a character has a following shadda (doubled letter marker)
pub fn is_following_shadda(verse_chars: &[char], start_idx: usize) -> bool {
    let mut idx = start_idx;
    while idx < verse_chars.len() {
        let c = verse_chars[idx];
        if is_shadda(c) {
            return true;
        }
        // Check if we hit a non-diacritic character (which would mean no shadda follows)
        if !is_tajweed_ignorable(c) && !is_vowel(c) {
            return false;
        }
        idx += 1;
    }
    false
}

/// Check if a character is a word boundary (space, punctuation, waqf mark)
pub fn is_word_boundary(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r') || is_punctuation(c)
}

/// Check if there is a word boundary between two indices (exclusive)
pub fn has_word_boundary_between(verse_chars: &[char], start: usize, end: usize) -> bool {
    let end = end.min(verse_chars.len());
    for i in start..end {
        if is_word_boundary(verse_chars[i]) {
            return true;
        }
    }
    false
}

/// Find the next Arabic letter at or after `start_idx`.
pub fn next_arabic_letter(verse_chars: &[char], start_idx: usize) -> Option<(usize, char)> {
    let mut idx = start_idx;
    while idx < verse_chars.len() {
        let c = verse_chars[idx];
        if is_arabic_letter(c) {
            return Some((idx, c));
        }
        idx += 1;
    }
    None
}

/// Find the previous Arabic letter before `start_idx`.
pub fn prev_arabic_letter(verse_chars: &[char], start_idx: usize) -> Option<(usize, char)> {
    if start_idx == 0 {
        return None;
    }

    let mut idx = start_idx - 1;
    loop {
        let c = verse_chars[idx];
        if is_arabic_letter(c) {
            return Some((idx, c));
        }
        if idx == 0 {
            break;
        }
        idx -= 1;
    }
    None
}

/// Check if a diacritic satisfying `predicate` appears after `start_idx`.
pub fn has_diacritic_after<F>(verse_chars: &[char], start_idx: usize, predicate: F) -> bool
where
    F: Fn(char) -> bool,
{
    let mut idx = start_idx + 1;
    while idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[idx]) {
        if predicate(verse_chars[idx]) {
            return true;
        }
        idx += 1;
    }
    false
}

/// Check if a madd letter is at the end of a word
pub fn is_word_end(verse_chars: &[char], current_index: usize) -> bool {
    let mut idx = current_index + 1;
    while idx < verse_chars.len() {
        let c = verse_chars[idx];
        if is_word_boundary(c) {
            return true;
        }
        if is_arabic_letter(c) {
            return false;
        }
        if !is_tajweed_ignorable(c) {
            return true;
        }
        idx += 1;
    }
    true
}

/// Check if a character is punctuation
///
/// This function identifies both standard Arabic/Universal punctuation marks
/// and Quranic stop signs (Waqf marks).
pub fn is_punctuation(c: char) -> bool {
    match c {
        // Standard Arabic/Universal punctuation
        ' ' | '.' | '،' | '؛' | '؟' | '!' => true,

        // Quranic Stop Signs (Waqf)
        '\u{0615}' // Small High Tah (ط)
        | '\u{0617}' // Small High Zain (ز)
        | '\u{0618}' // Small High Meem Isolated (م)
        | '\u{0619}' // Small High Seen (س)
        | '\u{06d6}' // Small High Ligature Sad-Lam-Alef (صلى)
        | '\u{06d7}' // Small High Ligature Qaf-Lam-Alef (قلى)
        | '\u{06d8}' // Small High Meem Initial (ۘ)
        | '\u{06d9}' // Small High Lam-Alef (ۙ)
        | '\u{06da}' // Small High Jeem (ج)
        | '\u{06db}' // Small High Three Dots (ۛ)
        | '\u{06dd}' // End of Ayah marker (۝)
        => true,

        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vowel() {
        assert!(is_vowel('\u{064E}')); // Fatha
        assert!(is_vowel('\u{064F}')); // Damma
        assert!(is_vowel('\u{0650}')); // Kasra
        assert!(!is_vowel('ا'));
    }

    #[test]
    fn test_word_boundary_helpers() {
        assert!(is_word_boundary(' '));
        assert!(is_word_boundary('،')); // Arabic comma
        assert!(is_word_boundary('\u{06DA}')); // Waqf mark (Jeem)
        assert!(!is_word_boundary('ب'));

        let chars: Vec<char> = "بِسْمِ،اللَّهِ".chars().collect();
        // boundary between meem and lam after the comma
        assert!(has_word_boundary_between(&chars, 5, 7));
    }

    #[test]
    fn test_letter_navigation_helpers() {
        let chars: Vec<char> = "بِسْمِ،".chars().collect();
        assert_eq!(next_arabic_letter(&chars, 0), Some((0, 'ب')));
        assert_eq!(next_arabic_letter(&chars, 1), Some((2, 'س')));
        assert_eq!(prev_arabic_letter(&chars, chars.len()), Some((4, 'م')));
    }

    #[test]
    fn test_has_diacritic_after() {
        let chars: Vec<char> = "رَ رْ".chars().collect();
        let ra_fatha_idx = 0;
        let ra_sukun_idx = 3;
        assert!(has_diacritic_after(&chars, ra_fatha_idx, |c| c == '\u{064E}'));
        assert!(has_diacritic_after(&chars, ra_sukun_idx, is_sukun));
    }

    #[test]
    fn test_get_preceding_vowel_stops_at_boundary() {
        let chars: Vec<char> = "بَ ا".chars().collect();
        let alif_idx = 3;
        assert_eq!(get_preceding_vowel(&chars, alif_idx), None);
    }

    #[test]
    fn test_is_word_end_with_punctuation() {
        let chars: Vec<char> = "نْ،".chars().collect();
        let nun_idx = 0;
        assert!(is_word_end(&chars, nun_idx));
    }

    #[test]
    fn test_is_hamza() {
        assert!(is_hamza('أ'));
        assert!(is_hamza('ؤ'));
        assert!(is_hamza('ئ'));
        assert!(is_hamza('ء'));
        assert!(!is_hamza('ا'));
    }

    #[test]
    fn test_is_tajweed_ignorable() {
        assert!(is_tajweed_ignorable('\u{064B}')); // Fathatan
        assert!(is_tajweed_ignorable(' '));
        assert!(!is_tajweed_ignorable('ن'));
    }
}
