//! Main Tajweed rule processor
//!
//! This module contains the core `TajweedProcessor` struct that analyzes Quranic verses
//! and detects applicable Tajweed rules.

use crate::types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
use crate::utils::*;
use std::collections::HashMap;

/// The main Tajweed processor for analyzing Quranic verses
///
/// This processor detects and classifies Tajweed rules according to the specified
/// recitation style (Warsh or Hafs). It handles:
/// - Noon/Mim Sakinah rules
/// - Lam Al-Ta'rif rules
/// - Madd rules
/// - Qalqalah rules
/// - Ra emphasis rules
/// - Special rules like Tafkhim Lafz Al-Jalalah
pub struct TajweedProcessor {
    style: RecitationStyle,

    // Letter mappings for quick lookup
    izhar_halqi_map: HashMap<char, TajweedRuleType>,
    idgham_bi_ghunnah_map: HashMap<char, TajweedRuleType>,
    idgham_bila_ghunnah_map: HashMap<char, TajweedRuleType>,
    ikhfaa_letters: Vec<char>,
    iqlab_letter: char,

    // Mim Sakinah
    ikhfaa_shafawi_letter: char,
    idgham_shafawi_letter: char,

    // Lam Al-Ta'rif
    izhar_qamari_map: HashMap<char, TajweedRuleType>,
    idgham_shamsi_map: HashMap<char, TajweedRuleType>,

    // Madd and Qalqalah
    madd_letters: Vec<char>,
    qalqalah_letters: Vec<char>,
}

impl TajweedProcessor {
    /// Create a new TajweedProcessor for the specified recitation style
    pub fn new(style: RecitationStyle) -> Self {
        // Izhar Halqi Letters (حروف الحلق)
        const IZHAR_HALQI_LETTERS: [char; 6] = ['أ', 'ه', 'ع', 'ح', 'غ', 'خ'];

        // Idgham Letters
        const IDGHAM_BI_GHUNNAH_LETTERS: [char; 4] = ['ي', 'ن', 'م', 'و'];
        const IDGHAM_BILA_GHUNNAH_LETTERS: [char; 2] = ['ل', 'ر'];

        // Ikhfaa Letters (15 letters)
        const IKHFAA_LETTERS: [char; 15] = [
            'ص', 'ذ', 'ث', 'ك', 'ج', 'ش', 'ق', 'س', 'د', 'ط', 'ز', 'ف', 'ت', 'ض', 'ظ',
        ];

        const IQLAB_LETTER: char = 'ب';
        const IKHFAA_SHAFAWI_LETTER: char = 'ب';
        const IDGHAM_SHAFAWI_LETTER: char = 'م';

        // Lunar Letters (حروف الإظهار القمري)
        const IZHAR_QAMARI_LETTERS: [char; 14] = [
            'ا', 'ب', 'غ', 'ح', 'ج', 'ك', 'و', 'خ', 'ف', 'ع', 'ق', 'ي', 'م', 'ه',
        ];

        // Solar Letters (حروف الإدغام الشمسي)
        const IDGHAM_SHAMSI_LETTERS: [char; 14] = [
            'ت', 'ث', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ل', 'ن',
        ];

        // Madd Letters (حروف المد)
        const MADD_LETTERS: [char; 3] = ['ا', 'و', 'ي'];

        // Qalqalah Letters (حروف القلقلة): ق ط ب ج د
        const QALQALAH_LETTERS: [char; 5] = ['ق', 'ط', 'ب', 'ج', 'د'];

        // Build maps
        let izhar_halqi_map = IZHAR_HALQI_LETTERS
            .iter()
            .map(|&l| (l, TajweedRuleType::IzharHalqi))
            .collect();

        let idgham_bi_ghunnah_map = IDGHAM_BI_GHUNNAH_LETTERS
            .iter()
            .map(|&l| (l, TajweedRuleType::IdghamBiGhunnah))
            .collect();

        let idgham_bila_ghunnah_map = IDGHAM_BILA_GHUNNAH_LETTERS
            .iter()
            .map(|&l| (l, TajweedRuleType::IdghamBilaGhunnah))
            .collect();

        let izhar_qamari_map = IZHAR_QAMARI_LETTERS
            .iter()
            .map(|&l| (l, TajweedRuleType::IzharQamari))
            .collect();

        let idgham_shamsi_map = IDGHAM_SHAMSI_LETTERS
            .iter()
            .map(|&l| (l, TajweedRuleType::IdghamShamsi))
            .collect();

        TajweedProcessor {
            style,
            izhar_halqi_map,
            idgham_bi_ghunnah_map,
            idgham_bila_ghunnah_map,
            ikhfaa_letters: IKHFAA_LETTERS.to_vec(),
            iqlab_letter: IQLAB_LETTER,
            ikhfaa_shafawi_letter: IKHFAA_SHAFAWI_LETTER,
            idgham_shafawi_letter: IDGHAM_SHAFAWI_LETTER,
            izhar_qamari_map,
            idgham_shamsi_map,
            madd_letters: MADD_LETTERS.to_vec(),
            qalqalah_letters: QALQALAH_LETTERS.to_vec(),
        }
    }

