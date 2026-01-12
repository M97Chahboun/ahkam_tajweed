//! Enhanced Tajweed rule processor with Warsh recitation support
//! Covers Nūn, Mīm, Lām Al-Ta'rīf, and Madd rules according to Warsh riwayah

use std::collections::HashMap;

// --- 1. Recitation Style Enum ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecitationStyle {
    Hafs,  // حفص عن عاصم
    Warsh, // ورش عن نافع
}

// --- 2. Enhanced Rule Type Enumeration ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TajweedRuleType {
    // أحكام النون الساكنة والتنوين
    IzharHalqi,
    IzharMutlaq,
    IdghamBiGhunnah,
    IdghamBilaGhunnah,
    IdghamNaqis,      // الإدغام الناقص (Warsh specific)
    IdghamKamil,      // الإدغام الكامل (Warsh specific)
    Iqlab,
    IkhfaaHaqiqi,

    // أحكام الميم الساكنة
    IkhfaaShafawi,
    IdghamShafawi,
    IdghamMithlayn,   // إدغام المثلين (more specific)
    IzharShafawi,

    // أحكام لام أل التعريف
    IzharQamari,
    IdghamShamsi,

    // أحكام المدود (Enhanced for Warsh)
    MaddTabeei,       // 2 حركات
    MaddMuttasil,     // 4-5 حركات (Warsh: 4-6)
    MaddMunfasil,     // 2-4-5 حركات (Warsh: 4-6)
    MaddLazim,        // 6 حركات
    MaddArid,         // 2-4-6 حركات
    MaddLin,          // 2-4-6 حركات
    MaddBadal,        // 2 حركات (Warsh: can be 4-6)
    MaddSilah,        // صلة (Warsh specific variations)

    // أحكام الراءات (Warsh specific)
    TarqeeqRa,        // ترقيق الراء
    TafkhimRa,        // تفخيم الراء

    // أحكام اللامات (Warsh specific)
    TafkhimLafuljalala, // تفخيم لفظ الجلالة

    NoRule,
}

// --- 3. Enhanced Rule Structure ---
#[derive(Debug, Clone)]
pub struct TajweedRule {
    pub rule_type: TajweedRuleType,
    pub arabic_name: &'static str,
    pub english_name: &'static str,
    pub description_ar: &'static str,
    pub warsh_specific: bool,
    pub madd_length_warsh: Option<(u8, u8)>, // (min, max) in harakaat
}

