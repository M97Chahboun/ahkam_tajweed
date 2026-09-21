//! Regression tests for tester-reported issues (GitHub #4, #5, #6, #8)
//!
//! All three reports come from the same verse — سورة الإسراء (17:1) in the
//! Warsh (Azraq) orthography — and all three are about *narration attribution*
//! or *madd length*, not about whether the rule fires:
//!
//! * **#4 المد المنفصل** — in Warsh via Al-Azraq the Munfasil is six harakaat
//!   (إشباع) only; the 4/5/6 range in the table was wrong. Via Al-Asbahani it
//!   is 2–4 with no ishbaa'.
//! * **#5 صلة الهاء** — Madd Silah is not Warsh-specific (all readers apply it,
//!   with differences), and the Haa is *voweled between two voweled letters*,
//!   not a sakinah Haa.
//! * **#6 ترقيق الراء** — a Ra carrying a kasra (لِنُرِيَهُۥ) is thinned by every
//!   reader; only a *fatha/damma* Ra after a kasra or a sakin Ya is Warsh's own
//!   tarqeeq. The occurrence-level flag must reflect that.
//!
//! **#8 النقل** comes from a different verse — سورة الحجرات (49:7) — and is a
//! *missing rule*, not a mislabelled one: Warsh transfers the vowel of a
//! hamzat qat' onto the sakin Lam of the definite article (الْإِيمَٰن →
//! لِايمَٰن). The Warsh mushaf already writes the result — the Lam carries the
//! transferred vowel and the hamza is left as a bare, silent Alif — so
//! detection has to recognise both that spelling and the ordinary one.

#[cfg(test)]
#[allow(clippy::module_inception)]
mod reported_issues_tests {
    use crate::types::{RecitationStyle, RuleMatch, TajweedRule, TajweedRuleType};
    use crate::TajweedProcessor;

    /// سورة الإسراء (17:1) exactly as submitted in the three reports.
    const AL_ISRA_1: &str = "سُبْحَٰنَ اَ۬لذِےٓ أَسْر۪ىٰ بِعَبْدِهِۦ لَيْلاٗ مِّنَ اَ۬لْمَسْجِدِ اِ۬لْحَرَامِ إِلَى اَ۬لْمَسْجِدِ اِ۬لَاقْصَا اَ۬لذِے بَٰرَكْنَا حَوْلَهُۥ لِنُرِيَهُۥ مِنَ اٰيَٰتِنَآۖ إِنَّهُۥ هُوَ اَ۬لسَّمِيعُ اُ۬لْبَصِيرُۖ";

    /// سورة الحجرات (49:7) exactly as submitted in report #8.
    const AL_HUJURAT_7: &str = "وَاعْلَمُوٓاْ أَنَّ فِيكُمْ رَسُولَ اَ۬للَّهِ لَوْ يُطِيعُكُمْ فِے كَثِيرٖ مِّنَ اَ۬لَامْرِ لَعَنِتُّمْۖ وَلَٰكِنَّ اَ۬للَّهَ حَبَّبَ إِلَيْكُمُ اُ۬لِايمَٰنَ وَزَيَّنَهُۥ فِے قُلُوبِكُمْ وَكَرَّهَ إِلَيْكُمُ اُ۬لْكُفْرَ وَالْفُسُوقَ وَالْعِصْيَانَۖ أُوْلَٰٓئِكَ هُمُ اُ۬لرَّٰشِدُونَ";

    fn analyze(verse: &str, style: RecitationStyle) -> Vec<RuleMatch> {
        TajweedProcessor::new(style).process_verse(verse)
    }

    fn of_type(matches: &[RuleMatch], rule: TajweedRuleType) -> Vec<&RuleMatch> {
        matches
            .iter()
            .filter(|m| m.rule.rule_type == rule)
            .collect()
    }

    fn has_rule(matches: &[RuleMatch], rule: TajweedRuleType) -> bool {
        matches.iter().any(|m| m.rule.rule_type == rule)
    }

