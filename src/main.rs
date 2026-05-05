//! Interactive CLI for Tajweed rule processing
//!
//! This is a command-line interface to the tajweed_rules library.
//! It allows interactive analysis of Quranic verses for Tajweed rules.

use std::io::{self, Write};
use tajweed_rules::{RecitationStyle, RuleMatch, TajweedProcessor};

fn main() {
    println!("=======================================================");
    println!("  Tajweed Processor - Interactive CLI");
    println!("  Version: {}", tajweed_rules::VERSION);
    println!("=======================================================\n");

    let processor_warsh = TajweedProcessor::new(RecitationStyle::Warsh);
    let processor_hafs = TajweedProcessor::new(RecitationStyle::Hafs);

    enum SelectedStyle {
        Warsh,
        Hafs,
        Both,
    }

    let mut selected = SelectedStyle::Both;

    println!("Interactive mode: enter a verse and press Enter to analyze.");
    println!("Commands: :q or q to quit, :style warsh|hafs|both to switch styles\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input. Exiting.");
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input == "q" || input == ":q" || input == "quit" {
            println!("Goodbye.");
            break;
        }

        if input.starts_with(":style") {
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() >= 2 {
                match parts[1].to_lowercase().as_str() {
                    "warsh" => {
                        selected = SelectedStyle::Warsh;
                        println!("Style set to Warsh.");
                    }
                    "hafs" => {
                        selected = SelectedStyle::Hafs;
                        println!("Style set to Hafs.");
                    }
                    "both" => {
                        selected = SelectedStyle::Both;
                        println!("Style set to Both.");
                    }
                    _ => println!("Unknown style. Use warsh, hafs, or both."),
                }
            } else {
                println!("Usage: :style warsh|hafs|both");
            }

            continue;
        }

        match selected {
            SelectedStyle::Warsh => {
                let matches = processor_warsh.process_verse(input);
                display_results(input, matches, "Warsh");
            }
            SelectedStyle::Hafs => {
                let matches = processor_hafs.process_verse(input);
                display_results(input, matches, "Hafs");
            }
            SelectedStyle::Both => {
                println!("--- WARSH ---");
                let matches_w = processor_warsh.process_verse(input);
                display_results(input, matches_w.clone(), "Warsh");

                println!("--- HAFS ---");
                let matches_h = processor_hafs.process_verse(input);
                display_results(input, matches_h, "Hafs");
            }
        }
    }
}

fn display_results(verse: &str, matches: Vec<RuleMatch>, style_name: &str) {
    println!("Verse: {}", verse);
    println!("Style: {}\n", style_name);

    if matches.is_empty() {
        println!("  No Tajweed rules detected.\n");
        return;
    }

    for (idx, m) in matches.iter().enumerate() {
        println!("  Rule #{}", idx + 1);
        println!("    Position: {} to {}", m.start_index, m.end_index);
        println!("    Target Letter: '{}'", m.target_letter);
        if let Some(following) = m.following_letter {
            println!("    Following Letter: '{}'", following);
        }
        println!("    Rule (Arabic): {}", m.rule.arabic_name);
        println!("    Rule (English): {}", m.rule.english_name);
        println!("    Description: {}", m.rule.description_ar);
        if m.rule.warsh_specific {
            println!("    ⚠️  Warsh-Specific Rule");
        }
        if let Some((min, max)) = m.rule.madd_length_warsh {
            println!("    Madd Length: {} - {} harakaat", min, max);
        }
        println!("    Context: {}", m.context);
        println!();
    }
    println!("{}\n", "=".repeat(55));
}
