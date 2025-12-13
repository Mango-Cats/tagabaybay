/// Represents a Filipino phoneme (output sound unit)
/// These are the sounds in native Filipino phonology
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phoneme {
    // Affricates [AFF]
    AFFTs, // /ts/     - e.g., (CH/TS)ocolate
    AFFDy, // /dʒ/     - e.g., (J/DY)eep

    // Dipthongs [DIP]
    DIPAw,
    DIPAy,
    DIPiw,
    DIPuy,

    // Vowels Sounds; Monophthongs
    A, // /a/      - e.g., Apoy
    E, // /ɛ/      - e.g., Elepante
    I, // /i/      - e.g., Isip
    O, // /o/      - e.g., Okey
    U, // /u/      - e.g., Ulan

    // Native consonants sounds
    // Stops
    P, // /p/      - e.g., isiP
    B, // /b/      - e.g., gaBay
    T, // /t/      - e.g., Tula
    D, // /d/      - e.g., lakaD
    K, // /k/      - e.g., Kanta
    G, // /g/      - e.g., puGot

    // Nasals
    M,  // /m/      - e.g., alaM
    N,  // /n/      - e.g., siNturon
    Ng, // /ŋ/      - e.g., NGiti
    Ny, // /ɲ/      - e.g., niÑo

    // Fricatives
    H, // /h/      - e.g., aHoy
    S, // /s/      - e.g., iSip

    // Approximants
    L, // /l/      - e.g., aLam
    R, // /ɾ/      - e.g., duRa
    W, // /w/      - e.g., Walis
    Y, // /j/      - e.g., Yap or Juan

    // Modern consonants sounds
    F, // /f/      - e.g., Filipino

    // Other
    Other,
}

impl Phoneme {
    /// Convert the phoneme to its Filipino orthographic representation
    pub fn as_str(&self) -> &'static str {
        match self {
            // Affricates [AFF]
            Phoneme::AFFTs => "ts",
            Phoneme::AFFDy => "dy",

            // Dipthongs [DIP]
            Phoneme::DIPAw => "aw",
            Phoneme::DIPAy => "ay",
            Phoneme::DIPiw => "iw",
            Phoneme::DIPuy => "uy",

            // Vowel Sounds
            Phoneme::A => "a",
            Phoneme::E => "e",
            Phoneme::I => "i",
            Phoneme::O => "o",
            Phoneme::U => "u",

            // Native consonants sounds
            // Stops
            Phoneme::P => "p",
            Phoneme::B => "b",
            Phoneme::T => "t",
            Phoneme::D => "d",
            Phoneme::K => "k",
            Phoneme::G => "g",

            // Nasals
            Phoneme::M => "m",
            Phoneme::N => "n",
            Phoneme::Ng => "ng",
            Phoneme::Ny => "ny",

            // Frivatives
            Phoneme::H => "h",
            Phoneme::S => "s",

            // Approximants
            Phoneme::L => "l",
            Phoneme::R => "r",
            Phoneme::W => "w",
            Phoneme::Y => "y",

            // Modern consonant sounds
            Phoneme::F => "f",

            // Other
            Phoneme::Other => "#",
        }
    }

    /// Check if this phoneme is a vowel
    pub fn is_vowel(&self) -> bool {
        matches!(
            self,
            Phoneme::A | Phoneme::E | Phoneme::I | Phoneme::O | Phoneme::U
        )
    }

    /// Check if this phoneme is a consonant
    pub fn is_consonant(&self) -> bool {
        !self.is_vowel() && !matches!(self, Phoneme::Other)
    }
}

impl std::fmt::Display for Phoneme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Convert a Vec<Phoneme> to a String
pub fn phonemes_to_string(phonemes: &[Phoneme]) -> String {
    phonemes.iter().map(|p| p.to_string()).collect()
}
