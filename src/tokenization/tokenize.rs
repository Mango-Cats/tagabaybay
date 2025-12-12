use super::graphemes::{Grapheme, match_bigraph, match_trigraph};

/// Tokenize a string into graphemes, matching longest patterns first
pub fn tokenize(input: &str) -> Vec<Grapheme> {
    let chars: Vec<char> = input.chars().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        // Check trigraphs first (longest match)
        if i + 3 <= chars.len() {
            let tri: String = chars[i..i + 3].iter().collect();
            if let Some(g) = match_trigraph(&tri) {
                result.push(g);
                i += 3;
                continue;
            }
        }

        // Then bigraphs
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

/// Convert a Vec<Grapheme> back to a String
pub fn detokenize(graphemes: &[Grapheme]) -> String {
    graphemes.iter().map(|g| g.to_string()).collect()
}
