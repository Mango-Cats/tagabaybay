#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArpabetSymbols {
    // Vowels
    ArpaAA,
    ArpaAE,
    ArpaAH,
    ArpaAO,
    ArpaAW,
    ArpaAY,
    ArpaEH,
    ArpaER,
    ArpaEY,
    ArpaIH,
    ArpaIY,
    ArpaOW,
    ArpaOY,
    ArpaUH,
    ArpaUW,

    // Consonants, only used for checking
    ArpaB,
    ArpaCH,
    ArpaD,
    ArpaDH,
    ArpaDX,
    ArpaEL,
    ArpaEM,
    ArpaEN,
    ArpaF,
    ArpaG,
    ArpaHH,
    ArpaJH,
    ArpaK,
    ArpaL,
    ArpaM,
    ArpaN,
    ArpaNG,
    ArpaNX,
    ArpaP,
    ArpaQ,
    ArpaR,
    ArpaS,
    ArpaSH,
    ArpaT,
    ArpaTH,
    ArpaV,
    ArpaW,
    ArpaWH,
    ArpaY,
    ArpaZ,
    ArpaZH,
}

impl ArpabetSymbols {
    pub fn as_str(&self) -> String {
        match self {
            ArpabetSymbols::ArpaAA => "aa".to_string(),
            ArpabetSymbols::ArpaAE => "ae".to_string(),
            ArpabetSymbols::ArpaAH => "ah".to_string(),
            ArpabetSymbols::ArpaAO => "ao".to_string(),
            ArpabetSymbols::ArpaAW => "aw".to_string(),
            ArpabetSymbols::ArpaAY => "ay".to_string(),
            ArpabetSymbols::ArpaEH => "eh".to_string(),
            ArpabetSymbols::ArpaER => "er".to_string(),
            ArpabetSymbols::ArpaEY => "ey".to_string(),
            ArpabetSymbols::ArpaIH => "ih".to_string(),
            ArpabetSymbols::ArpaIY => "iy".to_string(),
            ArpabetSymbols::ArpaOW => "ow".to_string(),
            ArpabetSymbols::ArpaOY => "oy".to_string(),
            ArpabetSymbols::ArpaUH => "uh".to_string(),
            ArpabetSymbols::ArpaUW => "uw".to_string(),
            _ => "".to_string(),
        }
    }
}

pub fn match_arpabet(s: &str) -> Option<ArpabetSymbols> {
    match s.to_lowercase().as_str() {
        // Vowels
        "aa" => Some(ArpabetSymbols::ArpaAA),
        "ae" => Some(ArpabetSymbols::ArpaAE),
        "ah" => Some(ArpabetSymbols::ArpaAH),
        "ao" => Some(ArpabetSymbols::ArpaAO),
        "aw" => Some(ArpabetSymbols::ArpaAW),
        "ay" => Some(ArpabetSymbols::ArpaAY),
        "eh" => Some(ArpabetSymbols::ArpaEH),
        "er" => Some(ArpabetSymbols::ArpaER),
        "ey" => Some(ArpabetSymbols::ArpaEY),
        "ih" => Some(ArpabetSymbols::ArpaIH),
        "iy" => Some(ArpabetSymbols::ArpaIY),
        "ow" => Some(ArpabetSymbols::ArpaOW),
        "oy" => Some(ArpabetSymbols::ArpaOY),
        "uh" => Some(ArpabetSymbols::ArpaUH),
        "uw" => Some(ArpabetSymbols::ArpaUW),

        // Consonants
        "b" => Some(ArpabetSymbols::ArpaB),
        "ch" => Some(ArpabetSymbols::ArpaCH),
        "d" => Some(ArpabetSymbols::ArpaD),
        "dh" => Some(ArpabetSymbols::ArpaDH),
        "dx" => Some(ArpabetSymbols::ArpaDX),
        "el" => Some(ArpabetSymbols::ArpaEL),
        "em" => Some(ArpabetSymbols::ArpaEM),
        "en" => Some(ArpabetSymbols::ArpaEN),
        "f" => Some(ArpabetSymbols::ArpaF),
        "g" => Some(ArpabetSymbols::ArpaG),
        "hh" => Some(ArpabetSymbols::ArpaHH),
        "jh" => Some(ArpabetSymbols::ArpaJH),
        "k" => Some(ArpabetSymbols::ArpaK),
        "l" => Some(ArpabetSymbols::ArpaL),
        "m" => Some(ArpabetSymbols::ArpaM),
        "n" => Some(ArpabetSymbols::ArpaN),
        "ng" => Some(ArpabetSymbols::ArpaNG),
        "p" => Some(ArpabetSymbols::ArpaP),
        "q" => Some(ArpabetSymbols::ArpaQ),
        "r" => Some(ArpabetSymbols::ArpaR),
        "s" => Some(ArpabetSymbols::ArpaS),
        "sh" => Some(ArpabetSymbols::ArpaSH),
        "t" => Some(ArpabetSymbols::ArpaT),
        "th" => Some(ArpabetSymbols::ArpaTH),
        "v" => Some(ArpabetSymbols::ArpaV),
        "w" => Some(ArpabetSymbols::ArpaW),
        "wh" => Some(ArpabetSymbols::ArpaWH),
        "y" => Some(ArpabetSymbols::ArpaY),
        "z" => Some(ArpabetSymbols::ArpaZ),
        "zh" => Some(ArpabetSymbols::ArpaZH),

        _ => None,
    }
}

impl std::fmt::Display for ArpabetSymbols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
