//! A comprehensive Tajweed rule processor covering Nūn, Mīm, and Lām Al-Ta'rīf rules from Tuhfat Al-Atfal.

use std::collections::HashMap;

// --- 1. Enumeration for All Covered Rule Types ---

/// يمثل الأحكام الرئيسية للنون الساكنة والتنوين والميم الساكنة ولام أل التعريف.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TajweedRuleType {
    // أحكام النون الساكنة والتنوين (Nūn Sākinah & Tanwīn Rules)
    IzharHalqi,
    IzharMutlaq,
    IdghamBiGhunnah,
    IdghamBilaGhunnah,
    Iqlab,
    IkhfaaHaqiqi,

    // أحكام الميم الساكنة (Mīm Sākinah Rules)
    IkhfaaShafawi,
    IdghamShafawi,
    IzharShafawi,

    // أحكام لام أل التعريف (Lām Al-Ta'rīf Rules)
    IzharQamari,  // الإظهار القمري (حروف أبغ حجك وخف عقيمه)
    IdghamShamsi, // الإدغام الشمسي (باقي الحروف)

    // أحكام المدود (Al-Mudūd - Types only for structure)
    MaddTabeei, // المد الطبيعي (الأصلي)
    MaddFarI,   // المد الفرعي (يشمل المتصل والمنفصل واللازم والعارض)

    NoRule,
}

// --- 2. Structure for the Rule Details (Updated) ---

#[derive(Debug, Clone)]
pub struct TajweedRule {
    pub rule_type: TajweedRuleType,
    pub arabic_name: &'static str,
    pub english_name: &'static str,
    pub description_ar: &'static str,
}

impl TajweedRule {
    fn from_type(rule_type: TajweedRuleType) -> Self {
        match rule_type {
            // Nūn Sākinah Rules (Defined in previous steps)
            TajweedRuleType::IzharHalqi => TajweedRule {
                rule_type: rule_type,
                arabic_name: "الإظهار الحلقي",
                english_name: "Al-Izhar Al-Halqi",
                description_ar: "إظهار النون الساكنة عند حروف الحلق (ء هـ ع ح غ خ).",
            },
            TajweedRuleType::IzharMutlaq => TajweedRule {
                rule_type: rule_type,
                arabic_name: "الإظهار المطلق",
                english_name: "Absolute Izhar",
                description_ar: "نطق النون الساكنة بوضوح عند (ي أو و) إذا كانتا في كلمة واحدة (دنيا).",
            },
            TajweedRuleType::IdghamBiGhunnah => TajweedRule {
                rule_type: rule_type,
                arabic_name: "الإدغام بغنة",
                english_name: "Idgham with Ghunnah",
                description_ar: "إدغام النون في (ينمو) مع غنة.",
            },
            TajweedRuleType::IdghamBilaGhunnah => TajweedRule {
                rule_type: rule_type,
                arabic_name: "الإدغام بغير غنة",
                english_name: "Idgham without Ghunnah",
                description_ar: "إدغام النون في (ل، ر) بدون غنة.",
            },
            TajweedRuleType::Iqlab => TajweedRule {
                rule_type: rule_type,
                arabic_name: "الإقلاب",
                english_name: "Al-Iqlab",
                description_ar: "قلب النون الساكنة ميماً مخفاة بغنة عند حرف الباء.",
            },
            TajweedRuleType::IkhfaaHaqiqi => TajweedRule {
                rule_type: rule_type,
                arabic_name: "الإخفاء الحقيقي",
                english_name: "Al-Ikhfaa Al-Haqiqi",
                description_ar: "نطق النون بحالة بين الإظهار والإدغام مع غنة عند الـ 15 حرفاً.",
            },

            // Mīm Sākinah Rules (Defined in previous steps)
            TajweedRuleType::IkhfaaShafawi => TajweedRule {
                rule_type: rule_type,
                arabic_name: "الإخفاء الشفوي",
                english_name: "Al-Ikhfaa Al-Shafawi",
                description_ar: "إخفاء الميم الساكنة بغنة عند حرف الباء.",
            },
            TajweedRuleType::IdghamShafawi => TajweedRule {
                rule_type: rule_type,
                arabic_name: "الإدغام الشفوي",
                english_name: "Al-Idgham Al-Shafawi",
                description_ar: "إدغام الميم الساكنة في ميم متحركة تليها.",
            },
            TajweedRuleType::IzharShafawi => TajweedRule {
                rule_type: rule_type,
                arabic_name: "الإظهار الشفوي",
                english_name: "Al-Izhar Al-Shafawi",
                description_ar: "إظهار الميم الساكنة بوضوح عند باقي الحروف.",
            },

            // Lām Al-Ta'rīf Rules (NEW)
            TajweedRuleType::IzharQamari => TajweedRule {
                rule_type: rule_type,
                arabic_name: "الإظهار القمري",
                english_name: "Al-Izhar Al-Qamari",
                description_ar: "إظهار اللام الساكنة في (أل) عند الحروف القمرية الـ 14 (أبغ حجك وخف عقيمه).",
            },
            TajweedRuleType::IdghamShamsi => TajweedRule {
                rule_type: rule_type,
                arabic_name: "الإدغام الشمسي",
                english_name: "Al-Idgham Al-Shamsi",
                description_ar: "إدغام اللام الساكنة في (أل) في الحروف الشمسية الـ 14 (مثل الطاء والدال والراء).",
            },

            // Madd Rules (NEW - For structure only)
            TajweedRuleType::MaddTabeei => TajweedRule {
                rule_type: rule_type,
                arabic_name: "المد الطبيعي",
                english_name: "Madd Tabeei",
                description_ar: "مد الألف، الواو، والياء بمقدار حركتين عند خلوهما من سبب المد الفرعي.",
            },
            TajweedRuleType::MaddFarI => TajweedRule {
                rule_type: rule_type,
                arabic_name: "المد الفرعي",
                english_name: "Madd Far'i",
                description_ar: "المد الزائد عن الطبيعي بسبب همز أو سكون.",
            },

            TajweedRuleType::NoRule => TajweedRule {
                rule_type: rule_type,
                arabic_name: "لا يوجد حكم",
                english_name: "No Applicable Rule",
                description_ar: "لا يوجد حكم تجويدي.",
            },
        }
    }
}

