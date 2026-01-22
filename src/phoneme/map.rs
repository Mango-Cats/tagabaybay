use super::ipa::IPASymbol;
use super::symbols::ArpabetSymbols;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// IPA string to IPASymbol mapping (for tokenization)
///
/// Maps IPA character sequences to their corresponding IPASymbol enum variants.
/// Order matters: longer sequences (diphthongs, affricates) should be checked first.
pub static IPA_STR_TO_SYMBOL: Lazy<HashMap<&'static str, IPASymbol>> = Lazy::new(|| {
    HashMap::from([
        // Diphthongs (check these first - 2 chars)
        ("aʊ", IPASymbol::DiphthongAU),
        ("aɪ", IPASymbol::DiphthongAI),
        ("eɪ", IPASymbol::DiphthongEI),
        ("oʊ", IPASymbol::DiphthongOU),
        ("ɔɪ", IPASymbol::DiphthongOI),
        // Affricates (2 chars)
        ("tʃ", IPASymbol::VoicelessPostalveolarAffricate),
        ("dʒ", IPASymbol::VoicedPostalveolarAffricate),
        // Monophthong vowels
        ("ɑ", IPASymbol::OpenBackUnrounded),
        ("æ", IPASymbol::NearOpenFront),
        ("ʌ", IPASymbol::OpenMidBack),
        ("ɐ", IPASymbol::OpenMidBack),
        ("ə", IPASymbol::Schwa),
        ("ɔ", IPASymbol::OpenMidBackRounded),
        ("ɛ", IPASymbol::OpenMidFront),
        ("ɝ", IPASymbol::RColoredMid),
        ("ɚ", IPASymbol::RColoredSchwa),
        ("ɪ", IPASymbol::NearCloseFront),
        ("ᵻ", IPASymbol::NearCloseFront),
        ("i", IPASymbol::CloseFront),
        ("ʊ", IPASymbol::NearCloseBack),
        ("u", IPASymbol::CloseBack),
        ("e", IPASymbol::DiphthongEI),
        ("o", IPASymbol::DiphthongOU),
        ("a", IPASymbol::OpenBackUnrounded),
        // Consonants - Stops
        ("p", IPASymbol::VoicelessBilabialStop),
        ("b", IPASymbol::VoicedBilabialStop),
        ("t", IPASymbol::VoicelessAlveolarStop),
        ("d", IPASymbol::VoicedAlveolarStop),
        ("k", IPASymbol::VoicelessVelarStop),
        ("g", IPASymbol::VoicedVelarStop),
        ("ɡ", IPASymbol::VoicedVelarStop),
        ("ʔ", IPASymbol::GlottalStop),
        // Consonants - Fricatives
        ("f", IPASymbol::VoicelessLabiodentalFricative),
        ("v", IPASymbol::VoicedLabiodentalFricative),
        ("θ", IPASymbol::VoicelessDentalFricative),
        ("ð", IPASymbol::VoicedDentalFricative),
        ("s", IPASymbol::VoicelessAlveolarFricative),
        ("z", IPASymbol::VoicedAlveolarFricative),
        ("ʃ", IPASymbol::VoicelessPostalveolarFricative),
        ("ʒ", IPASymbol::VoicedPostalveolarFricative),
        ("h", IPASymbol::VoicelessGlottalFricative),
        // Consonants - Nasals
        ("m", IPASymbol::BilabialNasal),
        ("n", IPASymbol::AlveolarNasal),
        ("ŋ", IPASymbol::VelarNasal),
        ("ɲ", IPASymbol::PalatalNasal),
        // Consonants - Approximants
        ("l", IPASymbol::AlveolarLateral),
        ("ɹ", IPASymbol::AlveolarApproximant),
        ("r", IPASymbol::AlveolarTrill),
        ("w", IPASymbol::LabialVelarApproximant),
        ("j", IPASymbol::PalatalApproximant),
        ("ʍ", IPASymbol::VoicelessLabialVelar),
        // Tap
        ("ɾ", IPASymbol::AlveolarTap),
    ])
});

