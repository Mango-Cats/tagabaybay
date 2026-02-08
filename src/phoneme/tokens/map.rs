use crate::{grapheme::{filipino::FilipinoGrapheme, source::SourceGrapheme}, phoneme::tokens::arpabet::ArpabetSymbols};
use crate::adaptation::cursor::Cursor;
use super::ipa::IPASymbol;
use once_cell::sync::Lazy;
use std::{collections::HashMap, vec};
use crate::adaptation::alignment::AlignedString;

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
        // Miscellaneous 
        // ("ː", IPASymbol::TriangularColon),
        // (":", IPASymbol::RegularColon),
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

/// IPA to FilipinoGraphemes
/// Converting IPA symbols to its corresponding filipino graphemes 
/// !* Subject to editing since im not sure im correct for all of these *
pub static IPA_TO_FG: Lazy<HashMap<IPASymbol, Vec<FilipinoGrapheme>>> = Lazy::new(|| {
    HashMap::from([
        // Vowels
        (IPASymbol::OpenBackUnrounded, vec![FilipinoGrapheme::A]), // "ɑ"
        (IPASymbol::NearOpenFront, vec![FilipinoGrapheme::A]), // "æ"
        (IPASymbol::OpenMidBack, vec![FilipinoGrapheme::A]), // "ʌ" 
        (IPASymbol::Schwa, vec![FilipinoGrapheme::E]), // "ə" 
        (IPASymbol::OpenMidBackRounded, vec![FilipinoGrapheme::O]), // "ɔ"
        (IPASymbol::OpenMidFront, vec![FilipinoGrapheme::E, FilipinoGrapheme::Y]), // "ɛ"
        (IPASymbol::RColoredMid, vec![FilipinoGrapheme::I]), // "ɝ"
        (IPASymbol::RColoredSchwa, vec![FilipinoGrapheme::E, FilipinoGrapheme::R]), // "ɚ"
        // Default is 'I', but it is context sensititive
        (IPASymbol::NearCloseFront, vec![FilipinoGrapheme::I]), // "ɪ"
        (IPASymbol::CloseFront, vec![FilipinoGrapheme::I]), // "i"
        (IPASymbol::NearCloseBack, vec![FilipinoGrapheme::U]), // "ʊ"
        (IPASymbol::CloseBack, vec![FilipinoGrapheme::U]), // "u"
        
        // Diphthongs
        (IPASymbol::DiphthongAU, vec![FilipinoGrapheme::A, FilipinoGrapheme::W]), // "aʊ"
        (IPASymbol::DiphthongAI, vec![FilipinoGrapheme::A, FilipinoGrapheme::Y]), // "aɪ"
        (IPASymbol::DiphthongEI, vec![FilipinoGrapheme::E, FilipinoGrapheme::Y]), // "eɪ"
        (IPASymbol::DiphthongOU, vec![FilipinoGrapheme::O]), // "oʊ"
        (IPASymbol::DiphthongOI, vec![FilipinoGrapheme::O, FilipinoGrapheme::Y]), // "ɔɪ"
        
        // Stops 
        (IPASymbol::VoicelessBilabialStop, vec![FilipinoGrapheme::P]), // "p"
        (IPASymbol::VoicedBilabialStop, vec![FilipinoGrapheme::B]), // "b"
        (IPASymbol::VoicelessAlveolarStop, vec![FilipinoGrapheme::T]), // "t"
        (IPASymbol::VoicedAlveolarStop, vec![FilipinoGrapheme::D]), // "d"
        (IPASymbol::VoicelessVelarStop, vec![FilipinoGrapheme::K]), // "k"
        (IPASymbol::VoicedVelarStop, vec![FilipinoGrapheme::G]), // "g"
        (IPASymbol::GlottalStop, vec![FilipinoGrapheme::T]), // "ʔ"
        
        // Fricatives
        (IPASymbol::VoicelessLabiodentalFricative, vec![FilipinoGrapheme::F]), // "f"
        (IPASymbol::VoicedLabiodentalFricative, vec![FilipinoGrapheme::B]), // "v"
        (IPASymbol::VoicelessDentalFricative, vec![FilipinoGrapheme::T]), // "θ"
        (IPASymbol::VoicedDentalFricative, vec![FilipinoGrapheme::D]), // "ð"
        (IPASymbol::VoicelessAlveolarFricative, vec![FilipinoGrapheme::S]), // "s"
        (IPASymbol::VoicedAlveolarFricative, vec![FilipinoGrapheme::S]), // "z"
        (IPASymbol::VoicelessPostalveolarFricative, vec![FilipinoGrapheme::S, FilipinoGrapheme::I]), // "ʃ"
        (IPASymbol::VoicedPostalveolarFricative, vec![FilipinoGrapheme::S]), // "ʒ"
        (IPASymbol::VoicelessGlottalFricative, vec![FilipinoGrapheme::H]), // "h"

        // Affricates
        (IPASymbol::VoicelessPostalveolarAffricate, vec![FilipinoGrapheme::T, FilipinoGrapheme::S]), // "tʃ"
        (IPASymbol::VoicedPostalveolarAffricate, vec![FilipinoGrapheme::D, FilipinoGrapheme::Y]), // "dʒ"

        // Nasals
        (IPASymbol::BilabialNasal, vec![FilipinoGrapheme::M]), // "m"
        (IPASymbol::AlveolarNasal, vec![FilipinoGrapheme::N]), // "n"
        (IPASymbol::VelarNasal, vec![FilipinoGrapheme::N, FilipinoGrapheme::G]), // "ŋ"
        (IPASymbol::PalatalNasal, vec![FilipinoGrapheme::N, FilipinoGrapheme::Y]), // "ɲ"

        // Approximants
        (IPASymbol::AlveolarLateral, vec![FilipinoGrapheme::L]), // "l"
        (IPASymbol::AlveolarApproximant, vec![FilipinoGrapheme::R]), // "ɹ"
        (IPASymbol::AlveolarTrill, vec![FilipinoGrapheme::R]), // "r"
        (IPASymbol::LabialVelarApproximant, vec![FilipinoGrapheme::W]), // "w"
        (IPASymbol::PalatalApproximant, vec![FilipinoGrapheme::Y]), // "j"
        (IPASymbol::VoicelessLabialVelar, vec![FilipinoGrapheme::W]), // "ʍ"

        // Tap
        (IPASymbol::AlveolarTap, vec![FilipinoGrapheme::T]), // "ɾ"

    ])
});

