//! Letter-to-phonetic spelling for abbreviations
//!
//! Converts individual letters to their Filipino phonetic alphabet names,
//! used for spelling out abbreviations and single letters.

use crate::grapheme::filipino::FilipinoGrapheme;
use crate::grapheme::source::SourceGrapheme;

/// Convert a single letter to its Filipino phonetic alphabet name
///
/// Used for spelling out abbreviations and single letters.
///
/// # Arguments
///
/// * `letter` - The grapheme to spell out
///
/// # Returns
///
/// Returns `Some(Vec<FilipinoGrapheme>)` with the phonetic spelling, or `None` if
/// the grapheme is not a letter.
///
/// # Examples
///
/// - A → "ey"
/// - B → "bi"
/// - C → "si"
/// - W → "dabolyu"
pub fn letter_to_phonetic(letter: SourceGrapheme) -> Option<Vec<FilipinoGrapheme>> {
    let l = letter.to_lowercase();
    match l {
        SourceGrapheme::A => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::Y]),
        SourceGrapheme::B => Some(vec![FilipinoGrapheme::B, FilipinoGrapheme::I]),
        SourceGrapheme::C => Some(vec![FilipinoGrapheme::S, FilipinoGrapheme::I]),
        SourceGrapheme::D => Some(vec![FilipinoGrapheme::D, FilipinoGrapheme::I]),
        SourceGrapheme::E => Some(vec![FilipinoGrapheme::I]),
        SourceGrapheme::F => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::F]),
        SourceGrapheme::G => Some(vec![
            FilipinoGrapheme::D,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::I,
        ]),
        SourceGrapheme::H => Some(vec![
            FilipinoGrapheme::E,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::TS,
        ]),
        SourceGrapheme::I => Some(vec![FilipinoGrapheme::A, FilipinoGrapheme::Y]),
        SourceGrapheme::J => Some(vec![
            FilipinoGrapheme::J,
            FilipinoGrapheme::E,
            FilipinoGrapheme::Y,
        ]),
        SourceGrapheme::K => Some(vec![
            FilipinoGrapheme::K,
            FilipinoGrapheme::E,
            FilipinoGrapheme::Y,
        ]),
        SourceGrapheme::L => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::L]),
        SourceGrapheme::M => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::M]),
        SourceGrapheme::N => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::N]),
        SourceGrapheme::O => Some(vec![FilipinoGrapheme::O]),
        SourceGrapheme::P => Some(vec![FilipinoGrapheme::P, FilipinoGrapheme::I]),
        SourceGrapheme::Q => Some(vec![
            FilipinoGrapheme::K,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::U,
        ]),
        SourceGrapheme::R => Some(vec![FilipinoGrapheme::A, FilipinoGrapheme::R]),
        SourceGrapheme::S => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::S]),
        SourceGrapheme::T => Some(vec![FilipinoGrapheme::T, FilipinoGrapheme::I]),
        SourceGrapheme::U => Some(vec![FilipinoGrapheme::Y, FilipinoGrapheme::U]),
        SourceGrapheme::V => Some(vec![FilipinoGrapheme::V, FilipinoGrapheme::I]),
        SourceGrapheme::W => Some(vec![
            FilipinoGrapheme::D,
            FilipinoGrapheme::O,
            FilipinoGrapheme::B,
            FilipinoGrapheme::O,
            FilipinoGrapheme::L,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::U,
        ]),
        SourceGrapheme::X => Some(vec![
            FilipinoGrapheme::E,
            FilipinoGrapheme::K,
            FilipinoGrapheme::S,
        ]),
        SourceGrapheme::Y => Some(vec![
            FilipinoGrapheme::W,
            FilipinoGrapheme::A,
            FilipinoGrapheme::Y,
        ]),
        SourceGrapheme::Z => Some(vec![FilipinoGrapheme::Z, FilipinoGrapheme::I]),
        _ => None,
    }
}
