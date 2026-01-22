//! Common utilities for G2P (Grapheme-to-Phoneme) modules.
//!
//! This module provides shared functionality used by both the Rust-native
//! Phonetisaurus backend ([`g2p`]) and the Python espeak backend ([`g2py`]).
//!
//! # Word Separator
//!
//! The `$` character is used as a phoneme boundary marker between words
//! and hyphenated subparts, enabling downstream syllabification to
//! properly handle word boundaries.

use crate::configs::AdapterConfig;
use crate::error::{ErrorTypes, PhonetizationError};

/// Phonemize a phrase by processing each word and subpart.
///
/// This function handles the common logic for both G2P backends:
/// - Splits input on whitespace into words
/// - Handles hyphenated compounds by processing each subpart
/// - Passes through numbers and special tokens unchanged
/// - Strips non-alphabetic trailing characters from words
///
/// # Arguments
///
/// * `phrase` - The input phrase to phonemize (may contain multiple words)
/// * `word_number` - Optional line/word number for error reporting
/// * `dataset_name` - Optional dataset name for error context
/// * `config` - Adapter configuration controlling error behavior
/// * `phonemize_fn` - The backend-specific phonemization function
///
/// # Returns
///
/// A string of phonemes with `$` separating word/subpart boundaries.
///
/// # Errors
///
/// Returns [`ErrorTypes::Phonetization`] if any word fails to phonemize
/// and `config.panic_at_error` is false.
pub fn phonemize_phrase<F>(
    phrase: &str,
    word_number: Option<usize>,
    dataset_name: Option<&str>,
    config: &AdapterConfig,
    phonemize_fn: F,
) -> Result<String, ErrorTypes>
where
    F: Fn(&str) -> Result<String, PhonetizationError>,
{
    let words: Vec<&str> = phrase.split_whitespace().collect();

    if words.is_empty() {
        return Ok(String::new());
    }

    let mut phonetic_parts: Vec<String> = Vec::new();

    for word_part in words {
        let subpart_result =
            process_word_part(word_part, word_number, dataset_name, config, &phonemize_fn)?;
        phonetic_parts.push(subpart_result);
    }

    Ok(phonetic_parts.join("$"))
}

/// Process a single word part (which may contain hyphens).
///
/// Splits on hyphens and processes each subpart individually.
fn process_word_part<F>(
    word_part: &str,
    word_number: Option<usize>,
    dataset_name: Option<&str>,
    config: &AdapterConfig,
    phonemize_fn: &F,
) -> Result<String, ErrorTypes>
where
    F: Fn(&str) -> Result<String, PhonetizationError>,
{
    let subparts: Vec<&str> = word_part.split('-').collect();
    let mut subpart_phonetics: Vec<String> = Vec::new();

    for subpart in &subparts {
        let phonetic = process_subpart(subpart, word_number, dataset_name, config, phonemize_fn)?;
        subpart_phonetics.push(phonetic);
    }

    Ok(subpart_phonetics.join("$"))
}

/// Process a single subpart (no hyphens).
///
/// Handles special cases:
/// - Numbers and special tokens (`0-9`, `.`, `/`, `+`) → empty string
/// - Empty strings → empty string
/// - Strips trailing non-alphabetic characters
fn process_subpart<F>(
    subpart: &str,
    word_number: Option<usize>,
    dataset_name: Option<&str>,
    config: &AdapterConfig,
    phonemize_fn: &F,
) -> Result<String, ErrorTypes>
where
    F: Fn(&str) -> Result<String, PhonetizationError>,
{
    if subpart
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == '/' || c == '+')
    {
        return Ok(String::new());
    }

    if subpart.is_empty() {
        return Ok(String::new());
    }

    let clean_subpart: String = subpart
        .chars()
        .take_while(|c| c.is_ascii_alphabetic())
        .collect();

    if clean_subpart.is_empty() {
        return Ok(String::new());
    }

    phonemize_fn(&clean_subpart).map_err(|mut err| {
        err.word_number = word_number;
        err.dataset_name = dataset_name.map(str::to_string);
        err.print_error();

        if config.panic_at_error {
            panic!("Phonetization failed: {:?}", err);
        }

        ErrorTypes::Phonetization(err)
    })
}
