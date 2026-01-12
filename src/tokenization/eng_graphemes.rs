/// Represents an input grapheme from English orthography
///
/// A grapheme is a unit of written language. This enum captures the patterns
/// we recognize when tokenizing English text, including:
/// - Single letters (a, b, c, ...)
/// - Bigraphs (ph, ch, th, sh, ...)
/// - Uppercase variants (for abbreviation detection)
/// - Special characters (spaces, punctuation, etc.)
///
/// Each variant can be converted back to its string form using `as_str()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnglishGrapheme {
    // Bigraphs (English spelling patterns)
    PH,
    PS,
    CH,
    TH,
    SH,
    EE,
    OO,

    // Vowels
    A,
    E,
    I,
    O,
    U,

    // Consonants
    B,
    C,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    M,
    N,
    P,
    Q,
    R,
    S,
    T,
    V,
    W,
    X,
    Y,
    Z,

    // Uppercase variants (for abbreviation detection)
    UpperA,
    UpperB,
    UpperC,
    UpperD,
    UpperE,
    UpperF,
    UpperG,
    UpperH,
    UpperI,
    UpperJ,
    UpperK,
    UpperL,
    UpperM,
    UpperN,
    UpperO,
    UpperP,
    UpperQ,
    UpperR,
    UpperS,
    UpperT,
    UpperU,
    UpperV,
    UpperW,
    UpperX,
    UpperY,
    UpperZ,

    // Spanish
    Enye,

    // Whitespace
    Space,

    // ASCII non-alphanumeric passthrough (digits, punctuation, etc.)
    Passthrough(String),

    // Non-ASCII or unknown
    Other,
}

impl EnglishGrapheme {
    /// Convert the grapheme back to its original string representation
    pub fn as_str(&self) -> String {
        match self {
            // Bigraphs
            EnglishGrapheme::PH => "ph".to_string(),
            EnglishGrapheme::PS => "ps".to_string(),
            EnglishGrapheme::CH => "ch".to_string(),
            EnglishGrapheme::TH => "th".to_string(),
            EnglishGrapheme::SH => "sh".to_string(),
            EnglishGrapheme::EE => "ee".to_string(),
            EnglishGrapheme::OO => "oo".to_string(),

            // Vowels
            EnglishGrapheme::A => "a".to_string(),
            EnglishGrapheme::E => "e".to_string(),
            EnglishGrapheme::I => "i".to_string(),
            EnglishGrapheme::O => "o".to_string(),
            EnglishGrapheme::U => "u".to_string(),

            // Consonants
            EnglishGrapheme::B => "b".to_string(),
            EnglishGrapheme::C => "c".to_string(),
            EnglishGrapheme::D => "d".to_string(),
            EnglishGrapheme::F => "f".to_string(),
            EnglishGrapheme::G => "g".to_string(),
            EnglishGrapheme::H => "h".to_string(),
            EnglishGrapheme::J => "j".to_string(),
            EnglishGrapheme::K => "k".to_string(),
            EnglishGrapheme::L => "l".to_string(),
            EnglishGrapheme::M => "m".to_string(),
            EnglishGrapheme::N => "n".to_string(),
            EnglishGrapheme::P => "p".to_string(),
            EnglishGrapheme::Q => "q".to_string(),
            EnglishGrapheme::R => "r".to_string(),
            EnglishGrapheme::S => "s".to_string(),
            EnglishGrapheme::T => "t".to_string(),
            EnglishGrapheme::V => "v".to_string(),
            EnglishGrapheme::W => "w".to_string(),
            EnglishGrapheme::X => "x".to_string(),
            EnglishGrapheme::Y => "y".to_string(),
            EnglishGrapheme::Z => "z".to_string(),

            // Uppercase
            EnglishGrapheme::UpperA => "A".to_string(),
            EnglishGrapheme::UpperB => "B".to_string(),
            EnglishGrapheme::UpperC => "C".to_string(),
            EnglishGrapheme::UpperD => "D".to_string(),
            EnglishGrapheme::UpperE => "E".to_string(),
            EnglishGrapheme::UpperF => "F".to_string(),
            EnglishGrapheme::UpperG => "G".to_string(),
            EnglishGrapheme::UpperH => "H".to_string(),
            EnglishGrapheme::UpperI => "I".to_string(),
            EnglishGrapheme::UpperJ => "J".to_string(),
            EnglishGrapheme::UpperK => "K".to_string(),
            EnglishGrapheme::UpperL => "L".to_string(),
            EnglishGrapheme::UpperM => "M".to_string(),
            EnglishGrapheme::UpperN => "N".to_string(),
            EnglishGrapheme::UpperO => "O".to_string(),
            EnglishGrapheme::UpperP => "P".to_string(),
            EnglishGrapheme::UpperQ => "Q".to_string(),
            EnglishGrapheme::UpperR => "R".to_string(),
            EnglishGrapheme::UpperS => "S".to_string(),
            EnglishGrapheme::UpperT => "T".to_string(),
            EnglishGrapheme::UpperU => "U".to_string(),
            EnglishGrapheme::UpperV => "V".to_string(),
            EnglishGrapheme::UpperW => "W".to_string(),
            EnglishGrapheme::UpperX => "X".to_string(),
            EnglishGrapheme::UpperY => "Y".to_string(),
            EnglishGrapheme::UpperZ => "Z".to_string(),

            // Spanish
            EnglishGrapheme::Enye => "ñ".to_string(),

            // Whitespace
            EnglishGrapheme::Space => " ".to_string(),

            // ASCII passthrough
            EnglishGrapheme::Passthrough(c) => c.to_string(),

            // Other
            EnglishGrapheme::Other => "#".to_string(),
        }
    }

