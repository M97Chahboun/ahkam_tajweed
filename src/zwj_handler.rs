//! ZWJ (Zero Width Joiner) handling for Arabic text rendering
//!
//! This module provides functionality to properly handle Arabic letter connectivity
//! by inserting ZWJ characters where needed to maintain proper joining behavior
//! in text rendering contexts.

/// Zero Width Joiner character
pub const ZWJ: char = '\u{200D}';

/// Letters that do NOT connect to the letter that FOLLOWS them (right-side non-connectors)
/// These letters can receive connections from the right but don't extend to the left
pub const RIGHT_NON_CONNECTORS: [char; 42] = [
    // Alef family - connects from right, not to left
    'ا', 'أ', 'إ', 'آ', 'ٱ', // Dal family - connects from right, not to left
    'د', 'ذ', // Ra family - connects from right, not to left
    'ر', 'ز', // Waw family - connects from right, not to left
    'و', 'ؤ', // Other non-connectors
    'ة', // Ta marbuta
    '\u{0621}', // Hamza (Independent)
    '\u{0649}', // Alef Maqsura
    // Spaces and marks
    ' ', '\u{00A0}', '\u{200B}', '\u{200C}', '\u{200D}', // Quranic marks
    'ۙ', 'ۚ', 'ۖ', 'ۗ', 'ۘ', 'ۜ', 'ۢ', '۟', '۠', // Small letters
    '\u{06DB}', '\u{06DD}', '\u{06DE}', '\u{06E9}', '\u{06E4}', '\u{06E3}', '\u{06EA}', '\u{06EB}', '\u{06EC}', '\u{06ED}',
    'ۥ', 'ۦ', 'ۧ', 'ۨ',
];

/// Letters that do NOT connect FROM the right (can't receive connections)
pub const NO_RIGHT_CONNECTION: [char; 29] = [
    ' ', '\u{00A0}', '\u{200B}', '\u{200C}', '\u{200D}', 
    '\u{0621}', // Hamza acts as a separator
    'ۙ', 'ۚ', 'ۖ', 'ۗ', 'ۘ', 'ۜ', 'ۢ', '۟', '۠', 'ۥ',
    '\u{06DB}', '\u{06DD}', '\u{06DE}', '\u{06E9}', '\u{06E4}', '\u{06E3}', '\u{06EA}', '\u{06EB}', '\u{06EC}', '\u{06ED}',
    'ۦ', 'ۧ', 'ۨ',
];

/// Check if a character is a space
pub fn is_space(c: char) -> bool {
    matches!(c, ' ' | '\u{00A0}' | '\u{200B}' | '\u{200C}' | '\u{200D}')
}

/// Check if a character is an Arabic letter
pub fn is_arabic_letter(c: char) -> bool {
    matches!(c,
        '\u{0621}'..='\u{064A}' |  // Basic Arabic range
        '\u{0671}'..='\u{06D3}' |  // Extended Arabic range
        '\u{06D5}'                 // Arabic letter AE
    )
}

/// Check if a character is a diacritic
pub fn is_diacritic(c: char) -> bool {
    matches!(c,
        '\u{064B}'..='\u{065F}' |  // Arabic diacritics (Harakat and Tanween)
        '\u{0670}' |                // Superscript alef
        '\u{06D6}'..='\u{06ED}' |  // Quranic annotation marks
        '\u{08D4}'..='\u{08E1}' |  // Extended Arabic diacritics
        '\u{08E3}'..='\u{08FF}'    // More Arabic marks
    )
}

/// Get the base letter from a character cluster (removing diacritics)
pub fn get_base_letter(cluster: &str) -> Option<char> {
    cluster.chars().find(|&c| is_arabic_letter(c))
}

