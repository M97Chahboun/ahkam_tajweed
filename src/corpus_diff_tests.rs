//! Regression tests derived from a whole-Quran differential comparison
//! against the `cpfair/quran-tajweed` reference corpus (CC-BY-4.0), run over
//! the Tanzil Uthmani text in the Hafs style.
//!
//! Every case below is a *concrete discrepancy* the comparison surfaced and
//! that was then confirmed against the tajweed itself — not a taxonomy
//! difference between the two projects. The verse fragments are quoted
//! verbatim from the Uthmani text so the exact orthography (Alif Wasla
//! U+0671, the silent Alif U+06DF, unmarked Meem Sakinah) is preserved.

#[cfg(test)]
mod corpus_diff_tests {
    use crate::types::{RecitationStyle, RuleMatch, TajweedRuleType};
    use crate::TajweedProcessor;

    fn analyze(verse: &str) -> Vec<RuleMatch> {
        TajweedProcessor::new(RecitationStyle::Hafs).process_verse(verse)
    }

    fn has_rule(matches: &[RuleMatch], rule: TajweedRuleType) -> bool {
        matches.iter().any(|m| m.rule.rule_type == rule)
    }

    fn count(matches: &[RuleMatch], rule: TajweedRuleType) -> usize {
        matches.iter().filter(|m| m.rule.rule_type == rule).count()
    }

    // ═════════════════════════════════════════════════════════════════════
    // Hamzat Wasl — the Uthmani script writes it as Alif Wasla (U+0671)
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn hamzat_wasl_on_alif_wasla_of_the_definite_article() {
        // بِسْمِ ٱللَّهِ — the ٱ is U+0671, not a plain Alif.
        let m = analyze("بِسْمِ ٱللَّهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ");
        assert_eq!(
            count(&m, TajweedRuleType::HamzatWasl),
            3,
            "each ٱ of the three definite articles is a Hamzat Wasl"
        );
    }

    #[test]
    fn hamzat_wasl_on_alif_wasla_after_a_prefix() {
        // بِٱسْمِ — the Alif Wasla sits *inside* the word, after the Ba prefix.
        let m = analyze("ٱقْرَأْ بِٱسْمِ رَبِّكَ ٱلَّذِى خَلَقَ");
        assert_eq!(
            count(&m, TajweedRuleType::HamzatWasl),
            2,
            "بِٱسْمِ and ٱلَّذِى each carry a Hamzat Wasl; the one opening ٱقْرَأْ \
             begins the recitation and is pronounced, not dropped"
        );
    }

    #[test]
    fn a_hamzat_wasl_opening_the_verse_is_pronounced() {
        let m = analyze("ٱقْرَأْ بِٱسْمِ");
        assert!(
            m.iter().all(|r| r.start_index != 0
                || r.rule.rule_type != TajweedRuleType::HamzatWasl),
            "nothing precedes the first letter, so nothing connects to it"
        );
    }

    #[test]
    fn hamzat_qata_is_not_a_hamzat_wasl() {
        let m = analyze("أَنْعَمْتَ عَلَيْهِمْ");
        assert!(
            !has_rule(&m, TajweedRuleType::HamzatWasl),
            "أ is Hamzat Qat', it is never dropped"
        );
    }

    // ═════════════════════════════════════════════════════════════════════
    // Mim Sakinah written without a Sukun (Uthmani convention before م / ب)
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn idgham_shafawi_on_an_unmarked_mim_before_mim() {
        // فِى قُلُوبِهِم مَّرَضٌ (2:10) — the Mim of قُلُوبِهِم carries no Sukun.
        let m = analyze("فِى قُلُوبِهِم مَّرَضٌ");
        assert!(
            has_rule(&m, TajweedRuleType::IdghamMithlayn)
                || has_rule(&m, TajweedRuleType::IdghamShafawi),
            "قُلُوبِهِم مَّرَضٌ is Idgham Shafawi"
        );
    }

    #[test]
    fn ikhfa_shafawi_on_an_unmarked_mim_before_ba() {
        // وَمَا هُم بِمُؤْمِنِينَ (2:8)
        let m = analyze("وَمَا هُم بِمُؤْمِنِينَ");
        assert!(
            has_rule(&m, TajweedRuleType::IkhfaaShafawi),
            "هُم بِمُؤْمِنِينَ is Ikhfaa Shafawi"
        );
    }