    /// Check if this grapheme is an uppercase letter
    pub fn is_uppercase(&self) -> bool {
        matches!(
            self,
            EnglishGrapheme::UpperA
                | EnglishGrapheme::UpperB
                | EnglishGrapheme::UpperC
                | EnglishGrapheme::UpperD
                | EnglishGrapheme::UpperE
                | EnglishGrapheme::UpperF
                | EnglishGrapheme::UpperG
                | EnglishGrapheme::UpperH
                | EnglishGrapheme::UpperI
                | EnglishGrapheme::UpperJ
                | EnglishGrapheme::UpperK
                | EnglishGrapheme::UpperL
                | EnglishGrapheme::UpperM
                | EnglishGrapheme::UpperN
                | EnglishGrapheme::UpperO
                | EnglishGrapheme::UpperP
                | EnglishGrapheme::UpperQ
                | EnglishGrapheme::UpperR
                | EnglishGrapheme::UpperS
                | EnglishGrapheme::UpperT
                | EnglishGrapheme::UpperU
                | EnglishGrapheme::UpperV
                | EnglishGrapheme::UpperW
                | EnglishGrapheme::UpperX
                | EnglishGrapheme::UpperY
                | EnglishGrapheme::UpperZ
        )
    }

    /// Convert uppercase grapheme to lowercase, returns self if already lowercase or not a letter
    pub fn to_lowercase(&self) -> EnglishGrapheme {
        match self {
            EnglishGrapheme::UpperA => EnglishGrapheme::A,
            EnglishGrapheme::UpperB => EnglishGrapheme::B,
            EnglishGrapheme::UpperC => EnglishGrapheme::C,
            EnglishGrapheme::UpperD => EnglishGrapheme::D,
            EnglishGrapheme::UpperE => EnglishGrapheme::E,
            EnglishGrapheme::UpperF => EnglishGrapheme::F,
            EnglishGrapheme::UpperG => EnglishGrapheme::G,
            EnglishGrapheme::UpperH => EnglishGrapheme::H,
            EnglishGrapheme::UpperI => EnglishGrapheme::I,
            EnglishGrapheme::UpperJ => EnglishGrapheme::J,
            EnglishGrapheme::UpperK => EnglishGrapheme::K,
            EnglishGrapheme::UpperL => EnglishGrapheme::L,
            EnglishGrapheme::UpperM => EnglishGrapheme::M,
            EnglishGrapheme::UpperN => EnglishGrapheme::N,
            EnglishGrapheme::UpperO => EnglishGrapheme::O,
            EnglishGrapheme::UpperP => EnglishGrapheme::P,
            EnglishGrapheme::UpperQ => EnglishGrapheme::Q,
            EnglishGrapheme::UpperR => EnglishGrapheme::R,
            EnglishGrapheme::UpperS => EnglishGrapheme::S,
            EnglishGrapheme::UpperT => EnglishGrapheme::T,
            EnglishGrapheme::UpperU => EnglishGrapheme::U,
            EnglishGrapheme::UpperV => EnglishGrapheme::V,
            EnglishGrapheme::UpperW => EnglishGrapheme::W,
            EnglishGrapheme::UpperX => EnglishGrapheme::X,
            EnglishGrapheme::UpperY => EnglishGrapheme::Y,
            EnglishGrapheme::UpperZ => EnglishGrapheme::Z,
            _ => self.clone(),
        }
    }

