use super::graphemes::{Grapheme, match_bigraph};

/// Tokenize a string into graphemes, matching longest patterns first
///
/// Converts a string into a sequence of graphemes, recognizing special
/// bigraphs like "ph", "ch", "th", etc.
///
/// # Arguments
///
/// * `input` - The string to tokenize
///
/// # Returns
///
/// Returns a vector of `Grapheme` enum values.
pub fn tokenize(input: &str) -> Vec<Grapheme> {
    let chars: Vec<char> = input.chars().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        // Check bigraphs first (2 characters)
        if i + 2 <= chars.len() {
            let bi: String = chars[i..i + 2].iter().collect();
            if let Some(g) = match_bigraph(&bi) {
                result.push(g);
                i += 2;
                continue;
            }
        }

        // Fall back to single character
        result.push(Grapheme::from_char(chars[i]));
        i += 1;
    }

    result
}

/// Convert a `Vec<Grapheme>` back to a String
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
pub fn detokenize(graphemes: &[Grapheme]) -> String {
    graphemes.iter().map(|g| g.to_string()).collect()
}
