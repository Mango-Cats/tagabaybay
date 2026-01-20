use crate::grapheme::types::GraphemesSet;

use super::filipino;
use super::source;

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
pub fn source_tokenizer(input: &str) -> Vec<source::SourceGrapheme> {
    let chars: Vec<char> = input.chars().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        // Check digraphs first (2 characters)
        if i + 2 <= chars.len() {
            let substring_2: String = chars[i..i + 2].iter().collect();
            if let Some(g) = source::match_digraph(&substring_2) {
                result.push(g);
                i += 2;
                continue;
            }
        }

        // Fall back to single character
        result.push(source::SourceGrapheme::from_char(chars[i]));
        i += 1;
    }

    result
}

/// Tokenize a Filipino orthographic string into graphemes
///
/// Parses a Filipino string into a sequence of graphemes,
/// recognizing digraphs like "ng", "ny", "ts", "dy", "sh", "sy".
///
/// # Arguments
///
/// * `input` - The Filipino string to tokenize
///
/// # Returns
///
/// Returns a vector of `FilipinoGrapheme` values.
pub fn filipino_tokenizer(input: &str) -> Vec<filipino::FilipinoGrapheme> {
    let input = input.to_lowercase();
    let chars: Vec<char> = input.chars().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        // Preserve hyphens as syllable boundary markers
        if chars[i] == '-' {
            result.push(filipino::FilipinoGrapheme::Hyphen);
            i += 1;
            continue;
        }

        // Check digraphs first (2 characters)
        if i + 2 <= chars.len() {
            if let Some(g) = filipino::match_digraph(chars[i], chars[i + 1]) {
                result.push(g);
                i += 2;
                continue;
            }
        }

        // Fall back to single character
        result.push(filipino::FilipinoGrapheme::from_char(chars[i]));
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
#[warn(unused)]
fn detokenizer(graphemes: &[GraphemesSet]) -> String {
    match graphemes.split_first() {
        Some((first, rest)) => {
            let mut current = match first {
                GraphemesSet::Src(s) => s.to_string_rep(),
                GraphemesSet::Fil(f) => f.to_string_rep(),
            };

            current.push_str(&detokenizer(rest));
            current
        }
        None => String::new(),
    }
}