    #[test]
    fn a_voweled_mim_is_not_a_mim_sakinah() {
        let m = analyze("رَبُّهُمَ بِهِمْ");
        assert!(
            !has_rule(&m, TajweedRuleType::IkhfaaShafawi),
            "a Mim carrying a Fatha is not Sakinah, so no Ikhfaa Shafawi"
        );
    }

    // ═════════════════════════════════════════════════════════════════════
    // Madd Munfasil across the silent Alif (ا۟, U+0627 + U+06DF)
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn munfasil_is_seen_across_the_silent_alif() {
        // قَالُوٓا۟ إِنَّمَا (2:11) — the Maddah sits on the Waw, the Alif after
        // it is silent, and the Hamza belongs to the *next* word.
        let m = analyze("قَالُوٓا۟ إِنَّمَا نَحْنُ مُصْلِحُونَ");
        assert!(
            has_rule(&m, TajweedRuleType::MaddMunfasil),
            "قَالُوٓا۟ إِنَّمَا is Madd Munfasil"
        );
        assert!(
            !has_rule(&m, TajweedRuleType::MaddMuttasil),
            "the Hamza is in the next word, so this is not Muttasil"
        );
    }

    #[test]
    fn muttasil_is_still_detected_within_one_word() {
        let m = analyze("سَوَآءٌ عَلَيْهِمْ");
        assert!(
            has_rule(&m, TajweedRuleType::MaddMuttasil),
            "سَوَآءٌ carries the Hamza in the same word"
        );
    }

    // ═════════════════════════════════════════════════════════════════════
    // Madd Lazim — Harfi (the disjoined letters) and Kalimi
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn muqattaat_letters_are_madd_lazim_harfi() {
        // الٓمٓ (2:1) — the Maddah sits over the Lam and over the Mim.
        let m = analyze("الٓمٓ");
        assert_eq!(
            count(&m, TajweedRuleType::MaddLazim),
            2,
            "لَام and مِيم are each six harakaat (Madd Lazim Harfi Mukhaffaf)"
        );
        assert!(
            !has_rule(&m, TajweedRuleType::MaddMuttasil),
            "there is no Hamza in الٓمٓ"
        );
    }

    #[test]
    fn madd_lazim_kalimi_muthaqqal_is_detected() {
        let m = analyze("وَلَا ٱلضَّآلِّينَ");
        assert!(
            has_rule(&m, TajweedRuleType::MaddLazim),
            "ٱلضَّآلِّينَ — Madd letter followed by a Shadda in the same word"
        );
    }

    #[test]
    fn a_shadda_on_the_madd_letter_itself_is_not_madd_lazim() {
        // إِيَّاكَ — the Shadda is on the Ya, not on the letter *after* the
        // Madd letter, so there is no Madd Lazim here.
        let m = analyze("إِيَّاكَ نَعْبُدُ وَإِيَّاكَ نَسْتَعِينُ");
        assert!(
            !has_rule(&m, TajweedRuleType::MaddLazim),
            "إِيَّاكَ has no Madd Lazim"
        );
    }

    #[test]
    fn a_shadda_in_the_next_word_is_not_madd_lazim() {
        // ...وَمِمَّا رَزَقْنَٰهُمْ — the final Alif of مِمَّا is a natural Madd;
        // a Shadda opening the next word cannot make it Lazim.
        let m = analyze("وَمِمَّا رَّزَقْنَٰهُمْ يُنفِقُونَ");
        assert!(
            !has_rule(&m, TajweedRuleType::MaddLazim),
            "Madd Lazim requires the Sukun/Shadda in the same word"
        );
    }

    // ═════════════════════════════════════════════════════════════════════
    // Lam Al-Ta'rif written without its Alif (after the لـ prefix)
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn article_after_the_lam_prefix_is_shamsi() {
        // لِلنَّاسِ = لِ + النَّاس — the article's Alif is dropped in writing.
        let m = analyze("هُدًى لِّلنَّاسِ");
        assert!(
            has_rule(&m, TajweedRuleType::IdghamShamsi),
            "لِلنَّاسِ — the article's Lam before a sun letter"
        );
    }

    #[test]
    fn article_after_the_lam_prefix_is_qamari() {
        let m = analyze("هُدًى لِّلْمُتَّقِينَ");
        assert!(
            has_rule(&m, TajweedRuleType::IzharQamari),
            "لِلْمُتَّقِينَ — the article's Lam before a moon letter"
        );
    }

