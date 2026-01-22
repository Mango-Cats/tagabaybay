//! Rust-native G2P module using Phonetisaurus FST model.
//!
//! This module provides ARPA phoneme transcription using an embedded FST model,
//! offering a pure-Rust alternative to the Python-based [`g2py`] module.
//!
//! # Usage
//!
//! ```ignore
//! use tagabaybay::g2p::phonemize_to_arpa;
//! use tagabaybay::configs::AdapterConfig;
//!
//! let config = AdapterConfig::default();
//! let phonemes = phonemize_to_arpa("action", None, None, &config)?;
//! // Returns ARPA phonemes like "AE$K$SH$AH$N"
//! ```
//!
//! # Depracated
//!
//! This is depracated due to poor performance. Instead see [`g2py`]
//! for IPA phonemization for better accuracy.
//!
//! # See Also
//!
//! - [`g2py`]: Python-based IPA phonemization (recommended for accuracy)
//! - [`g2p_common`]: Shared phrase processing utilities

use crate::configs::AdapterConfig;
use crate::error::{ErrorTypes, PhonetizationError};
use crate::g2p::common::phonemize_phrase;
use once_cell::sync::Lazy;
use phonetisaurus_g2p::PhonetisaurusModel;

/// Embedded FST model binary.
///
/// The model file is included at compile time, ensuring the G2P
/// functionality works without external file dependencies.
static MODEL_FILE: &[u8] = include_bytes!("../../.model/model.fst");

/// Lazily-initialized Phonetisaurus model.
///
/// The model is loaded once on first use and cached for subsequent calls.
/// Panics at runtime if the embedded model is corrupted or invalid.
static MODEL: Lazy<PhonetisaurusModel> = Lazy::new(|| {
    PhonetisaurusModel::try_from(MODEL_FILE)
        .expect("Embedded Phonetisaurus model missing or invalid")
});

/// Convert a single word to ARPA phonemes using the FST model.
///
/// # Arguments
///
/// * `word` - A single word (no spaces or hyphens) to phonemize
///
/// # Returns
///
/// ARPA phoneme string with `$` as phoneme separator.
///
/// # Example
///
/// ```ignore
/// let phonemes = phonemize_internal("cat")?;
/// // Returns something like "K$AE$T"
/// ```
fn phonemize_internal(word: &str) -> Result<String, PhonetizationError> {
    let result = MODEL
        .phonemize_word(&word.to_lowercase())
        .map_err(|_| PhonetizationError::new(word.to_string(), None, None))?;

    // Convert non-alphabetic characters to phoneme boundary markers
    Ok(result
        .phonemes
        .chars()
        .map(|c| if c.is_alphabetic() { c } else { '$' })
        .collect())
}

/// Phonemize a phrase to ARPA phonemes.
///
/// Processes multi-word phrases and hyphenated compounds, returning
/// ARPA phonemes with `$` separating word boundaries.
///
/// # Arguments
///
/// * `phrase` - Input text (may contain multiple words separated by spaces)
/// * `word_number` - Optional line number for error reporting
/// * `dataset_name` - Optional dataset identifier for error context
/// * `config` - Adapter configuration controlling error behavior
///
/// # Returns
///
/// * `Ok(String)` - ARPA phonemes with `$` word separators
/// * `Err(ErrorTypes)` - If phonemization fails
///
/// # Example
///
/// ```ignore
/// let config = AdapterConfig::default();
///
/// // Single word
/// let result = phonemize_to_arpa("hello", None, None, &config)?;
///
/// // Multi-word phrase
/// let result = phonemize_to_arpa("hello world", None, None, &config)?;
///
/// // Hyphenated compound
/// let result = phonemize_to_arpa("self-care", None, None, &config)?;
/// ```
///
/// # Error Handling
///
/// If `config.panic_at_error` is `true`, panics on phonemization failure.
/// Otherwise, returns an `ErrorTypes::Phonetization` error.
pub fn phonemize_to_arpa(
    phrase: &str,
    word_number: Option<usize>,
    dataset_name: Option<&str>,
    config: &AdapterConfig,
) -> Result<String, ErrorTypes> {
    phonemize_phrase(
        phrase,
        word_number,
        dataset_name,
        config,
        phonemize_internal,
    )
}