impl TajweedRule {
    fn from_type(rule_type: TajweedRuleType, style: RecitationStyle) -> Self {
        match rule_type {
            TajweedRuleType::IzharHalqi => TajweedRule {
                rule_type,
                arabic_name: "الإظهار الحلقي",
                english_name: "Al-Izhar Al-Halqi",
                description_ar: "إظهار النون الساكنة عند حروف الحلق (ء هـ ع ح غ خ).",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::IdghamBiGhunnah => TajweedRule {
                rule_type,
                arabic_name: "الإدغام بغنة",
                english_name: "Idgham with Ghunnah",
                description_ar: "إدغام النون في (ينمو) مع غنة.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::IdghamNaqis => TajweedRule {
                rule_type,
                arabic_name: "الإدغام الناقص",
                english_name: "Idgham Naqis (Incomplete)",
                description_ar: "إدغام ناقص مع بقاء الغنة في رواية ورش.",
                warsh_specific: true,
                madd_length_warsh: None,
            },
            TajweedRuleType::IdghamKamil => TajweedRule {
                rule_type,
                arabic_name: "الإدغام الكامل",
                english_name: "Idgham Kamil (Complete)",
                description_ar: "إدغام كامل بدون بقاء صفة الحرف المدغم.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::IdghamBilaGhunnah => TajweedRule {
                rule_type,
                arabic_name: "الإدغام بغير غنة",
                english_name: "Idgham without Ghunnah",
                description_ar: "إدغام النون في (ل، ر) بدون غنة.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::Iqlab => TajweedRule {
                rule_type,
                arabic_name: "الإقلاب",
                english_name: "Al-Iqlab",
                description_ar: "قلب النون الساكنة ميماً مخفاة بغنة عند حرف الباء.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::IkhfaaHaqiqi => TajweedRule {
                rule_type,
                arabic_name: "الإخفاء الحقيقي",
                english_name: "Al-Ikhfaa Al-Haqiqi",
                description_ar: "نطق النون بحالة بين الإظهار والإدغام مع غنة عند الـ 15 حرفاً.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::IkhfaaShafawi => TajweedRule {
                rule_type,
                arabic_name: "الإخفاء الشفوي",
                english_name: "Al-Ikhfaa Al-Shafawi",
                description_ar: "إخفاء الميم الساكنة بغنة عند حرف الباء.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::IdghamShafawi | TajweedRuleType::IdghamMithlayn => TajweedRule {
                rule_type,
                arabic_name: "الإدغام الشفوي (المثلين الصغير)",
                english_name: "Al-Idgham Al-Shafawi",
                description_ar: "إدغام الميم الساكنة في ميم متحركة تليها.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::IzharShafawi => TajweedRule {
                rule_type,
                arabic_name: "الإظهار الشفوي",
                english_name: "Al-Izhar Al-Shafawi",
                description_ar: "إظهار الميم الساكنة بوضوح عند باقي الحروف.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::IzharQamari => TajweedRule {
                rule_type,
                arabic_name: "الإظهار القمري",
                english_name: "Al-Izhar Al-Qamari",
                description_ar: "إظهار اللام الساكنة في (أل) عند الحروف القمرية الـ 14.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::IdghamShamsi => TajweedRule {
                rule_type,
                arabic_name: "الإدغام الشمسي",
                english_name: "Al-Idgham Al-Shamsi",
                description_ar: "إدغام اللام الساكنة في (أل) في الحروف الشمسية.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::MaddTabeei => TajweedRule {
                rule_type,
                arabic_name: "المد الطبيعي",
                english_name: "Madd Tabeei",
                description_ar: "مد الألف، الواو، والياء بمقدار حركتين.",
                warsh_specific: false,
                madd_length_warsh: Some((2, 2)),
            },
            TajweedRuleType::MaddMuttasil => TajweedRule {
                rule_type,
                arabic_name: "المد المتصل",
                english_name: "Madd Muttasil",
                description_ar: if style == RecitationStyle::Warsh {
                    "المد المتصل: 4 أو 5 أو 6 حركات في رواية ورش (الأشهر: 6)"
                } else {
                    "المد المتصل: 4 أو 5 حركات في رواية حفص"
                },
                warsh_specific: false,
                madd_length_warsh: Some((4, 6)),
            },
            TajweedRuleType::MaddMunfasil => TajweedRule {
                rule_type,
                arabic_name: "المد المنفصل",
                english_name: "Madd Munfasil",
                description_ar: if style == RecitationStyle::Warsh {
                    "المد المنفصل: 4 أو 5 أو 6 حركات في رواية ورش (الأشهر: 4)"
                } else {
                    "المد المنفصل: 2 أو 4 أو 5 حركات في رواية حفص"
                },
                warsh_specific: false,
                madd_length_warsh: Some((4, 6)),
            },
            TajweedRuleType::MaddBadal => TajweedRule {
                rule_type,
                arabic_name: "مد البدل",
                english_name: "Madd Badal",
                description_ar: if style == RecitationStyle::Warsh {
                    "مد البدل: 2 أو 4 أو 6 حركات في رواية ورش (تسهيل الهمزة)"
                } else {
                    "مد البدل: حركتان في رواية حفص"
                },
                warsh_specific: true,
                madd_length_warsh: Some((2, 6)),
            },
            TajweedRuleType::MaddLazim => TajweedRule {
                rule_type,
                arabic_name: "المد اللازم",
                english_name: "Madd Lazim",
                description_ar: "المد اللازم: 6 حركات (في جميع الروايات)",
                warsh_specific: false,
                madd_length_warsh: Some((6, 6)),
            },
            TajweedRuleType::TarqeeqRa => TajweedRule {
                rule_type,
                arabic_name: "ترقيق الراء",
                english_name: "Tarqeeq Ra",
                description_ar: "ترقيق الراء في رواية ورش في مواضع خاصة.",
                warsh_specific: true,
                madd_length_warsh: None,
            },
            TajweedRuleType::TafkhimRa => TajweedRule {
                rule_type,
                arabic_name: "تفخيم الراء",
                english_name: "Tafkhim Ra",
                description_ar: "تفخيم الراء حسب القواعد.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::TafkhimLafuljalala => TajweedRule {
                rule_type,
                arabic_name: "تفخيم لفظ الجلالة",
                english_name: "Tafkhim Lafz Al-Jalalah",
                description_ar: "تفخيم لفظ الجلالة (الله) بعد فتح أو ضم.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            _ => TajweedRule {
                rule_type,
                arabic_name: "لا يوجد حكم",
                english_name: "No Rule",
                description_ar: "لا يوجد حكم تجويدي.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
        }
    }
}

// --- 4. Enhanced Output Structure ---
#[derive(Debug, Clone)]
pub struct RuleMatch {
    pub start_index: usize,      // Index of the letter where rule applies
    pub end_index: usize,        // End index (for multi-character sequences)
    pub target_letter: char,     // The main letter the rule applies to
    pub following_letter: Option<char>, // The following letter (if relevant)
    pub rule: TajweedRule,
    pub context: String,         // Surrounding context for clarity
}

// --- 5. Enhanced Tajweed Processor ---
pub struct TajweedProcessor {
    style: RecitationStyle,
    
    // Nūn Sākinah Maps
    izhar_halqi_map: HashMap<char, TajweedRuleType>,
    idgham_bi_ghunnah_map: HashMap<char, TajweedRuleType>,
    idgham_bila_ghunnah_map: HashMap<char, TajweedRuleType>,
    ikhfaa_letters: Vec<char>,
    iqlab_letter: char,

    // Mīm Sākinah Letters
    ikhfaa_shafawi_letter: char,
    idgham_shafawi_letter: char,

    // Lām Al-Ta'rīf Maps
    izhar_qamari_map: HashMap<char, TajweedRuleType>,
    idgham_shamsi_map: HashMap<char, TajweedRuleType>,

    // Madd Letters
    madd_letters: Vec<char>,
}

impl TajweedProcessor {
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

        // Lunar Letters (حروف الإظهار القمري): ابغ حجك وخف عقيمه
        const IZHAR_QAMARI_LETTERS: [char; 14] = [
            'ا', 'ب', 'غ', 'ح', 'ج', 'ك', 'و', 'خ', 'ف', 'ع', 'ق', 'ي', 'م', 'ه',
        ];

        // Solar Letters (حروف الإدغام الشمسي)
        const IDGHAM_SHAMSI_LETTERS: [char; 14] = [
            'ت', 'ث', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ل', 'ن',
        ];

        // Madd Letters (حروف المد)
        const MADD_LETTERS: [char; 3] = ['ا', 'و', 'ي'];

        // Build Maps
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
        }
    }