    /// The match of `rule` whose context contains `needle`, if any.
    fn match_at<'a>(
        matches: &'a [RuleMatch],
        rule: TajweedRuleType,
        needle: &str,
    ) -> Option<&'a RuleMatch> {
        matches
            .iter()
            .find(|m| m.rule.rule_type == rule && m.context.contains(needle))
    }

    // ═════════════════════════════════════════════════════════════════════
    // Issue #4 — المد المنفصل في رواية ورش (طريق الأزرق) = 6 حركات فقط
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn issue4_munfasil_warsh_length_is_six_only() {
        let rule = TajweedRule::from_type(TajweedRuleType::MaddMunfasil, RecitationStyle::Warsh);
        assert_eq!(
            rule.madd_length_warsh,
            Some((6, 6)),
            "Warsh (Azraq) Munfasil is ishbaa' — six harakaat, no other wajh"
        );
    }

    #[test]
    fn issue4_munfasil_warsh_description_names_both_turuq() {
        let rule = TajweedRule::from_type(TajweedRuleType::MaddMunfasil, RecitationStyle::Warsh);
        assert!(
            rule.description_ar.contains("الأزرق"),
            "Warsh description must name طريق الأزرق: {}",
            rule.description_ar
        );
        assert!(
            rule.description_ar.contains("الأصبهاني"),
            "Warsh description must name طريق الأصبهاني: {}",
            rule.description_ar
        );
        assert!(
            !rule.description_ar.contains("4 أو 5 أو 6"),
            "the old 4/5/6 range must be gone: {}",
            rule.description_ar
        );
    }

    #[test]
    fn issue4_munfasil_hafs_description_untouched() {
        let rule = TajweedRule::from_type(TajweedRuleType::MaddMunfasil, RecitationStyle::Hafs);
        assert!(
            rule.description_ar.contains("حفص"),
            "Hafs description must still describe Hafs: {}",
            rule.description_ar
        );
    }

    #[test]
    fn issue4_munfasil_is_shared_by_all_readers() {
        for style in [RecitationStyle::Warsh, RecitationStyle::Hafs] {
            let rule = TajweedRule::from_type(TajweedRuleType::MaddMunfasil, style);
            assert!(
                !rule.warsh_specific,
                "Madd Munfasil exists for every reader; only its length differs"
            );
        }
    }

    #[test]
    fn issue4_every_munfasil_in_al_isra_1_reports_six_harakaat() {
        let matches = analyze(AL_ISRA_1, RecitationStyle::Warsh);
        let munfasil = of_type(&matches, TajweedRuleType::MaddMunfasil);
        assert!(!munfasil.is_empty(), "17:1 contains Munfasil (اَ۬لذِےٓ أَسْر۪ىٰ)");
        for m in munfasil {
            assert_eq!(
                m.rule.madd_length_warsh,
                Some((6, 6)),
                "Munfasil at {} ('{}') must report 6 harakaat",
                m.start_index,
                m.context
            );
        }
    }

    #[test]
    fn issue4_silah_kubra_also_reports_six_harakaat() {
        // صلة كبرى (Haa Al-Kinayah before a Hamza) is read as a Munfasil.
        let matches = analyze("بِهِۦٓ أَحَدٌ", RecitationStyle::Warsh);
        let munfasil = of_type(&matches, TajweedRuleType::MaddMunfasil);
        assert!(!munfasil.is_empty(), "صلة كبرى should classify as Munfasil");
        for m in munfasil {
            assert_eq!(m.rule.madd_length_warsh, Some((6, 6)));
        }
    }

    // ═════════════════════════════════════════════════════════════════════
    // Same class as #4 — المد المتصل من طريق الأزرق = 6 حركات (الإشباع)
    // Not separately reported, but the table carried the same 4/5/6 error.
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn muttasil_warsh_length_is_six_only() {
        let rule = TajweedRule::from_type(TajweedRuleType::MaddMuttasil, RecitationStyle::Warsh);
        assert_eq!(
            rule.madd_length_warsh,
            Some((6, 6)),
            "Warsh (Azraq) Muttasil is ishbaa' — six harakaat"
        );
        assert!(
            rule.description_ar.contains("الأزرق"),
            "Warsh description must name طريق الأزرق: {}",
            rule.description_ar
        );
        assert!(
            !rule.description_ar.contains("4 أو 5 أو 6"),
            "the old 4/5/6 range must be gone: {}",
            rule.description_ar
        );
    }

    #[test]
    fn muttasil_hafs_description_untouched() {
        let rule = TajweedRule::from_type(TajweedRuleType::MaddMuttasil, RecitationStyle::Hafs);
        assert!(
            rule.description_ar.contains("4 أو 5"),
            "Hafs keeps 4/5 harakaat: {}",
            rule.description_ar
        );
    }

    #[test]
    fn every_muttasil_reports_six_harakaat_in_warsh() {
        let matches = analyze("وَٱلسَّمَآءِ وَٱلطَّارِقِ", RecitationStyle::Warsh);
        let muttasil = of_type(&matches, TajweedRuleType::MaddMuttasil);
        assert!(!muttasil.is_empty(), "السَّمَآءِ contains a Muttasil");
        for m in muttasil {
            assert_eq!(
                m.rule.madd_length_warsh,
                Some((6, 6)),
                "Muttasil at {} ('{}') must report 6 harakaat",
                m.start_index,
                m.context
            );
        }
    }

    // ═════════════════════════════════════════════════════════════════════
    // Issue #5 — صلة الهاء ليست خاصة بورش، والهاء متحركة بين متحركين
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn issue5_silah_is_not_warsh_specific() {
        for style in [RecitationStyle::Warsh, RecitationStyle::Hafs] {
            let rule = TajweedRule::from_type(TajweedRuleType::MaddSilah, style);
            assert!(
                !rule.warsh_specific,
                "Madd Silah is read by all qurra' with differences in detail"
            );
        }
    }

    #[test]
    fn issue5_silah_description_says_voweled_haa_not_sakinah() {
        for style in [RecitationStyle::Warsh, RecitationStyle::Hafs] {
            let rule = TajweedRule::from_type(TajweedRuleType::MaddSilah, style);
            assert!(
                rule.description_ar.contains("متحركة"),
                "description must state the Haa is voweled: {}",
                rule.description_ar
            );
            assert!(
                !rule.description_ar.contains("الساكنة"),
                "description must not call the Haa sakinah: {}",
                rule.description_ar
            );
        }
    }

    #[test]
    fn issue5_no_silah_match_in_al_isra_1_is_flagged_warsh_specific() {
        let matches = analyze(AL_ISRA_1, RecitationStyle::Warsh);
        let silah = of_type(&matches, TajweedRuleType::MaddSilah);
        assert!(
            silah.len() >= 3,
            "17:1 has several صلة صغرى (بِعَبْدِهِۦ، حَوْلَهُۥ، لِنُرِيَهُۥ، إِنَّهُۥ), found {}",
            silah.len()
        );
        for m in silah {
            assert!(
                !m.rule.warsh_specific,
                "Silah at {} ('{}') wrongly flagged Warsh-specific",
                m.start_index, m.context
            );
        }
    }

    #[test]
    fn issue5_silah_is_detected_for_hafs_too() {
        let matches = analyze("لَهُ بُنْيَانًا", RecitationStyle::Hafs);
        assert!(
            has_rule(&matches, TajweedRuleType::MaddSilah),
            "Hafs also joins the Haa of the pronoun (صلة صغرى)"
        );
    }

    #[test]
    fn issue5_voweled_haa_between_two_voweled_letters_triggers_silah() {
        for style in [RecitationStyle::Warsh, RecitationStyle::Hafs] {
            let matches = analyze("بِهِ كَثِيرًا", style);
            assert!(
                has_rule(&matches, TajweedRuleType::MaddSilah),
                "هاء متحركة بين متحركين → صلة"
            );
        }
    }

    #[test]
    fn issue5_haa_after_a_sakin_letter_has_no_silah() {
        // مِنْهُ — the Haa is preceded by a sakin Noon, so there is no Silah.
        for style in [RecitationStyle::Warsh, RecitationStyle::Hafs] {
            let matches = analyze("مِنْهُ كِتَابٌ", style);
            assert!(
                !has_rule(&matches, TajweedRuleType::MaddSilah),
                "no Silah when the letter before the Haa is sakin"
            );
        }
    }

    #[test]
    fn issue5_sakin_haa_has_no_silah() {
        for style in [RecitationStyle::Warsh, RecitationStyle::Hafs] {
            let matches = analyze("فَأَلْقِهْ إِلَيْهِمْ", style);
            assert!(
                !has_rule(&matches, TajweedRuleType::MaddSilah),
                "a sakinah Haa is never joined"
            );
        }
    }

    // ═════════════════════════════════════════════════════════════════════
    // Issue #6 — ترقيق الراء المكسورة متفق عليه، وليس من خصائص ورش
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn issue6_tarqeeq_ra_base_metadata_is_not_warsh_specific() {
        for style in [RecitationStyle::Warsh, RecitationStyle::Hafs] {
            let rule = TajweedRule::from_type(TajweedRuleType::TarqeeqRa, style);
            assert!(
                !rule.warsh_specific,
                "the agreed-upon positions are the default; Warsh's own \
                 positions are flagged per occurrence"
            );
        }
    }

    #[test]
    fn issue6_kasra_ra_in_linuriyahu_is_not_warsh_specific() {
        for style in [RecitationStyle::Warsh, RecitationStyle::Hafs] {
            let matches = analyze("لِنُرِيَهُۥ", style);
            let m =
                match_at(&matches, TajweedRuleType::TarqeeqRa, "ر").expect("راء مكسورة → ترقيق");
            assert!(
                !m.rule.warsh_specific,
                "a Ra carrying a kasra is thinned by every reader, not only Warsh"
            );
        }
    }

    #[test]
    fn issue6_kasra_ra_inside_al_isra_1_is_not_warsh_specific() {
        let matches = analyze(AL_ISRA_1, RecitationStyle::Warsh);
        let m =
            match_at(&matches, TajweedRuleType::TarqeeqRa, "نُرِيَ").expect("ترقيق الراء في لِنُرِيَهُۥ");
        assert!(
            !m.rule.warsh_specific,
            "reported in issue #6: لِنُرِيَهُۥ is agreed upon, not a Warsh feature"
        );
    }

    #[test]
    fn issue6_sakin_ra_after_kasra_is_not_warsh_specific() {
        for style in [RecitationStyle::Warsh, RecitationStyle::Hafs] {
            let matches = analyze("فَاصْبِرْ", style);
            let m = match_at(&matches, TajweedRuleType::TarqeeqRa, "بِرْ")
                .expect("راء ساكنة بعد كسر أصلي → ترقيق");
            assert!(
                !m.rule.warsh_specific,
                "a sakin Ra after an original kasra is agreed upon"
            );
        }
    }

    #[test]
    fn issue6_damma_ra_after_sakin_ya_is_warsh_specific() {
        // اُ۬لْبَصِيرُ — Ra with damma after a sakin Ya: Warsh thins it, Hafs does not.
        let warsh = analyze(AL_ISRA_1, RecitationStyle::Warsh);
        let m = match_at(&warsh, TajweedRuleType::TarqeeqRa, "صِيرُ").expect("ورش يرقق راء البَصِيرُ");
        assert!(
            m.rule.warsh_specific,
            "Ra with damma after a sakin Ya is Warsh's own tarqeeq"
        );

        let hafs = analyze(AL_ISRA_1, RecitationStyle::Hafs);
        assert!(
            match_at(&hafs, TajweedRuleType::TarqeeqRa, "صِيرُ").is_none(),
            "Hafs reads البَصِيرُ with tafkhim while joining"
        );
    }

    #[test]
    fn issue6_fatha_ra_after_sakin_ya_is_warsh_specific() {
        let warsh = analyze("خَيْرًا", RecitationStyle::Warsh);
        let m = of_type(&warsh, TajweedRuleType::TarqeeqRa);
        assert_eq!(m.len(), 1, "خَيْرًا → ترقيق عند ورش");
        assert!(
            m[0].rule.warsh_specific,
            "fatha Ra after a sakin Ya is Warsh-specific"
        );

        let hafs = analyze("خَيْرًا", RecitationStyle::Hafs);
        assert!(
            !has_rule(&hafs, TajweedRuleType::TarqeeqRa),
            "Hafs keeps tafkhim in خَيْرًا"
        );
    }

    #[test]
    fn issue6_fatha_ra_after_kasra_is_warsh_specific() {
        let warsh = analyze("سِرَاجًا", RecitationStyle::Warsh);
        let m = of_type(&warsh, TajweedRuleType::TarqeeqRa);
        assert_eq!(m.len(), 1, "سِرَاجًا → ترقيق عند ورش");
        assert!(m[0].rule.warsh_specific);
    }

    #[test]
    fn issue6_warsh_tafkhim_exceptions_still_hold() {
        // إبراهيم is an exception: Warsh reads its Ra with tafkhim.
        let warsh = analyze("إِبْرَاهِيمَ", RecitationStyle::Warsh);
        assert!(
            !has_rule(&warsh, TajweedRuleType::TarqeeqRa),
            "foreign-name exception must keep tafkhim"
        );
        assert!(has_rule(&warsh, TajweedRuleType::TafkhimRa));
    }

    #[test]
    fn issue6_agreed_and_warsh_specific_tarqeeq_can_coexist_in_one_verse() {
        let matches = analyze(AL_ISRA_1, RecitationStyle::Warsh);
        let tarqeeq = of_type(&matches, TajweedRuleType::TarqeeqRa);
        assert!(
            tarqeeq.iter().any(|m| !m.rule.warsh_specific),
            "17:1 has an agreed-upon tarqeeq (لِنُرِيَهُۥ)"
        );
        assert!(
            tarqeeq.iter().any(|m| m.rule.warsh_specific),
            "17:1 has a Warsh-only tarqeeq (اُ۬لْبَصِيرُ)"
        );
    }

    // ═════════════════════════════════════════════════════════════════════
    // Issue #8 — النقل: نقل حركة همزة القطع إلى لام التعريف الساكنة (ورش)
    // ═════════════════════════════════════════════════════════════════════

    #[test]
    fn issue8_naql_detected_on_lam_al_tarif_written_in_warsh_orthography() {
        // اُ۬لِايمَٰنَ — the Lam already carries the kasra of the elided hamza.
        let warsh = analyze(AL_HUJURAT_7, RecitationStyle::Warsh);
        let naql = match_at(&warsh, TajweedRuleType::Naql, "لِا")
            .expect("النقل في (اُ۬لِايمَٰنَ) must be reported");
        assert_eq!(
            naql.target_letter, 'ل',
            "the Lam receives the transferred vowel"
        );
    }

    #[test]
    fn issue8_naql_detected_for_the_fatha_form_in_the_same_verse() {
        // اَ۬لَامْرِ — same rule, the hamza of (الأمر) carried a fatha.
        let warsh = analyze(AL_HUJURAT_7, RecitationStyle::Warsh);
        assert!(
            match_at(&warsh, TajweedRuleType::Naql, "لَامْ").is_some(),
            "النقل في (اَ۬لَامْرِ) must be reported"
        );
    }

    #[test]
    fn issue8_al_hujurat_7_has_exactly_two_naql_positions() {
        let warsh = analyze(AL_HUJURAT_7, RecitationStyle::Warsh);
        let naql = of_type(&warsh, TajweedRuleType::Naql);
        assert_eq!(
            naql.len(),
            2,
            "49:7 has النقل twice — اَ۬لَامْرِ and اُ۬لِايمَٰنَ — found: {:?}",
            naql.iter().map(|m| m.context.as_str()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn issue8_naql_detected_on_lam_al_tarif_in_ordinary_orthography() {
        // The same word as normally written: sakin Lam followed by hamzat qat'.
        let warsh = analyze("إِلَيْكُمُ الْإِيمَانَ", RecitationStyle::Warsh);
        let naql = of_type(&warsh, TajweedRuleType::Naql);
        assert_eq!(naql.len(), 1, "الْإِيمَان → النقل عند ورش");
        assert_eq!(naql[0].target_letter, 'ل');
        assert_eq!(naql[0].following_letter, Some('إ'));
    }

    #[test]
    fn issue8_naql_is_warsh_only() {
        let hafs = analyze(AL_HUJURAT_7, RecitationStyle::Hafs);
        assert!(
            !has_rule(&hafs, TajweedRuleType::Naql),
            "حفص يحقق الهمزة ولا ينقل"
        );
        assert!(!has_rule(
            &analyze("الْإِيمَانَ", RecitationStyle::Hafs),
            TajweedRuleType::Naql
        ));
    }

    #[test]
    fn issue8_naql_is_flagged_warsh_specific() {
        let warsh = analyze(AL_HUJURAT_7, RecitationStyle::Warsh);
        let naql = of_type(&warsh, TajweedRuleType::Naql);
        assert!(!naql.is_empty());
        assert!(
            naql.iter().all(|m| m.rule.warsh_specific),
            "النقل من خصائص ورش"
        );
    }

    #[test]
    fn issue8_article_before_a_plain_letter_has_no_naql() {
        // اُ۬لْكُفْرَ — sakin Lam, but the next letter is a Kaf, not a hamza.
        let warsh = analyze("إِلَيْكُمُ اُ۬لْكُفْرَ", RecitationStyle::Warsh);
        assert!(!has_rule(&warsh, TajweedRuleType::Naql));
    }

    #[test]
    fn issue8_sun_letter_article_has_no_naql() {
        let warsh = analyze("هُمُ اُ۬لرَّٰشِدُونَ", RecitationStyle::Warsh);
        assert!(!has_rule(&warsh, TajweedRuleType::Naql));
    }

    #[test]
    fn issue8_a_madd_alif_after_lam_is_not_naql() {
        // قَالَا — Lam + fatha + Alif is a plain madd, not an article at all.
        let warsh = analyze("قَالَا لَنْ", RecitationStyle::Warsh);
        assert!(!has_rule(&warsh, TajweedRuleType::Naql));
    }

    #[test]
    fn issue8_cross_word_naql_still_detected() {
        // Pre-existing behaviour must survive: sakin letter + hamza of the next word.
        let warsh = analyze("قَدْ أَفْلَحَ", RecitationStyle::Warsh);
        assert!(has_rule(&warsh, TajweedRuleType::Naql));
    }

    #[test]
    fn issue8_naql_description_names_lam_al_tarif() {
        let rule = TajweedRule::from_type(TajweedRuleType::Naql, RecitationStyle::Warsh);
        assert!(
            rule.description_ar.contains("لام التعريف"),
            "the description should tell the reader where Naql shows up: {}",
            rule.description_ar
        );
    }
}