    /// Determine the rule for Noon Sakinah or Tanwin
    fn determine_rule_for_noon(
        &self,
        following_letter: char,
        is_same_word: bool,
    ) -> TajweedRuleType {
        // 1. Izhar Mutlaq (استثناء الإدغام)
        if is_same_word && (following_letter == 'ي' || following_letter == 'و') {
            return TajweedRuleType::IzharMutlaq;
        }

        // 2. Iqlab (الإقلاب)
        if following_letter == self.iqlab_letter {
            return TajweedRuleType::Iqlab;
        }

        // 3. Izhar Halqi (الإظهار الحلقي)
        if self.izhar_halqi_map.contains_key(&following_letter) {
            return TajweedRuleType::IzharHalqi;
        }

        // 4. Idgham bila Ghunnah (الإدغام بغير غنة)
        if self.idgham_bila_ghunnah_map.contains_key(&following_letter) {
            return TajweedRuleType::IdghamBilaGhunnah;
        }

        // 5. Idgham bi Ghunnah (الإدغام بغنة)
        if self.idgham_bi_ghunnah_map.contains_key(&following_letter) {
            return TajweedRuleType::IdghamBiGhunnah;
        }

        // 6. Ikhfaa Haqiqi (الإخفاء الحقيقي)
        if self.ikhfaa_letters.contains(&following_letter) {
            return TajweedRuleType::IkhfaaHaqiqi;
        }

        TajweedRuleType::NoRule
    }

    /// Determine the rule for Mim Sakinah
    fn determine_rule_for_mim(&self, following_letter: char) -> TajweedRuleType {
        // 1. Ikhfaa Shafawi (الإخفاء الشفوي) - before Ba
        if following_letter == self.ikhfaa_shafawi_letter {
            return TajweedRuleType::IkhfaaShafawi;
        }

        // 2. Idgham Shafawi (الإدغام الشفوي) - before Mim
        if following_letter == self.idgham_shafawi_letter {
            return TajweedRuleType::IdghamMithlayn;
        }

        // 3. Izhar Shafawi (الإظهار الشفوي) - before other letters
        const ARABIC_LETTERS: &str = "ءأبةتثجحخدذرزسشصضطظعغفقكلمنهوي";
        if ARABIC_LETTERS.contains(following_letter) {
            return TajweedRuleType::IzharShafawi;
        }

        TajweedRuleType::NoRule
    }

    /// Determine the rule for Lam Al-Ta'rif
    fn determine_rule_for_lam_al(&self, following_letter: char) -> TajweedRuleType {
        // 1. Izhar Qamari (الإظهار القمري)
        if self.izhar_qamari_map.contains_key(&following_letter) {
            return TajweedRuleType::IzharQamari;
        }

        // 2. Idgham Shamsi (الإدغام الشمسي)
        if self.idgham_shamsi_map.contains_key(&following_letter) {
            return TajweedRuleType::IdghamShamsi;
        }

        TajweedRuleType::NoRule
    }

