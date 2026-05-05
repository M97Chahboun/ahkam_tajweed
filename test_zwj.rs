use tajweed_rules::{TajweedProcessor, RecitationStyle, apply_zwj_to_text};

fn main() {
    // Test basic ZWJ functionality
    println!("Testing ZWJ functionality:");
    
    let test_text = "السلام عليكم";
    let result = apply_zwj_to_text(test_text);
    println!("Original: {}", test_text);
    println!("With ZWJ: {:?}", result);
    println!("ZWJ count: {}", result.matches('\u{200D}').count());
    
    // Test with Tajweed processor
    let processor = TajweedProcessor::new(RecitationStyle::Warsh);
    let (rules, processed_text) = processor.process_verse_with_zwj("الحمد لله");
    println!("\nTajweed processing with ZWJ:");
    println!("Original: {}", "الحمد لله");
    println!("With ZWJ: {}", processed_text);
    println!("Rules detected: {}", rules.len());
    
    for rule in rules {
        println!("  - {}: {} ({}-{})", 
                 rule.rule.english_name, 
                 rule.context, 
                 rule.start_index, 
                 rule.end_index);
    }
}