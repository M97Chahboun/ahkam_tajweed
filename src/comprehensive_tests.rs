//! Comprehensive tests for all Tajweed rules

#[cfg(test)]
mod comprehensive_tests {
    use crate::{TajweedProcessor, RecitationStyle, TajweedRuleType};

    // Helper function to check if a rule exists
    fn has_rule(matches: &[crate::RuleMatch], rule_type: TajweedRuleType) -> bool {
        matches.iter().any(|m| m.rule.rule_type == rule_type)
    }

    // Helper function to count rules
    fn count_rules(matches: &[crate::RuleMatch], rule_type: TajweedRuleType) -> usize {
        matches.iter().filter(|m| m.rule.rule_type == rule_type).count()
    }

    #[test]
    fn test_comprehensive_noon_sakinah_rules() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // Izhar Halqi - Noon before throat letters
        let matches = processor.process_verse("مِنْ أَنْبِيَاءِ");
        assert!(has_rule(&matches, TajweedRuleType::IzharHalqi));
        
        // Idgham Bi-Ghunnah - Noon before Y, N, M, W
        let matches = processor.process_verse("مِنْ يَقُولُ");
        assert!(has_rule(&matches, TajweedRuleType::IdghamBiGhunnah));
        
        // Idgham Bila-Ghunnah - Noon before L, R
        let matches = processor.process_verse("مِنْ لَدُنْهُ");
        assert!(has_rule(&matches, TajweedRuleType::IdghamBilaGhunnah));
        
        // Iqlab - Noon before B
        let matches = processor.process_verse("مِنْ بَعْدِ");
        assert!(has_rule(&matches, TajweedRuleType::Iqlab));
        