/// Determine if leading ZWJ is needed
/// Leading ZWJ is needed when:
/// 1. Current letter CAN receive connection from right (not in NO_RIGHT_CONNECTION)
/// 2. Previous letter CAN send connection to left (not in RIGHT_NON_CONNECTORS)
/// 3. Previous is not a space
pub fn needs_leading_zwj(current_cluster: &str, previous_cluster: Option<&str>) -> bool {
    let first_base_letter = match get_base_letter(current_cluster) {
        Some(letter) => letter,
        None => return false,
    };

    // Current letter must be able to receive connection from right
    if NO_RIGHT_CONNECTION.contains(&first_base_letter) {
        return false;
    }

    if let Some(prev_cluster) = previous_cluster {
        // Check if previous cluster is a space
        if let Some(first_char) = prev_cluster.chars().next() {
            if is_space(first_char) {
                return false;
            }
        }

        // Previous letter must be able to send connection to left
        if let Some(prev_base_letter) = get_base_letter(prev_cluster) {
            if !RIGHT_NON_CONNECTORS.contains(&prev_base_letter) {
                return true;
            }
        }
    }

    false
}

/// Determine if trailing ZWJ is needed
/// Trailing ZWJ is needed when:
/// 1. Current (last) letter CAN send connection to left (not in RIGHT_NON_CONNECTORS)
/// 2. Next letter CAN receive connection from right (not in NO_RIGHT_CONNECTION)
/// 3. Next is not a space
pub fn needs_trailing_zwj(current_cluster: &str, next_cluster: Option<&str>) -> bool {
    let last_base_letter = match get_base_letter(current_cluster) {
        Some(letter) => letter,
        None => return false,
    };

    // Current letter must be able to send connection to left
    if RIGHT_NON_CONNECTORS.contains(&last_base_letter) {
        return false;
    }

    if let Some(next_cluster) = next_cluster {
        // Check if next cluster is a space
        if let Some(first_char) = next_cluster.chars().next() {
            if is_space(first_char) {
                return false;
            }
        }

        // Next letter must be able to receive connection from right
        if let Some(next_base_letter) = get_base_letter(next_cluster) {
            if !NO_RIGHT_CONNECTION.contains(&next_base_letter) {
                return true;
            }
        }
    }

    false
}

/// Split text into grapheme clusters to keep diacritics with their base letters
/// A cluster consists of a base Arabic letter followed by any diacritics
pub fn split_into_grapheme_clusters(text: &str) -> Vec<String> {
    let mut clusters = Vec::new();
    let mut current_cluster = String::new();

    for c in text.chars() {
        if is_arabic_letter(c) {
            // If we have a previous cluster, add it to the list
            if !current_cluster.is_empty() {
                clusters.push(current_cluster.clone());
                current_cluster.clear();
            }
            // Start a new cluster with the current Arabic letter
            current_cluster.push(c);
        } else if is_diacritic(c) {
            // Add diacritics to the current cluster
            current_cluster.push(c);
        } else {
            // For non-letter, non-diacritic characters (spaces, marks, etc.)
            // Treat them as their own cluster
            if !current_cluster.is_empty() {
                clusters.push(current_cluster.clone());
                current_cluster.clear();
            }
            current_cluster.push(c);
        }
    }

    // Add the last cluster if it exists
    if !current_cluster.is_empty() {
        clusters.push(current_cluster);
    }

    clusters
}

