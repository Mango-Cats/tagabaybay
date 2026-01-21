use crate::grapheme::filipino::FilipinoGrapheme;
use crate::phoneme::symbols::ArpabetSymbols;
use crate::tokens;

/// Performs P2G (phoneme-to-grapheme) given a specific phoneme.
///
/// Returns (graphemes, is_diphthong) where is_diphthong indicates
/// if this phoneme represents two sounds (and might consume two graphemes).
pub fn graphemize(phoneme: &ArpabetSymbols) -> Option<(Vec<FilipinoGrapheme>, bool)> {
    match phoneme {
        // for A
        ArpabetSymbols::AA => Some((tokens![FilipinoGrapheme::A], false)),
        ArpabetSymbols::AE => Some((tokens![FilipinoGrapheme::A], false)),
        ArpabetSymbols::AH => Some((tokens![FilipinoGrapheme::A], false)),
        ArpabetSymbols::AO => Some((tokens![FilipinoGrapheme::O], false)),
        ArpabetSymbols::AY => Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::Y], true)),
        ArpabetSymbols::AW => Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::W], true)),

        // for E
        ArpabetSymbols::EH => Some((tokens![FilipinoGrapheme::E], false)),
        ArpabetSymbols::ER => Some((tokens![FilipinoGrapheme::E, FilipinoGrapheme::R], false)),

        // for I
        ArpabetSymbols::IH => Some((tokens![FilipinoGrapheme::I], false)),
        ArpabetSymbols::IY => Some((tokens![FilipinoGrapheme::I], false)),

        // for U
        ArpabetSymbols::UW => Some((tokens![FilipinoGrapheme::U], false)),
        ArpabetSymbols::UH => Some((tokens![FilipinoGrapheme::U], false)),

        // for O
        ArpabetSymbols::OW => Some((tokens![FilipinoGrapheme::O], true)),
        ArpabetSymbols::OY => Some((tokens![FilipinoGrapheme::O], true)),

        // for ai, ey (like bait)
        ArpabetSymbols::EY => Some((tokens![FilipinoGrapheme::E, FilipinoGrapheme::Y], true)),
        _ => None,
    }
}
