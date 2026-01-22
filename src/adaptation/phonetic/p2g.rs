//! Phoneme-to-Grapheme (P2G) conversion for Filipino adaptation.
//!
//! Converts IPA phonemes to their Filipino grapheme representations.
//! This handles all sounds - vowels, consonants, and special cases.

use crate::grapheme::filipino::FilipinoGrapheme;
use crate::phoneme::tokens::ipa::IPASymbol;
use crate::tokens;

/// Performs P2G (phoneme-to-grapheme) given a specific IPA phoneme.
///
/// Returns (graphemes, is_diphthong) where is_diphthong indicates
/// if this phoneme represents two sounds (and might consume two graphemes).
///
/// # Arguments
///
/// * `phoneme` - The IPA phoneme to convert
///
/// # Returns
///
/// `Some((Vec<FilipinoGrapheme>, bool))` where bool indicates if it's a diphthong
pub fn graphemize(phoneme: &IPASymbol) -> Option<(Vec<FilipinoGrapheme>, bool)> {
    match phoneme {
        // Monothongs
        // A-like vowels
        IPASymbol::OpenBackUnrounded => Some((tokens![FilipinoGrapheme::A], false)),
        IPASymbol::NearOpenFront => Some((tokens![FilipinoGrapheme::A], false)),
        IPASymbol::OpenMidBack => Some((tokens![FilipinoGrapheme::A], false)),

        // E-like vowels
        IPASymbol::OpenMidFront => Some((tokens![FilipinoGrapheme::E], false)),
        IPASymbol::Schwa => Some((tokens![FilipinoGrapheme::A], false)),

        // I-like vowels
        IPASymbol::NearCloseFront => Some((tokens![FilipinoGrapheme::I], false)),
        IPASymbol::CloseFront => Some((tokens![FilipinoGrapheme::I], false)),

        // O-like vowels
        IPASymbol::OpenMidBackRounded => Some((tokens![FilipinoGrapheme::O], false)),

        // U-like vowels
        IPASymbol::NearCloseBack => Some((tokens![FilipinoGrapheme::U], false)),
        IPASymbol::CloseBack => Some((tokens![FilipinoGrapheme::U], false)),

        // Diphthongs
        IPASymbol::DiphthongAU => Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::W], true)),
        IPASymbol::DiphthongAI => Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::Y], true)),
        IPASymbol::DiphthongEI => Some((tokens![FilipinoGrapheme::E, FilipinoGrapheme::Y], true)),
        IPASymbol::DiphthongOU => Some((tokens![FilipinoGrapheme::O], true)),
        IPASymbol::DiphthongOI => Some((tokens![FilipinoGrapheme::O, FilipinoGrapheme::Y], true)),

        _ => None,
    }
}