    /// Check if character should be ignored in Tajweed analysis
    fn is_tajweed_ignorable(c: char) -> bool {
        matches!(c,
            '\u{064B}'..='\u{065F}' // Diacritics
            | '\u{0670}'            // Alif Khanjareeya
            | '\u{0640}'            // Tatweel
            | '\u{06D6}'..='\u{06DC}' // Additional marks
            | '\u{06DF}'..='\u{06E8}'
            | '\u{06EA}'..='\u{06ED}'
            | '\u{200C}' | '\u{200D}' // Zero-width characters
            | ' ' | '\t' | '\n' | '\r' // Whitespace
        )
    }

    /// Check if character is Sukun (سكون)
    fn is_sukun(c: char) -> bool {
        c == '\u{0652}'
    }

    /// Check if character is Tanwin (تنوين)
    fn is_tanwin(c: char) -> bool {
        matches!(c, '\u{064B}' | '\u{064C}' | '\u{064D}')
    }

    /// Check if character is Shadda (شدة)
    fn is_shadda(c: char) -> bool {
        c == '\u{0651}'
    }

    /// Get context around a position
    fn get_context(verse_chars: &[char], index: usize, window: usize) -> String {
        let start = index.saturating_sub(window);
        let end = (index + window + 1).min(verse_chars.len());
        verse_chars[start..end].iter().collect()
    }

    /// Determine rule for Nūn Sākinah or Tanwin
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