/// ARPABET to IPA symbol mapping (for conversion from Phonetisaurus output)
pub static ARPA_TO_IPA: Lazy<HashMap<ArpabetSymbols, IPASymbol>> = Lazy::new(|| {
    HashMap::from([
        // Vowels
        (ArpabetSymbols::AA, IPASymbol::OpenBackUnrounded),
        (ArpabetSymbols::AE, IPASymbol::NearOpenFront),
        (ArpabetSymbols::AH, IPASymbol::Schwa),
        (ArpabetSymbols::AO, IPASymbol::OpenMidBackRounded),
        (ArpabetSymbols::AW, IPASymbol::DiphthongAU),
        (ArpabetSymbols::AY, IPASymbol::DiphthongAI),
        (ArpabetSymbols::EH, IPASymbol::OpenMidFront),
        (ArpabetSymbols::ER, IPASymbol::RColoredMid),
        (ArpabetSymbols::EY, IPASymbol::DiphthongEI),
        (ArpabetSymbols::IH, IPASymbol::NearCloseFront),
        (ArpabetSymbols::IY, IPASymbol::CloseFront),
        (ArpabetSymbols::OW, IPASymbol::DiphthongOU),
        (ArpabetSymbols::OY, IPASymbol::DiphthongOI),
        (ArpabetSymbols::UH, IPASymbol::NearCloseBack),
        (ArpabetSymbols::UW, IPASymbol::CloseBack),
        // Consonants
        (ArpabetSymbols::B, IPASymbol::VoicedBilabialStop),
        (
            ArpabetSymbols::CH,
            IPASymbol::VoicelessPostalveolarAffricate,
        ),
        (ArpabetSymbols::D, IPASymbol::VoicedAlveolarStop),
        (ArpabetSymbols::DH, IPASymbol::VoicedDentalFricative),
        (ArpabetSymbols::F, IPASymbol::VoicelessLabiodentalFricative),
        (ArpabetSymbols::G, IPASymbol::VoicedVelarStop),
        (ArpabetSymbols::HH, IPASymbol::VoicelessGlottalFricative),
        (ArpabetSymbols::JH, IPASymbol::VoicedPostalveolarAffricate),
        (ArpabetSymbols::K, IPASymbol::VoicelessVelarStop),
        (ArpabetSymbols::L, IPASymbol::AlveolarLateral),
        (ArpabetSymbols::M, IPASymbol::BilabialNasal),
        (ArpabetSymbols::N, IPASymbol::AlveolarNasal),
        (ArpabetSymbols::NG, IPASymbol::VelarNasal),
        (ArpabetSymbols::P, IPASymbol::VoicelessBilabialStop),
        (ArpabetSymbols::R, IPASymbol::AlveolarApproximant),
        (ArpabetSymbols::S, IPASymbol::VoicelessAlveolarFricative),
        (
            ArpabetSymbols::SH,
            IPASymbol::VoicelessPostalveolarFricative,
        ),
        (ArpabetSymbols::T, IPASymbol::VoicelessAlveolarStop),
        (ArpabetSymbols::TH, IPASymbol::VoicelessDentalFricative),
        (ArpabetSymbols::V, IPASymbol::VoicedLabiodentalFricative),
        (ArpabetSymbols::W, IPASymbol::LabialVelarApproximant),
        (ArpabetSymbols::Y, IPASymbol::PalatalApproximant),
        (ArpabetSymbols::Z, IPASymbol::VoicedAlveolarFricative),
        (ArpabetSymbols::ZH, IPASymbol::VoicedPostalveolarFricative),
    ])
});

/// IPA to ARPABET mapping (legacy, for backwards compatibility)
pub(super) static IPA_TO_ARPA: Lazy<HashMap<&'static str, ArpabetSymbols>> = Lazy::new(|| {
    HashMap::from([
        // Vowels
        ("ɑ", ArpabetSymbols::AA),
        ("æ", ArpabetSymbols::AE),
        ("ʌ", ArpabetSymbols::AH),
        ("ɐ", ArpabetSymbols::AH),
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