pub fn ipa_to_filipino_graphemes(
    symbols: Vec<IPASymbol>, 
    aligned: &AlignedString
) -> String {
    let mut result = String::new();
    
    for (idx, symbol) in symbols.iter().enumerate() {
        if *symbol == IPASymbol::NearCloseFront { // "ɪ"
            if let Some((grapheme, _)) = aligned.get(idx) {
                let fg = match grapheme {
                    SourceGrapheme::A => vec![FilipinoGrapheme::E, FilipinoGrapheme::Y],
                    SourceGrapheme::E => vec![FilipinoGrapheme::E],
                    _ => vec![FilipinoGrapheme::I],
                };
                result.push_str(&fg.iter().map(|g| g.to_string()).collect::<String>());
                continue;
            }
        }
        
        if let Some(graphemes) = IPA_TO_FG.get(symbol) {
            result.push_str(&graphemes.iter().map(|g| g.to_string()).collect::<String>());
        }
    }
    
    result
}

fn case_i_sound(ctx: &Cursor) -> Vec<FilipinoGrapheme> {
    if ctx.current_grapheme() == SourceGrapheme::A {
        return vec![FilipinoGrapheme::E, FilipinoGrapheme::Y];
    } else if ctx.current_grapheme() == SourceGrapheme::E {
        return vec![FilipinoGrapheme::E];
    } else {
        return vec![FilipinoGrapheme::I];
    }
}

/// IPA to ARPABET mapping (legacy, for backwards compatibility)
/// Use this when using Phonetisaurus: see src/g2p/arpa
#[allow(dead_code)]
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
