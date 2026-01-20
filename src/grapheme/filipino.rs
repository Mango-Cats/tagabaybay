/// Represents a Filipino grapheme
///
/// A grapheme is a unit of sound. This enum captures the sounds in native and
/// modern Filipino phonology, including:
/// - Vowels (a, e, i, o, u)
/// - Consonants (native: p, b, t, d, k, g, m, n, ng, ny, h, s, l, r, w, y)
/// - Modern consonants (f, z)
/// - Affricates (ts, dy)
/// - Special characters (spaces, punctuation)
///
/// Each variant can be converted to Filipino orthography using `as_str()`.
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilipinoGrapheme {
    // Affricates
    TS, // /ts/  - e.g., (CH/TS)ocolate
    DY, // /dʒ/  - e.g., (J/DY)eep

    // Fricatives
    // SH/SY depends if it is allowed in the configuration
    SH, // /ʃ/   - e.g., (SH)aron
    SY, // /ʃ/   - e.g., (SY)ampu

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
    J, // /j/       - e.g., sabJek
    V, // /v/       - e.g., Value

    // Whitespace
    Space,

    // ASCII non-alphanumeric passthrough (digits, punctuation, etc.)
    Passthrough(String),

    // Non-ASCII or unknown
    Other,
}

impl FilipinoGrapheme {
    /// Convert the grapheme to its Filipino orthographic representation
    pub fn as_str(&self) -> String {
        match self {
            // Affricates
            FilipinoGrapheme::TS => "ts",
            FilipinoGrapheme::DY => "dy",

            // Fricatives
            FilipinoGrapheme::SH => "sh",
            FilipinoGrapheme::SY => "sy",

            // Vowel Sounds
            FilipinoGrapheme::A => "a",
            FilipinoGrapheme::E => "e",
            FilipinoGrapheme::I => "i",
            FilipinoGrapheme::O => "o",
            FilipinoGrapheme::U => "u",

            // Native consonants sounds
            // Stops
            FilipinoGrapheme::P => "p",
            FilipinoGrapheme::B => "b",
            FilipinoGrapheme::T => "t",
            FilipinoGrapheme::D => "d",
            FilipinoGrapheme::K => "k",
            FilipinoGrapheme::G => "g",

            // Nasals
            FilipinoGrapheme::M => "m",
            FilipinoGrapheme::N => "n",
            FilipinoGrapheme::Ng => "ng",
            FilipinoGrapheme::Ny => "ny",

            // Frivatives
            FilipinoGrapheme::H => "h",
            FilipinoGrapheme::S => "s",

            // Approximants
            FilipinoGrapheme::L => "l",
            FilipinoGrapheme::R => "r",
            FilipinoGrapheme::W => "w",
            FilipinoGrapheme::Y => "y",

            // Modern consonant sounds
            FilipinoGrapheme::F => "f",
            FilipinoGrapheme::Z => "z",
            FilipinoGrapheme::J => "j",
            FilipinoGrapheme::V => "v",

            // Whitespace
            FilipinoGrapheme::Space => " ",

            // ASCII passthrough
            FilipinoGrapheme::Passthrough(c) => c,

            // Other
            FilipinoGrapheme::Other => "#",
        }
        .to_string()
    }

    /// Check if this grapheme is a vowel
    pub fn is_vowel(&self) -> bool {
        matches!(
            self,
            FilipinoGrapheme::A
                | FilipinoGrapheme::E
                | FilipinoGrapheme::I
                | FilipinoGrapheme::O
                | FilipinoGrapheme::U
        )
    }

    /// Check if this grapheme is a consonant
    pub fn is_consonant(&self) -> bool {
        !self.is_vowel()
            && !matches!(
                self,
                FilipinoGrapheme::Other
                    | FilipinoGrapheme::Space
                    | FilipinoGrapheme::Passthrough(_)
            )
    }
}

/// Convert a `Vec<FilipinoGrapheme>` to a String
///
/// Converts a sequence of graphemes into their Filipino orthographic representation.
///
/// # Arguments
///
/// * `graphemes` - Slice of graphemes to convert
///
/// # Returns
///
/// Returns the string representation in Filipino orthography.
pub fn phl_graphemes_to_string(graphemes: &[FilipinoGrapheme]) -> String {
    graphemes.iter().map(|p| p.to_string()).collect()
}

impl std::fmt::Display for FilipinoGrapheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Convert syllables (Vec of grapheme Vecs) to a hyphenated string representation
///
/// # Arguments
///
/// * `syllables` - Slice of syllables, where each syllable is a Vec of FilipinoGrapheme
///
/// # Returns
///
/// Returns a hyphenated string like "buk-san" or "eks-tra-di-syon"
///
/// # Example
///
/// ```ignore
/// use tagabaybay::grapheme::filipino::{syllables_to_string, FilipinoGrapheme::*};
///
/// let syllables = tokens![tokens![B, U, K], tokens![S, A, N]];
/// assert_eq!(syllables_to_string(&syllables), "buk-san");
/// ```
pub fn hyphenate(syllables: &[Vec<FilipinoGrapheme>]) -> String {
    syllables
        .iter()
        .map(|syl| syl.iter().map(FilipinoGrapheme::as_str).join(""))
        .join("-")
}
