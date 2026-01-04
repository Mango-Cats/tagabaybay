/// Represents a Filipino phoneme (output sound unit)
///
/// A phoneme is a unit of sound. This enum captures the sounds in native and
/// modern Filipino phonology, including:
/// - Vowels (a, e, i, o, u)
/// - Consonants (native: p, b, t, d, k, g, m, n, ng, ny, h, s, l, r, w, y)
/// - Modern consonants (f, z)
/// - Affricates (ts, dy)
/// - Special characters (spaces, punctuation)
///
/// Each variant can be converted to Filipino orthography using `as_str()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phoneme {
    // Affricates
    TS, // /ts/  - e.g., (CH/TS)ocolate
    DY, // /dʒ/  - e.g., (J/DY)eep

    // Fricatives
    SH, // /ʃ/   - e.g., (SH/Siy)aron

    // Vowels Sounds; Monophthongs
    A, // /a/       - e.g., Apoy
    E, // /ɛ/       - e.g., Elepante
    I, // /i/       - e.g., Isip
    O, // /o/       - e.g., Okey
    U, // /u/       - e.g., Ulan

    // Native consonants sounds
    // Stops
    P, // /p/       - e.g., isiP
    B, // /b/       - e.g., gaBay
    T, // /t/       - e.g., Tula
    D, // /d/       - e.g., lakaD
    K, // /k/       - e.g., Kanta
    G, // /g/       - e.g., puGot

    // Nasals
    M,  // /m/      - e.g., alaM
    N,  // /n/      - e.g., siNturon
    Ng, // /ŋ/      - e.g., NGiti
    Ny, // /ɲ/      - e.g., niÑo

    // Fricatives
    H, // /h/       - e.g., aHoy
    S, // /s/       - e.g., iSip

    // Approximants
    L, // /l/       - e.g., aLam
    R, // /ɾ/       - e.g., duRa
    W, // /w/       - e.g., Walis
    Y, // /j/       - e.g., Yap or Juan

    // Modern consonants sounds
    F, // /f/       - e.g., Filipino
    Z, // /z/       - e.g., Zig-Zag

    // Whitespace
    Space,

    // ASCII non-alphanumeric passthrough (digits, punctuation, etc.)
    Passthrough(String),

    // Non-ASCII or unknown
    Other,
}

impl Phoneme {
    /// Convert the phoneme to its Filipino orthographic representation
    pub fn as_str(&self) -> String {
        match self {
            // Affricates
            Phoneme::TS => "ts",
            Phoneme::DY => "dy",

            // Fricatives
            Phoneme::SH => "sh",

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
            Phoneme::Z => "z",

            // Whitespace
            Phoneme::Space => " ",

            // ASCII passthrough
            Phoneme::Passthrough(c) => c,

            // Other
            Phoneme::Other => "#",
        }
        .to_string()
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

/// Convert a `Vec<Phoneme>` to a String
///
/// Converts a sequence of phonemes into their Filipino orthographic representation.
///
/// # Arguments
///
/// * `phonemes` - Slice of phonemes to convert
///
/// # Returns
///
/// Returns the string representation in Filipino orthography.
pub fn phonemes_to_string(phonemes: &[Phoneme]) -> String {
    phonemes.iter().map(|p| p.to_string()).collect()
}

impl std::fmt::Display for Phoneme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
