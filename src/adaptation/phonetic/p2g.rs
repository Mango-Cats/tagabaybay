use crate::grapheme::filipino::FilipinoGrapheme;
use crate::phoneme::symbols::ArpabetSymbols;
use crate::tokens;

/// Performs P2G (phoneme-to-grapheme) given a specific phoneme.
///
/// Returns (graphemes, is_diphthong) where is_diphthong indicates
/// if this phoneme represents two sounds (and might consume two graphemes).
pub fn graphemize(phoneme: &ArpabetSymbols) -> (Vec<FilipinoGrapheme>, bool) {
    match phoneme {
        ArpabetSymbols::AA => (tokens![FilipinoGrapheme::A], false),
        ArpabetSymbols::AE => (tokens![FilipinoGrapheme::A], false),
        ArpabetSymbols::AH => (tokens![FilipinoGrapheme::A], false),
        ArpabetSymbols::AO => (tokens![FilipinoGrapheme::O], false),
        ArpabetSymbols::EH => (tokens![FilipinoGrapheme::E], false),
        ArpabetSymbols::ER => (tokens![FilipinoGrapheme::E, FilipinoGrapheme::R], false),
        ArpabetSymbols::IH => (tokens![FilipinoGrapheme::I], false),
        ArpabetSymbols::IY => (tokens![FilipinoGrapheme::I], false),
        ArpabetSymbols::UH => (tokens![FilipinoGrapheme::U], false),
        ArpabetSymbols::UW => (tokens![FilipinoGrapheme::U], false),
        ArpabetSymbols::AW => (tokens![FilipinoGrapheme::A, FilipinoGrapheme::W], true),
        ArpabetSymbols::AY => (tokens![FilipinoGrapheme::A, FilipinoGrapheme::Y], true),
        ArpabetSymbols::EY => (tokens![FilipinoGrapheme::E, FilipinoGrapheme::Y], true),
        ArpabetSymbols::OW => (tokens![FilipinoGrapheme::O], true),
        ArpabetSymbols::OY => (tokens![FilipinoGrapheme::O], true),

        _ => (tokens![FilipinoGrapheme::A], false),
    }
}
