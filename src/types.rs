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

/// Internal metadata row — one entry per [`TajweedRuleType`] variant.
///
/// Fields: `(rule_type, arabic_name, english_name, desc_hafs, desc_warsh, warsh_specific, madd_length_warsh)`
///
/// `desc_hafs` is used for both narrations when the description is identical;
/// `desc_warsh` holds the Warsh-specific phrasing for rules that differ.
struct RuleMeta {
    arabic_name: &'static str,
    english_name: &'static str,
    desc_hafs: &'static str,
    desc_warsh: &'static str,
    warsh_specific: bool,
    madd_length_warsh: Option<(u8, u8)>,
}

/// Static lookup table — one row per [`TajweedRuleType`] variant.
/// Ordered by the enum definition for readability; lookup is linear O(n)
/// over a tiny slice so it is effectively free.
static RULE_TABLE: &[(TajweedRuleType, RuleMeta)] = &[
    (TajweedRuleType::IzharHalqi, RuleMeta {
        arabic_name: "الإظهار الحلقي",
        english_name: "Al-Izhar Al-Halqi",
        desc_hafs: "إظهار النون الساكنة عند حروف الحلق (ء هـ ع ح غ خ).",
        desc_warsh: "إظهار النون الساكنة عند حروف الحلق (ء هـ ع ح غ خ).",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::IzharMutlaq, RuleMeta {
        arabic_name: "الإظهار المطلق",
        english_name: "Al-Izhar Al-Mutlaq",
        desc_hafs: "إظهار النون الساكنة من نفس الكلمة قبل الواو والياء.",
        desc_warsh: "إظهار النون الساكنة من نفس الكلمة قبل الواو والياء.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::WaqfLazim, RuleMeta {
        arabic_name: "الوقف اللازم",
        english_name: "Waqf Lazim",
        desc_hafs: "علامة (مـ): يجب الوقف هنا.",
        desc_warsh: "علامة (مـ): يجب الوقف هنا.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::WaqfMamnou, RuleMeta {
        arabic_name: "الوقف الممنوع",
        english_name: "Waqf Mamnou",
        desc_hafs: "علامة (لا): لا يجوز الوقف هنا (إلا إذا انقطع النفس).",
        desc_warsh: "علامة (لا): لا يجوز الوقف هنا (إلا إذا انقطع النفس).",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::WaqfJaiz, RuleMeta {
        arabic_name: "الوقف الجائز",
        english_name: "Waqf Jaiz",
        desc_hafs: "علامة (ج): يجوز الوقف ويجوز الوصل (مستوي الطرفين).",
        desc_warsh: "علامة (ج): يجوز الوقف ويجوز الوصل (مستوي الطرفين).",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::WaqfAwla, RuleMeta {
        arabic_name: "الوقف أولى",
        english_name: "Waqf Awla",
        desc_hafs: "علامة (قلى): الوقف أولى من الوصل.",
        desc_warsh: "علامة (قلى): الوقف أولى من الوصل.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::WaslAwla, RuleMeta {
        arabic_name: "الوصل أولى",
        english_name: "Wasl Awla",
        desc_hafs: "علامة (صلى): الوصل أولى من الوقف.",
        desc_warsh: "علامة (صلى): الوصل أولى من الوقف.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::WaqfMuanaqah, RuleMeta {
        arabic_name: "تعانق الوقف",
        english_name: "Mu'anaqah",
        desc_hafs: "علامة (∴): إذا وقفت على أحدهما لا تقف على الآخر.",
        desc_warsh: "علامة (∴): إذا وقفت على أحدهما لا تقف على الآخر.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::Sakt, RuleMeta {
        arabic_name: "السكت",
        english_name: "Sakt",
        desc_hafs: "علامة (س): سكتة لطيفة دون تنفس.",
        desc_warsh: "علامة (س): سكتة لطيفة دون تنفس.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::IdghamBiGhunnah, RuleMeta {
        arabic_name: "الإدغام بغنة",
        english_name: "Idgham with Ghunnah",
        desc_hafs: "إدغام النون في (ينمو) مع غنة.",
        desc_warsh: "إدغام النون في (ينمو) مع غنة.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::IdghamBilaGhunnah, RuleMeta {
        arabic_name: "الإدغام بغير غنة",
        english_name: "Idgham without Ghunnah",
        desc_hafs: "إدغام النون في (ل، ر) بدون غنة.",
        desc_warsh: "إدغام النون في (ل، ر) بدون غنة.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::IdghamNaqis, RuleMeta {
        arabic_name: "الإدغام الناقص",
        english_name: "Idgham Naqis (Incomplete)",
        desc_hafs: "إدغام ناقص مع بقاء الغنة في رواية ورش.",
        desc_warsh: "إدغام ناقص مع بقاء الغنة في رواية ورش.",
        warsh_specific: true,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::IdghamKamil, RuleMeta {
        arabic_name: "الإدغام الكامل",
        english_name: "Idgham Kamil (Complete)",
        desc_hafs: "إدغام كامل بدون بقاء صفة الحرف المدغم.",
        desc_warsh: "إدغام كامل بدون بقاء صفة الحرف المدغم.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::Iqlab, RuleMeta {
        arabic_name: "الإقلاب",
        english_name: "Al-Iqlab",
        desc_hafs: "قلب النون الساكنة ميماً مخفاة بغنة عند حرف الباء.",
        desc_warsh: "قلب النون الساكنة ميماً مخفاة بغنة عند حرف الباء.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::IkhfaaHaqiqi, RuleMeta {
        arabic_name: "الإخفاء الحقيقي",
        english_name: "Al-Ikhfaa Al-Haqiqi",
        desc_hafs: "نطق النون بحالة بين الإظهار والإدغام مع غنة عند الـ 15 حرفاً.",
        desc_warsh: "نطق النون بحالة بين الإظهار والإدغام مع غنة عند الـ 15 حرفاً.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::IkhfaaShafawi, RuleMeta {
        arabic_name: "الإخفاء الشفوي",
        english_name: "Al-Ikhfaa Al-Shafawi",
        desc_hafs: "إخفاء الميم الساكنة بغنة عند حرف الباء.",
        desc_warsh: "إخفاء الميم الساكنة بغنة عند حرف الباء.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::IdghamShafawi, RuleMeta {
        arabic_name: "الإدغام الشفوي (المثلين الصغير)",
        english_name: "Al-Idgham Al-Shafawi",
        desc_hafs: "إدغام الميم الساكنة في ميم متحركة تليها.",
        desc_warsh: "إدغام الميم الساكنة في ميم متحركة تليها.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::IdghamMithlayn, RuleMeta {
        arabic_name: "الإدغام الشفوي (المثلين الصغير)",
        english_name: "Al-Idgham Al-Shafawi",
        desc_hafs: "إدغام الميم الساكنة في ميم متحركة تليها.",
        desc_warsh: "إدغام الميم الساكنة في ميم متحركة تليها.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::IzharShafawi, RuleMeta {
        arabic_name: "الإظهار الشفوي",
        english_name: "Al-Izhar Al-Shafawi",
        desc_hafs: "إظهار الميم الساكنة بوضوح عند باقي الحروف.",
        desc_warsh: "إظهار الميم الساكنة بوضوح عند باقي الحروف.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::IzharQamari, RuleMeta {
        arabic_name: "الإظهار القمري",
        english_name: "Al-Izhar Al-Qamari",
        desc_hafs: "إظهار اللام الساكنة في (أل) عند الحروف القمرية الـ 14.",
        desc_warsh: "إظهار اللام الساكنة في (أل) عند الحروف القمرية الـ 14.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::IdghamShamsi, RuleMeta {
        arabic_name: "الإدغام الشمسي",
        english_name: "Al-Idgham Al-Shamsi",
        desc_hafs: "إدغام اللام الساكنة في (أل) في الحروف الشمسية.",
        desc_warsh: "إدغام اللام الساكنة في (أل) في الحروف الشمسية.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    // ── Madd rules (Option B: Hafs and Warsh descriptions stored as pairs) ─
    (TajweedRuleType::MaddTabeei, RuleMeta {
        arabic_name: "المد الطبيعي",
        english_name: "Madd Tabeei",
        desc_hafs: "مد الألف، الواو، والياء بمقدار حركتين.",
        desc_warsh: "مد الألف، الواو، والياء بمقدار حركتين.",
        warsh_specific: false,
        madd_length_warsh: Some((2, 2)),
    }),
    (TajweedRuleType::MaddMuttasil, RuleMeta {
        arabic_name: "المد المتصل",
        english_name: "Madd Muttasil",
        desc_hafs: "المد المتصل: 4 أو 5 حركات في رواية حفص.",
        desc_warsh: "المد المتصل: 4 أو 5 أو 6 حركات في رواية ورش (الأشهر: 6).",
        warsh_specific: false,
        madd_length_warsh: Some((4, 6)),
    }),
    (TajweedRuleType::MaddMunfasil, RuleMeta {
        arabic_name: "المد المنفصل",
        english_name: "Madd Munfasil",
        desc_hafs: "المد المنفصل: 2 أو 4 أو 5 حركات في رواية حفص.",
        desc_warsh: "المد المنفصل: 4 أو 5 أو 6 حركات في رواية ورش (الأشهر: 4).",
        warsh_specific: false,
        madd_length_warsh: Some((4, 6)),
    }),
    (TajweedRuleType::MaddLazim, RuleMeta {
        arabic_name: "المد اللازم",
        english_name: "Madd Lazim",
        desc_hafs: "المد اللازم: 6 حركات (في جميع الروايات).",
        desc_warsh: "المد اللازم: 6 حركات (في جميع الروايات).",
        warsh_specific: false,
        madd_length_warsh: Some((6, 6)),
    }),
    (TajweedRuleType::MaddArid, RuleMeta {
        arabic_name: "المد العارض للسكون",
        english_name: "Madd Arid",
        desc_hafs: "المد العارض للسكون: 2 أو 4 أو 6 حركات.",
        desc_warsh: "المد العارض للسكون: 2 أو 4 أو 6 حركات (حرف المد في آخر الكلمة).",
        warsh_specific: false,
        madd_length_warsh: Some((2, 6)),
    }),
    (TajweedRuleType::MaddLin, RuleMeta {
        arabic_name: "المد اللين",
        english_name: "Madd Lin",
        desc_hafs: "المد اللين: الواو أو الياء الساكنة بعد فتح (مثل: خيل، بيت).",
        desc_warsh: "المد اللين: الواو أو الياء الساكنة بعد فتح (مثل: خيل، بيت).",
        warsh_specific: false,
        madd_length_warsh: Some((2, 6)),
    }),
    (TajweedRuleType::MaddBadal, RuleMeta {
        arabic_name: "مد البدل",
        english_name: "Madd Badal",
        desc_hafs: "مد البدل: حركتان في رواية حفص.",
        desc_warsh: "مد البدل: 2 أو 4 أو 6 حركات في رواية ورش (تسهيل الهمزة).",
        warsh_specific: true,
        madd_length_warsh: Some((2, 6)),
    }),
    (TajweedRuleType::MaddSilah, RuleMeta {
        arabic_name: "صلة الهاء",
        english_name: "Madd Silah",
        desc_hafs: "صلة الهاء الساكنة (تحويل ه الساكنة إلى حرف مد).",
        desc_warsh: "صلة الهاء الساكنة (تحويل ه الساكنة إلى حرف مد).",
        warsh_specific: true,
        madd_length_warsh: None,
    }),
    // ── Ra / Allah Name ──────────────────────────────────────────────────────
    (TajweedRuleType::TarqeeqRa, RuleMeta {
        arabic_name: "ترقيق الراء",
        english_name: "Tarqeeq Ra",
        desc_hafs: "ترقيق الراء في رواية ورش في مواضع خاصة.",
        desc_warsh: "ترقيق الراء في رواية ورش في مواضع خاصة.",
        warsh_specific: true,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::TafkhimRa, RuleMeta {
        arabic_name: "تفخيم الراء",
        english_name: "Tafkhim Ra",
        desc_hafs: "تفخيم الراء حسب القواعد.",
        desc_warsh: "تفخيم الراء حسب القواعد.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::TafkhimLafuljalala, RuleMeta {
        arabic_name: "تفخيم لفظ الجلالة",
        english_name: "Tafkhim Lafz Al-Jalalah",
        desc_hafs: "تفخيم لفظ الجلالة (الله) بعد فتح أو ضم.",
        desc_warsh: "تفخيم لفظ الجلالة (الله) بعد فتح أو ضم.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    // ── Qalqalah ─────────────────────────────────────────────────────────────
    (TajweedRuleType::QalqalahKubra, RuleMeta {
        arabic_name: "القلقلة الكبرى",
        english_name: "Qalqalah Kubra (Major)",
        desc_hafs: "القلقلة الكبرى: رجع الصوت بالقاف أو الطاء أو الباء أو الجيم أو الدال عند الوقف.",
        desc_warsh: "القلقلة الكبرى: رجع الصوت بالقاف أو الطاء أو الباء أو الجيم أو الدال عند الوقف.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::QalqalahSughra, RuleMeta {
        arabic_name: "القلقلة الصغرى",
        english_name: "Qalqalah Sughra (Minor)",
        desc_hafs: "القلقلة الصغرى: رجع الصوت بأحد أحرف القلقلة في الوصل (غير متطرفة).",
        desc_warsh: "القلقلة الصغرى: رجع الصوت بأحد أحرف القلقلة في الوصل (غير متطرفة).",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::QalqalahAkbar, RuleMeta {
        arabic_name: "القلقلة الأكبر",
        english_name: "Qalqalah Akbar (Greatest)",
        desc_hafs: "القلقلة الأكبر: أحد أحرف القلقلة مع شدة عند الوقف — أقوى مراتب القلقلة.",
        desc_warsh: "القلقلة الأكبر: أحد أحرف القلقلة مع شدة عند الوقف — أقوى مراتب القلقلة.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    // ── Ghunnah ──────────────────────────────────────────────────────────────
    (TajweedRuleType::GhunnahMushadda, RuleMeta {
        arabic_name: "الغنة في المشدد",
        english_name: "Ghunnah Mushadda",
        desc_hafs: "غنة بمقدار حركتين عند النون أو الميم المشددتين (مثل: إنّ، ثمّ).",
        desc_warsh: "غنة بمقدار حركتين عند النون أو الميم المشددتين (مثل: إنّ، ثمّ).",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    // ── Allah Name (Tarqeeq) ─────────────────────────────────────────────────
    (TajweedRuleType::TarqeeqLafuljalala, RuleMeta {
        arabic_name: "ترقيق لفظ الجلالة",
        english_name: "Tarqeeq Lafz Al-Jalalah",
        desc_hafs: "ترقيق لفظ الجلالة (الله) إذا سبقه كسر (مثل: بِاللَّه).",
        desc_warsh: "ترقيق لفظ الجلالة (الله) إذا سبقه كسر (مثل: بِاللَّه).",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    // ── Warsh-specific ───────────────────────────────────────────────────────
    (TajweedRuleType::Naql, RuleMeta {
        arabic_name: "النقل",
        english_name: "An-Naql",
        desc_hafs: "نقل حركة همزة القطع إلى الحرف الساكن قبلها وحذف الهمزة — خاصة برواية ورش.",
        desc_warsh: "نقل حركة همزة القطع إلى الحرف الساكن قبلها وحذف الهمزة — خاصة برواية ورش.",
        warsh_specific: true,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::TasheelHamza, RuleMeta {
        arabic_name: "تسهيل الهمزة",
        english_name: "Tasheel Al-Hamza",
        desc_hafs: "تسهيل الهمزة الثانية بين الهمزة وحرف المد المجانس لحركتها — خاصة برواية ورش.",
        desc_warsh: "تسهيل الهمزة الثانية بين الهمزة وحرف المد المجانس لحركتها — خاصة برواية ورش.",
        warsh_specific: true,
        madd_length_warsh: None,
    }),
    // ── Assimilation ─────────────────────────────────────────────────────────
    (TajweedRuleType::IdghamMutajanisayn, RuleMeta {
        arabic_name: "إدغام المتجانسين",
        english_name: "Idgham Mutajanisayn",
        desc_hafs: "إدغام حرف ساكن في حرف متحرك من نفس المخرج مع اختلاف الصفات (مثل: ط+ت، ذ+ظ، د+ت).",
        desc_warsh: "إدغام حرف ساكن في حرف متحرك من نفس المخرج مع اختلاف الصفات (مثل: ط+ت، ذ+ظ، د+ت).",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    (TajweedRuleType::IdghamMutaqaribayn, RuleMeta {
        arabic_name: "إدغام المتقاربين",
        english_name: "Idgham Mutaqaribayn",
        desc_hafs: "إدغام حرف ساكن في حرف متحرك من مخرج قريب (مثل: ق+ك، ب+م، ل+ر).",
        desc_warsh: "إدغام حرف ساكن في حرف متحرك من مخرج قريب (مثل: ق+ك، ب+م، ل+ر).",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    // ── Hamzat Wasl ──────────────────────────────────────────────────────────
    (TajweedRuleType::HamzatWasl, RuleMeta {
        arabic_name: "همزة الوصل",
        english_name: "Hamzat Al-Wasl",
        desc_hafs: "همزة الوصل: تُنطق عند الابتداء وتُحذف في الوصل (مثل: اذهب، الرحمن).",
        desc_warsh: "همزة الوصل: تُنطق عند الابتداء وتُحذف في الوصل (مثل: اذهب، الرحمن).",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    // ── Tafkhim Isti'la ───────────────────────────────────────────────────────
    (TajweedRuleType::TafkhimHuruf, RuleMeta {
        arabic_name: "تفخيم حروف الاستعلاء",
        english_name: "Tafkhim (Heavy Letters)",
        desc_hafs: "تفخيم أحرف الاستعلاء السبعة المجموعة في (خُصَّ ضَغْطٍ قِظْ).",
        desc_warsh: "تفخيم أحرف الاستعلاء السبعة المجموعة في (خُصَّ ضَغْطٍ قِظْ).",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    // ── Ishmam ────────────────────────────────────────────────────────────────
    (TajweedRuleType::Ishmam, RuleMeta {
        arabic_name: "الإشمام",
        english_name: "Al-Ishmam",
        desc_hafs: "الإشمام: ضم الشفتين بعيد تسكين النون مع بقاء الغنة للإشارة إلى الضمة المحذوفة، ويجوز فيه الاختلاس (الروم) في (تَأْمَنَّا / تَامَ۬نَّا) بسورة يوسف.",
        desc_warsh: "الإشمام: ضم الشفتين بعيد تسكين النون مع بقاء الغنة للإشارة إلى الضمة المحذوفة، ويجوز فيه الاختلاس (الروم) في (تَأْمَنَّا / تَامَ۬نَّا) بسورة يوسف.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
    // ── Fallback ──────────────────────────────────────────────────────────────
    (TajweedRuleType::NoRule, RuleMeta {
        arabic_name: "لا يوجد حكم",
        english_name: "No Rule",
        desc_hafs: "لا يوجد حكم تجويدي.",
        desc_warsh: "لا يوجد حكم تجويدي.",
        warsh_specific: false,
        madd_length_warsh: None,
    }),
];

impl TajweedRule {
    /// Create a [`TajweedRule`] from its type and the active recitation style.
    ///
    /// Performs a linear scan over [`RULE_TABLE`] — the table is tiny (~35 entries)
    /// so this is effectively free.  Returns a `NoRule` fallback if the variant
    /// is not yet registered in the table.
    pub fn from_type(rule_type: TajweedRuleType, style: RecitationStyle) -> Self {
        let meta = RULE_TABLE
            .iter()
            .find(|(t, _)| *t == rule_type)
            .map(|(_, m)| m)
            .unwrap_or_else(|| {
                // Safety valve: should never happen if the table is complete.
                &RULE_TABLE[RULE_TABLE.len() - 1].1  // NoRule entry
            });

        let description_ar = if style == RecitationStyle::Warsh {
            meta.desc_warsh
        } else {
            meta.desc_hafs
        };

        TajweedRule {
            rule_type,
            arabic_name: meta.arabic_name,
            english_name: meta.english_name,
            description_ar,
            warsh_specific: meta.warsh_specific,
            madd_length_warsh: meta.madd_length_warsh,
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
