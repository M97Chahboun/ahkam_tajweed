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

/// Check if character is Sukun (سكون) - U+0652
pub fn is_sukun(c: char) -> bool {
    c == '\u{0652}'
}

/// Check if character is Tanwin (تنوين) - includes Fathatan, Dammatan, Kasratan
pub fn is_tanwin(c: char) -> bool {
    matches!(c, '\u{064B}' | '\u{064C}' | '\u{064D}')
}

/// Check if character is Shadda (شدة) - U+0651
pub fn is_shadda(c: char) -> bool {
    c == '\u{0651}'
}

/// Check if character is a vowel (Fatha, Damma, Kasra)
pub fn is_vowel(c: char) -> bool {
    matches!(c, '\u{064E}' | '\u{064C}' | '\u{0650}')
}

/// Check if character is a Hamza (همزة) in any form
pub fn is_hamza(c: char) -> bool {
    matches!(c, 'أ' | 'ؤ' | 'ئ' | 'ء')
}

/// Get the vowel that precedes or immediately follows a character
///
/// This function checks:
/// 1. First, if there's a vowel immediately following the character (in diacritics)
/// 2. If not, looks backward for the vowel before the character
pub fn get_preceding_vowel(verse_chars: &[char], index: usize) -> Option<char> {
    if index == 0 {
        return None;
    }

    // First, check if there's a vowel immediately following the character (in diacritics)
    if index + 1 < verse_chars.len() && is_vowel(verse_chars[index + 1]) {
        return Some(verse_chars[index + 1]);
    }

    // If not, look backwards for the vowel before the character
    let mut idx = index - 1;
    loop {
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

/// Check if a madd letter is at the end of a word
pub fn is_word_end(verse_chars: &[char], current_index: usize) -> bool {
    let mut idx = current_index + 1;
    while idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[idx]) {
        if verse_chars[idx].is_whitespace() {
            return true;
        }
        idx += 1;
    }
    idx >= verse_chars.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vowel() {
        assert!(is_vowel('\u{064E}')); // Fatha
        assert!(is_vowel('\u{064C}')); // Damma
        assert!(is_vowel('\u{0650}')); // Kasra
        assert!(!is_vowel('ا'));
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
