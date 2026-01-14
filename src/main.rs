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
    IdghamNaqis, // الإدغام الناقص (Warsh specific)
    IdghamKamil, // الإدغام الكامل (Warsh specific)
    Iqlab,
    IkhfaaHaqiqi,

    // أحكام الميم الساكنة
    IkhfaaShafawi,
    IdghamShafawi,
    IdghamMithlayn, // إدغام المثلين (more specific)
    IzharShafawi,

    // أحكام لام أل التعريف
    IzharQamari,
    IdghamShamsi,

    // أحكام المدود (Enhanced for Warsh)
    MaddTabeei,   // 2 حركات
    MaddMuttasil, // 4-5 حركات (Warsh: 4-6)
    MaddMunfasil, // 2-4-5 حركات (Warsh: 4-6)
    MaddLazim,    // 6 حركات
    MaddArid,     // 2-4-6 حركات
    MaddLin,      // 2-4-6 حركات
    MaddBadal,    // 2 حركات (Warsh: can be 4-6)
    MaddSilah,    // صلة (Warsh specific variations)

    // أحكام الراءات (Warsh specific)
    TarqeeqRa, // ترقيق الراء
    TafkhimRa, // تفخيم الراء

    // أحكام اللامات (Warsh specific)
    TafkhimLafuljalala, // تفخيم لفظ الجلالة

    // أحكام القلقلة (Qalqalah)
    QalqalahKubra,  // القلقلة الكبرى (في الوقف)
    QalqalahSughra, // القلقلة الصغرى (في الوصل)

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
            TajweedRuleType::MaddArid => TajweedRule {
                rule_type,
                arabic_name: "المد العارض للسكون",
                english_name: "Madd Arid",
                description_ar: if style == RecitationStyle::Warsh {
                    "المد العارض للسكون: 2 أو 4 أو 6 حركات (حرف المد في آخر الكلمة)"
                } else {
                    "المد العارض للسكون: 2 أو 4 أو 6 حركات"
                },
                warsh_specific: false,
                madd_length_warsh: Some((2, 6)),
            },
            TajweedRuleType::MaddLin => TajweedRule {
                rule_type,
                arabic_name: "المد اللين",
                english_name: "Madd Lin",
                description_ar: "المد اللين: الواو أو الياء الساكنة بعد فتح (مثل: خيل، بيت)",
                warsh_specific: false,
                madd_length_warsh: Some((2, 6)),
            },
            TajweedRuleType::MaddSilah => TajweedRule {
                rule_type,
                arabic_name: "صلة الهاء",
                english_name: "Madd Silah",
                description_ar: "صلة الهاء الساكنة (تحويل ه الساكنة إلى حرف مد)",
                warsh_specific: true,
                madd_length_warsh: None,
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
            TajweedRuleType::QalqalahKubra => TajweedRule {
                rule_type,
                arabic_name: "القلقلة الكبرى",
                english_name: "Qalqalah Kubra (Major)",
                description_ar: "القلقلة الكبرى: رجع الصوت بالقاف أو الطاء أو الباء أو الجيم أو الدال عند الوقف.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::QalqalahSughra => TajweedRule {
                rule_type,
                arabic_name: "القلقلة الصغرى",
                english_name: "Qalqalah Sughra (Minor)",
                description_ar: "القلقلة الصغرى: رجع الصوت بأحد أحرف القلقلة في الوصل (غير متطرفة).",
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

// --- 3.5 Enhanced Madd Detection Helper ---
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaddContext {
    Muttasil, // Hamza in same word
    Munfasil, // Hamza in next word
    Tabeeii,  // Natural (no hamza)
    Badal,    // Hamza at beginning, alif at beginning (both alif)
    Lazim,    // Extended/prolonged letter (shadda)
    Arid,     // Medium context
    Lin,      // Lam or Ra with sukun
}

// --- 4. Enhanced Output Structure ---
#[derive(Debug, Clone)]
pub struct RuleMatch {
    pub start_index: usize,             // Index of the letter where rule applies
    pub end_index: usize,               // End index (for multi-character sequences)
    pub target_letter: char,            // The main letter the rule applies to
    pub following_letter: Option<char>, // The following letter (if relevant)
    pub rule: TajweedRule,
    pub context: String, // Surrounding context for clarity
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

    // Qalqalah Letters
    qalqalah_letters: Vec<char>,
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

        // Qalqalah Letters (حروف القلقلة): ق ط ب ج د
        const QALQALAH_LETTERS: [char; 5] = ['ق', 'ط', 'ب', 'ج', 'د'];

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
            qalqalah_letters: QALQALAH_LETTERS.to_vec(),
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

    /// Check if character is a vowel (fatha, damma, kasra)
    fn is_vowel(c: char) -> bool {
        matches!(c, '\u{064E}' | '\u{064C}' | '\u{0650}')
    }

    /// Get the vowel that precedes a character at given index (or immediately after in diacritics)
    fn get_preceding_vowel(verse_chars: &[char], index: usize) -> Option<char> {
        if index == 0 {
            return None;
        }

        // First, check if there's a vowel immediately following the madd letter (in diacritics)
        if index + 1 < verse_chars.len() && Self::is_vowel(verse_chars[index + 1]) {
            return Some(verse_chars[index + 1]);
        }

        // If not, look backwards for the vowel before the letter
        let mut idx = index - 1;
        loop {
            if Self::is_vowel(verse_chars[idx]) {
                return Some(verse_chars[idx]);
            }
            if !Self::is_tajweed_ignorable(verse_chars[idx]) && verse_chars[idx] != '\u{0651}' {
                return None;
            }
            if idx == 0 {
                break;
            }
            idx -= 1;
        }
        None
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
        // 1. Ikhfaa Shafawi (الإخفاء الشفوي) - before Ba
        if following_letter == self.ikhfaa_shafawi_letter {
            return TajweedRuleType::IkhfaaShafawi;
        }

        // 2. Idgham Shafawi (الإدغام الشفوي) - before Mim
        if following_letter == self.idgham_shafawi_letter {
            return TajweedRuleType::IdghamMithlayn;
        }

        // 3. Izhar Shafawi (الإظهار الشفوي) - before other letters
        // Must be an actual Arabic letter
        const ARABIC_LETTERS: &str = "ءأبةتثجحخدذرزسشصضطظعغفقكلمنهوي";
        if ARABIC_LETTERS.contains(following_letter) {
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

    /// Check if character is a Hamza (همزة)
    fn is_hamza(c: char) -> bool {
        matches!(c, 'أ' | 'ؤ' | 'ئ' | 'ء')
    }

    /// Check if following letter after a madd letter is Hamza
    fn is_following_hamza(verse_chars: &[char], start_idx: usize) -> bool {
        let mut idx = start_idx;
        while idx < verse_chars.len() && Self::is_tajweed_ignorable(verse_chars[idx]) {
            idx += 1;
        }
        idx < verse_chars.len() && Self::is_hamza(verse_chars[idx])
    }

    /// Check if following letter after a madd letter is shadda (doubled)
    fn is_following_shadda(verse_chars: &[char], start_idx: usize) -> bool {
        let mut idx = start_idx;
        while idx < verse_chars.len() {
            let c = verse_chars[idx];
            if Self::is_shadda(c) {
                return true;
            }
            // Check if we hit a non-diacritic character (which would mean no shadda follows)
            if !Self::is_tajweed_ignorable(c) && !Self::is_vowel(c) {
                return false;
            }
            idx += 1;
        }
        false
    }

    /// Detect Madd rules for a madd letter
    fn detect_madd(
        &self,
        madd_letter: char,
        verse_chars: &[char],
        current_index: usize,
    ) -> Option<TajweedRuleType> {
        // Check what follows the madd letter
        let has_following_hamza = Self::is_following_hamza(verse_chars, current_index + 1);
        let has_following_shadda = Self::is_following_shadda(verse_chars, current_index + 1);

        // Check if it's at word end (for Munfasil detection)
        let mut is_word_end = false;
        let mut idx = current_index + 1;
        while idx < verse_chars.len() && Self::is_tajweed_ignorable(verse_chars[idx]) {
            if verse_chars[idx].is_whitespace() {
                is_word_end = true;
                break;
            }
            idx += 1;
        }
        if idx >= verse_chars.len() {
            is_word_end = true;
        }

        // Determine Madd type based on what follows
        if has_following_shadda {
            // Madd Lazim (المد اللازم) - 6 harakaat always
            Some(TajweedRuleType::MaddLazim)
        } else if has_following_hamza {
            // Either Muttasil or Munfasil based on word boundary
            if is_word_end {
                // Madd Munfasil (المد المنفصل) - hamza in next word
                Some(TajweedRuleType::MaddMunfasil)
            } else {
                // Madd Muttasil (المد المتصل) - hamza in same word
                Some(TajweedRuleType::MaddMuttasil)
            }
        } else if madd_letter == 'ي' || madd_letter == 'و' {
            // Check for Madd Lin (المد اللين) - waaw/ya with sukun before Lam or Ra
            let mut next_idx = current_index + 1;
            while next_idx < verse_chars.len()
                && Self::is_tajweed_ignorable(verse_chars[next_idx])
                && !Self::is_sukun(verse_chars[next_idx])
            {
                next_idx += 1;
            }

            if next_idx < verse_chars.len() && Self::is_sukun(verse_chars[next_idx]) {
                // Check next letter after sukun
                let mut after_sukun_idx = next_idx + 1;
                while after_sukun_idx < verse_chars.len()
                    && Self::is_tajweed_ignorable(verse_chars[after_sukun_idx])
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

            // Check for Madd Arid (المد العارض) - at word end with soft letters
            if is_word_end {
                return Some(TajweedRuleType::MaddArid);
            }

            // Default to natural madd
            Some(TajweedRuleType::MaddTabeei)
        } else {
            // Alif
            // Check for Madd Badal:
            // 1. When Alif has Hamza before it (أ + ا = آ as one character)
            // 2. Or when hamza comes before alif in sequence

            // Check if current alif is actually آ (Alif with Madda)
            // This is represented as a single Unicode character U+0622
            if verse_chars[current_index] == 'آ' {
                return Some(TajweedRuleType::MaddBadal);
            }

            // Check if regular ا follows a hamza
            if current_index > 0 {
                let mut back_idx = current_index - 1;
                loop {
                    if !Self::is_tajweed_ignorable(verse_chars[back_idx]) {
                        if Self::is_hamza(verse_chars[back_idx]) {
                            return Some(TajweedRuleType::MaddBadal);
                        }
                        break;
                    }
                    if back_idx == 0 {
                        break;
                    }
                    back_idx -= 1;
                }
            }

            // Check for Madd Arid if at word end
            if is_word_end {
                return Some(TajweedRuleType::MaddArid);
            }

            // Default to natural madd
            Some(TajweedRuleType::MaddTabeei)
        }
    }

    /// Detect Tafkhim Ra (تفخيم الراء - Ra Emphasis)
    /// Ra is emphasized when preceded by fatha (َ), damma (ُ), or sukun after fatha/damma
    fn detect_tafkhim_ra(verse_chars: &[char], current_index: usize) -> Option<TajweedRuleType> {
        // Get the vowel preceding Ra
        if let Some(vowel) = Self::get_preceding_vowel(&verse_chars, current_index) {
            // Ra is emphasized with Fatha or Damma
            match vowel {
                '\u{064E}' | '\u{064C}' => return Some(TajweedRuleType::TafkhimRa), // Fatha or Damma
                '\u{0652}' => {
                    // If Ra has Sukun, check the vowel before the sukun
                    // This handles cases like "رْ" after fatha/damma
                    if current_index >= 2 {
                        let mut back_idx = current_index - 1;
                        while back_idx > 0 && Self::is_tajweed_ignorable(verse_chars[back_idx]) {
                            back_idx -= 1;
                        }
                        if back_idx < current_index {
                            if let Some(prev_vowel) =
                                Self::get_preceding_vowel(&verse_chars, back_idx)
                            {
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

    /// Detect Tarqeeq Ra (ترقيق الراء - Ra Thinning)
    /// Ra is thinned when preceded by kasra (ِ) or sukun after kasra (Warsh only)
    fn detect_tarqeeq_ra(verse_chars: &[char], current_index: usize) -> Option<TajweedRuleType> {
        // Get the vowel preceding Ra
        if let Some(vowel) = Self::get_preceding_vowel(&verse_chars, current_index) {
            match vowel {
                '\u{0650}' => return Some(TajweedRuleType::TarqeeqRa), // Kasra
                '\u{0652}' => {
                    // If Ra has Sukun, check the vowel before the sukun
                    if current_index >= 2 {
                        let mut back_idx = current_index - 1;
                        while back_idx > 0 && Self::is_tajweed_ignorable(verse_chars[back_idx]) {
                            back_idx -= 1;
                        }
                        if back_idx < current_index {
                            if let Some(prev_vowel) =
                                Self::get_preceding_vowel(&verse_chars, back_idx)
                            {
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

    /// Detect Tafkhim Lafz Al-Jalala (تفخيم لفظ الجلالة - Allah Emphasis)
    /// The word "الله" (Allah) is always emphasized in both Hafs and Warsh
    fn detect_tafkhim_lafuljalala(verse_chars: &[char], current_index: usize) -> bool {
        // Look for the sequence: ا (alif) followed by ل ل ه (lam, lam, ha)
        if current_index + 3 >= verse_chars.len() {
            return false;
        }

        // Current should be alif
        if verse_chars[current_index] != 'ا' {
            return false;
        }

        // Check next character is lam (allowing diacritics)
        let mut check_idx = current_index + 1;
        while check_idx < verse_chars.len() && Self::is_tajweed_ignorable(verse_chars[check_idx]) {
            check_idx += 1;
        }

        if check_idx >= verse_chars.len() || verse_chars[check_idx] != 'ل' {
            return false;
        }

        // Check next character is also lam or ha
        let mut check_idx = check_idx + 1;
        while check_idx < verse_chars.len() && Self::is_tajweed_ignorable(verse_chars[check_idx]) {
            check_idx += 1;
        }

        if check_idx >= verse_chars.len() {
            return false;
        }

        // Could be second lam or ha directly
        if verse_chars[check_idx] == 'ل' {
            // Find ha after second lam
            check_idx += 1;
            while check_idx < verse_chars.len()
                && Self::is_tajweed_ignorable(verse_chars[check_idx])
            {
                check_idx += 1;
            }
            if check_idx < verse_chars.len() && verse_chars[check_idx] == 'ه' {
                return true;
            }
        } else if verse_chars[check_idx] == 'ه' {
            // Ha directly after first lam (less common but valid)
            return true;
        }

        false
    }

    /// Detect Qalqalah rules for qalqalah letters
    fn detect_qalqalah(verse_chars: &[char], current_index: usize) -> Option<TajweedRuleType> {
        // Check if the qalqalah letter has sukun (سكون)
        let mut has_sukun = false;
        let mut sukun_idx = current_index + 1;

        while sukun_idx < verse_chars.len() && Self::is_tajweed_ignorable(verse_chars[sukun_idx]) {
            if Self::is_sukun(verse_chars[sukun_idx]) {
                has_sukun = true;
                break;
            }
            sukun_idx += 1;
        }

        if !has_sukun {
            return None; // No qalqalah without sukun
        }
        // Check if it's at the end of word/verse (Qalqalah Kubra)
        let mut is_at_end = false;
        let mut end_idx = sukun_idx + 1;
        while end_idx < verse_chars.len() && Self::is_tajweed_ignorable(verse_chars[end_idx]) {
            if verse_chars[end_idx].is_whitespace() {
                is_at_end = true;
                break;
            }
            end_idx += 1;
        }
        if end_idx >= verse_chars.len() {
            is_at_end = true;
        }

        if is_at_end {
            Some(TajweedRuleType::QalqalahKubra)
        } else {
            Some(TajweedRuleType::QalqalahSughra)
        }
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

            // Handle Tanwin (تنوين) as if it were a Noon Sakinah
            if Self::is_tanwin(current_char) {
                // Find the base letter (the letter which carries the tanwin)
                let mut base_idx_opt: Option<usize> = None;
                let mut k = i;
                while k > 0 {
                    k -= 1;
                    if !Self::is_tajweed_ignorable(verse_chars[k]) {
                        base_idx_opt = Some(k);
                        break;
                    }
                }

                if let Some(base_idx) = base_idx_opt {
                    // Look ahead for the next meaningful letter
                    let mut next_char_index = i + 1;
                    while next_char_index < verse_chars.len()
                        && Self::is_tajweed_ignorable(verse_chars[next_char_index])
                    {
                        next_char_index += 1;
                    }

                    if next_char_index < verse_chars.len() {
                        let following_letter = verse_chars[next_char_index];
                        let is_same_word = !verse_chars[base_idx..next_char_index]
                            .iter()
                            .any(|&c| c.is_whitespace());

                        let rule_type =
                            self.determine_rule_for_noon(following_letter, is_same_word);

                        if rule_type != TajweedRuleType::NoRule {
                            matches.push(RuleMatch {
                                start_index: base_idx,
                                end_index: i,
                                target_letter: 'ن',
                                following_letter: Some(following_letter),
                                rule: TajweedRule::from_type(rule_type, self.style),
                                context: Self::get_context(&verse_chars, base_idx, 3),
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

        // Second pass: Detect Madd rules (المد)
        let mut i = 0;
        while i < verse_chars.len() {
            let current_char = verse_chars[i];

            // Check if current character is a madd letter (including آ for Badal)
            if self.madd_letters.contains(&current_char) || current_char == 'آ' {
                // Check if it has correct vowel before it
                let has_correct_vowel = if current_char == 'آ' {
                    // آ is Alif with Madda, always considered a Madd case
                    true
                } else if let Some(vowel) = Self::get_preceding_vowel(&verse_chars, i) {
                    match current_char {
                        'ا' => vowel == '\u{064E}', // Alif needs Fatha
                        'و' => vowel == '\u{064C}', // Waaw needs Damma
                        'ي' => vowel == '\u{0650}', // Ya needs Kasra
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
                            context: Self::get_context(&verse_chars, i, 3),
                        });
                    }
                }
            }

            i += 1;
        }

        // Third pass: Detect Qalqalah rules (القلقلة)
        let mut i = 0;
        while i < verse_chars.len() {
            let current_char = verse_chars[i];

            // Check if current character is a qalqalah letter
            if self.qalqalah_letters.contains(&current_char) {
                if let Some(qalqalah_type) = Self::detect_qalqalah(&verse_chars, i) {
                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: i,
                        target_letter: current_char,
                        following_letter: None,
                        rule: TajweedRule::from_type(qalqalah_type, self.style),
                        context: Self::get_context(&verse_chars, i, 3),
                    });
                }
            }

            i += 1;
        }

        // Fourth pass: Detect Ra emphasis rules (أحكام الراء)
        let mut i = 0;
        while i < verse_chars.len() {
            let current_char = verse_chars[i];

            if current_char == 'ر' {
                // Check for Tarqeeq Ra first (higher priority for Warsh)
                if self.style == RecitationStyle::Warsh {
                    if let Some(tarqeeq_type) = Self::detect_tarqeeq_ra(&verse_chars, i) {
                        matches.push(RuleMatch {
                            start_index: i,
                            end_index: i,
                            target_letter: current_char,
                            following_letter: None,
                            rule: TajweedRule::from_type(tarqeeq_type, self.style),
                            context: Self::get_context(&verse_chars, i, 3),
                        });
                        i += 1;
                        continue;
                    }
                }

                // Check for Tafkhim Ra
                if let Some(tafkhim_type) = Self::detect_tafkhim_ra(&verse_chars, i) {
                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: i,
                        target_letter: current_char,
                        following_letter: None,
                        rule: TajweedRule::from_type(tafkhim_type, self.style),
                        context: Self::get_context(&verse_chars, i, 3),
                    });
                }
            }

            i += 1;
        }

        // Fifth pass: Detect Tafkhim Lafz Al-Jalala (تفخيم لفظ الجلالة)
        let mut i = 0;
        while i < verse_chars.len() {
            let current_char = verse_chars[i];

            if current_char == 'ا' {
                if Self::detect_tafkhim_lafuljalala(&verse_chars, i) {
                    matches.push(RuleMatch {
                        start_index: i,
                        end_index: i,
                        target_letter: current_char,
                        following_letter: None,
                        rule: TajweedRule::from_type(
                            TajweedRuleType::TafkhimLafuljalala,
                            self.style,
                        ),
                        context: Self::get_context(&verse_chars, i, 3),
                    });
                    // Skip ahead to avoid duplicate matches
                    i += 3;
                    continue;
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
    println!("  Tajweed Processor - Interactive CLI");
    println!("=======================================================\n");

    let processor_warsh = TajweedProcessor::new(RecitationStyle::Warsh);
    let processor_hafs = TajweedProcessor::new(RecitationStyle::Hafs);

    // Reusable display helper (moved out for interactive use)
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

    use std::io::{self, Write};

    enum SelectedStyle {
        Warsh,
        Hafs,
        Both,
    }

    let mut selected = SelectedStyle::Both;

    println!("Interactive mode: enter a verse and press Enter to analyze.");
    println!("Commands: :q or q to quit, :style warsh|hafs|both to switch styles\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input. Exiting.");
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input == "q" || input == ":q" || input == "quit" {
            println!("Goodbye.");
            break;
        }

        if input.starts_with(":style") {
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() >= 2 {
                match parts[1].to_lowercase().as_str() {
                    "warsh" => {
                        selected = SelectedStyle::Warsh;
                        println!("Style set to Warsh.");
                    }
                    "hafs" => {
                        selected = SelectedStyle::Hafs;
                        println!("Style set to Hafs.");
                    }
                    "both" => {
                        selected = SelectedStyle::Both;
                        println!("Style set to Both.");
                    }
                    _ => println!("Unknown style. Use warsh, hafs, or both."),
                }
            } else {
                println!("Usage: :style warsh|hafs|both");
            }

            continue;
        }

        match selected {
            SelectedStyle::Warsh => {
                let matches = processor_warsh.process_verse(input);
                display_results(input, matches, "Warsh");
            }
            SelectedStyle::Hafs => {
                let matches = processor_hafs.process_verse(input);
                display_results(input, matches, "Hafs");
            }
            SelectedStyle::Both => {
                println!("--- WARSH ---");
                let matches_w = processor_warsh.process_verse(input);
                display_results(input, matches_w, "Warsh");

                println!("--- HAFS ---");
                let matches_h = processor_hafs.process_verse(input);
                display_results(input, matches_h, "Hafs");
            }
        }
    }
}
