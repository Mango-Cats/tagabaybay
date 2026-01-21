use super::symbols::ArpabetSymbols;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// IPA to ARPABET mapping
pub(super) static IPA_TO_ARPA: Lazy<HashMap<&'static str, ArpabetSymbols>> = Lazy::new(|| {
    HashMap::from([
        // Vowels
        ("ɑ", ArpabetSymbols::AA),
        ("æ", ArpabetSymbols::AE),
        ("ʌ", ArpabetSymbols::AH),
        ("ɔ", ArpabetSymbols::AO),
        ("aʊ", ArpabetSymbols::AW),
        ("aɪ", ArpabetSymbols::AY),
        ("ɛ", ArpabetSymbols::EH),
        ("ɝ", ArpabetSymbols::ER),
        ("eɪ", ArpabetSymbols::EY),
        ("ɪ", ArpabetSymbols::IH),
        ("i", ArpabetSymbols::IY),
        ("oʊ", ArpabetSymbols::OW),
        ("ɔɪ", ArpabetSymbols::OY),
        ("ʊ", ArpabetSymbols::UH),
        ("u", ArpabetSymbols::UW),
        ("ə", ArpabetSymbols::AH),
        ("ɚ", ArpabetSymbols::ER),
        ("ᵻ", ArpabetSymbols::IH),
        ("e", ArpabetSymbols::EY),
        ("o", ArpabetSymbols::OW),
        ("a", ArpabetSymbols::AA),
        // Consonants
        ("b", ArpabetSymbols::B),
        ("tʃ", ArpabetSymbols::CH),
        ("d", ArpabetSymbols::D),
        ("ð", ArpabetSymbols::DH),
        ("f", ArpabetSymbols::F),
        ("g", ArpabetSymbols::G),
        ("h", ArpabetSymbols::HH),
        ("dʒ", ArpabetSymbols::JH),
        ("k", ArpabetSymbols::K),
        ("l", ArpabetSymbols::L),
        ("m", ArpabetSymbols::M),
        ("n", ArpabetSymbols::N),
        ("ŋ", ArpabetSymbols::NG),
        ("p", ArpabetSymbols::P),
        ("ɹ", ArpabetSymbols::R),
        ("r", ArpabetSymbols::R),
        ("s", ArpabetSymbols::S),
        ("ʃ", ArpabetSymbols::SH),
        ("t", ArpabetSymbols::T),
        ("θ", ArpabetSymbols::TH),
        ("v", ArpabetSymbols::V),
        ("w", ArpabetSymbols::W),
        ("j", ArpabetSymbols::Y),
        ("z", ArpabetSymbols::Z),
        ("ʒ", ArpabetSymbols::ZH),
    ])
});