    /// Detect Madd rules for a madd letter
    fn detect_madd(
        &self,
        madd_letter: char,
        verse_chars: &[char],
        current_index: usize,
    ) -> Option<TajweedRuleType> {
        let has_following_hamza = is_following_hamza(verse_chars, current_index + 1);
        let has_following_shadda = is_following_shadda(verse_chars, current_index + 1);
        let word_end = is_word_end(verse_chars, current_index);

        if has_following_shadda {
            // Madd Lazim (المد اللازم) - 6 harakaat always
            Some(TajweedRuleType::MaddLazim)
        } else if has_following_hamza {
            // Either Muttasil or Munfasil based on word boundary
            if word_end {
                Some(TajweedRuleType::MaddMunfasil)
            } else {
                Some(TajweedRuleType::MaddMuttasil)
            }
        } else if madd_letter == 'ي' || madd_letter == 'و' {
            self.detect_soft_madd(madd_letter, verse_chars, current_index, word_end)
        } else {
            // Alif - check for Badal
            if verse_chars[current_index] == 'آ' {
                Some(TajweedRuleType::MaddBadal)
            } else if current_index > 0 {
                let mut back_idx = current_index - 1;
                loop {
                    if !is_tajweed_ignorable(verse_chars[back_idx]) {
                        if is_hamza(verse_chars[back_idx]) {
                            return Some(TajweedRuleType::MaddBadal);
                        }
                        break;
                    }
                    if back_idx == 0 {
                        break;
                    }
                    back_idx -= 1;
                }
                if word_end {
                    Some(TajweedRuleType::MaddArid)
                } else {
                    Some(TajweedRuleType::MaddTabeei)
                }
            } else if word_end {
                Some(TajweedRuleType::MaddArid)
            } else {
                Some(TajweedRuleType::MaddTabeei)
            }
        }
    }

    /// Detect Soft madd types (for waaw and ya)
    fn detect_soft_madd(
        &self,
        _madd_letter: char,
        verse_chars: &[char],
        current_index: usize,
        word_end: bool,
    ) -> Option<TajweedRuleType> {
        // Check for Madd Lin (المد اللين)
        let mut next_idx = current_index + 1;
        while next_idx < verse_chars.len()
            && is_tajweed_ignorable(verse_chars[next_idx])
            && !is_sukun(verse_chars[next_idx])
        {
            next_idx += 1;
        }

        if next_idx < verse_chars.len() && is_sukun(verse_chars[next_idx]) {
            let mut after_sukun_idx = next_idx + 1;
            while after_sukun_idx < verse_chars.len()
                && is_tajweed_ignorable(verse_chars[after_sukun_idx])
            {
                after_sukun_idx += 1;
            }

            if after_sukun_idx < verse_chars.len() {
                let next_letter = verse_chars[after_sukun_idx];
                if next_letter == 'ل' || next_letter == 'ر' {
                    return Some(TajweedRuleType::MaddLin);
                }
            }
        }

        // Check for Madd Arid
        if word_end {
            Some(TajweedRuleType::MaddArid)
        } else {
            Some(TajweedRuleType::MaddTabeei)
        }
    }

