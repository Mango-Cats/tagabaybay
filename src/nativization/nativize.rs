use crate::tokenization::phoneme::Phoneme;
use crate::tokenization::tokenize::tokenize;
use crate::nativization::replacement::{sensitive_replacement, free_replacement, letter_to_phonetic, postprocess};
use crate::nativization::matchers::{is_abbreviation, is_single_letter};
use crate::nativization::error::printe;

/// Nativize an entire word or phrase: String -> Vec<Grapheme> -> Vec<Phoneme> -> String
/// Handles multi-word inputs and abbreviations
pub fn nativize_word(input: &str) -> Vec<Phoneme> {
    // Split by spaces to handle multi-word inputs
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut result = Vec::new();

    for (idx, word) in words.iter().enumerate() {
        if idx > 0 {
            result.push(Phoneme::Space);
        }

        if is_abbreviation(word) {
            // Transcribe abbreviation phonetically
            result.extend(nativize_abbreviation(word));
        } else {
            // Regular nativization
            let phonemes = nativize(word, None, None);
            result.extend(phonemes);
        }
    }

    result
}

pub fn nativize_word_set(word_list: &[&str], dataset_name: &str) -> Vec<Vec<Phoneme>> {
    let mut res: Vec<Vec<Phoneme>> = Vec::new();

    for (i, word) in word_list.iter().enumerate() {
        res.push(nativize(&word, Some(i), Some(dataset_name)));
    }

    res
}

fn nativize(word: &str, word_number: Option<usize>, dataset_name: Option<&str>) -> Vec<Phoneme> {
    let mut res: Vec<Phoneme> = Vec::new();

    // Check if word contains hyphen - handle each part separately
    if word.contains('-') {
        let parts: Vec<&str> = word.split('-').collect();
        for (idx, part) in parts.iter().enumerate() {
            // Add hyphen between parts
            if idx > 0 {
                res.push(Phoneme::Passthrough('-'));
            }

            // Check if this part is an abbreviation or single letter
            if is_abbreviation(part) || is_single_letter(part) {
                res.extend(nativize_abbreviation(part));
            } else {
                // Regular nativization for this part
                let graphemes = tokenize(part);
                for (i, _) in graphemes.iter().enumerate() {
                    if let Some(sens_res) = sensitive_replacement(&graphemes, i) {
                        res.extend(sens_res);
                    } else if let Some(free_res) = free_replacement(&graphemes, i) {
                        res.extend(free_res);
                    } else {
                        printe(&graphemes, i, word_number, dataset_name);
                    }
                }
            }
        }
        return postprocess(&mut res);
    }

    // Check if it's an abbreviation before lowercasing
    if is_abbreviation(word) {
            res.extend(nativize_abbreviation(word));
    } else {
        // Regular nativization
        let graphemes = tokenize(word);

        for (i, _) in graphemes.iter().enumerate() {
            // Try context-sensitive replacement first
            if let Some(sens_res) = sensitive_replacement(&graphemes, i) {
                res.extend(sens_res);
            } else {
                // Fall back to context-free replacement
                if let Some(free_res) = free_replacement(&graphemes, i) {
                    res.extend(free_res);
                } else {
                    printe(&graphemes, i, word_number, dataset_name);
                }
            }
        }
    }

    // Apply post-processing patterns
    postprocess(&mut res)
}

/// Convert an abbreviation to Filipino phonetic transcription
/// E.g., "XR" -> "eks ar", "IV" -> "ay vi"
fn nativize_abbreviation(abbr: &str) -> Vec<Phoneme> {
    abbr.chars()
        .filter_map(letter_to_phonetic)
        .flatten()
        .collect()
}