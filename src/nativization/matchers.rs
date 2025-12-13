use crate::tokenization::graphemes::Grapheme;

/// Check if a word is an abbreviation (all uppercase letters)
pub fn is_abbreviation(graphemes: &[Grapheme]) -> bool {
    !graphemes.is_empty() && graphemes.iter().all(|g| g.is_uppercase())
}

/// Check if a single character is an uppercase letter (for prefix handling like "L-", "C")
pub fn is_single_letter(graphemes: &[Grapheme], i: usize) -> bool {
    if graphemes.len() == 1 {
        true
    } else {
        let next = graphemes.get(i + 1);
        let prev = if i > 0 { graphemes.get(i - 1) } else { None };

        let next_is_separator = next
            .map(|n| *n == Grapheme::Space || *n == Grapheme::Passthrough('-'))
            .unwrap_or(true);

        let prev_is_separator = prev
            .map(|p| *p == Grapheme::Space || *p == Grapheme::Passthrough('-'))
            .unwrap_or(true);

        next_is_separator && prev_is_separator
    }
}
