//! Dump every detected rule as TSV, for differential testing against an
//! external tajweed corpus.
//!
//! Reads Tanzil-style `surah|ayah|text` lines on stdin and writes one
//! `surah<TAB>ayah<TAB>rule<TAB>start<TAB>end` line per match.
//!
//! ```sh
//! cargo run --example dump_rules -- hafs < quran-uthmani.txt > ours.tsv
//! ```

use std::io::{self, BufRead, Write};

use tajweed_rules::{RecitationStyle, TajweedProcessor};

fn main() {
    let style = match std::env::args().nth(1).as_deref() {
        Some("warsh") => RecitationStyle::Warsh,
        _ => RecitationStyle::Hafs,
    };
    let processor = TajweedProcessor::new(style);

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for line in stdin.lock().lines() {
        let line = line.expect("stdin is valid UTF-8");
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.splitn(3, '|');
        let (Some(surah), Some(ayah), Some(text)) = (parts.next(), parts.next(), parts.next())
        else {
            continue;
        };
        for m in processor.process_verse(text) {
            writeln!(
                out,
                "{}\t{}\t{:?}\t{}\t{}",
                surah, ayah, m.rule.rule_type, m.start_index, m.end_index
            )
            .unwrap();
        }
    }
}
