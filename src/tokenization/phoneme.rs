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
    Z, // /z/      - e.g., Zig-Zag

    // Whitespace
    Space,

    // ASCII non-alphanumeric passthrough (digits, punctuation, etc.)
    Passthrough(char),

    // Non-ASCII or unknown
    Other,
}

impl Phoneme {
    /// Convert the phoneme to its Filipino orthographic representation
    pub fn as_str(&self) -> String {
        match self {
            // Affricates [AFF]
            Phoneme::AFFTs => "ts".to_string(),
            Phoneme::AFFDy => "dy".to_string(),

            // Dipthongs [DIP]
            Phoneme::DIPAw => "aw".to_string(),
            Phoneme::DIPAy => "ay".to_string(),
            Phoneme::DIPiw => "iw".to_string(),
            Phoneme::DIPuy => "uy".to_string(),

            // Vowel Sounds
            Phoneme::A => "a".to_string(),
            Phoneme::E => "e".to_string(),
            Phoneme::I => "i".to_string(),
            Phoneme::O => "o".to_string(),
            Phoneme::U => "u".to_string(),

            // Native consonants sounds
            // Stops
            Phoneme::P => "p".to_string(),
            Phoneme::B => "b".to_string(),
            Phoneme::T => "t".to_string(),
            Phoneme::D => "d".to_string(),
            Phoneme::K => "k".to_string(),
            Phoneme::G => "g".to_string(),

            // Nasals
            Phoneme::M => "m".to_string(),
            Phoneme::N => "n".to_string(),
            Phoneme::Ng => "ng".to_string(),
            Phoneme::Ny => "ny".to_string(),

            // Frivatives
            Phoneme::H => "h".to_string(),
            Phoneme::S => "s".to_string(),

            // Approximants
            Phoneme::L => "l".to_string(),
            Phoneme::R => "r".to_string(),
            Phoneme::W => "w".to_string(),
            Phoneme::Y => "y".to_string(),

            // Modern consonant sounds
            Phoneme::F => "f".to_string(),
            Phoneme::Z => "z".to_string(),

            // Whitespace
            Phoneme::Space => " ".to_string(),

            // ASCII passthrough
            Phoneme::Passthrough(c) => c.to_string(),

            // Other
            Phoneme::Other => "#".to_string(),
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
        !self.is_vowel()
            && !matches!(
                self,
                Phoneme::Other | Phoneme::Space | Phoneme::Passthrough(_)
            )
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