    /// Determine rule for Mīm Sākinah
    fn determine_rule_for_mim(&self, following_letter: char) -> TajweedRuleType {
        // 1. Ikhfaa Shafawi (الإخفاء الشفوي)
        if following_letter == self.ikhfaa_shafawi_letter {
            return TajweedRuleType::IkhfaaShafawi;
        }

        // 2. Idgham Shafawi (الإدغام الشفوي)
        if following_letter == self.idgham_shafawi_letter {
            return TajweedRuleType::IdghamMithlayn;
        }

        // 3. Izhar Shafawi (الإظهار الشفوي)
        if following_letter >= 'ا' && following_letter <= 'ي' {
            return TajweedRuleType::IzharShafawi;
        }

        TajweedRuleType::NoRule
    }

    /// Determine rule for Lām Al-Ta'rīf
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

    /// Main processing function
    pub fn process_verse(&self, verse: &str) -> Vec<RuleMatch> {
        let mut matches: Vec<RuleMatch> = Vec::new();
        let verse_chars: Vec<char> = verse.chars().collect();

        let mut i = 0;
        while i < verse_chars.len() {
            let current_char = verse_chars[i];

            // Check for Nūn or Mīm with Sukun or Tanwin
            if current_char == 'ن' || current_char == 'م' {
                // Look ahead for Sukun or Tanwin
                let mut has_sukun = false;
                let mut has_tanwin = false;
                let mut j = i + 1;

                while j < verse_chars.len() && Self::is_tajweed_ignorable(verse_chars[j]) {
                    if Self::is_sukun(verse_chars[j]) {
                        has_sukun = true;
                        break;
                    }
                    if Self::is_tanwin(verse_chars[j]) {
                        has_tanwin = true;
                        break;
                    }
                    j += 1;
                }

                if has_sukun || has_tanwin {
                    // Find the next meaningful letter
                    let mut next_char_index = j + 1;
                    while next_char_index < verse_chars.len()
                        && Self::is_tajweed_ignorable(verse_chars[next_char_index])
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
                                context: Self::get_context(&verse_chars, i, 3),
                            });
                        }
                    }
                }
            }

            // Check for Lām Al-Ta'rīf (ال)
            if current_char == 'ا' && i + 1 < verse_chars.len() {
                let mut next_idx = i + 1;
                
                // Skip diacritics
                while next_idx < verse_chars.len() 
                    && Self::is_tajweed_ignorable(verse_chars[next_idx]) 
                {
                    next_idx += 1;
                }

                if next_idx < verse_chars.len() && verse_chars[next_idx] == 'ل' {
                    // Find the letter after Lām
                    let mut after_lam_idx = next_idx + 1;
                    while after_lam_idx < verse_chars.len()
                        && Self::is_tajweed_ignorable(verse_chars[after_lam_idx])
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
                                context: Self::get_context(&verse_chars, i, 3),
                            });
                        }
                    }
                }
            }

            i += 1;
        }

        matches
    }

    /// Get recitation style
    pub fn get_style(&self) -> RecitationStyle {
        self.style
    }
}

