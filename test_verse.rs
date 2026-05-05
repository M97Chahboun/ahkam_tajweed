use tajweed_rules::{TajweedProcessor, RecitationStyle, TajweedRuleType};

fn main() {
    let p = TajweedProcessor::new(RecitationStyle::Hafs);
    let verse = "إِذَآ أُلۡقُواْ فِيهَا سَمِعُواْ لَهَا شَهِيقٗا وَهِيَ تَفُورُ";
    let matches = p.process_verse(verse);
    
    println!("Verse: {}", verse);
    println!("Matches found:");
    for m in &matches {
        println!("  - Rule: {:?}, Start: {}, End: {}, Context: '{}'", 
                 m.rule.rule_type, m.start_index, m.end_index, m.context);
    }
    
    let has_tafkhim_jalal = matches.iter().any(|m| m.rule.rule_type == TajweedRuleType::TafkhimLafuljalala);
    println!("Has TafkhimLafuljalala rule: {}", has_tafkhim_jalal);
}