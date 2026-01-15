use crate::arpabet::symbols::{ArpabetSymbols, match_arpabet};

/// Tokenize a string into arpabet symbols, matching longest patterns first
///
/// # Arguments
///
/// * `input` - The string to tokenize
///
/// # Returns
///
/// Returns a vector of `ArpabetSymbols` enum values.
pub fn tokenize(input: &str) -> Vec<ArpabetSymbols> {
    let chars: Vec<char> = input.chars().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '$' {
            i += 1;
            continue;
        }
        // Check digraphs first (2 characters)
        if i + 2 <= chars.len() {
            let substring_2: String = chars[i..i + 2].iter().collect();
            if let Some(replacement) = match_arpabet(&substring_2) {
                result.push(replacement);
                i += 2;
                continue;
            }
        }

        // Fall back to single character
        let single_char: &str = &chars[i].to_string();
        if let Some(single_repl) = match_arpabet(single_char) {
            result.push(single_repl);
            i += 1;
        } else {
            dbg!(&single_char);
            panic!("panic at arpabet/tokenize.rs @ tokenize()")
        }
    }

    result
}

/// Convert a `Vec<SourceGrapheme>` back to a String
///
/// Reconstructs the original string from a sequence of graphemes.
///
/// # Arguments
///
/// * `graphemes` - Slice of graphemes to convert
///
/// # Returns
///
/// Returns the reconstructed string.
pub fn detokenize(graphemes: &[ArpabetSymbols]) -> String {
    graphemes.iter().map(|g| g.to_string()).collect()
}
