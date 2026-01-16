use crate::grapheme::filipino::FilipinoGrapheme;
use crate::phoneme::symbols::ArpabetSymbols;

/// Performs P2G (phoneme-to-grapheme) given a specific phoneme.
///
/// Returns (graphemes, is_diphthong) where is_diphthong indicates
/// if this phoneme represents two sounds (and might consume two graphemes).
pub fn graphemize(phoneme: &ArpabetSymbols) -> (Vec<FilipinoGrapheme>, bool) {
    match phoneme {
        ArpabetSymbols::AA => (vec![FilipinoGrapheme::A], false),
        ArpabetSymbols::AE => (vec![FilipinoGrapheme::A], false),
        ArpabetSymbols::AH => (vec![FilipinoGrapheme::A], false),
        ArpabetSymbols::AO => (vec![FilipinoGrapheme::O], false),
        ArpabetSymbols::EH => (vec![FilipinoGrapheme::E], false),
        ArpabetSymbols::ER => (vec![FilipinoGrapheme::E, FilipinoGrapheme::R], false),
        ArpabetSymbols::IH => (vec![FilipinoGrapheme::I], false),
        ArpabetSymbols::IY => (vec![FilipinoGrapheme::I], false),
        ArpabetSymbols::UH => (vec![FilipinoGrapheme::U], false),
        ArpabetSymbols::UW => (vec![FilipinoGrapheme::U], false),
        ArpabetSymbols::AW => (vec![FilipinoGrapheme::A, FilipinoGrapheme::W], true),
        ArpabetSymbols::AY => (vec![FilipinoGrapheme::A, FilipinoGrapheme::Y], true),
        ArpabetSymbols::EY => (vec![FilipinoGrapheme::E, FilipinoGrapheme::Y], true),
        ArpabetSymbols::OW => (vec![FilipinoGrapheme::O], true),
        ArpabetSymbols::OY => (vec![FilipinoGrapheme::O], true),

        _ => (vec![FilipinoGrapheme::A], false),
    }
}
