use tajweed_rules::{TajweedProcessor, RecitationStyle};

fn main() {
    let processor = TajweedProcessor::new(RecitationStyle::Hafs);
    let verse = "سَمِعُواْ لَهَا";
    
    println!("Processing verse: {}", verse);
    let matches = processor.process_verse(verse);

    println!("Found {} rules:", matches.len());
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

    // Let's also check the individual characters
    let chars: Vec<char> = verse.chars().collect();
    println!("\nCharacter breakdown:");
    for (i, c) in chars.iter().enumerate() {
        println!("Index {}: '{}' (U+{:04X})", i, c, *c as u32);
    }
}