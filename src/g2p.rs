//! Deprecated G2P module using Phonetisaurus FST model.
//!
//! This module is kept for reference but is no longer functional.
//! Use `g2py` instead, which uses Python's phonemizer with espeak-ng backend.
//!
//! To restore this module, add these dependencies to Cargo.toml:
//! ```toml
//! phonetisaurus-g2p = "0.1.1"
//! once_cell = "1.21.3"
//! ```

use crate::{
    configs::AdapterConfig,
    error::{ErrorTypes, PhonetizationError},
};
use once_cell::sync::Lazy;
use phonetisaurus_g2p::PhonetisaurusModel;

/// Binary of the model
static MODEL_FILE: &[u8] = include_bytes!("../.model/model.fst");

/// Model object
static MODEL: Lazy<PhonetisaurusModel> = Lazy::new(|| {
    PhonetisaurusModel::try_from(MODEL_FILE)
        .expect("Embedded Phonetisaurus model missing or invalid")
});

/// Phonetic transducer (DEPRECATED - use g2py::phonemize instead)
fn phonemize_internal(word: &str) -> Result<String, PhonetizationError> {
    let result = MODEL
        .phonemize_word(&word.to_lowercase())
        .map_err(|_| PhonetizationError::new(word.to_string(), None, None))?;

    Ok(result
        .phonemes
        .chars()
        .map(|c| if c.is_alphabetic() { c } else { '$' })
        .collect())
}

pub fn phonemize_to_arpa(
    phrase: &str,
    word_number: Option<usize>,
    dataset_name: Option<&str>,
    config: &AdapterConfig,
) -> Result<String, ErrorTypes> {
    let words: Vec<&str> = phrase.split_whitespace().collect();

    if words.is_empty() {
        return Ok(String::new());
    }

    let mut phonetic_parts: Vec<String> = Vec::new();

    for word_part in words {
        let subparts: Vec<&str> = word_part.split('-').collect();
        let mut subpart_phonetics: Vec<String> = Vec::new();

        for subpart in &subparts {
            if subpart
                .chars()
                .all(|c| c.is_ascii_digit() || c == '.' || c == '/' || c == '+')
            {
                subpart_phonetics.push(String::new()); // Empty phonemes for numbers
                continue;
            }

            if subpart.is_empty() {
                subpart_phonetics.push(String::new());
                continue;
            }

            let clean_subpart: String = subpart
                .chars()
                .take_while(|c| c.is_ascii_alphabetic())
                .collect();

            if clean_subpart.is_empty() {
                subpart_phonetics.push(String::new());
                continue;
            }

            let phonetic_str = phonemize_internal(&clean_subpart).map_err(|mut err| {
                err.word_number = word_number;
                err.dataset_name = dataset_name.map(str::to_string);

                err.print_error();

                if config.panic_at_error {
                    panic!("Phonetization failed: {:?}", err);
                }

                ErrorTypes::Phonetization(err)
            })?;

            subpart_phonetics.push(phonetic_str);
        }

        phonetic_parts.push(subpart_phonetics.join("$"));
    }

    Ok(phonetic_parts.join("$"))
}