// --- 3. Output Structure for Rule Matching (No Change) ---
#[derive(Debug)]
pub struct RuleMatch {
    pub start_index: usize,
    pub following_letter: char,
    pub rule: TajweedRule,
}

// --- 4. The TajweedProcessor Object (Updated) ---
pub struct TajweedProcessor {
    // Nūn Sākinah Maps (Simplified initialization for brevity)
    izhar_halqi_map: HashMap<char, TajweedRuleType>,
    idgham_bi_ghunnah_map: HashMap<char, TajweedRuleType>,
    idgham_bila_ghunnah_map: HashMap<char, TajweedRuleType>,
    iqlab_letter: char,

    // Mīm Sākinah Letters
    ikhfaa_shafawi_letter: char,
    idgham_shafawi_letter: char,

    // Lām Al-Ta'rīf Maps (NEW)
    izhar_qamari_map: HashMap<char, TajweedRuleType>,
    idgham_shamsi_map: HashMap<char, TajweedRuleType>,
}

impl TajweedProcessor {
    pub fn new() -> Self {
        // ... (Initialization for Nūn and Mīm remains the same)
        const IZHAR_HALQI_LETTERS: [char; 6] = ['أ', 'ه', 'ع', 'ح', 'غ', 'خ'];
        const IDGHAM_BI_GHUNNAH_LETTERS: [char; 4] = ['ي', 'ن', 'م', 'و'];
        const IDGHAM_BILA_GHUNNAH_LETTERS: [char; 2] = ['ل', 'ر'];
        const IQLAB_LETTER: char = 'ب';
        const IKHFAA_SHAFAWI_LETTER: char = 'ب';
        const IDGHAM_SHAFAWI_LETTER: char = 'م';

        // Lām Al-Ta'rīf Letters (NEW)
        // حروف الإظهار القمري: أبغ حجك وخف عقيمه
        const IZHAR_QAMARI_LETTERS: [char; 14] = [
            'أ', 'ب', 'غ', 'ح', 'ج', 'ك', 'و', 'خ', 'ف', 'ع', 'ق', 'ي', 'م', 'ه',
        ];

        // حروف الإدغام الشمسي (باقي الحروف الـ 14):
        // (ت ث د ذ ر ز س ش ص ض ط ظ ن)
        // لا نحتاج لتعريفها صراحةً، نعتبرها افتراضياً إذا لم تكن قمرية

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

        // باقي حروف الهجاء الـ 28 (غير النون والميم) هي حروف شمسية
        let idgham_shamsi_map: HashMap<char, TajweedRuleType> = [
            'ت', 'ث', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ن',
        ]
        .iter()
        .map(|&l| (l, TajweedRuleType::IdghamShamsi))
        .collect();

        TajweedProcessor {
            izhar_halqi_map,
            idgham_bi_ghunnah_map,
            idgham_bila_ghunnah_map,
            iqlab_letter: IQLAB_LETTER,

            ikhfaa_shafawi_letter: IKHFAA_SHAFAWI_LETTER,
            idgham_shafawi_letter: IDGHAM_SHAFAWI_LETTER,

            izhar_qamari_map,
            idgham_shamsi_map,
        }
    }

