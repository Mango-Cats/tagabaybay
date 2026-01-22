#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArpabetSymbols {
    // Vowels
    AA,
    AE,
    AH,
    AO,
    AW,
    AY,
    EH,
    ER,
    EY,
    IH,
    IY,
    OW,
    OY,
    UH,
    UW,

    // Consonants, only used for checking
    B,
    CH,
    D,
    DH,
    DX,
    EL,
    EM,
    EN,
    F,
    G,
    HH,
    JH,
    K,
    L,
    M,
    N,
    NG,
    NX,
    P,
    Q,
    R,
    S,
    SH,
    T,
    TH,
    V,
    W,
    WH,
    Y,
    Z,
    ZH,
}

impl ArpabetSymbols {
    pub fn as_str(&self) -> String {
        match self {
            // Vowels
            ArpabetSymbols::AA => "aa".to_string(),
            ArpabetSymbols::AE => "ae".to_string(),
            ArpabetSymbols::AH => "ah".to_string(),
            ArpabetSymbols::AO => "ao".to_string(),
            ArpabetSymbols::AW => "aw".to_string(),
            ArpabetSymbols::AY => "ay".to_string(),
            ArpabetSymbols::EH => "eh".to_string(),
            ArpabetSymbols::ER => "er".to_string(),
            ArpabetSymbols::EY => "ey".to_string(),
            ArpabetSymbols::IH => "ih".to_string(),
            ArpabetSymbols::IY => "iy".to_string(),
            ArpabetSymbols::OW => "ow".to_string(),
            ArpabetSymbols::OY => "oy".to_string(),
            ArpabetSymbols::UH => "uh".to_string(),
            ArpabetSymbols::UW => "uw".to_string(),

            // Consonants
            ArpabetSymbols::B => "b".to_string(),
            ArpabetSymbols::CH => "ch".to_string(),
            ArpabetSymbols::D => "d".to_string(),
            ArpabetSymbols::DH => "dh".to_string(),
            ArpabetSymbols::DX => "dx".to_string(),
            ArpabetSymbols::EL => "el".to_string(),
            ArpabetSymbols::EM => "em".to_string(),
            ArpabetSymbols::EN => "en".to_string(),
            ArpabetSymbols::F => "f".to_string(),
            ArpabetSymbols::G => "g".to_string(),
            ArpabetSymbols::HH => "hh".to_string(),
            ArpabetSymbols::JH => "jh".to_string(),
            ArpabetSymbols::K => "k".to_string(),
            ArpabetSymbols::L => "l".to_string(),
            ArpabetSymbols::M => "m".to_string(),
            ArpabetSymbols::N => "n".to_string(),
            ArpabetSymbols::NG => "ng".to_string(),
            ArpabetSymbols::P => "p".to_string(),
            ArpabetSymbols::Q => "q".to_string(),
            ArpabetSymbols::R => "r".to_string(),
            ArpabetSymbols::S => "s".to_string(),
            ArpabetSymbols::SH => "sh".to_string(),
            ArpabetSymbols::T => "t".to_string(),
            ArpabetSymbols::TH => "th".to_string(),
            ArpabetSymbols::V => "v".to_string(),
            ArpabetSymbols::W => "w".to_string(),
            ArpabetSymbols::WH => "wh".to_string(),
            ArpabetSymbols::Y => "y".to_string(),
            ArpabetSymbols::Z => "z".to_string(),
            ArpabetSymbols::ZH => "zh".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn is_unpredictable_variant_pronunciation(&self) -> bool {
        match self {
            ArpabetSymbols::AA
            | ArpabetSymbols::AE
            | ArpabetSymbols::AH
            | ArpabetSymbols::AO
            | ArpabetSymbols::AW
            | ArpabetSymbols::AY
            | ArpabetSymbols::EH
            | ArpabetSymbols::ER
            | ArpabetSymbols::EY
            | ArpabetSymbols::IH
            | ArpabetSymbols::IY
            | ArpabetSymbols::OW
            | ArpabetSymbols::OY
            | ArpabetSymbols::UH
            | ArpabetSymbols::UW => true,
            _ => false,
        }
    }

    /// Check if this is a vowel phoneme
    pub fn is_vowel(&self) -> bool {
        matches!(
            self,
            ArpabetSymbols::AA
                | ArpabetSymbols::AE
                | ArpabetSymbols::AH
                | ArpabetSymbols::AO
                | ArpabetSymbols::AW
                | ArpabetSymbols::AY
                | ArpabetSymbols::EH
                | ArpabetSymbols::ER
                | ArpabetSymbols::EY
                | ArpabetSymbols::IH
                | ArpabetSymbols::IY
                | ArpabetSymbols::OW
                | ArpabetSymbols::OY
                | ArpabetSymbols::UH
                | ArpabetSymbols::UW
        )
    }

    /// Check if this is a consonant phoneme
    pub fn is_consonant(&self) -> bool {
        !self.is_vowel()
    }
}

pub fn match_arpabet(s: &str) -> Option<ArpabetSymbols> {
    match s.to_lowercase().as_str() {
        // Vowels
        "aa" => Some(ArpabetSymbols::AA),
        "ae" => Some(ArpabetSymbols::AE),
        "ah" => Some(ArpabetSymbols::AH),
        "ao" => Some(ArpabetSymbols::AO),
        "aw" => Some(ArpabetSymbols::AW),
        "ay" => Some(ArpabetSymbols::AY),
        "eh" => Some(ArpabetSymbols::EH),
        "er" => Some(ArpabetSymbols::ER),
        "ey" => Some(ArpabetSymbols::EY),
        "ih" => Some(ArpabetSymbols::IH),
        "iy" => Some(ArpabetSymbols::IY),
        "ow" => Some(ArpabetSymbols::OW),
        "oy" => Some(ArpabetSymbols::OY),
        "uh" => Some(ArpabetSymbols::UH),
        "uw" => Some(ArpabetSymbols::UW),

        // Consonants
        "b" => Some(ArpabetSymbols::B),
        "ch" => Some(ArpabetSymbols::CH),
        "d" => Some(ArpabetSymbols::D),
        "dh" => Some(ArpabetSymbols::DH),
        "dx" => Some(ArpabetSymbols::DX),
        "el" => Some(ArpabetSymbols::EL),
        "em" => Some(ArpabetSymbols::EM),
        "en" => Some(ArpabetSymbols::EN),
        "f" => Some(ArpabetSymbols::F),
        "g" => Some(ArpabetSymbols::G),
        "hh" => Some(ArpabetSymbols::HH),
        "jh" => Some(ArpabetSymbols::JH),
        "k" => Some(ArpabetSymbols::K),
        "l" => Some(ArpabetSymbols::L),
        "m" => Some(ArpabetSymbols::M),
        "n" => Some(ArpabetSymbols::N),
        "ng" => Some(ArpabetSymbols::NG),
        "p" => Some(ArpabetSymbols::P),
        "q" => Some(ArpabetSymbols::Q),
        "r" => Some(ArpabetSymbols::R),
        "s" => Some(ArpabetSymbols::S),
        "sh" => Some(ArpabetSymbols::SH),
        "t" => Some(ArpabetSymbols::T),
        "th" => Some(ArpabetSymbols::TH),
        "v" => Some(ArpabetSymbols::V),
        "w" => Some(ArpabetSymbols::W),
        "wh" => Some(ArpabetSymbols::WH),
        "y" => Some(ArpabetSymbols::Y),
        "z" => Some(ArpabetSymbols::Z),
        "zh" => Some(ArpabetSymbols::ZH),

        _ => None,
    }
}

impl std::fmt::Display for ArpabetSymbols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
