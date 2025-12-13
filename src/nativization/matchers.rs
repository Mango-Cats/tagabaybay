/// Check if a word is an abbreviation (all uppercase letters)
pub fn is_abbreviation(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| c.is_ascii_uppercase())
}

/// Check if a single character is an uppercase letter (for prefix handling like "L-", "C")
pub fn is_single_letter(s: &str) -> bool {
    s.len() == 1
        && s.chars()
            .next()
            .map(|c| c.is_ascii_uppercase())
            .unwrap_or(false)
}
