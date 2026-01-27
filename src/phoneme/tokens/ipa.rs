//! IPA (International Phonetic Alphabet) symbols for phoneme representation.
//!
//! This module provides the primary phoneme representation used throughout
//! tagabaybay. IPA is preferred over ARPABET for its:
//! - Universal standardization
//! - Direct mapping to Filipino phonology
//! - Better support for non-English sounds
//!
//! # Motivation
//!
//! This is why we introduce tokenization in the first place, handling
//! Unicode characters (like IPA symbols) would make it difficult to process
//! in the loanword adaptation later on. So, by tokenizing it, we only introduce
//! Unicode characters here.

use std::fmt;

/// IPA phoneme symbols used in English and Filipino phonology.
///
/// Organized by manner and place of articulation following IPA conventions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IPASymbol {
    // VOWELS - Monophthongs
    /// /ɑ/ - Open back unrounded (father, spa)
    OpenBackUnrounded,
    /// /æ/ - Near-open front unrounded (cat, bat)  
    NearOpenFront,
    /// /ʌ/ - Open-mid back unrounded (strut, cup)
    OpenMidBack,
    /// /ə/ - Mid central (schwa) (about, sofa)
    Schwa,
    /// /ɔ/ - Open-mid back rounded (thought, law)
    OpenMidBackRounded,
    /// /ɛ/ - Open-mid front unrounded (bed, get)
    OpenMidFront,
    /// /ɝ/ - R-colored mid central stressed (bird, nurse)
    RColoredMid,
    /// /ɚ/ - R-colored schwa unstressed (butter, sister)
    RColoredSchwa,
    /// /ɪ/ - Near-close near-front unrounded (bit, kid)
    NearCloseFront,
    /// /i/ - Close front unrounded (fleece, see)
    CloseFront,
    /// /ʊ/ - Near-close near-back rounded (foot, put)
    NearCloseBack,
    /// /u/ - Close back rounded (goose, blue)
    CloseBack,

    // VOWELS - Diphthongs
    /// /aʊ/ - (mouth, how)
    DiphthongAU,
    /// /aɪ/ - (price, my)
    DiphthongAI,
    /// /eɪ/ - (face, say)
    DiphthongEI,
    /// /oʊ/ - (goat, know)
    DiphthongOU,
    /// /ɔɪ/ - (choice, boy)
    DiphthongOI,

    // CONSONANTS - Stops (Plosives)
    /// /p/ - Voiceless bilabial stop (pat)
    VoicelessBilabialStop,
    /// /b/ - Voiced bilabial stop (bat)
    VoicedBilabialStop,
    /// /t/ - Voiceless alveolar stop (tap)
    VoicelessAlveolarStop,
    /// /d/ - Voiced alveolar stop (dad)
    VoicedAlveolarStop,
    /// /k/ - Voiceless velar stop (cat)
    VoicelessVelarStop,
    /// /g/ - Voiced velar stop (gap)
    VoicedVelarStop,
    /// /ʔ/ - Glottal stop (uh-oh)
    GlottalStop,

    // CONSONANTS - Fricatives
    /// /f/ - Voiceless labiodental fricative (fat)
    VoicelessLabiodentalFricative,
    /// /v/ - Voiced labiodental fricative (vat)
    VoicedLabiodentalFricative,
    /// /θ/ - Voiceless dental fricative (thin)
    VoicelessDentalFricative,
    /// /ð/ - Voiced dental fricative (this)
    VoicedDentalFricative,
    /// /s/ - Voiceless alveolar fricative (sat)
    VoicelessAlveolarFricative,
    /// /z/ - Voiced alveolar fricative (zap)
    VoicedAlveolarFricative,
    /// /ʃ/ - Voiceless postalveolar fricative (ship)
    VoicelessPostalveolarFricative,
    /// /ʒ/ - Voiced postalveolar fricative (measure)
    VoicedPostalveolarFricative,
    /// /h/ - Voiceless glottal fricative (hat)
    VoicelessGlottalFricative,

    // CONSONANTS - Affricates
    /// /tʃ/ - Voiceless postalveolar affricate (chip)
    VoicelessPostalveolarAffricate,
    /// /dʒ/ - Voiced postalveolar affricate (jug)
    VoicedPostalveolarAffricate,

    // CONSONANTS - Nasals
    /// /m/ - Bilabial nasal (map)
    BilabialNasal,
    /// /n/ - Alveolar nasal (nap)
    AlveolarNasal,
    /// /ŋ/ - Velar nasal (sing)
    VelarNasal,
    /// /ɲ/ - Palatal nasal (ñ in Spanish señor)
    PalatalNasal,

    // CONSONANTS - Approximants
    /// /l/ - Alveolar lateral approximant (lap)
    AlveolarLateral,
    /// /ɹ/ - Alveolar approximant (rap) - American English R
    AlveolarApproximant,
    /// /r/ - Alveolar trill (Spanish perro) - also used as R variant
    AlveolarTrill,
    /// /w/ - Labial-velar approximant (wag)
    LabialVelarApproximant,
    /// /j/ - Palatal approximant (yes)
    PalatalApproximant,
    /// /ʍ/ - Voiceless labial-velar fricative (which - in some dialects)
    VoicelessLabialVelar,

    // CONSONANTS - Tap/Flap
    /// /ɾ/ - Alveolar tap (better in American English, Filipino r)
    AlveolarTap,

    // MISCALLANEOUS - Not exactly sure what to name it rn
    /// /ː/ - Triangular colon implies that the previous phoneme has a long sound
    TriangularColon,
    /// /:/ - Regular colon implies that the previous phoneme has a long sound
    RegularColon,
}

