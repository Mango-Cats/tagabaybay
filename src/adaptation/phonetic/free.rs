//! Phonetic vowel rules
//!
//! Handles vowel adaptation based on ARPAbet/IPA phonemes rather than
//! orthographic patterns. This is useful when spelling doesn't reflect
//! actual pronunciation.
//!
//! For example:
//! - "make" is spelled with 'a' but pronounced /eɪ/ (AY in ARPAbet)
//! - "women" is spelled with 'o' but pronounced /ɪ/ (IH in ARPAbet)

use crate::adaptation::cursor::Cursor;
use crate::grapheme::filipino::FilipinoGrapheme;


/// 
pub fn phonetic_replacements(ctx: &Cursor) ->Option<(Vec<FilipinoGrapheme>, usize)> {
    None
}

/// Handles vowel replacement based on phonetic transcription
///
/// Looks at the ARPAbet phoneme corresponding to the current grapheme position
/// and returns the appropriate Filipino grapheme(s).
///
/// # Arguments
///
/// * `ctx` - Cursor containing both graphemes and phonemes
///
/// # Returns
///
/// Returns `Some((Vec<FilipinoGrapheme>, consumed))` if a phonetic vowel rule applies,
/// `None` otherwise.
fn handle_phonetic(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // Get the phoneme at the aligned position
    let _phoneme = ctx.current_phoneme()?;
    None
}