/// Apply ZWJ to text based on connectivity rules
pub fn apply_zwj_to_text(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(text.len() * 2);

    // Buffers for the streaming process
    // prev_cluster: The previously emitted group (needed for leading ZWJ check of current)
    // We only need the base letter and whether it was a space effectively, but helper takes &str.
    // To avoid cloning Strings excessively, we maintain:
    // - pending_cluster: the one we are about to emit (waiting for next to decide trailing)
    // - next_cluster: the one we are building
    
    // Actually, to use existing helpers `needs_leading_zwj` which takes `Option<&str>`, 
    // we would need to keep full strings.
    // Let's implement an optimized loop that builds `next_cluster` char by char.
    
    let mut prev_cluster: Option<String> = None;
    let mut curr_cluster = String::new();
    let mut next_cluster = String::new();
    
    // We need to prime the loop.
    // Iterator over chars
    let mut chars = text.chars().peekable();
    
    // Read first cluster into curr_cluster
    while let Some(&c) = chars.peek() {
         let is_arabic = is_arabic_letter(c);
         let is_dia = is_diacritic(c);
         
         if curr_cluster.is_empty() {
             curr_cluster.push(c);
             chars.next();
         } else if is_arabic {
             // New cluster starts
             break;
         } else if is_dia {
             // Append to current
             curr_cluster.push(c);
             chars.next();
         } else {
             // Other char
             // If curr_cluster has arabic/dia content, this starts new.
             // If curr_cluster is just other chars? existing logic groups "others" as single cluster?
             // split_into_grapheme_clusters says "Treat them as their own cluster... if !current.is_empty push... push(c)"
             break;
         }
    }
    
    // If text was empty or exhausted
    if curr_cluster.is_empty() {
        return result;
    }

    loop {
        // Build next_cluster
        next_cluster.clear();
        
        // We need to peek to decide where next_cluster ends
        while let Some(&c) = chars.peek() {
             let is_arabic = is_arabic_letter(c);
             let is_dia = is_diacritic(c);
             
             if next_cluster.is_empty() {
                 next_cluster.push(c);
                 chars.next();
             } else if is_arabic {
                 // New cluster starts
                 break;
             } else if is_dia {
                 next_cluster.push(c);
                 chars.next();
             } else {
                 // Other char starts new cluster
                 break;
             }
        }
        
        let next_opt = if next_cluster.is_empty() { None } else { Some(next_cluster.as_str()) };
        let prev_opt = prev_cluster.as_deref();
        
        let leading = needs_leading_zwj(&curr_cluster, prev_opt);
        let trailing = needs_trailing_zwj(&curr_cluster, next_opt);
        
        if leading { result.push(ZWJ); }
        result.push_str(&curr_cluster);
        if trailing { result.push(ZWJ); }
        
        if next_cluster.is_empty() {
            break;
        }
        
        // Rotate
        if let Some(ref mut p) = prev_cluster {
            p.clear();
            p.push_str(&curr_cluster);
        } else {
            prev_cluster = Some(curr_cluster.clone());
        }
        
        curr_cluster.clear();
        curr_cluster.push_str(&next_cluster);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_base_letter() {
        assert_eq!(get_base_letter("ن"), Some('ن'));
        assert_eq!(get_base_letter("نَ"), Some('ن')); // with fatha
        assert_eq!(get_base_letter("رُ"), Some('ر')); // with damma
        assert_eq!(get_base_letter("َ"), None); // only diacritic
        assert_eq!(get_base_letter(""), None);
    }

    #[test]
    fn test_is_space() {
        assert!(is_space(' '));
        assert!(is_space('\u{00A0}'));
        assert!(!is_space('ن'));
    }

    #[test]
    fn test_is_arabic_letter() {
        assert!(is_arabic_letter('ن'));
        assert!(is_arabic_letter('ب'));
        assert!(is_arabic_letter('ا'));
        assert!(!is_arabic_letter('a'));
        assert!(!is_arabic_letter(' '));
        assert!(!is_arabic_letter('\u{064E}')); // fatha diacritic
    }

    #[test]
    fn test_is_diacritic() {
        assert!(is_diacritic('\u{064E}')); // fatha
        assert!(is_diacritic('\u{064F}')); // damma
        assert!(is_diacritic('\u{0650}')); // kasra
        assert!(is_diacritic('\u{0652}')); // sukun
        assert!(!is_diacritic('ن'));
        assert!(!is_diacritic(' '));
    }

    #[test]
    fn test_right_non_connectors() {
        assert!(RIGHT_NON_CONNECTORS.contains(&'ا'));
        assert!(RIGHT_NON_CONNECTORS.contains(&'ر'));
        assert!(RIGHT_NON_CONNECTORS.contains(&'و'));
        assert!(RIGHT_NON_CONNECTORS.contains(&'د'));
        assert!(!RIGHT_NON_CONNECTORS.contains(&'ب'));
        assert!(!RIGHT_NON_CONNECTORS.contains(&'ن'));
    }

    #[test]
    fn test_no_right_connection() {
        assert!(NO_RIGHT_CONNECTION.contains(&' '));
        assert!(NO_RIGHT_CONNECTION.contains(&'\u{200D}'));
        assert!(!NO_RIGHT_CONNECTION.contains(&'ا')); // Alef CAN receive connections
    }

    #[test]
    fn test_split_into_grapheme_clusters() {
        // "بِسْمِ" should split into clusters keeping diacritics with letters
        let clusters = split_into_grapheme_clusters("بِسْمِ");

        // Debug: print the clusters
        println!("Clusters for 'بِسْمِ': {:?}", clusters);

        // The text "بِسْمِ" consists of:
        // ب (ba) + ِ (kasra) = "بِ"
        // س (seen) + ْ (sukun) = "سْ"
        // م (meem) + ِ (kasra) = "مِ"
        // Total: 3 clusters
        assert_eq!(
            clusters.len(),
            3,
            "Expected 3 clusters, got {}: {:?}",
            clusters.len(),
            clusters
        );
        assert_eq!(clusters[0], "بِ");
        assert_eq!(clusters[1], "سْ");
        assert_eq!(clusters[2], "مِ");

        // Test simple text without diacritics
        let clusters = split_into_grapheme_clusters("بسم");
        assert_eq!(clusters.len(), 3);
        assert_eq!(clusters[0], "ب");
        assert_eq!(clusters[1], "س");
        assert_eq!(clusters[2], "م");

        // Test with spaces
        let clusters = split_into_grapheme_clusters("ب س");
        assert_eq!(clusters.len(), 3);
        assert_eq!(clusters[0], "ب");
        assert_eq!(clusters[1], " ");
        assert_eq!(clusters[2], "س");
    }

    #[test]
    fn test_needs_leading_zwj() {
        // 'ب' connects to 'ن', so 'ن' needs leading ZWJ
        assert!(needs_leading_zwj("ن", Some("ب")));

        // 'ر' doesn't connect forward, so next letter doesn't need leading ZWJ
        assert!(!needs_leading_zwj("ن", Some("ر")));

        // Space doesn't trigger ZWJ
        assert!(!needs_leading_zwj("ن", Some(" ")));

        // 'ا' (Alef) CAN receive connection from 'ب'
        assert!(needs_leading_zwj("ا", Some("ب")));
    }

    #[test]
    fn test_needs_trailing_zwj() {
        // 'ب' connects to 'ن', so 'ب' needs trailing ZWJ
        assert!(needs_trailing_zwj("ب", Some("ن")));

        // 'ر' doesn't connect forward, so no trailing ZWJ needed
        assert!(!needs_trailing_zwj("ر", Some("ن")));

        // Space doesn't trigger ZWJ
        assert!(!needs_trailing_zwj("ب", Some(" ")));

        // 'ب' connects to 'ا' (Alef), so 'ب' needs trailing ZWJ
        assert!(needs_trailing_zwj("ب", Some("ا")));

        // 'ا' (Alef) does NOT connect forward to 'ل'
        assert!(!needs_trailing_zwj("ا", Some("ل")));
    }

    #[test]
    fn test_apply_zwj_basic() {
        // Test basic connection: بن should have ZWJ between
        let result = apply_zwj_to_text("بن");
        assert!(result.contains(ZWJ));

        // Test non-connector: ار should NOT have ZWJ between (ر doesn't connect forward)
        let result = apply_zwj_to_text("ار");
        assert!(!result.contains(ZWJ));

        // Test: رن - ر doesn't connect to ن
        let result = apply_zwj_to_text("رن");
        assert!(!result.contains(ZWJ));
    }

    #[test]
    fn test_apply_zwj_with_diacritics() {
        // Test with diacritics - should maintain clusters
        let result = apply_zwj_to_text("بِنَ");
        assert!(result.contains(ZWJ));
        assert!(result.contains('ِ'));
        assert!(result.contains('َ'));
    }

    #[test]
    fn test_alef_connection() {
        // Test: بال
        // 'ب' should connect to 'ا' (Alef receives connection from right)
        // 'ا' should NOT connect to 'ل' (Alef doesn't extend to left)
        let result = apply_zwj_to_text("بال");

        println!("Result for 'بال': {:?}", result);
        println!("Chars: {:?}", result.chars().collect::<Vec<_>>());

        // Count ZWJ occurrences
        let zwj_count = result.chars().filter(|&c| c == ZWJ).count();

        // Trace through logic:
        // Clusters: ['ب', 'ا', 'ل']
        //
        // Processing 'ب' (index 0):
        //   - needs_leading_zwj('ب', None) = false (no previous)
        //   - needs_trailing_zwj('ب', Some('ا')) = true (ب connects, ا can receive)
        //   Result: ب + ZWJ
        //
        // Processing 'ا' (index 1):
        //   - needs_leading_zwj('ا', Some('ب')) = true (ا can receive, ب connects)
        //   - needs_trailing_zwj('ا', Some('ل')) = false (ا in RIGHT_NON_CONNECTORS)
        //   Result: ZWJ + ا
        //
        // Processing 'ل' (index 2):
        //   - needs_leading_zwj('ل', Some('ا')) = false (ا in RIGHT_NON_CONNECTORS)
        //   - needs_trailing_zwj('ل', None) = false (no next)
        //   Result: ل
        //
        // Final: ب + ZWJ + ZWJ + ا + ل
        // Total ZWJs: 2

        assert_eq!(zwj_count, 2, "Expected 2 ZWJs in 'بال'");

        // Verify structure
        let chars: Vec<char> = result.chars().collect();
        assert_eq!(chars[0], 'ب');
        assert_eq!(chars[1], ZWJ, "Expected ZWJ after ب");
        assert_eq!(chars[2], ZWJ, "Expected ZWJ before ا");
        assert_eq!(chars[3], 'ا');
        assert_eq!(chars[4], 'ل');
    }

    #[test]
    fn test_alef_no_forward_connection() {
        // Test: ال (Alef + Lam)
        // 'ا' should NOT connect forward to 'ل'
        let result = apply_zwj_to_text("ال");

        // Should have NO ZWJs because:
        // - No letter before 'ا', so no leading ZWJ
        // - 'ا' can't connect forward (in RIGHT_NON_CONNECTORS), so no trailing ZWJ
        // - 'ل' has 'ا' before it which is in RIGHT_NON_CONNECTORS, so no leading ZWJ
        let zwj_count = result.chars().filter(|&c| c == ZWJ).count();
        assert_eq!(zwj_count, 0);
    }

    #[test]
    fn test_complete_word() {
        // Test: بالقسط
        let result = apply_zwj_to_text("بالقسط");

        println!("Result for 'بالقسط': {:?}", result);

        // Expected connections:
        // ب → ا: 2 ZWJs (trailing ب + leading ا)
        // ا ↛ ل: 0 ZWJs (ا doesn't connect forward)
        // ل → ق: 2 ZWJs (trailing ل + leading ق)
        // ق → س: 2 ZWJs (trailing ق + leading س)
        // س → ط: 2 ZWJs (trailing س + leading ط)
        // Total: 2 + 0 + 2 + 2 + 2 = 8 ZWJs

        let zwj_count = result.chars().filter(|&c| c == ZWJ).count();
        assert_eq!(zwj_count, 8);
    }
}