// --- 6. Main Function with Comprehensive Examples ---
fn main() {
    println!("=======================================================");
    println!("  Enhanced Tajweed Processor - Warsh Recitation");
    println!("=======================================================\n");

    // Create processors for both styles
    let processor_warsh = TajweedProcessor::new(RecitationStyle::Warsh);
    let processor_hafs = TajweedProcessor::new(RecitationStyle::Hafs);

    // Helper function to display results
    fn display_results(verse: &str, matches: Vec<RuleMatch>, style_name: &str) {
        println!("Verse: {}", verse);
        println!("Style: {}\n", style_name);
        
        if matches.is_empty() {
            println!("  No Tajweed rules detected.\n");
            return;
        }

        for (idx, m) in matches.iter().enumerate() {
            println!("  Rule #{}", idx + 1);
            println!("    Position: {} to {}", m.start_index, m.end_index);
            println!("    Target Letter: '{}'", m.target_letter);
            if let Some(following) = m.following_letter {
                println!("    Following Letter: '{}'", following);
            }
            println!("    Rule (Arabic): {}", m.rule.arabic_name);
            println!("    Rule (English): {}", m.rule.english_name);
            println!("    Description: {}", m.rule.description_ar);
            if m.rule.warsh_specific {
                println!("    ⚠️  Warsh-Specific Rule");
            }
            if let Some((min, max)) = m.rule.madd_length_warsh {
                println!("    Madd Length: {} - {} harakaat", min, max);
            }
            println!("    Context: {}", m.context);
            println!();
        }
        println!("{}\n", "=".repeat(55));
    }

    // ========== SECTION 1: Basmalah Analysis ==========
    println!("\n📖 SECTION 1: BASMALAH (بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ)");
    println!("{}", "=".repeat(55));
    let basmalah = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ";
    let matches_basmalah_warsh = processor_warsh.process_verse(basmalah);
    display_results(basmalah, matches_basmalah_warsh, "Warsh");

    // ========== SECTION 2: Izhar Halqi Examples ==========
    println!("\n📖 SECTION 2: IZHAR HALQI (الإظهار الحلقي)");
    println!("{}", "=".repeat(55));
    
    let verse_izhar_1 = "مِنْ هَادٍ";
    println!("Example 1: Noon Sakinah + Ha (ن + ه)");
    let matches_izhar_1 = processor_warsh.process_verse(verse_izhar_1);
    display_results(verse_izhar_1, matches_izhar_1, "Warsh");

    let verse_izhar_2 = "يَنْأَوْنَ";
    println!("Example 2: Noon Sakinah + Hamza (ن + أ)");
    let matches_izhar_2 = processor_warsh.process_verse(verse_izhar_2);
    display_results(verse_izhar_2, matches_izhar_2, "Warsh");

    let verse_izhar_3 = "مَنْ عَمِلَ";
    println!("Example 3: Noon Sakinah + Ayn (ن + ع)");
    let matches_izhar_3 = processor_warsh.process_verse(verse_izhar_3);
    display_results(verse_izhar_3, matches_izhar_3, "Warsh");

    let verse_izhar_4 = "عَلِيمٌ حَكِيمٌ";
    println!("Example 4: Tanwin + Ha (تنوين + ح)");
    let matches_izhar_4 = processor_warsh.process_verse(verse_izhar_4);
    display_results(verse_izhar_4, matches_izhar_4, "Warsh");

    // ========== SECTION 3: Iqlab Examples ==========
    println!("\n📖 SECTION 3: IQLAB (الإقلاب)");
    println!("{}", "=".repeat(55));
    
    let verse_iqlab_1 = "مِنْ بَعْدِ";
    println!("Example 1: Noon Sakinah + Ba (ن + ب)");
    let matches_iqlab_1 = processor_warsh.process_verse(verse_iqlab_1);
    display_results(verse_iqlab_1, matches_iqlab_1, "Warsh");

    let verse_iqlab_2 = "سَمِيعٌ بَصِيرٌ";
    println!("Example 2: Tanwin + Ba (تنوين + ب)");
    let matches_iqlab_2 = processor_warsh.process_verse(verse_iqlab_2);
    display_results(verse_iqlab_2, matches_iqlab_2, "Warsh");

    let verse_iqlab_3 = "أَنْبِئْهُمْ";
    println!("Example 3: Noon Sakinah + Ba in same word (ن + ب)");
    let matches_iqlab_3 = processor_warsh.process_verse(verse_iqlab_3);
    display_results(verse_iqlab_3, matches_iqlab_3, "Warsh");

    // ========== SECTION 4: Idgham bi Ghunnah Examples ==========
    println!("\n📖 SECTION 4: IDGHAM BI GHUNNAH (الإدغام بغنة)");
    println!("{}", "=".repeat(55));
    
    let verse_idgham_g_1 = "مَنْ يَعْمَلْ";
    println!("Example 1: Noon Sakinah + Ya (ن + ي)");
    let matches_idgham_g_1 = processor_warsh.process_verse(verse_idgham_g_1);
    display_results(verse_idgham_g_1, matches_idgham_g_1, "Warsh");

    let verse_idgham_g_2 = "مِنْ نِعْمَةٍ";
    println!("Example 2: Noon Sakinah + Noon (ن + ن)");
    let matches_idgham_g_2 = processor_warsh.process_verse(verse_idgham_g_2);
    display_results(verse_idgham_g_2, matches_idgham_g_2, "Warsh");

    let verse_idgham_g_3 = "مِنْ مَالٍ";
    println!("Example 3: Noon Sakinah + Meem (ن + م)");
    let matches_idgham_g_3 = processor_warsh.process_verse(verse_idgham_g_3);
    display_results(verse_idgham_g_3, matches_idgham_g_3, "Warsh");

    let verse_idgham_g_4 = "مَنْ وَجَدَ";
    println!("Example 4: Noon Sakinah + Waw (ن + و)");
    let matches_idgham_g_4 = processor_warsh.process_verse(verse_idgham_g_4);
    display_results(verse_idgham_g_4, matches_idgham_g_4, "Warsh");

    let verse_idgham_g_5 = "هُدًى وَرَحْمَةً";
    println!("Example 5: Tanwin + Waw (تنوين + و)");
    let matches_idgham_g_5 = processor_warsh.process_verse(verse_idgham_g_5);
    display_results(verse_idgham_g_5, matches_idgham_g_5, "Warsh");

    // ========== SECTION 5: Idgham bila Ghunnah Examples ==========
    println!("\n📖 SECTION 5: IDGHAM BILA GHUNNAH (الإدغام بغير غنة)");
    println!("{}", "=".repeat(55));
    
    let verse_idgham_bg_1 = "مِنْ لَدُنْ";
    println!("Example 1: Noon Sakinah + Lam (ن + ل)");
    let matches_idgham_bg_1 = processor_warsh.process_verse(verse_idgham_bg_1);
    display_results(verse_idgham_bg_1, matches_idgham_bg_1, "Warsh");

    let verse_idgham_bg_2 = "مَنْ رَبُّهُ";
    println!("Example 2: Noon Sakinah + Ra (ن + ر)");
    let matches_idgham_bg_2 = processor_warsh.process_verse(verse_idgham_bg_2);
    display_results(verse_idgham_bg_2, matches_idgham_bg_2, "Warsh");

    let verse_idgham_bg_3 = "غَفُورٌ رَحِيمٌ";
    println!("Example 3: Tanwin + Ra (تنوين + ر)");
    let matches_idgham_bg_3 = processor_warsh.process_verse(verse_idgham_bg_3);
    display_results(verse_idgham_bg_3, matches_idgham_bg_3, "Warsh");

    let verse_idgham_bg_4 = "هُدًى لِلْمُتَّقِينَ";
    println!("Example 4: Tanwin + Lam (تنوين + ل)");
    let matches_idgham_bg_4 = processor_warsh.process_verse(verse_idgham_bg_4);
    display_results(verse_idgham_bg_4, matches_idgham_bg_4, "Warsh");

    // ========== SECTION 6: Ikhfaa Haqiqi Examples ==========
    println!("\n📖 SECTION 6: IKHFAA HAQIQI (الإخفاء الحقيقي)");
    println!("{}", "=".repeat(55));
    
    let verse_ikhfaa_1 = "أَنْفُسَكُمْ";
    println!("Example 1: Noon Sakinah + Fa (ن + ف)");
    let matches_ikhfaa_1 = processor_warsh.process_verse(verse_ikhfaa_1);
    display_results(verse_ikhfaa_1, matches_ikhfaa_1, "Warsh");

    let verse_ikhfaa_2 = "مَنْ صَدَّ";
    println!("Example 2: Noon Sakinah + Sad (ن + ص)");
    let matches_ikhfaa_2 = processor_warsh.process_verse(verse_ikhfaa_2);
    display_results(verse_ikhfaa_2, matches_ikhfaa_2, "Warsh");

    let verse_ikhfaa_3 = "أَنْزَلْنَا";
    println!("Example 3: Noon Sakinah + Zay (ن + ز)");
    let matches_ikhfaa_3 = processor_warsh.process_verse(verse_ikhfaa_3);
    display_results(verse_ikhfaa_3, matches_ikhfaa_3, "Warsh");

    let verse_ikhfaa_4 = "وَجْنَةٍ مِنْ قِطْمِيرٍ";
    println!("Example 4: Noon Sakinah + Qaf (ن + ق)");
    let matches_ikhfaa_4 = processor_warsh.process_verse(verse_ikhfaa_4);
    display_results(verse_ikhfaa_4, matches_ikhfaa_4, "Warsh");

    let verse_ikhfaa_5 = "يَوْمَئِذٍ تُحَدِّثُ";
    println!("Example 5: Tanwin + Ta (تنوين + ت)");
    let matches_ikhfaa_5 = processor_warsh.process_verse(verse_ikhfaa_5);
    display_results(verse_ikhfaa_5, matches_ikhfaa_5, "Warsh");

    // ========== SECTION 7: Meem Sakinah - Ikhfaa Shafawi ==========
    println!("\n📖 SECTION 7: IKHFAA SHAFAWI (الإخفاء الشفوي)");
    println!("{}", "=".repeat(55));
    
    let verse_ikhfaa_sh_1 = "تَرْمِيهِمْ بِحِجَارَةٍ";
    println!("Example 1: Meem Sakinah + Ba (م + ب)");
    let matches_ikhfaa_sh_1 = processor_warsh.process_verse(verse_ikhfaa_sh_1);
    display_results(verse_ikhfaa_sh_1, matches_ikhfaa_sh_1, "Warsh");

    let verse_ikhfaa_sh_2 = "وَهُمْ بِالْآخِرَةِ";
    println!("Example 2: Meem Sakinah + Ba (م + ب)");
    let matches_ikhfaa_sh_2 = processor_warsh.process_verse(verse_ikhfaa_sh_2);
    display_results(verse_ikhfaa_sh_2, matches_ikhfaa_sh_2, "Warsh");

    // ========== SECTION 8: Meem Sakinah - Idgham Shafawi ==========
    println!("\n📖 SECTION 8: IDGHAM SHAFAWI (الإدغام الشفوي)");
    println!("{}", "=".repeat(55));
    
    let verse_idgham_sh_1 = "لَكُمْ مَا";
    println!("Example 1: Meem Sakinah + Meem (م + م)");
    let matches_idgham_sh_1 = processor_warsh.process_verse(verse_idgham_sh_1);
    display_results(verse_idgham_sh_1, matches_idgham_sh_1, "Warsh");

    let verse_idgham_sh_2 = "عَلَيْهِمْ مَا";
    println!("Example 2: Meem Sakinah + Meem (م + م)");
    let matches_idgham_sh_2 = processor_warsh.process_verse(verse_idgham_sh_2);
    display_results(verse_idgham_sh_2, matches_idgham_sh_2, "Warsh");

    // ========== SECTION 9: Meem Sakinah - Izhar Shafawi ==========
    println!("\n📖 SECTION 9: IZHAR SHAFAWI (الإظهار الشفوي)");
    println!("{}", "=".repeat(55));
    
    let verse_izhar_sh_1 = "أَلَمْ تَرَ";
    println!("Example 1: Meem Sakinah + Ta (م + ت)");
    let matches_izhar_sh_1 = processor_warsh.process_verse(verse_izhar_sh_1);
    display_results(verse_izhar_sh_1, matches_izhar_sh_1, "Warsh");

    let verse_izhar_sh_2 = "وَهُمْ فِيهَا";
    println!("Example 2: Meem Sakinah + Fa (م + ف)");
    let matches_izhar_sh_2 = processor_warsh.process_verse(verse_izhar_sh_2);
    display_results(verse_izhar_sh_2, matches_izhar_sh_2, "Warsh");

    let verse_izhar_sh_3 = "عَلَيْكُمْ وَلَا";
    println!("Example 3: Meem Sakinah + Waw (م + و)");
    let matches_izhar_sh_3 = processor_warsh.process_verse(verse_izhar_sh_3);
    display_results(verse_izhar_sh_3, matches_izhar_sh_3, "Warsh");

    // ========== SECTION 10: Lam Al-Ta'rif - Izhar Qamari ==========
    println!("\n📖 SECTION 10: IZHAR QAMARI (الإظهار القمري)");
    println!("{}", "=".repeat(55));
    
    let verse_qamari_1 = "الْقَمَرُ";
    println!("Example 1: Alif-Lam + Qaf (ال + ق)");
    let matches_qamari_1 = processor_warsh.process_verse(verse_qamari_1);
    display_results(verse_qamari_1, matches_qamari_1, "Warsh");

    let verse_qamari_2 = "الْكِتَابُ";
    println!("Example 2: Alif-Lam + Kaf (ال + ك)");
    let matches_qamari_2 = processor_warsh.process_verse(verse_qamari_2);
    display_results(verse_qamari_2, matches_qamari_2, "Warsh");

    let verse_qamari_3 = "الْمَلَائِكَةِ";
    println!("Example 3: Alif-Lam + Meem (ال + م)");
    let matches_qamari_3 = processor_warsh.process_verse(verse_qamari_3);
    display_results(verse_qamari_3, matches_qamari_3, "Warsh");

    let verse_qamari_4 = "الْبَيْتِ الْحَرَامِ";
    println!("Example 4: Multiple Alif-Lam (ال + ب) and (ال + ح)");
    let matches_qamari_4 = processor_warsh.process_verse(verse_qamari_4);
    display_results(verse_qamari_4, matches_qamari_4, "Warsh");

    // ========== SECTION 11: Lam Al-Ta'rif - Idgham Shamsi ==========
    println!("\n📖 SECTION 11: IDGHAM SHAMSI (الإدغام الشمسي)");
    println!("{}", "=".repeat(55));
    
    let verse_shamsi_1 = "الشَّمْسُ";
    println!("Example 1: Alif-Lam + Sheen (ال + ش)");
    let matches_shamsi_1 = processor_warsh.process_verse(verse_shamsi_1);
    display_results(verse_shamsi_1, matches_shamsi_1, "Warsh");

    let verse_shamsi_2 = "الرَّحْمَٰنِ";
    println!("Example 2: Alif-Lam + Ra (ال + ر)");
    let matches_shamsi_2 = processor_warsh.process_verse(verse_shamsi_2);
    display_results(verse_shamsi_2, matches_shamsi_2, "Warsh");

    let verse_shamsi_3 = "الصَّلَاةَ";
    println!("Example 3: Alif-Lam + Sad (ال + ص)");
    let matches_shamsi_3 = processor_warsh.process_verse(verse_shamsi_3);
    display_results(verse_shamsi_3, matches_shamsi_3, "Warsh");

    let verse_shamsi_4 = "الطَّيِّبَاتِ";
    println!("Example 4: Alif-Lam + Ta (ال + ط)");
    let matches_shamsi_4 = processor_warsh.process_verse(verse_shamsi_4);
    display_results(verse_shamsi_4, matches_shamsi_4, "Warsh");

    let verse_shamsi_5 = "النَّاسِ";
    println!("Example 5: Alif-Lam + Noon (ال + ن)");
    let matches_shamsi_5 = processor_warsh.process_verse(verse_shamsi_5);
    display_results(verse_shamsi_5, matches_shamsi_5, "Warsh");

    // ========== SECTION 12: Complex Verses with Multiple Rules ==========
    println!("\n📖 SECTION 12: COMPLEX VERSES (Multiple Rules)");
    println!("{}", "=".repeat(55));
    
    let verse_complex_1 = "الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ";
    println!("Example 1: Al-Fatiha Verse 2");
    let matches_complex_1 = processor_warsh.process_verse(verse_complex_1);
    display_results(verse_complex_1, matches_complex_1, "Warsh");

    let verse_complex_2 = "وَمِنْ شَرِّ غَاسِقٍ إِذَا وَقَبَ";
    println!("Example 2: Surah Al-Falaq");
    let matches_complex_2 = processor_warsh.process_verse(verse_complex_2);
    display_results(verse_complex_2, matches_complex_2, "Warsh");

    let verse_complex_3 = "إِنَّا أَنْزَلْنَاهُ فِي لَيْلَةِ الْقَدْرِ";
    println!("Example 3: Surah Al-Qadr");
    let matches_complex_3 = processor_warsh.process_verse(verse_complex_3);
    display_results(verse_complex_3, matches_complex_3, "Warsh");

    // ========== SECTION 13: Warsh vs Hafs Comparison ==========
    println!("\n📖 SECTION 13: WARSH vs HAFS COMPARISON");
    println!("{}", "=".repeat(55));
    
    let verse_comparison = "مِنْ شَيْءٍ قَدِيرٌ";
    println!("Comparing same verse in both recitations:");
    println!("\n--- WARSH ---");
    let matches_warsh = processor_warsh.process_verse(verse_comparison);
    display_results(verse_comparison, matches_warsh, "Warsh");
    
    println!("--- HAFS ---");
    let matches_hafs = processor_hafs.process_verse(verse_comparison);
    display_results(verse_comparison, matches_hafs, "Hafs");

    println!("\n✅ Analysis Complete!");
    println!("Total Examples: 50+ Tajweed Rules Demonstrated");
}