    // ... (is_tajweed_ignorable, determine_rule_for_noon, determine_rule_for_mim remain the same) ...
    // Note: Since the core logic is large, I will only include the new Lām rule function and the updated process_verse.

    /// دالة لتحديد ما إذا كان الحرف يجب تجاهله (حركة، مسافة، أو حرف مد).
    fn is_tajweed_ignorable(c: char) -> bool {
        // الحركات والتشكيلات الأساسية والتنوين (Diacritics & Tanwīn)
        match c {
            '\u{064B}'..='\u{0652}' // Fathatan, Dammatan, Kasratan, Fatha, Damma, Kasra, Shadda, Sukun
            | '\u{0670}' // Alif Khanjareeya (Dagger Alif)
            | '\u{0640}' // Tatweel
            | '\u{200C}' // Zero Width Non-Joiner
            | ' ' | '\t' | '\n' | '\r' // Whitespace
            => true,
            _ => false,
        }
    }

    /// تحديد حكم النون الساكنة أو التنوين. (كما في الخطوة السابقة)
    fn determine_rule_for_noon(
        &self,
        following_letter: char,
        is_same_word: bool,
    ) -> TajweedRuleType {
        // 1. الإظهار المطلق (استثناء الإدغام)
        if is_same_word && (following_letter == 'ي' || following_letter == 'و') {
            return TajweedRuleType::IzharMutlaq;
        }

        // 2. الإقلاب
        if following_letter == self.iqlab_letter {
            return TajweedRuleType::Iqlab;
        }

        // 3. الإظهار الحلقي
        if self.izhar_halqi_map.contains_key(&following_letter) {
            return TajweedRuleType::IzharHalqi;
        }

        // 4. الإدغام بغير غنة
        if self.idgham_bila_ghunnah_map.contains_key(&following_letter) {
            return TajweedRuleType::IdghamBilaGhunnah;
        }

        // 5. الإدغام بغنة
        if self.idgham_bi_ghunnah_map.contains_key(&following_letter) {
            return TajweedRuleType::IdghamBiGhunnah;
        }

        // 6. الإخفاء الحقيقي (افتراضياً لباقي الحروف العربية التي لم تُغطَّ)
        if following_letter.is_alphabetic() && following_letter.is_ascii() == false {
            return TajweedRuleType::IkhfaaHaqiqi;
        }

        TajweedRuleType::NoRule
    }

    /// تحديد حكم الميم الساكنة. (كما في الخطوة السابقة)
    fn determine_rule_for_mim(&self, following_letter: char) -> TajweedRuleType {
        // 1. الإخفاء الشفوي (ب)
        if following_letter == self.ikhfaa_shafawi_letter {
            return TajweedRuleType::IkhfaaShafawi;
        }

        // 2. الإدغام الشفوي (م)
        if following_letter == self.idgham_shafawi_letter {
            return TajweedRuleType::IdghamShafawi;
        }

        // 3. الإظهار الشفوي (باقي الحروف)
        if following_letter.is_alphabetic() && following_letter.is_ascii() == false {
            return TajweedRuleType::IzharShafawi;
        }

        TajweedRuleType::NoRule
    }

