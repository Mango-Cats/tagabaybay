//! Letter-to-phonetic spelling for abbreviations
//!
//! Converts individual letters to their Filipino phonetic alphabet names,
//! used for spelling out abbreviations and single letters.

use crate::grapheme::filipino::FilipinoGrapheme;
use crate::grapheme::source::SourceGrapheme;
use crate::tokens;

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
        SourceGrapheme::A => Some(tokens![FilipinoGrapheme::E, FilipinoGrapheme::Y]),
        SourceGrapheme::B => Some(tokens![FilipinoGrapheme::B, FilipinoGrapheme::I]),
        SourceGrapheme::C => Some(tokens![FilipinoGrapheme::S, FilipinoGrapheme::I]),
        SourceGrapheme::D => Some(tokens![FilipinoGrapheme::D, FilipinoGrapheme::I]),
        SourceGrapheme::E => Some(tokens![FilipinoGrapheme::I]),
        SourceGrapheme::F => Some(tokens![FilipinoGrapheme::E, FilipinoGrapheme::F]),
        SourceGrapheme::G => Some(tokens![
            FilipinoGrapheme::D,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::I,
        ]),
        SourceGrapheme::H => Some(tokens![
            FilipinoGrapheme::E,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::TS,
        ]),
        SourceGrapheme::I => Some(tokens![FilipinoGrapheme::A, FilipinoGrapheme::Y]),
        SourceGrapheme::J => Some(tokens![
            FilipinoGrapheme::J,
            FilipinoGrapheme::E,
            FilipinoGrapheme::Y,
        ]),
        SourceGrapheme::K => Some(tokens![
            FilipinoGrapheme::K,
            FilipinoGrapheme::E,
            FilipinoGrapheme::Y,
        ]),
        SourceGrapheme::L => Some(tokens![FilipinoGrapheme::E, FilipinoGrapheme::L]),
        SourceGrapheme::M => Some(tokens![FilipinoGrapheme::E, FilipinoGrapheme::M]),
        SourceGrapheme::N => Some(tokens![FilipinoGrapheme::E, FilipinoGrapheme::N]),
        SourceGrapheme::O => Some(tokens![FilipinoGrapheme::O]),
        SourceGrapheme::P => Some(tokens![FilipinoGrapheme::P, FilipinoGrapheme::I]),
        SourceGrapheme::Q => Some(tokens![
            FilipinoGrapheme::K,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::U,
        ]),
        SourceGrapheme::R => Some(tokens![FilipinoGrapheme::A, FilipinoGrapheme::R]),
        SourceGrapheme::S => Some(tokens![FilipinoGrapheme::E, FilipinoGrapheme::S]),
        SourceGrapheme::T => Some(tokens![FilipinoGrapheme::T, FilipinoGrapheme::I]),
        SourceGrapheme::U => Some(tokens![FilipinoGrapheme::Y, FilipinoGrapheme::U]),
        SourceGrapheme::V => Some(tokens![FilipinoGrapheme::V, FilipinoGrapheme::I]),
        SourceGrapheme::W => Some(tokens![
            FilipinoGrapheme::D,
            FilipinoGrapheme::O,
            FilipinoGrapheme::B,
            FilipinoGrapheme::O,
            FilipinoGrapheme::L,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::U,
        ]),
        SourceGrapheme::X => Some(tokens![
            FilipinoGrapheme::E,
            FilipinoGrapheme::K,
            FilipinoGrapheme::S,
        ]),
        SourceGrapheme::Y => Some(tokens![
            FilipinoGrapheme::W,
            FilipinoGrapheme::A,
            FilipinoGrapheme::Y,
        ]),
        SourceGrapheme::Z => Some(tokens![FilipinoGrapheme::Z, FilipinoGrapheme::I]),
        _ => None,
    }
}
