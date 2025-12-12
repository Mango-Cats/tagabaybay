/// Represents a Filipino phoneme (output sound unit)
/// These are the sounds in native Filipino phonology
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phoneme {
    // 2-Phonemes
    Ts, // /ts/     - e.g., (CH/TS)ocolate
    Dy, // /dʒ/     - e.g., (J/DY)eep

    // Vowels Sounds
    A, // /a/      - e.g., Apoy
    E, // /ɛ/      - e.g., Elepante
    I, // /i/      - e.g., Isip
    O, // /o/      - e.g., Okey
    U, // /u/      - e.g., Ulan

    // Native consonants sounds
    B,  // /b/      - e.g., gaBay
    D,  // /d/      - e.g., lakaD
    G,  // /g/      - e.g., puGot
    H,  // /h/      - e.g., aHoy
    K,  // /k/      - e.g., Kanta
    L,  // /l/      - e.g., aLam
    M,  // /m/      - e.g., alaM
    N,  // /n/      - e.g., siNturon
    Ny, // /ɲ/      - e.g., niÑo
    Ng, // /ŋ/      - e.g., NGiti
    P,  // /p/      - e.g., isiP
    R,  // /ɾ/      - e.g., duRa
    S,  // /s/      - e.g., iSip
    T,  // /t/      - e.g., Tula
    W,  // /w/      - e.g., Walis
    Y,  // /j/      - e.g., Yap or Juan

    // Modern consonants sounds
    F, // /f/      - e.g., Filipino

    // Other
    Other,
}

impl Phoneme {
    /// Convert the phoneme to its Filipino orthographic representation
    pub fn as_str(&self) -> &'static str {
        match self {
            // 2-Phonemes
            Phoneme::Ts => "ts",
            Phoneme::Dy => "dy",

            // Vowel Sounds
            Phoneme::A => "a",
            Phoneme::E => "e",
            Phoneme::I => "i",
            Phoneme::O => "o",
            Phoneme::U => "u",

            // Native consonants sounds
            Phoneme::B => "b",
            Phoneme::D => "d",
            Phoneme::G => "g",
            Phoneme::H => "h",
            Phoneme::K => "k",
            Phoneme::L => "l",
            Phoneme::M => "m",
            Phoneme::N => "n",
            Phoneme::Ny => "ny",
            Phoneme::Ng => "ng",
            Phoneme::P => "p",
            Phoneme::R => "r",
            Phoneme::S => "s",
            Phoneme::T => "t",
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
