use once_cell::sync::Lazy;
use phonetisaurus_g2p::PhonetisaurusModel;

// FIXME(model): do we add the root here or would it be better
// to add it in the config.
static MODEL_FILE: &[u8] = include_bytes!("../.model/model.fst");

static MODEL: Lazy<PhonetisaurusModel> = Lazy::new(|| {
    // Remember to change this function if you want to change the G2P model
    PhonetisaurusModel::try_from(MODEL_FILE)
        .expect("Embedded Phonetisaurus model missing or invalid")
});

pub fn phonemize(word: &str) -> Option<String> {
    if let Ok(result) = MODEL.phonemize_word(word) {
        // POTENTIAL_ISSUE: word might have multiple syllables and might be syllabified
        // so the output could be a bit fucky wucky.
        return Some(result.phonemes);
    } else {
        None
    }
}
