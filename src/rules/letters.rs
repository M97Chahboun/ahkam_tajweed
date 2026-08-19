//! Centralized Arabic letter sets used across all Tajweed rule modules.
//!
//! All character slices are `pub const` so they can be used as compile-time
//! data — zero runtime overhead, and the compiler enforces completeness.

// ─── Noon/Mim Sakinah & Tanwin ──────────────────────────────────────────────

/// حروف الحلق — Throat/pharyngeal letters that trigger Al-Izhar Al-Halqi
/// (ء أ إ ؤ ئ آ هـ ع ح غ خ)
pub const IZHAR_HALQI: &[char] = &['ء', 'أ', 'إ', 'ؤ', 'ئ', 'آ', 'ه', 'ع', 'ح', 'غ', 'خ'];

/// حروف إدغام بغنة — Letters triggering Idgham with Ghunnah (ينمو)
pub const IDGHAM_BI_GHUNNAH: &[char] = &['ي', 'ن', 'م', 'و'];

/// حروف إدغام بغير غنة — Letters triggering Idgham without Ghunnah (لر)
pub const IDGHAM_BILA_GHUNNAH: &[char] = &['ل', 'ر'];

/// حروف الإخفاء الحقيقي — 15 Ikhfaa letters for Noon Sakinah/Tanwin
pub const IKHFAA: &[char] = &[
    'ص', 'ذ', 'ث', 'ك', 'ج', 'ش', 'ق', 'س', 'د', 'ط', 'ز', 'ف', 'ت', 'ض', 'ظ',
];

/// حرف الإقلاب — Letter that triggers Iqlab (Baa ب)
pub const IQLAB: char = 'ب';

/// حرف الإخفاء الشفوي — Letter that triggers Ikhfaa Shafawi for Mim Sakinah (Baa ب)
pub const IKHFAA_SHAFAWI: char = 'ب';

/// حرف الإدغام الشفوي — Letter that triggers Idgham Shafawi for Mim Sakinah (Mim م)
pub const IDGHAM_SHAFAWI: char = 'م';

// ─── Lam Al-Ta'rif ──────────────────────────────────────────────────────────

/// الحروف القمرية — 14 Moon letters; Lam Al-Ta'rif is clear (Izhar Qamari)
pub const IZHAR_QAMARI: &[char] = &[
    'ا', 'ب', 'غ', 'ح', 'ج', 'ك', 'و', 'خ', 'ف', 'ع', 'ق', 'ي', 'م', 'ه',
];

/// الحروف الشمسية — 14 Sun letters; Lam Al-Ta'rif assimilates (Idgham Shamsi)
pub const IDGHAM_SHAMSI: &[char] = &[
    'ت', 'ث', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ل', 'ن',
];

// ─── Qalqalah ────────────────────────────────────────────────────────────────

/// أحرف القلقلة — 5 Qalqalah (bouncing) letters: قطبجد
pub const QALQALAH: &[char] = &['ق', 'ط', 'ب', 'ج', 'د'];

// ─── Madd (vowel prolongation) ───────────────────────────────────────────────

/// أحرف المد الأصلية — Primary Madd carrier letters (Alif, Waw, Ya, Arabic Ya)
pub const MADD_CARRIERS: &[char] = &['ا', 'و', 'ي', '\u{06CC}'];

// ─── Hamza forms ─────────────────────────────────────────────────────────────

/// أشكال الهمزة — All Unicode Hamza forms (for Naql and Tasheel detection)
pub const HAMZA_FORMS: &[char] = &['ء', 'أ', 'إ', 'ؤ', 'ئ', 'آ'];

// ─── Madd non-carrier (for Naql exclusion) ───────────────────────────────────

/// أحرف المد بدون الهمزة — Madd letters that prevent Naql transfer
pub const MADD_LETTERS: &[char] = &['ا', 'و', 'ي'];
