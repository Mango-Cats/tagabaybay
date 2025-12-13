use crate::nativization::error::printe;
use crate::nativization::replacement::{
    free_replacement, letter_to_phonetic, sensitive_replacement,
};
use crate::tokenization::graphemes::Grapheme;
use crate::tokenization::phoneme::Phoneme;
use crate::tokenization::tokenize::tokenize;

/// Nativize an entire word or phrase
pub fn nativize_word(input: &str) -> Vec<Phoneme> {
    nativize(input, None, None)
}

/// Nativize a list of words or phrase
pub fn nativize_word_set(word_list: &[&str], dataset_name: &str) -> Vec<Vec<Phoneme>> {
    let mut res: Vec<Vec<Phoneme>> = Vec::new();

    for (i, word) in word_list.iter().enumerate() {
        res.push(nativize(&word, Some(i), Some(dataset_name)));
    }

    res
}

/// The Nativization Algorithm
fn nativize(word: &str, word_number: Option<usize>, dataset_name: Option<&str>) -> Vec<Phoneme> {
    let mut res: Vec<Phoneme> = Vec::new();
    let graphemes = tokenize(&word);

    let mut i = 0;
    while i < graphemes.len() {
        let curr = &graphemes[i];

        // Handle abbreviations and single letters (spelled out phonetically)
        if curr.is_uppercase() {
            let prev = if i > 0 { Some(&graphemes[i - 1]) } else { None };
            let after_separator = prev.is_none()
                || matches!(
                    prev,
                    Some(Grapheme::Space) | Some(Grapheme::Passthrough('-'))
                );

            if after_separator {
                let start = i;
                while i < graphemes.len() && graphemes[i].is_uppercase() {
                    i += 1;
                }
                let end = i;
                let next = graphemes.get(i);
                let before_separator = next.is_none()
                    || matches!(
                        next,
                        Some(Grapheme::Space) | Some(Grapheme::Passthrough('-'))
                    );

                if (end - start >= 2) || before_separator {
                    let abbr_segment = &graphemes[start..end];
                    res.extend(nativize_abbreviation(abbr_segment));
                    continue;
                } else {
                    i = start;
                }
            }
        }

        // Try context-sensitive replacement first
        if let Some((sens_res, consumed)) = sensitive_replacement(&graphemes, i) {
            res.extend(sens_res);
            i += consumed;
        } else {
            // Fall back to context-free replacement
            if let Some((free_res, consumed)) = free_replacement(&graphemes, i) {
                res.extend(free_res);
                i += consumed;
            } else {
                printe(&graphemes, i, word_number, dataset_name);
                i += 1;
            }
        }
    }

    // Apply post-processing patterns
    res
}

/// Convert an abbreviation to Filipino phonetic transcription
/// E.g., "XR" -> "eks ar", "IV" -> "ay bi"
fn nativize_abbreviation(abbr: &[Grapheme]) -> Vec<Phoneme> {
    let mut result: Vec<Phoneme> = Vec::new();
    for (i, grapheme) in abbr.iter().enumerate() {
        if let Some(phonemes) = letter_to_phonetic(*grapheme) {
            // Add space before each letter except the first
            if i > 0 {
                result.push(Phoneme::Space);
            }
            result.extend(phonemes);
        }
    }
    result
}