    /// Check if this grapheme represents a vowel sound
    pub fn is_vowel(&self) -> bool {
        matches!(
            self.to_lowercase(),
            EnglishGrapheme::A
                | EnglishGrapheme::E
                | EnglishGrapheme::I
                | EnglishGrapheme::O
                | EnglishGrapheme::U
                | EnglishGrapheme::EE
                | EnglishGrapheme::OO
        )
    }

    /// Check if this grapheme represents a bigraph
    pub fn is_bigraph(&self) -> bool {
        matches!(
            self,
            EnglishGrapheme::PH
                | EnglishGrapheme::PS
                | EnglishGrapheme::CH
                | EnglishGrapheme::TH
                | EnglishGrapheme::SH
                | EnglishGrapheme::EE
                | EnglishGrapheme::OO
        )
    }

    /// Check if this grapheme represents a consonant sound
    pub fn is_consonant(&self) -> bool {
        matches!(
            self.to_lowercase(),
            EnglishGrapheme::B
                | EnglishGrapheme::C
                | EnglishGrapheme::D
                | EnglishGrapheme::F
                | EnglishGrapheme::G
                | EnglishGrapheme::H
                | EnglishGrapheme::J
                | EnglishGrapheme::K
                | EnglishGrapheme::L
                | EnglishGrapheme::M
                | EnglishGrapheme::N
                | EnglishGrapheme::P
                | EnglishGrapheme::Q
                | EnglishGrapheme::R
                | EnglishGrapheme::S
                | EnglishGrapheme::T
                | EnglishGrapheme::V
                | EnglishGrapheme::W
                | EnglishGrapheme::X
                | EnglishGrapheme::Y
                | EnglishGrapheme::Z
                | EnglishGrapheme::Enye
                | EnglishGrapheme::PH
                | EnglishGrapheme::PS
                | EnglishGrapheme::CH
                | EnglishGrapheme::TH
                | EnglishGrapheme::SH
        )
    }