    // ═════════════════════════════════════════════════════════════════════
    // A Madd letter carries no vowel of its own
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn a_voweled_waw_is_a_consonant_not_a_madd_letter() {
        // وُسْعَهَا — the Waw carries the Damma, so it is the consonant the
        // syllable starts with. A Madd Waw is bare and *follows* a Damma.
        let m = analyze("لَا يُكَلِّفُ ٱللَّهُ نَفْسًا إِلَّا وُسْعَهَا");
        assert!(
            !has_rule(&m, TajweedRuleType::MaddLazim),
            "وُسْعَهَا — the Waw is a consonant, there is no Madd on it"
        );
    }

    #[test]
    fn a_silent_alif_carries_no_madd() {
        // أَنَا۠ أُحْىِۦ — the Alif of أَنَا۠ is written but not read (U+06E0),
        // so it cannot be lengthened before the Hamza of the next word.
        let m = analyze("قَالَ أَنَا۠ أُحْىِۦ وَأُمِيتُ");
        assert!(
            !has_rule(&m, TajweedRuleType::MaddMunfasil),
            "the Alif of أَنَا۠ is silent — no Munfasil"
        );
    }

    // ═════════════════════════════════════════════════════════════════════
    // Madd Silah — the Haa has to be a pronoun
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn the_haa_of_the_name_of_allah_is_not_a_silah() {
        let m = analyze("وَعْدَ ٱللَّهِ إِنَّ ٱللَّهَ لَا يُخْلِفُ ٱلْمِيعَادَ");
        assert!(
            !has_rule(&m, TajweedRuleType::MaddSilah)
                && !has_rule(&m, TajweedRuleType::MaddMunfasil),
            "the Haa of ٱللَّهِ belongs to the word, it is not a pronoun"
        );
    }

    #[test]
    fn a_written_silah_is_still_detected() {
        let m = analyze("وَزَيَّنَهُۥ فِے قُلُوبِكُمْ");
        assert!(has_rule(&m, TajweedRuleType::MaddSilah));
    }

    // ═════════════════════════════════════════════════════════════════════
    // Madd 'Arid li-Sukun — the stop at the end of the verse
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn the_final_madd_of_a_verse_is_arid_li_sukun() {
        // ٱلرَّحِيمِ — the Ya is the Madd letter and the Mim is stopped on.
        let m = analyze("بِسْمِ ٱللَّهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ");
        assert!(
            has_rule(&m, TajweedRuleType::MaddArid),
            "the last Madd letter of a verse is 'Arid li-Sukun"
        );
    }

    #[test]
    fn a_madd_in_mid_verse_stays_natural() {
        let m = analyze("كَانَ ٱلنَّاسُ أُمَّةً وَٰحِدَةً");
        assert!(
            has_rule(&m, TajweedRuleType::MaddTabeei),
            "the Alif of كَانَ is read on, so the Madd stays natural"
        );
    }

    #[test]
    fn a_verse_ending_in_a_plain_consonant_has_no_arid() {
        let m = analyze("قُلْ هُوَ ٱللَّهُ أَحَدٌ");
        assert!(
            !has_rule(&m, TajweedRuleType::MaddArid),
            "أَحَدٌ has no Madd letter before its last consonant"
        );
    }

    // ═════════════════════════════════════════════════════════════════════
    // One occurrence, one match
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn an_iqlab_written_and_derived_is_reported_once() {
        // أَنۢبِئْهُم — the Noon carries the small high Meem *and* stands
        // before a Ba, so both detection stages see it.
        let m = analyze("قَالَ يَٰٓـَٔادَمُ أَنۢبِئْهُم بِأَسْمَآئِهِمْ");
        assert_eq!(count(&m, TajweedRuleType::Iqlab), 1);
    }

    // ═════════════════════════════════════════════════════════════════════
    // Idgham Mutaqaribayn is not the article's Lam
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn the_article_lam_before_ra_is_shamsi_not_mutaqaribayn() {
        let m = analyze("ٱلرَّحْمَٰنِ ٱلرَّحِيمِ");
        assert!(
            !has_rule(&m, TajweedRuleType::IdghamMutaqaribayn),
            "ٱلرَّحْمَٰنِ is Lam Shamsiyyah, not Idgham Mutaqaribayn"
        );
        assert!(has_rule(&m, TajweedRuleType::IdghamShamsi));
    }

    #[test]
    fn a_non_article_sakin_lam_before_ra_is_still_mutaqaribayn() {
        let m = analyze("قُل رَّبِّ");
        assert!(
            has_rule(&m, TajweedRuleType::IdghamMutaqaribayn),
            "قُل رَّبِّ is the classic Lam+Ra Idgham Mutaqaribayn"
        );
    }
}
