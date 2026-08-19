//! Core types and structures for Tajweed rule processing
//!
//! This module defines the fundamental types used throughout the tajweed processor,
//! including recitation styles, rule types, and data structures for rule matching.

/// Represents different Quranic recitation styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecitationStyle {
    /// حفص عن عاصم - Hafs narration
    Hafs,
    /// ورش عن نافع - Warsh narration
    Warsh,
}

/// Enumeration of all supported Tajweed rules
///
/// This comprehensive enum covers rules for:
/// - Noon/Mim Sakinah (أحكام النون الساكنة والتنوين / أحكام الميم الساكنة)
/// - Lam Al-Ta'rif (أحكام لام أل التعريف)
/// - Madd rules (أحكام المدود)
/// - Qalqalah (القلقلة)
/// - Special Warsh-specific rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TajweedRuleType {
    // أحكام النون الساكنة والتنوين (Noon Sakinah and Tanwin Rules)
    /// الإظهار الحلقي - Al-Izhar Al-Halqi
    IzharHalqi,
    /// الإظهار المطلق - Al-Izhar Al-Mutlaq (Same word exception)
    IzharMutlaq,
    /// الوقف اللازم (مـ) - Waqf Lazim (Compulsory Stop)
    WaqfLazim,
    /// الوقف الممنوع (لا) - Waqf Mamnou (Prohibited Stop)
    WaqfMamnou,
    /// الوقف الجائز (ج) - Waqf Jaiz (Permissible Stop)
    WaqfJaiz,
    /// الوقف الأولى (قلى) - Waqf Awla (Stop Preferred)
    WaqfAwla,
    /// الوصل الأولى (صلى) - Wasl Awla (Continue Preferred)
    WaslAwla,
    /// تعانق الوقف (∴) - Mu'anaqah (Stop at one of the two)
    WaqfMuanaqah,
    /// السكت (س) - Sakt (Pause without breath)
    Sakt,
    /// الإدغام بغنة - Idgham with Ghunnah
    IdghamBiGhunnah,
    /// الإدغام بغير غنة - Idgham without Ghunnah
    IdghamBilaGhunnah,
    /// الإدغام الناقص - Idgham Naqis (Warsh specific)
    IdghamNaqis,
    /// الإدغام الكامل - Idgham Kamil
    IdghamKamil,
    /// الإقلاب - Al-Iqlab (Noon before Ba)
    Iqlab,
    /// الإخفاء الحقيقي - Al-Ikhfaa Al-Haqiqi
    IkhfaaHaqiqi,

    // أحكام الميم الساكنة (Mim Sakinah Rules)
    /// الإخفاء الشفوي - Al-Ikhfaa Al-Shafawi
    IkhfaaShafawi,
    /// الإدغام الشفوي - Al-Idgham Al-Shafawi
    IdghamShafawi,
    /// إدغام المثلين - Idgham Mithlayn
    IdghamMithlayn,
    /// الإظهار الشفوي - Al-Izhar Al-Shafawi
    IzharShafawi,

    // أحكام لام أل التعريف (Lam Al-Ta'rif Rules)
    /// الإظهار القمري - Al-Izhar Al-Qamari
    IzharQamari,
    /// الإدغام الشمسي - Al-Idgham Al-Shamsi
    IdghamShamsi,

    // أحكام المدود (Madd Rules - Enhanced for Warsh)
    /// المد الطبيعي - Madd Tabeei (2 harakaat)
    MaddTabeei,
    /// المد المتصل - Madd Muttasil (4-5 harakaat, Warsh: 4-6)
    MaddMuttasil,
    /// المد المنفصل - Madd Munfasil (2-4-5 harakaat, Warsh: 4-6)
    MaddMunfasil,
    /// المد اللازم - Madd Lazim (6 harakaat)
    MaddLazim,
    /// المد العارض للسكون - Madd Arid (2-4-6 harakaat)
    MaddArid,
    /// المد اللين - Madd Lin (2-4-6 harakaat)
    MaddLin,
    /// مد البدل - Madd Badal (2 harakaat, Warsh: 4-6)
    MaddBadal,
    /// صلة الهاء - Madd Silah (Warsh specific)
    MaddSilah,

    // أحكام الراءات (Ra Rules - Warsh specific)
    /// ترقيق الراء - Tarqeeq Ra (Warsh)
    TarqeeqRa,
    /// تفخيم الراء - Tafkhim Ra
    TafkhimRa,

    // أحكام لفظ الجلالة (Allah Name)
    /// تفخيم لفظ الجلالة - Tafkhim Lafz Al-Jalalah
    TafkhimLafuljalala,

    // أحكام القلقلة (Qalqalah Rules)
    /// القلقلة الكبرى - Qalqalah Kubra (at word end)
    QalqalahKubra,
    /// القلقلة الصغرى - Qalqalah Sughra (connected)
    QalqalahSughra,

    /// القلقلة الأكبر - Qalqalah Akbar (Shadda at word end / Waqf)
    QalqalahAkbar,

    // الغنة (Ghunnah)
    /// الغنة في المشدد - Ghunnah Mushadda (Noon/Meem with Shadda — 2-harakat nasal)
    GhunnahMushadda,

    // أحكام لفظ الجلالة (Allah Name — complete)
    /// ترقيق لفظ الجلالة - Tarqeeq Lafz Al-Jalalah (after Kasra → light)
    TarqeeqLafuljalala,

    // أحكام ورش الخاصة (Warsh-specific)
    /// النقل - An-Naql (transfer Hamza vowel to preceding Sakin — Warsh)
    Naql,
    /// تسهيل الهمزة - Tasheel Al-Hamza (soften second Hamza in same word — Warsh)
    TasheelHamza,

    // إدغام المتماثلين والمتجانسين والمتقاربين
    /// إدغام المتجانسين - Idgham Mutajanisayn (same articulation point, diff. sifat)
    IdghamMutajanisayn,
    /// إدغام المتقاربين - Idgham Mutaqaribayn (adjacent articulation points)
    IdghamMutaqaribayn,

    // همزة الوصل
    /// همزة الوصل - Hamzat Al-Wasl (connecting Hamza, dropped in continuous reading)
    HamzatWasl,

    // تفخيم حروف الاستعلاء
    /// تفخيم حروف الاستعلاء - Tafkhim Isti'la Letters (خص ضغط قظ)
    TafkhimHuruf,

    // الإشمام والاختلاس
    /// الإشمام - Al-Ishmam (lip rounding for elided Dammah in تأمنا/تامنّا)
    Ishmam,

    /// No applicable rule
    NoRule,
}

