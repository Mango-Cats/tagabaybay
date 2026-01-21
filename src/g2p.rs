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

use crate::error::PhonetizationError;
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
pub fn phonemize(word: &str) -> Result<String, PhonetizationError> {
    let result = MODEL
        .phonemize_word(&word.to_lowercase())
        .map_err(|_| PhonetizationError::new(word.to_string(), None, None))?;

    Ok(result
        .phonemes
        .chars()
        .map(|c| if c.is_alphabetic() { c } else { '$' })
        .collect())
}
