use crate::phoneme::{
    map::IPA_TO_ARPA,
    symbols::{ArpabetSymbols, match_arpabet},
};

/// Tokenize a string of arpabet symbols to a vector of arpabet symbols.
/// Keyword: from string to a vector. Similar function: `tokenize_ipa`.
///
/// # Arguments
///
/// * `input` - Arpabet string (kwehschahn)
///
/// # Returns
///
/// Returns a vector of `ArpabetSymbols` enum values.
pub fn tokenize_arpa(input: &str) -> Vec<ArpabetSymbols> {
    let chars: Vec<char> = input.chars().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '$' {
            i += 1;
            continue;
        }
        if i + 2 <= chars.len() {
            let substring_2: String = chars[i..i + 2].iter().collect();
            if let Some(replacement) = match_arpabet(&substring_2) {
                result.push(replacement);
                i += 2;
                continue;
            }
        }

        let single_char: &str = &chars[i].to_string();
        if let Some(single_repl) = match_arpabet(single_char) {
            result.push(single_repl);
            i += 1;
        } else {
            i += 1;
        }
    }

    result
}

/// Tokenize string of IPA symbols to a vector of arpabet symbols.
/// Similar function: `tokenize_arpa`.
///
/// # Arguments
/// * `ipa` - IPA string (e.g., "ækʃən")
///
/// # Returns
/// Vector of ArpabetSymbols
pub fn tokenize_ipa(ipa: &str) -> Vec<ArpabetSymbols> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = ipa.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if i + 1 < chars.len() {
            let two_char: String = chars[i..=i + 1].iter().collect();
            if let Some(arpa) = IPA_TO_ARPA.get(two_char.as_str()) {
                tokens.push(arpa.clone());
                i += 2;
                continue;
            }
        }

        let one_char: String = chars[i..=i].iter().collect();
        if let Some(arpa) = IPA_TO_ARPA.get(one_char.as_str()) {
            tokens.push(arpa.clone());
        }

        i += 1;
    }

    tokens
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
pub fn detokenizer(graphemes: &[ArpabetSymbols]) -> String {
    graphemes.iter().map(|g| g.to_string()).collect()
}
