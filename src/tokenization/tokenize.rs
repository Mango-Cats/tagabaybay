use super::src_graphemes::{SourceGrapheme, match_digraph};

/// Tokenize a string into graphemes, matching longest patterns first
///
/// Converts a string into a sequence of graphemes, recognizing special
/// digraphs like "ph", "ch", "th", etc.
///
/// # Arguments
///
/// * `input` - The string to tokenize
///
/// # Returns
///
/// Returns a vector of `SourceGrapheme` enum values.
pub fn tokenize(input: &str) -> Vec<SourceGrapheme> {
    let chars: Vec<char> = input.chars().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        // Check digraphs first (2 characters)
        if i + 2 <= chars.len() {
            let substring_2: String = chars[i..i + 2].iter().collect();
            if let Some(g) = match_digraph(&substring_2) {
                result.push(g);
                i += 2;
                continue;
            }
        }

        // Fall back to single character
        result.push(SourceGrapheme::from_char(chars[i]));
        i += 1;
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
pub fn detokenize(graphemes: &[SourceGrapheme]) -> String {
    graphemes.iter().map(|g| g.to_string()).collect()
}