/// Detailed information about a Tajweed rule
#[derive(Debug, Clone)]
pub struct TajweedRule {
    /// The rule type
    pub rule_type: TajweedRuleType,
    /// Rule name in Arabic
    pub arabic_name: &'static str,
    /// Rule name in English
    pub english_name: &'static str,
    /// Detailed description in Arabic
    pub description_ar: &'static str,
    /// Whether this is Warsh-specific
    pub warsh_specific: bool,
    /// Madd length in Warsh variant: (min, max) in harakaat
    pub madd_length_warsh: Option<(u8, u8)>,
}

impl TajweedRule {
    /// Create a TajweedRule from its type, considering the recitation style
    pub fn from_type(rule_type: TajweedRuleType, style: RecitationStyle) -> Self {
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
                description_ar:
                    "نطق النون بحالة بين الإظهار والإدغام مع غنة عند الـ 15 حرفاً.",
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
            TajweedRuleType::IdghamShafawi | TajweedRuleType::IdghamMithlayn => {
                TajweedRule {
                    rule_type,
                    arabic_name: "الإدغام الشفوي (المثلين الصغير)",
                    english_name: "Al-Idgham Al-Shafawi",
                    description_ar: "إدغام الميم الساكنة في ميم متحركة تليها.",
                    warsh_specific: false,
                    madd_length_warsh: None,
                }
            }
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
                description_ar: "تفخيم الراء حسب القواعس.",
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
            TajweedRuleType::IzharMutlaq => TajweedRule {
                rule_type,
                arabic_name: "الإظهار المطلق",
                english_name: "Al-Izhar Al-Mutlaq",
                description_ar: "إظهار النون الساكنة من نفس الكلمة قبل الواو والياء.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::WaqfLazim => TajweedRule {
                rule_type,
                arabic_name: "الوقف اللازم",
                english_name: "Waqf Lazim",
                description_ar: "علامة (مـ): يجب الوقف هنا.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::WaqfMamnou => TajweedRule {
                rule_type,
                arabic_name: "الوقف الممنوع",
                english_name: "Waqf Mamnou",
                description_ar: "علامة (لا): لا يجوز الوقف هنا (إلا إذا انقطع النفس).",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::WaqfJaiz => TajweedRule {
                rule_type,
                arabic_name: "الوقف الجائز",
                english_name: "Waqf Jaiz",
                description_ar: "علامة (ج): يجوز الوقف ويجوز الوصل (مستوي الطرفين).",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::WaqfAwla => TajweedRule {
                rule_type,
                arabic_name: "الوقف أولى",
                english_name: "Waqf Awla",
                description_ar: "علامة (قلى): الوقف أولى من الوصل.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::WaslAwla => TajweedRule {
                rule_type,
                arabic_name: "الوصل أولى",
                english_name: "Wasl Awla",
                description_ar: "علامة (صلى): الوصل أولى من الوقف.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::WaqfMuanaqah => TajweedRule {
                rule_type,
                arabic_name: "تعانق الوقف",
                english_name: "Mu'anaqah",
                description_ar: "علامة (∴): إذا وقفت على أحدهما لا تقف على الآخر.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::Sakt => TajweedRule {
                rule_type,
                arabic_name: "السكت",
                english_name: "Sakt",
                description_ar: "علامة (س): سكتة لطيفة دون تنفس.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::QalqalahAkbar => TajweedRule {
                rule_type,
                arabic_name: "القلقلة الأكبر",
                english_name: "Qalqalah Akbar (Greatest)",
                description_ar: "القلقلة الأكبر: أحد أحرف القلقلة مع شدة عند الوقف — أقوى مراتب القلقلة.",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::GhunnahMushadda => TajweedRule {
                rule_type,
                arabic_name: "الغنة في المشدد",
                english_name: "Ghunnah Mushadda",
                description_ar: "غنة بمقدار حركتين عند النون أو الميم المشددتين (مثل: إنّ، ثمّ).",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::TarqeeqLafuljalala => TajweedRule {
                rule_type,
                arabic_name: "ترقيق لفظ الجلالة",
                english_name: "Tarqeeq Lafz Al-Jalalah",
                description_ar: "ترقيق لفظ الجلالة (الله) إذا سبقه كسر (مثل: بِاللَّه).",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::Naql => TajweedRule {
                rule_type,
                arabic_name: "النقل",
                english_name: "An-Naql",
                description_ar: "نقل حركة همزة القطع إلى الحرف الساكن قبلها وحذف الهمزة — خاصة برواية ورش.",
                warsh_specific: true,
                madd_length_warsh: None,
            },
            TajweedRuleType::TasheelHamza => TajweedRule {
                rule_type,
                arabic_name: "تسهيل الهمزة",
                english_name: "Tasheel Al-Hamza",
                description_ar: "تسهيل الهمزة الثانية بين الهمزة وحرف المد المجانس لحركتها — خاصة برواية ورش.",
                warsh_specific: true,
                madd_length_warsh: None,
            },
            TajweedRuleType::IdghamMutajanisayn => TajweedRule {
                rule_type,
                arabic_name: "إدغام المتجانسين",
                english_name: "Idgham Mutajanisayn",
                description_ar: "إدغام حرف ساكن في حرف متحرك من نفس المخرج مع اختلاف الصفات (مثل: ط+ت، ذ+ظ، د+ت).",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::IdghamMutaqaribayn => TajweedRule {
                rule_type,
                arabic_name: "إدغام المتقاربين",
                english_name: "Idgham Mutaqaribayn",
                description_ar: "إدغام حرف ساكن في حرف متحرك من مخرج قريب (مثل: ق+ك، ب+م، ل+ر).",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::HamzatWasl => TajweedRule {
                rule_type,
                arabic_name: "همزة الوصل",
                english_name: "Hamzat Al-Wasl",
                description_ar: "همزة الوصل: تُنطق عند الابتداء وتُحذف في الوصل (مثل: اذهب، الرحمن).",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::TafkhimHuruf => TajweedRule {
                rule_type,
                arabic_name: "تفخيم حروف الاستعلاء",
                english_name: "Tafkhim (Heavy Letters)",
                description_ar: "تفخيم أحرف الاستعلاء السبعة المجموعة في (خُصَّ ضَغْطٍ قِظْ).",
                warsh_specific: false,
                madd_length_warsh: None,
            },
            TajweedRuleType::Ishmam => TajweedRule {
                rule_type,
                arabic_name: "الإشمام",
                english_name: "Al-Ishmam",
                description_ar: "الإشمام: ضم الشفتين بعيد تسكين النون مع بقاء الغنة للإشارة إلى الضمة المحذوفة، ويجوز فيه الاختلاس (الروم) في (تَأْمَنَّا / تَامَ۬نَّا) بسورة يوسف.",
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

/// Represents a detected rule match in a Quranic verse
#[derive(Debug, Clone)]
pub struct RuleMatch {
    /// Index where the rule starts
    pub start_index: usize,
    /// Index where the rule ends
    pub end_index: usize,
    /// The target letter the rule applies to
    pub target_letter: char,
    /// The following letter (if relevant to the rule)
    pub following_letter: Option<char>,
    /// The detected rule details
    pub rule: TajweedRule,
    /// Context around the match (for display purposes)
    pub context: String,
}

/// Context information for Madd detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaddContext {
    /// Hamza in same word
    Muttasil,
    /// Hamza in next word
    Munfasil,
    /// Natural (no hamza)
    Tabeeii,
    /// Hamza at beginning, alif at beginning
    Badal,
    /// Extended/prolonged letter (shadda)
    Lazim,
    /// Medium context
    Arid,
    /// Lam or Ra with sukun
    Lin,
}