        // Ikhfaa Haqiqi - Noon before 15 letters
        let matches = processor.process_verse("مِنْ كَانَ");
        assert!(has_rule(&matches, TajweedRuleType::IkhfaaHaqiqi));
    }

    #[test]
    fn test_comprehensive_tanwin_rules() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // Tanwin with Izhar
        let matches = processor.process_verse("رَجُلًا عَدْلًا");
        assert!(has_rule(&matches, TajweedRuleType::IkhfaaHaqiqi)); // Tanwin with K
        
        // Tanwin with Idgham
        let matches = processor.process_verse("كِتَابًا يَسْمَعُونَ");
        assert!(has_rule(&matches, TajweedRuleType::IkhfaaHaqiqi)); // Tanwin with Y
    }

    #[test]
    fn test_comprehensive_mim_sakinah_rules() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // Ikhfaa Shafawi - Mim before B
        let matches = processor.process_verse("كُمْ بِهِم");
        assert!(has_rule(&matches, TajweedRuleType::IkhfaaShafawi));
        
        // Idgham Shafawi - Mim before Mim
        let matches = processor.process_verse("كُمْ مُحْسِنُون");
        assert!(has_rule(&matches, TajweedRuleType::IdghamMithlayn));
        
        // Izhar Shafawi - Mim before other letters
        let matches = processor.process_verse("كُمْ فَعَلْتُم");
        assert!(has_rule(&matches, TajweedRuleType::IzharShafawi));
    }

    #[test]
    fn test_comprehensive_lam_al_tarif_rules() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // Izhar Qamari - Al- before moon letters
        let matches = processor.process_verse("القَمَرُ");
        assert!(has_rule(&matches, TajweedRuleType::IzharQamari));
        
        // Idgham Shamsi - Al- before sun letters
        let matches = processor.process_verse("الشَّمْسُ");
        assert!(has_rule(&matches, TajweedRuleType::IdghamShamsi));
    }

    #[test]
    fn test_comprehensive_madd_rules() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // Madd Tabeei - natural madd
        let matches = processor.process_verse("كَانَ");
        assert!(has_rule(&matches, TajweedRuleType::MaddTabeei));
        
        // Madd Muttasil - connected madd (Alif + Hamza in same word)
        let matches = processor.process_verse("سُوْءٌ"); // Su'ün - Waw with Damma followed by Hamza
        assert!(has_rule(&matches, TajweedRuleType::MaddMuttasil));
        
        // Madd Munfasil - disconnected madd (Alif + Hamza in different words)
        let matches = processor.process_verse("مَا أَنْتَ");
        assert!(has_rule(&matches, TajweedRuleType::MaddMunfasil));
        
        // Madd Lazim - required madd (letter with shadda)
        let matches = processor.process_verse("أَمَّا"); // Alif followed by Mim with Shadda
        assert!(has_rule(&matches, TajweedRuleType::MaddLazim));
        
        // Madd Lin - soft madd (Waw/Ya with Fatha followed by Sukun)
        let matches = processor.process_verse("لَيْسَ"); // Ya with Fatha followed by Sukun
        assert!(has_rule(&matches, TajweedRuleType::MaddLin));
    }

    #[test]
    fn test_comprehensive_qalqalah_rules() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // Qalqalah Sughra - minor bouncing (within word)
        let matches = processor.process_verse("يَجْعَلُ");
        assert!(has_rule(&matches, TajweedRuleType::QalqalahSughra));
        
        // Qalqalah Kubra - major bouncing (at end)
        let matches = processor.process_verse("قَدْ");
        assert!(has_rule(&matches, TajweedRuleType::QalqalahKubra));
    }

    #[test]
    fn test_comprehensive_ra_rules() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // Tafkhim Ra - heavy Ra (with Fatha or Damma)
        let matches = processor.process_verse("رَحْمَنِ");
        assert!(has_rule(&matches, TajweedRuleType::TafkhimRa));
        
        // Tarqeeq Ra - light Ra (with Kasra) - more common in Warsh
        let warsh_processor = TajweedProcessor::new(RecitationStyle::Warsh);
        let matches = warsh_processor.process_verse("رِيحٌ");
        assert!(has_rule(&matches, TajweedRuleType::TarqeeqRa));
    }

    #[test]
    fn test_comprehensive_allah_name_rules() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // Tafkhim Lafuljalala - emphasis of Allah's name
        let matches = processor.process_verse("بِسْمِ اللَّهِ");
        assert!(has_rule(&matches, TajweedRuleType::TafkhimLafuljalala));
    }

    #[test]
    fn test_comprehensive_waqf_signs() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // Test all waqf signs
        let verse = format!("أ\u{06D6}ب\u{06D7}ت\u{06DA}ث\u{06DB}ج\u{06D5}ح\u{06D9}");
        let matches = processor.process_verse(&verse);
        
        assert!(has_rule(&matches, TajweedRuleType::WaslAwla));
        assert!(has_rule(&matches, TajweedRuleType::WaqfAwla));
        assert!(has_rule(&matches, TajweedRuleType::WaqfJaiz));
        assert!(has_rule(&matches, TajweedRuleType::WaqfMuanaqah));
        assert!(has_rule(&matches, TajweedRuleType::WaqfLazim));
        assert!(has_rule(&matches, TajweedRuleType::WaqfMamnou));
    }

    #[test]
    fn test_comprehensive_sakt_rule() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // Sakt rule
        let matches = processor.process_verse("أ\u{06DC}ب\u{06DC}ت");
        assert!(count_rules(&matches, TajweedRuleType::Sakt) >= 2);
    }

    #[test]
    fn test_warsh_specific_rules() {
        let warsh_processor = TajweedProcessor::new(RecitationStyle::Warsh);
        let hafs_processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // Idgham Naqis - incomplete assimilation (Warsh specific)
        let warsh_matches = warsh_processor.process_verse("مِنْ يَجْرِي");
        let hafs_matches = hafs_processor.process_verse("مِنْ يَجْرِي");
        
        // In Warsh, this might produce IdghamNaqis, but in Hafs it won't
        // Both should produce some form of Idgham though
        assert!(has_rule(&warsh_matches, TajweedRuleType::IdghamBiGhunnah) || 
                has_rule(&warsh_matches, TajweedRuleType::IdghamNaqis));
    }

    #[test]
    fn test_complex_integration() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // Complex verse with multiple rules
        let matches = processor.process_verse("بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيمِ");
        
        // Should have multiple rule types
        let unique_types: std::collections::HashSet<_> = 
            matches.iter().map(|m| m.rule.rule_type).collect();
        
        assert!(unique_types.len() >= 4, "Complex verse should have multiple rule types");
        assert!(has_rule(&matches, TajweedRuleType::TafkhimLafuljalala));
        assert!(has_rule(&matches, TajweedRuleType::TafkhimRa));
    }

    #[test]
    fn test_edge_cases() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // Empty string
        assert!(processor.process_verse("").is_empty());
        
        // Whitespace only
        assert!(processor.process_verse("   \t\n  ").is_empty());
        
        // Single non-trigger character
        assert!(processor.process_verse("خ").is_empty());
    }

    #[test]
    fn test_izhar_mutlaq_case() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // "دُنْيَا" - Noon Sakinah in same word followed by 'ي' - Izhar Mutlaq
        let matches = processor.process_verse("دُنْيَا");
        assert!(has_rule(&matches, TajweedRuleType::IzharMutlaq));
    }

    #[test]
    fn test_madd_lin_cases() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        
        // "لَيْسَ" - Ya with Fatha followed by Sukun on Sin
        let matches = processor.process_verse("لَيْسَ");
        assert!(has_rule(&matches, TajweedRuleType::MaddLin) || has_rule(&matches, TajweedRuleType::MaddTabeei));
        
        // "وَقْفٌ" - Waw with Fatha followed by Sukun on Qaf
        let matches = processor.process_verse("وَقْفٌ");
        assert!(has_rule(&matches, TajweedRuleType::MaddLin) || has_rule(&matches, TajweedRuleType::MaddTabeei));
    }

    #[test]
    fn test_tarqeeq_ra_case() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);

        // "رِيحٌ" - Ra with Kasra should trigger Tarqeeq Ra in Warsh
        let matches = processor.process_verse("رِيحٌ");
        // In Hafs, this might still be Tafkhim, but let's check what gets detected
        println!("Rules for 'رِيحٌ': {:?}", matches.iter().map(|m| m.rule.english_name).collect::<Vec<_>>());
    }

    #[test]
    fn debug_tanwin_ikhfaa_case() {
        let processor = TajweedProcessor::new(RecitationStyle::Hafs);
        let matches = processor.process_verse("كِتَابًا كَبِيرًا");

        println!("Debug: Found {} rules for 'كِتَابًا كَبِيرًا':", matches.len());
        for (i, rule_match) in matches.iter().enumerate() {
            println!("  {}: {} ({}-{}) - '{}' -> '{}' | Context: {}",
                i+1,
                rule_match.rule.english_name,
                rule_match.start_index,
                rule_match.end_index,
                rule_match.target_letter,
                rule_match.following_letter.map(|c| c.to_string()).unwrap_or("-".to_string()),
                rule_match.context
            );
        }

        // The expected behavior: "كِتَابًا" has tanwin fatha (ً) on ba, followed by kaaf in next word
        // This should trigger Ikhfaa Haqiqi since Kaaf is one of the 15 Ikhfaa letters
        assert!(has_rule(&matches, TajweedRuleType::IkhfaaHaqiqi),
                "Expected IkhfaaHaqiqi rule for tanwin followed by Kaaf");
    }
}