    /// Create a EnglishGrapheme from a single character
    pub fn from_char(c: char) -> EnglishGrapheme {
        match c {
            // Uppercase vowels
            'A' => EnglishGrapheme::UpperA,
            'E' => EnglishGrapheme::UpperE,
            'I' => EnglishGrapheme::UpperI,
            'O' => EnglishGrapheme::UpperO,
            'U' => EnglishGrapheme::UpperU,

            // Uppercase consonants
            'B' => EnglishGrapheme::UpperB,
            'C' => EnglishGrapheme::UpperC,
            'D' => EnglishGrapheme::UpperD,
            'F' => EnglishGrapheme::UpperF,
            'G' => EnglishGrapheme::UpperG,
            'H' => EnglishGrapheme::UpperH,
            'J' => EnglishGrapheme::UpperJ,
            'K' => EnglishGrapheme::UpperK,
            'L' => EnglishGrapheme::UpperL,
            'M' => EnglishGrapheme::UpperM,
            'N' => EnglishGrapheme::UpperN,
            'P' => EnglishGrapheme::UpperP,
            'Q' => EnglishGrapheme::UpperQ,
            'R' => EnglishGrapheme::UpperR,
            'S' => EnglishGrapheme::UpperS,
            'T' => EnglishGrapheme::UpperT,
            'V' => EnglishGrapheme::UpperV,
            'W' => EnglishGrapheme::UpperW,
            'X' => EnglishGrapheme::UpperX,
            'Y' => EnglishGrapheme::UpperY,
            'Z' => EnglishGrapheme::UpperZ,

            // Lowercase vowels
            'a' => EnglishGrapheme::A,
            'e' => EnglishGrapheme::E,
            'i' => EnglishGrapheme::I,
            'o' => EnglishGrapheme::O,
            'u' => EnglishGrapheme::U,

            // Lowercase consonants
            'b' => EnglishGrapheme::B,
            'c' => EnglishGrapheme::C,
            'd' => EnglishGrapheme::D,
            'f' => EnglishGrapheme::F,
            'g' => EnglishGrapheme::G,
            'h' => EnglishGrapheme::H,
            'j' => EnglishGrapheme::J,
            'k' => EnglishGrapheme::K,
            'l' => EnglishGrapheme::L,
            'm' => EnglishGrapheme::M,
            'n' => EnglishGrapheme::N,
            'p' => EnglishGrapheme::P,
            'q' => EnglishGrapheme::Q,
            'r' => EnglishGrapheme::R,
            's' => EnglishGrapheme::S,
            't' => EnglishGrapheme::T,
            'v' => EnglishGrapheme::V,
            'w' => EnglishGrapheme::W,
            'x' => EnglishGrapheme::X,
            'y' => EnglishGrapheme::Y,
            'z' => EnglishGrapheme::Z,

            // Spanish
            'ñ' | 'Ñ' => EnglishGrapheme::Enye,

            // Whitespace
            ' ' => EnglishGrapheme::Space,

            // ASCII non-alphanumeric passthrough
            c if c.is_ascii() && !c.is_ascii_alphabetic() => {
                EnglishGrapheme::Passthrough(c.to_string())
            }

            // Non-ASCII or unknown
            _ => EnglishGrapheme::Other,
        }
    }
}

/// Match a 3-character string to a trigraph
// pub fn match_trigraph(s: &str) -> Option<EnglishGrapheme> {
//     match s.to_lowercase().as_str() {
//         _ => None,
//     }
// }

/// Match a 2-character string to a bigraph grapheme
///
/// Recognizes common English bigraphs (two-letter combinations that represent
/// a single sound or pattern).
///
/// # Arguments
///
/// * `s` - A 2-character string to match (case-insensitive)
///
/// # Returns
///
/// Returns `Some(EnglishGrapheme)` if the string matches a known bigraph.
/// Returns `None` if no match is found.
pub fn match_bigraph(s: &str) -> Option<EnglishGrapheme> {
    match s.to_lowercase().as_str() {
        "ph" => Some(EnglishGrapheme::PH),
        "ch" => Some(EnglishGrapheme::CH),
        "th" => Some(EnglishGrapheme::TH),
        "sh" => Some(EnglishGrapheme::SH),
        "ee" => Some(EnglishGrapheme::EE),
        "oo" => Some(EnglishGrapheme::OO),

        _ => None,
    }
}

impl std::fmt::Display for EnglishGrapheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