    ///
    /// تحديد حكم لام أل التعريف (Al-Ta'rīf Lām).
    fn determine_rule_for_lam_al(&self, following_letter: char) -> TajweedRuleType {
        // 1. الإظهار القمري (Izhar Qamari)
        if self.izhar_qamari_map.contains_key(&following_letter) {
            return TajweedRuleType::IzharQamari;
        }

        // 2. الإدغام الشمسي (Idgham Shamsi)
        // يتم تطبيقه إذا كان الحرف شمسيًا (وذلك إذا لم يكن قمريًا ولا حرف نون/ميم ساكنة ولا أي شيء آخر).
        if self.idgham_shamsi_map.contains_key(&following_letter) {
            return TajweedRuleType::IdghamShamsi;
        }

        TajweedRuleType::NoRule
    }

    /// الدالة الرئيسية لمعالجة الآية واستخراج الأحكام.
    pub fn process_verse(&self, verse: &str) -> Vec<RuleMatch> {
        let mut matches: Vec<RuleMatch> = Vec::new();
        let verse_chars: Vec<char> = verse.chars().collect();

        for i in 0..verse_chars.len() {
            let current_char = verse_chars[i];

            // 1. فحص النون الساكنة والميم الساكنة
            if current_char == 'ن' || current_char == 'م' {
                // ... (Logic for Nūn and Mīm remains the same)
                let mut next_char_index = i + 1;
                while next_char_index < verse_chars.len()
                    && TajweedProcessor::is_tajweed_ignorable(verse_chars[next_char_index])
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
                        // current_char == 'م'
                        self.determine_rule_for_mim(following_letter)
                    };

                    if rule_type != TajweedRuleType::NoRule {
                        matches.push(RuleMatch {
                            start_index: i,
                            following_letter,
                            rule: TajweedRule::from_type(rule_type),
                        });
                    }
                }
            }

            // 2. فحص لام أل التعريف (ل) إذا سبقتها ألف (ا)
            if current_char == 'ل' && i > 0 && verse_chars[i - 1] == 'ا' {
                // البحث عن الحرف الذي يلي اللام مباشرة
                let mut next_char_index = i + 1;
                while next_char_index < verse_chars.len()
                    && TajweedProcessor::is_tajweed_ignorable(verse_chars[next_char_index])
                {
                    next_char_index += 1;
                }

                if next_char_index < verse_chars.len() {
                    let following_letter = verse_chars[next_char_index];

                    let rule_type = self.determine_rule_for_lam_al(following_letter);

                    if rule_type != TajweedRuleType::NoRule {
                        // نبدأ من الألف (ا) لتمثيل "أل"
                        matches.push(RuleMatch {
                            start_index: i - 1,
                            following_letter,
                            rule: TajweedRule::from_type(rule_type),
                        });
                    }
                }
            }
            // *تجاهل فحص المدود لأنه يتطلب فحصاً معقداً للحرف السابق والحرف اللاحق وظروف الوقف.*
        }

        matches
    }
}

