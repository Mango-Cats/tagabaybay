use crate::phoneme::tokens::{
    arpabet::{ArpabetSymbols, match_arpabet},
    ipa::IPASymbol,
    map::ARPA_TO_IPA,
};

/// Convert ARPABET symbols to IPA symbols.
///
/// Used to convert output from Phonetisaurus (ARPA) to the IPA-first system.
///
/// # Arguments
///
/// * `arpa` - Slice of ArpabetSymbols
///
/// # Returns
///
/// Vector of corresponding IPASymbol values.
pub fn arpa_to_ipa(arpa: &[ArpabetSymbols]) -> Vec<IPASymbol> {
    arpa.iter()
        .filter_map(|a| ARPA_TO_IPA.get(a).cloned())
        .collect()
}

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

/// Tokenize ARPA string directly to IPA symbols.
///
/// Convenience function that combines tokenize_arpa + arpa_to_ipa.
pub fn tokenize_arpa_to_ipa(input: &str) -> Vec<IPASymbol> {
    let arpa = tokenize_arpa(input);
    arpa_to_ipa(&arpa)
}

/// Convert a `Vec<ArpabetSymbols>` back to a String (legacy)
pub fn detokenizer_arpa(graphemes: &[ArpabetSymbols]) -> String {
    graphemes.iter().map(|g| g.to_string()).collect()
}
