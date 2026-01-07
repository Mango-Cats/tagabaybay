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
/// Returns `Some(String)` if a phonetic transcription is possible.
/// Returns `None` otherwise.
pub fn phonemize(word: &str) -> Option<String> {
    if let Ok(result) = MODEL.phonemize_word(word.to_lowercase().as_str()) {
        // POTENTIAL_ISSUE: word might have multiple syllables and might be syllabified
        // so the output could be a bit fucky wucky.
        return Some(result.phonemes);
    } else {
        None
    }
}