fn main() {
    let processor = TajweedProcessor::new();

    println!("=====================================================");
    println!("  RUST TAJWEED PROCESSOR EXAMPLES");
    println!("=====================================================");

    // // --- I. Lām Al-Ta'rīf (Alif-Lām) Examples ---

    // // 1. الإظهار القمري (Izhar Qamari) - Clear Lām before Lunar Letters
    // let verse_qamari_1 = "فِي الْقَمَرُ";
    // let matches_qamari_1 = processor.process_verse(verse_qamari_1);
    // println!(
    //     "\n--- 1. Izhar Qamari (Lunar): {} (ل + ق) ---",
    //     verse_qamari_1
    // );
    // for m in matches_qamari_1 {
    //     println!(
    //         "Match: Alif-Lām + '{}' -> Rule: {} ({})",
    //         m.following_letter, m.rule.arabic_name, m.rule.english_name
    //     );
    // }

    // // 2. الإظهار القمري (Izhar Qamari) - Example 2
    // let verse_qamari_2 = "الْكِتَابُ";
    // let matches_qamari_2 = processor.process_verse(verse_qamari_2);
    // println!(
    //     "\n--- 2. Izhar Qamari (Lunar): {} (ل + ك) ---",
    //     verse_qamari_2
    // );
    // for m in matches_qamari_2 {
    //     println!(
    //         "Match: Alif-Lām + '{}' -> Rule: {} ({})",
    //         m.following_letter, m.rule.arabic_name, m.rule.english_name
    //     );
    // }

    // // 3. الإدغام الشمسي (Idgham Shamsi) - Merged Lām before Solar Letters
    // let verse_shamsi_1 = "وَالشَّمْسُ";
    // let matches_shamsi_1 = processor.process_verse(verse_shamsi_1);
    // println!(
    //     "\n--- 3. Idgham Shamsi (Solar): {} (ل + ش) ---",
    //     verse_shamsi_1
    // );
    // for m in matches_shamsi_1 {
    //     println!(
    //         "Match: Alif-Lām + '{}' -> Rule: {} ({})",
    //         m.following_letter, m.rule.arabic_name, m.rule.english_name
    //     );
    // }

    // // 4. الإدغام الشمسي (Idgham Shamsi) - Example 2
    // let verse_shamsi_2 = "الزَّيْتُ";
    // let matches_shamsi_2 = processor.process_verse(verse_shamsi_2);
    // println!(
    //     "\n--- 4. Idgham Shamsi (Solar): {} (ل + ز) ---",
    //     verse_shamsi_2
    // );
    // for m in matches_shamsi_2 {
    //     println!(
    //         "Match: Alif-Lām + '{}' -> Rule: {} ({})",
    //         m.following_letter, m.rule.arabic_name, m.rule.english_name
    //     );
    // }

    // // --- II. Nūn Sākinah and Tanwīn Examples ---

    // // 5. الإظهار الحلقي (Izhar Ḥalqī) - Clear Nūn/Tanwīn (Before throat letters ء ه ع ح غ خ)
    // let verse_izhar_1 = "مِنْ هَادٍ";
    // let matches_izhar_1 = processor.process_verse(verse_izhar_1);
    // println!("\n--- 5. Izhar Ḥalqī: {} (ن + ه) ---", verse_izhar_1);
    // for m in matches_izhar_1 {
    //     println!(
    //         "Match: Nūn/Tanwīn + '{}' -> Rule: {} ({})",
    //         m.following_letter, m.rule.arabic_name, m.rule.english_name
    //     );
    // }

    // // 6. الإظهار الحلقي (Izhar Ḥalqī) - Example 2 (Tanwīn)
    // let verse_izhar_2 = "عَلِيمٌ حَكِيمٌ";
    // let matches_izhar_2 = processor.process_verse(verse_izhar_2);
    // println!("\n--- 6. Izhar Ḥalqī: {} (تنوين + ح) ---", verse_izhar_2);
    // for m in matches_izhar_2 {
    //     println!(
    //         "Match: Nūn/Tanwīn + '{}' -> Rule: {} ({})",
    //         m.following_letter, m.rule.arabic_name, m.rule.english_name
    //     );
    // }

    // // 7. الإقلاب (Iqlab) - Conversion to Mīm (Before ب)
    // let verse_iqlab_1 = "مِن بَعْدِ";
    // let matches_iqlab_1 = processor.process_verse(verse_iqlab_1);
    // println!("\n--- 7. Iqlab: {} (ن + ب) ---", verse_iqlab_1);
    // for m in matches_iqlab_1 {
    //     println!(
    //         "Match: Nūn/Tanwīn + '{}' -> Rule: {} ({})",
    //         m.following_letter, m.rule.arabic_name, m.rule.english_name
    //     );
    // }

    // // 8. الإخفاء الحقيقي (Ikhfa' Ḥaqīqī) - Concealment (Before 15 Ikhfa' letters)
    // let verse_ikhfa_1 = "أَنْفُسَكُمْ";
    // let matches_ikhfa_1 = processor.process_verse(verse_ikhfa_1);
    // println!("\n--- 8. Ikhfa' Ḥaqīqī: {} (ن + ف) ---", verse_ikhfa_1);
    // for m in matches_ikhfa_1 {
    //     println!(
    //         "Match: Nūn/Tanwīn + '{}' -> Rule: {} ({})",
    //         m.following_letter, m.rule.arabic_name, m.rule.english_name
    //     );
    // }

    // // 9. الإدغام بغنة (Idgham bi-Ghunnah) - Merging with Nasalization (Before ي ن م و)
    // let verse_idgham_g_1 = "مَنْ يَعْمَلْ";
    // let matches_idgham_g_1 = processor.process_verse(verse_idgham_g_1);
    // println!(
    //     "\n--- 9. Idgham bi-Ghunnah: {} (ن + ي) ---",
    //     verse_idgham_g_1
    // );
    // for m in matches_idgham_g_1 {
    //     println!(
    //         "Match: Nūn/Tanwīn + '{}' -> Rule: {} ({})",
    //         m.following_letter, m.rule.arabic_name, m.rule.english_name
    //     );
    // }

    // // 10. الإدغام بغير غنة (Idgham bi-Ghayr Ghunnah) - Merging without Nasalization (Before ل ر)
    // let verse_idgham_bg_1 = "مِن لَّدُنْ";
    // let matches_idgham_bg_1 = processor.process_verse(verse_idgham_bg_1);
    // println!(
    //     "\n--- 10. Idgham bi-Ghayr Ghunnah: {} (ن + ل) ---",
    //     verse_idgham_bg_1
    // );
    // for m in matches_idgham_bg_1 {
    //     println!(
    //         "Match: Nūn/Tanwīn + '{}' -> Rule: {} ({})",
    //         m.following_letter, m.rule.arabic_name, m.rule.english_name
    //     );
    // }

    // // 11. الإدغام بغير غنة (Idgham bi-Ghayr Ghunnah) - Example 2 (Tanwīn)
    // let verse_idgham_bg_2 = "غَفُورٌ رَحِيمٌ";
    // let matches_idgham_bg_2 = processor.process_verse(verse_idgham_bg_2);
    // println!(
    //     "\n--- 11. Idgham bi-Ghayr Ghunnah: {} (تنوين + ر) ---",
    //     verse_idgham_bg_2
    // );
    // for m in matches_idgham_bg_2 {
    //     println!(
    //         "Match: Nūn/Tanwīn + '{}' -> Rule: {} ({})",
    //         m.following_letter, m.rule.arabic_name, m.rule.english_name
    //     );
    // }

    // --- III. Mīm Sākinah Examples ---

    // 12. الإخفاء الشفوي (Ikhfa' Shafawī) - Lip Concealment (Before ب)
    let verse_ikhfa_sh_1 = "تَبَٰرَكَ اَ۬لذِے بِيَدِهِ اِ۬لْمُلْكُ وَهُوَ عَلَيٰ كُلِّ شَےْءٖ قَدِيرٌۖ";
    let matches_ikhfa_sh_1 = processor.process_verse(verse_ikhfa_sh_1);
    println!("\n--- 12. Ikhfa' Shafawī: {} (م + ب) ---", verse_ikhfa_sh_1);
    for m in matches_ikhfa_sh_1 {
        println!(
            "Match: Mīm Sākinah + '{}' -> Rule: {} ({})",
            m.following_letter, m.rule.arabic_name, m.rule.english_name
        );
    }

    // // 13. الإدغام الشفوي (Idgham Shafawī) - Lip Merging (Before م)
    // let verse_idgham_sh_1 = "لَكُمْ مَا";
    // let matches_idgham_sh_1 = processor.process_verse(verse_idgham_sh_1);
    // println!(
    //     "\n--- 13. Idgham Shafawī: {} (م + م) ---",
    //     verse_idgham_sh_1
    // );
    // for m in matches_idgham_sh_1 {
    //     println!(
    //         "Match: Mīm Sākinah + '{}' -> Rule: {} ({})",
    //         m.following_letter, m.rule.arabic_name, m.rule.english_name
    //     );
    // }

    // // 14. الإظهار الشفوي (Izhar Shafawī) - Lip Clear Pronunciation (Before all other letters)
    // let verse_izhar_sh_1 = "أَلَمْ تَرَ";
    // let matches_izhar_sh_1 = processor.process_verse(verse_izhar_sh_1);
    // println!("\n--- 14. Izhar Shafawī: {} (م + ت) ---", verse_izhar_sh_1);
    // for m in matches_izhar_sh_1 {
    //     println!(
    //         "Match: Mīm Sākinah + '{}' -> Rule: {} ({})",
    //         m.following_letter, m.rule.arabic_name, m.rule.english_name
    //     );
    // }
}