impl IPASymbol {
    /// Get the IPA character representation
    pub fn as_str(&self) -> &'static str {
        match self {
            // Monophthongs
            IPASymbol::OpenBackUnrounded => "ɑ",
            IPASymbol::NearOpenFront => "æ",
            IPASymbol::OpenMidBack => "ʌ",
            IPASymbol::Schwa => "ə",
            IPASymbol::OpenMidBackRounded => "ɔ",
            IPASymbol::OpenMidFront => "ɛ",
            IPASymbol::RColoredMid => "ɝ",
            IPASymbol::RColoredSchwa => "ɚ",
            IPASymbol::NearCloseFront => "ɪ",
            IPASymbol::CloseFront => "i",
            IPASymbol::NearCloseBack => "ʊ",
            IPASymbol::CloseBack => "u",

            // Diphthongs
            IPASymbol::DiphthongAU => "aʊ",
            IPASymbol::DiphthongAI => "aɪ",
            IPASymbol::DiphthongEI => "eɪ",
            IPASymbol::DiphthongOU => "oʊ",
            IPASymbol::DiphthongOI => "ɔɪ",

            // Stops
            IPASymbol::VoicelessBilabialStop => "p",
            IPASymbol::VoicedBilabialStop => "b",
            IPASymbol::VoicelessAlveolarStop => "t",
            IPASymbol::VoicedAlveolarStop => "d",
            IPASymbol::VoicelessVelarStop => "k",
            IPASymbol::VoicedVelarStop => "g",
            IPASymbol::GlottalStop => "ʔ",

            // Fricatives
            IPASymbol::VoicelessLabiodentalFricative => "f",
            IPASymbol::VoicedLabiodentalFricative => "v",
            IPASymbol::VoicelessDentalFricative => "θ",
            IPASymbol::VoicedDentalFricative => "ð",
            IPASymbol::VoicelessAlveolarFricative => "s",
            IPASymbol::VoicedAlveolarFricative => "z",
            IPASymbol::VoicelessPostalveolarFricative => "ʃ",
            IPASymbol::VoicedPostalveolarFricative => "ʒ",
            IPASymbol::VoicelessGlottalFricative => "h",

            // Affricates
            IPASymbol::VoicelessPostalveolarAffricate => "tʃ",
            IPASymbol::VoicedPostalveolarAffricate => "dʒ",

            // Nasals
            IPASymbol::BilabialNasal => "m",
            IPASymbol::AlveolarNasal => "n",
            IPASymbol::VelarNasal => "ŋ",
            IPASymbol::PalatalNasal => "ɲ",

            // Approximants
            IPASymbol::AlveolarLateral => "l",
            IPASymbol::AlveolarApproximant => "ɹ",
            IPASymbol::AlveolarTrill => "r",
            IPASymbol::LabialVelarApproximant => "w",
            IPASymbol::PalatalApproximant => "j",
            IPASymbol::VoicelessLabialVelar => "ʍ",

            // Tap
            IPASymbol::AlveolarTap => "ɾ",

            // Miscellaneous
            IPASymbol::TriangularColon => "ː",
            IPASymbol::RegularColon => ":",
        }
    }

    /// Check if this is a vowel phoneme (monophthong or diphthong)
    pub fn is_vowel(&self) -> bool {
        matches!(
            self,
            // Monophthongs
            IPASymbol::OpenBackUnrounded
                | IPASymbol::NearOpenFront
                | IPASymbol::OpenMidBack
                | IPASymbol::Schwa
                | IPASymbol::OpenMidBackRounded
                | IPASymbol::OpenMidFront
                | IPASymbol::RColoredMid
                | IPASymbol::RColoredSchwa
                | IPASymbol::NearCloseFront
                | IPASymbol::CloseFront
                | IPASymbol::NearCloseBack
                | IPASymbol::CloseBack
                // Diphthongs
                | IPASymbol::DiphthongAU
                | IPASymbol::DiphthongAI
                | IPASymbol::DiphthongEI
                | IPASymbol::DiphthongOU
                | IPASymbol::DiphthongOI
        )
    }

    /// Check if this is a diphthong
    pub fn is_diphthong(&self) -> bool {
        matches!(
            self,
            IPASymbol::DiphthongAU
                | IPASymbol::DiphthongAI
                | IPASymbol::DiphthongEI
                | IPASymbol::DiphthongOU
                | IPASymbol::DiphthongOI
        )
    }

    /// Check if this is a consonant phoneme
    pub fn is_consonant(&self) -> bool {
        !self.is_vowel()
    }

    /// Check if this phoneme represents an unpredictable variant sound
    /// (vowels whose spelling doesn't reliably predict pronunciation)
    pub fn is_unpredictable_variant(&self) -> bool {
        self.is_vowel()
    }
}

impl fmt::Display for IPASymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
