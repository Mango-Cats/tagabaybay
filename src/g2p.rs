use crate::nativization::error::PhonetizationError;
use once_cell::sync::Lazy;
use phonetisaurus_g2p::PhonetisaurusModel;

/// Binary of the model
static MODEL_FILE: &[u8] = include_bytes!("../.model/model.fst");

/// Model object
static MODEL: Lazy<PhonetisaurusModel> = Lazy::new(|| {
    // Remember to change this function if you want to change the G2P model
    PhonetisaurusModel::try_from(MODEL_FILE)
        .expect("Embedded Phonetisaurus model missing or invalid")
});

/// Phonetic transducer
///
/// # Arguments
///
/// * `word` - The word to transcribe phonetically
///
/// # Returns
///
/// Returns `Some(String)` if a phonetic transcription is possible,
/// the phonetic transcription is separated per syllable. The syllable
/// boundary is given by `$`.
/// Returns `None` otherwise.
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
