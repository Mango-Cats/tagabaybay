use crate::phoneme::tokens::{ipa::IPASymbol, map::IPA_STR_TO_SYMBOL};

/// Tokenize an IPA string into a vector of IPASymbol.
///
/// This is the primary tokenization function for the IPA-first approach.
/// Handles multi-character IPA symbols (diphthongs, affricates) correctly.
///
/// # Arguments
///
/// * `ipa` - IPA string (e.g., "həloʊ" for "hello")
///
/// # Returns
///
/// Vector of `IPASymbol` enum values.
///
/// # Example
///
/// ```ignore
/// let symbols = tokenize_ipa_to_symbols("həloʊ");
/// // Returns [VoicelessGlottalFricative, Schwa, AlveolarLateral, DiphthongOU]
/// ```
pub fn tokenize_ipa(ipa: &str) -> Vec<IPASymbol> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = ipa.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Try 2-character sequences first (diphthongs, affricates)
        if i + 1 < chars.len() {
            let two_char: String = chars[i..=i + 1].iter().collect();
            if let Some(symbol) = IPA_STR_TO_SYMBOL.get(two_char.as_str()) {
                tokens.push(symbol.clone());
                i += 2;
                continue;
            }
        }

        // Try single character
        let one_char: String = chars[i..=i].iter().collect();
        if let Some(symbol) = IPA_STR_TO_SYMBOL.get(one_char.as_str()) {
            tokens.push(symbol.clone());
        }
        // Skip unknown characters (stress marks, etc.)

        i += 1;
    }

    tokens
}

/// Convert a vector of IPASymbol back to an IPA string.
///
/// # Arguments
///
/// * `symbols` - Slice of IPASymbol to convert
///
/// # Returns
///
/// IPA string representation.
pub fn detokenize_ipa(symbols: &[IPASymbol]) -> String {
    symbols.iter().map(|s| s.as_str()).collect()
}
