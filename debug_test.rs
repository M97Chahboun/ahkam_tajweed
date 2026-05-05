fn debug_test() {
    use tajweed_rules::{TajweedProcessor, RecitationStyle, TajweedRuleType};

    let processor = TajweedProcessor::new(RecitationStyle::Hafs);
    let matches = processor.process_verse("كِتَابًا كَبِيرًا");

    println!("Found {} rules:", matches.len());
    for (i, rule_match) in matches.iter().enumerate() {
        println!("  {}: {} ({}-{}) - {}",
            i+1,
            rule_match.rule.english_name,
            rule_match.start_index,
            rule_match.end_index,
            rule_match.context
        );
    }
}

fn test_verse_with_possible_false_positive() {
    use tajweed_rules::{TajweedProcessor, RecitationStyle, TajweedRuleType};

    let processor = TajweedProcessor::new(RecitationStyle::Hafs);
    let verse = "إِذَآ أُلۡقُواْ فِيهَا سَمِعُواْ لَهَا شَهِيقٗا وَهِيَ تَفُورُ";
    let matches = processor.process_verse(verse);

    println!("Testing verse: {}", verse);
    println!("Matches found:");
    for m in &matches {
        println!("  - Rule: {:?}, Start: {}, End: {}, Context: '{}'",
                 m.rule.rule_type, m.start_index, m.end_index, m.context);
    }

    let has_tafkhim_jalal = matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::TafkhimLafuljalala);
    println!("Has TafkhimLafuljalala rule: {}", has_tafkhim_jalal);
}

fn test_problematic_phrase() {
    use tajweed_rules::{TajweedProcessor, RecitationStyle, TajweedRuleType};

    let processor = TajweedProcessor::new(RecitationStyle::Hafs);
    let verse = "سَمِعُواْ لَهَا";
    let matches = processor.process_verse(verse);

    println!("Testing problematic phrase: {}", verse);
    println!("Matches found:");
    for m in &matches {
        println!("  - Rule: {:?}, Start: {}, End: {}, Target: '{}', Context: '{}'",
                 m.rule.rule_type, m.start_index, m.end_index, m.target_letter, m.context);
    }

    let has_tafkhim_jalal = matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::TafkhimLafuljalala);
    println!("Has TafkhimLafuljalala rule: {}", has_tafkhim_jalal);

    // Let's also check the individual characters
    let chars: Vec<char> = verse.chars().collect();
    println!("\nCharacter breakdown:");
    for (i, c) in chars.iter().enumerate() {
        println!("Index {}: '{}' (U+{:04X})", i, c, *c as u32);
    }
}

fn main() {
    debug_test();
    println!("\n--- Testing verse that might trigger false positive ---");
    test_verse_with_possible_false_positive();
    println!("\n--- Testing problematic phrase specifically ---");
    test_problematic_phrase();
}