    /// Detect Tafkhim Ra (تفخيم الراء)
    fn detect_tafkhim_ra(verse_chars: &[char], current_index: usize) -> Option<TajweedRuleType> {
        if let Some(vowel) = get_preceding_vowel(verse_chars, current_index) {
            match vowel {
                '\u{064E}' | '\u{064C}' => return Some(TajweedRuleType::TafkhimRa),
                '\u{0652}' => {
                    if current_index >= 2 {
                        let mut back_idx = current_index - 1;
                        while back_idx > 0 && is_tajweed_ignorable(verse_chars[back_idx]) {
                            back_idx -= 1;
                        }
                        if back_idx < current_index {
                            if let Some(prev_vowel) = get_preceding_vowel(verse_chars, back_idx) {
                                if matches!(prev_vowel, '\u{064E}' | '\u{064C}') {
                                    return Some(TajweedRuleType::TafkhimRa);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Detect Tarqeeq Ra (ترقيق الراء)
    fn detect_tarqeeq_ra(verse_chars: &[char], current_index: usize) -> Option<TajweedRuleType> {
        if let Some(vowel) = get_preceding_vowel(verse_chars, current_index) {
            match vowel {
                '\u{0650}' => return Some(TajweedRuleType::TarqeeqRa),
                '\u{0652}' => {
                    if current_index >= 2 {
                        let mut back_idx = current_index - 1;
                        while back_idx > 0 && is_tajweed_ignorable(verse_chars[back_idx]) {
                            back_idx -= 1;
                        }
                        if back_idx < current_index {
                            if let Some(prev_vowel) = get_preceding_vowel(verse_chars, back_idx) {
                                if prev_vowel == '\u{0650}' {
                                    return Some(TajweedRuleType::TarqeeqRa);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Detect Tafkhim Lafz Al-Jalala
    fn detect_tafkhim_lafuljalala(verse_chars: &[char], current_index: usize) -> bool {
        if current_index + 3 >= verse_chars.len() {
            return false;
        }

        if verse_chars[current_index] != 'ا' {
            return false;
        }

        let mut check_idx = current_index + 1;
        while check_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[check_idx]) {
            check_idx += 1;
        }

        if check_idx >= verse_chars.len() || verse_chars[check_idx] != 'ل' {
            return false;
        }

        check_idx += 1;
        while check_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[check_idx]) {
            check_idx += 1;
        }

        if check_idx >= verse_chars.len() {
            return false;
        }

        if verse_chars[check_idx] == 'ل' {
            check_idx += 1;
            while check_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[check_idx]) {
                check_idx += 1;
            }
            check_idx < verse_chars.len() && verse_chars[check_idx] == 'ه'
        } else {
            verse_chars[check_idx] == 'ه'
        }
    }

    /// Detect Qalqalah rules
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

    /// Process a Quranic verse and detect all applicable Tajweed rules
    ///
    /// Returns a vector of `RuleMatch` objects containing all detected rules
    pub fn process_verse(&self, verse: &str) -> Vec<RuleMatch> {
        let mut matches: Vec<RuleMatch> = Vec::new();
        let verse_chars: Vec<char> = verse.chars().collect();

        // First pass: Noon/Mim Sakinah rules
        self.detect_noon_mim_rules(&verse_chars, &mut matches);

        // Second pass: Madd rules
        self.detect_madd_rules(&verse_chars, &mut matches);

        // Third pass: Qalqalah rules
        self.detect_qalqalah_rules(&verse_chars, &mut matches);

        // Fourth pass: Ra emphasis rules
        self.detect_ra_rules(&verse_chars, &mut matches);

        // Fifth pass: Tafkhim Lafz Al-Jalalah
        self.detect_allah_name_rules(&verse_chars, &mut matches);

        matches
    }

    /// Detect Noon/Mim Sakinah and Lam Al-Ta'rif rules
    fn detect_noon_mim_rules(&self, verse_chars: &[char], matches: &mut Vec<RuleMatch>) {
        let mut i = 0;
        while i < verse_chars.len() {
            let current_char = verse_chars[i];

            // Noon or Mim with Sukun/Tanwin
            if current_char == 'ن' || current_char == 'م' {
                self.check_noon_mim(&verse_chars, i, matches);
            }

            // Tanwin handling
            if is_tanwin(current_char) {
                self.check_tanwin(&verse_chars, i, matches);
            }

            // Lam Al-Ta'rif
            if current_char == 'ا' && i + 1 < verse_chars.len() {
                self.check_lam_al_tarif(&verse_chars, i, matches);
            }

            i += 1;
        }
    }

    fn check_noon_mim(&self, verse_chars: &[char], i: usize, matches: &mut Vec<RuleMatch>) {
        let current_char = verse_chars[i];
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
                    self.determine_rule_for_noon(following_letter, is_same_word)
                } else {
                    self.determine_rule_for_mim(following_letter)
                };

                if rule_type != TajweedRuleType::NoRule {
                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: j,
                        target_letter: current_char,
                        following_letter: Some(following_letter),
                        rule: TajweedRule::from_type(rule_type, self.style),
                        context: get_context(&verse_chars, i, 3),
                    });
                }
            }
        }
    }

    fn check_tanwin(&self, verse_chars: &[char], i: usize, matches: &mut Vec<RuleMatch>) {
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

                let rule_type = self.determine_rule_for_noon(following_letter, is_same_word);

                if rule_type != TajweedRuleType::NoRule {
                    matches.push(RuleMatch {
                        start_index: base_idx,
                        end_index: i,
                        target_letter: 'ن',
                        following_letter: Some(following_letter),
                        rule: TajweedRule::from_type(rule_type, self.style),
                        context: get_context(&verse_chars, base_idx, 3),
                    });
                }
            }
        }
    }

    fn check_lam_al_tarif(&self, verse_chars: &[char], i: usize, matches: &mut Vec<RuleMatch>) {
        let mut next_idx = i + 1;
        while next_idx < verse_chars.len() && is_tajweed_ignorable(verse_chars[next_idx]) {
            next_idx += 1;
        }

        if next_idx < verse_chars.len() && verse_chars[next_idx] == 'ل' {
            let mut after_lam_idx = next_idx + 1;
            while after_lam_idx < verse_chars.len()
                && is_tajweed_ignorable(verse_chars[after_lam_idx])
            {
                after_lam_idx += 1;
            }

            if after_lam_idx < verse_chars.len() {
                let following_letter = verse_chars[after_lam_idx];
                let rule_type = self.determine_rule_for_lam_al(following_letter);

                if rule_type != TajweedRuleType::NoRule {
                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: next_idx,
                        target_letter: 'ل',
                        following_letter: Some(following_letter),
                        rule: TajweedRule::from_type(rule_type, self.style),
                        context: get_context(&verse_chars, i, 3),
                    });
                }
            }
        }
    }

    /// Detect Madd rules
    fn detect_madd_rules(&self, verse_chars: &[char], matches: &mut Vec<RuleMatch>) {
        let mut i = 0;
        while i < verse_chars.len() {
            let current_char = verse_chars[i];

            if self.madd_letters.contains(&current_char) || current_char == 'آ' {
                let has_correct_vowel = if current_char == 'آ' {
                    true
                } else if let Some(vowel) = get_preceding_vowel(&verse_chars, i) {
                    match current_char {
                        'ا' => vowel == '\u{064E}',
                        'و' => vowel == '\u{064C}',
                        'ي' => vowel == '\u{0650}',
                        _ => false,
                    }
                } else {
                    false
                };

                if has_correct_vowel || current_char == 'آ' {
                    if let Some(madd_type) = self.detect_madd(current_char, &verse_chars, i) {
                        matches.push(RuleMatch {
                            start_index: i,
                            end_index: i,
                            target_letter: current_char,
                            following_letter: None,
                            rule: TajweedRule::from_type(madd_type, self.style),
                            context: get_context(&verse_chars, i, 3),
                        });
                    }
                }
            }

            i += 1;
        }
    }

    /// Detect Qalqalah rules
    fn detect_qalqalah_rules(&self, verse_chars: &[char], matches: &mut Vec<RuleMatch>) {
        let mut i = 0;
        while i < verse_chars.len() {
            if self.qalqalah_letters.contains(&verse_chars[i]) {
                if let Some(qalqalah_type) = Self::detect_qalqalah(&verse_chars, i) {
                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: i,
                        target_letter: verse_chars[i],
                        following_letter: None,
                        rule: TajweedRule::from_type(qalqalah_type, self.style),
                        context: get_context(&verse_chars, i, 3),
                    });
                }
            }
            i += 1;
        }
    }

    /// Detect Ra emphasis rules
    fn detect_ra_rules(&self, verse_chars: &[char], matches: &mut Vec<RuleMatch>) {
        let mut i = 0;
        while i < verse_chars.len() {
            if verse_chars[i] == 'ر' {
                if self.style == RecitationStyle::Warsh {
                    if let Some(tarqeeq_type) = Self::detect_tarqeeq_ra(&verse_chars, i) {
                        matches.push(RuleMatch {
                            start_index: i,
                            end_index: i,
                            target_letter: verse_chars[i],
                            following_letter: None,
                            rule: TajweedRule::from_type(tarqeeq_type, self.style),
                            context: get_context(&verse_chars, i, 3),
                        });
                        i += 1;
                        continue;
                    }
                }

                if let Some(tafkhim_type) = Self::detect_tafkhim_ra(&verse_chars, i) {
                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: i,
                        target_letter: verse_chars[i],
                        following_letter: None,
                        rule: TajweedRule::from_type(tafkhim_type, self.style),
                        context: get_context(&verse_chars, i, 3),
                    });
                }
            }
            i += 1;
        }
    }

    /// Detect Tafkhim Lafz Al-Jalalah rules
    fn detect_allah_name_rules(&self, verse_chars: &[char], matches: &mut Vec<RuleMatch>) {
        let mut i = 0;
        while i < verse_chars.len() {
            if verse_chars[i] == 'ا' {
                if Self::detect_tafkhim_lafuljalala(&verse_chars, i) {
                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: i,
                        target_letter: verse_chars[i],
                        following_letter: None,
                        rule: TajweedRule::from_type(
                            TajweedRuleType::TafkhimLafuljalala,
                            self.style,
                        ),
                        context: get_context(&verse_chars, i, 3),
                    });
                    i += 3;
                    continue;
                }
            }
            i += 1;
        }
    }

    /// Get the recitation style of this processor
    pub fn get_style(&self) -> RecitationStyle {
        self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_creation() {
        let processor = TajweedProcessor::new(RecitationStyle::Warsh);
        assert_eq!(processor.get_style(), RecitationStyle::Warsh);
    }

    #[test]
    fn test_basic_rule_detection() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        let verse = "الحَمْدُ";
        let matches = processor.process_verse(verse);
        assert!(!matches.is_empty());
    }
}
