/// Graphemes from the source languages
///
/// A grapheme is a unit of written language. This enum captures the patterns
/// we recognize when tokenizing text, including:
/// - Single letters (a, b, c, ...)
/// - Digraphs (ph, ch, th, sh, ...)
/// - Uppercase variants (for abbreviation detection)
/// - Special characters (spaces, punctuation, etc.)
///
/// Each variant can be converted back to its string form using `as_str()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceGrapheme {
    // Digraphs (English spelling patterns)
    PH,
    PS,
    CH,
    TH,
    SH,
    EE,
    OO,
    ED,
    GH,

    // Trigraphs
    ORE,

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

impl SourceGrapheme {
    /// Convert the grapheme back to its original string representation
    pub fn to_string_rep(&self) -> String {
        match self {
            // Digraphs
            SourceGrapheme::PH => "ph".to_string(),
            SourceGrapheme::PS => "ps".to_string(),
            SourceGrapheme::CH => "ch".to_string(),
            SourceGrapheme::TH => "th".to_string(),
            SourceGrapheme::SH => "sh".to_string(),
            SourceGrapheme::EE => "ee".to_string(),
            SourceGrapheme::OO => "oo".to_string(),
            SourceGrapheme::ED => "ed".to_string(),
            SourceGrapheme::GH => "gh".to_string(),
            SourceGrapheme::ORE => "ore".to_string(),

            // Vowels
            SourceGrapheme::A => "a".to_string(),
            SourceGrapheme::E => "e".to_string(),
            SourceGrapheme::I => "i".to_string(),
            SourceGrapheme::O => "o".to_string(),
            SourceGrapheme::U => "u".to_string(),

            // Consonants
            SourceGrapheme::B => "b".to_string(),
            SourceGrapheme::C => "c".to_string(),
            SourceGrapheme::D => "d".to_string(),
            SourceGrapheme::F => "f".to_string(),
            SourceGrapheme::G => "g".to_string(),
            SourceGrapheme::H => "h".to_string(),
            SourceGrapheme::J => "j".to_string(),
            SourceGrapheme::K => "k".to_string(),
            SourceGrapheme::L => "l".to_string(),
            SourceGrapheme::M => "m".to_string(),
            SourceGrapheme::N => "n".to_string(),
            SourceGrapheme::P => "p".to_string(),
            SourceGrapheme::Q => "q".to_string(),
            SourceGrapheme::R => "r".to_string(),
            SourceGrapheme::S => "s".to_string(),
            SourceGrapheme::T => "t".to_string(),
            SourceGrapheme::V => "v".to_string(),
            SourceGrapheme::W => "w".to_string(),
            SourceGrapheme::X => "x".to_string(),
            SourceGrapheme::Y => "y".to_string(),
            SourceGrapheme::Z => "z".to_string(),

            // Uppercase
            SourceGrapheme::UpperA => "A".to_string(),
            SourceGrapheme::UpperB => "B".to_string(),
            SourceGrapheme::UpperC => "C".to_string(),
            SourceGrapheme::UpperD => "D".to_string(),
            SourceGrapheme::UpperE => "E".to_string(),
            SourceGrapheme::UpperF => "F".to_string(),
            SourceGrapheme::UpperG => "G".to_string(),
            SourceGrapheme::UpperH => "H".to_string(),
            SourceGrapheme::UpperI => "I".to_string(),
            SourceGrapheme::UpperJ => "J".to_string(),
            SourceGrapheme::UpperK => "K".to_string(),
            SourceGrapheme::UpperL => "L".to_string(),
            SourceGrapheme::UpperM => "M".to_string(),
            SourceGrapheme::UpperN => "N".to_string(),
            SourceGrapheme::UpperO => "O".to_string(),
            SourceGrapheme::UpperP => "P".to_string(),
            SourceGrapheme::UpperQ => "Q".to_string(),
            SourceGrapheme::UpperR => "R".to_string(),
            SourceGrapheme::UpperS => "S".to_string(),
            SourceGrapheme::UpperT => "T".to_string(),
            SourceGrapheme::UpperU => "U".to_string(),
            SourceGrapheme::UpperV => "V".to_string(),
            SourceGrapheme::UpperW => "W".to_string(),
            SourceGrapheme::UpperX => "X".to_string(),
            SourceGrapheme::UpperY => "Y".to_string(),
            SourceGrapheme::UpperZ => "Z".to_string(),

            // Spanish
            SourceGrapheme::Enye => "ñ".to_string(),

            // Whitespace
            SourceGrapheme::Space => " ".to_string(),

            // ASCII passthrough
            SourceGrapheme::Passthrough(c) => c.to_string(),

            // Other
            SourceGrapheme::Other => "#".to_string(),
        }
    }

    /// Check if this grapheme is an uppercase letter
    pub fn is_uppercase(&self) -> bool {
        matches!(
            self,
            SourceGrapheme::UpperA
                | SourceGrapheme::UpperB
                | SourceGrapheme::UpperC
                | SourceGrapheme::UpperD
                | SourceGrapheme::UpperE
                | SourceGrapheme::UpperF
                | SourceGrapheme::UpperG
                | SourceGrapheme::UpperH
                | SourceGrapheme::UpperI
                | SourceGrapheme::UpperJ
                | SourceGrapheme::UpperK
                | SourceGrapheme::UpperL
                | SourceGrapheme::UpperM
                | SourceGrapheme::UpperN
                | SourceGrapheme::UpperO
                | SourceGrapheme::UpperP
                | SourceGrapheme::UpperQ
                | SourceGrapheme::UpperR
                | SourceGrapheme::UpperS
                | SourceGrapheme::UpperT
                | SourceGrapheme::UpperU
                | SourceGrapheme::UpperV
                | SourceGrapheme::UpperW
                | SourceGrapheme::UpperX
                | SourceGrapheme::UpperY
                | SourceGrapheme::UpperZ
        )
    }

    /// Convert uppercase grapheme to lowercase, returns self if already lowercase or not a letter
    pub fn to_lowercase(&self) -> SourceGrapheme {
        match self {
            SourceGrapheme::UpperA => SourceGrapheme::A,
            SourceGrapheme::UpperB => SourceGrapheme::B,
            SourceGrapheme::UpperC => SourceGrapheme::C,
            SourceGrapheme::UpperD => SourceGrapheme::D,
            SourceGrapheme::UpperE => SourceGrapheme::E,
            SourceGrapheme::UpperF => SourceGrapheme::F,
            SourceGrapheme::UpperG => SourceGrapheme::G,
            SourceGrapheme::UpperH => SourceGrapheme::H,
            SourceGrapheme::UpperI => SourceGrapheme::I,
            SourceGrapheme::UpperJ => SourceGrapheme::J,
            SourceGrapheme::UpperK => SourceGrapheme::K,
            SourceGrapheme::UpperL => SourceGrapheme::L,
            SourceGrapheme::UpperM => SourceGrapheme::M,
            SourceGrapheme::UpperN => SourceGrapheme::N,
            SourceGrapheme::UpperO => SourceGrapheme::O,
            SourceGrapheme::UpperP => SourceGrapheme::P,
            SourceGrapheme::UpperQ => SourceGrapheme::Q,
            SourceGrapheme::UpperR => SourceGrapheme::R,
            SourceGrapheme::UpperS => SourceGrapheme::S,
            SourceGrapheme::UpperT => SourceGrapheme::T,
            SourceGrapheme::UpperU => SourceGrapheme::U,
            SourceGrapheme::UpperV => SourceGrapheme::V,
            SourceGrapheme::UpperW => SourceGrapheme::W,
            SourceGrapheme::UpperX => SourceGrapheme::X,
            SourceGrapheme::UpperY => SourceGrapheme::Y,
            SourceGrapheme::UpperZ => SourceGrapheme::Z,
            _ => self.clone(),
        }
    }

    /// Check if this grapheme represents a vowel sound
    pub fn is_vowel(&self) -> bool {
        matches!(
            self.to_lowercase(),
            SourceGrapheme::A
                | SourceGrapheme::E
                | SourceGrapheme::I
                | SourceGrapheme::O
                | SourceGrapheme::U
                | SourceGrapheme::EE
                | SourceGrapheme::OO
        )
    }

    /// Check if this grapheme represents a digraph
    pub fn is_digraph(&self) -> bool {
        matches!(
            self,
            SourceGrapheme::PH
                | SourceGrapheme::PS
                | SourceGrapheme::CH
                | SourceGrapheme::TH
                | SourceGrapheme::SH
                | SourceGrapheme::EE
                | SourceGrapheme::OO
        )
    }

    /// Check if this grapheme represents a consonant sound
    pub fn is_consonant(&self) -> bool {
        matches!(
            self.to_lowercase(),
            SourceGrapheme::B
                | SourceGrapheme::C
                | SourceGrapheme::D
                | SourceGrapheme::F
                | SourceGrapheme::G
                | SourceGrapheme::H
                | SourceGrapheme::J
                | SourceGrapheme::K
                | SourceGrapheme::L
                | SourceGrapheme::M
                | SourceGrapheme::N
                | SourceGrapheme::P
                | SourceGrapheme::Q
                | SourceGrapheme::R
                | SourceGrapheme::S
                | SourceGrapheme::T
                | SourceGrapheme::V
                | SourceGrapheme::W
                | SourceGrapheme::X
                | SourceGrapheme::Y
                | SourceGrapheme::Z
                | SourceGrapheme::Enye
                | SourceGrapheme::PH
                | SourceGrapheme::PS
                | SourceGrapheme::CH
                | SourceGrapheme::TH
                | SourceGrapheme::SH
        )
    }

    /// Checks if the grapheme is a grapheme whose pronunciation is
    /// variant and unpredictable.
    pub fn is_unpredictable_variant(&self) -> bool {
        match self {
            SourceGrapheme::A
            | SourceGrapheme::E
            | SourceGrapheme::I
            | SourceGrapheme::O
            | SourceGrapheme::U
            | SourceGrapheme::Y => true,
            _ => false,
        }
    }

    /// Create a SourceGrapheme from a single character
    pub fn from_char(c: char) -> SourceGrapheme {
        match c {
            // Uppercase vowels
            'A' => SourceGrapheme::UpperA,
            'E' => SourceGrapheme::UpperE,
            'I' => SourceGrapheme::UpperI,
            'O' => SourceGrapheme::UpperO,
            'U' => SourceGrapheme::UpperU,

            // Uppercase consonants
            'B' => SourceGrapheme::UpperB,
            'C' => SourceGrapheme::UpperC,
            'D' => SourceGrapheme::UpperD,
            'F' => SourceGrapheme::UpperF,
            'G' => SourceGrapheme::UpperG,
            'H' => SourceGrapheme::UpperH,
            'J' => SourceGrapheme::UpperJ,
            'K' => SourceGrapheme::UpperK,
            'L' => SourceGrapheme::UpperL,
            'M' => SourceGrapheme::UpperM,
            'N' => SourceGrapheme::UpperN,
            'P' => SourceGrapheme::UpperP,
            'Q' => SourceGrapheme::UpperQ,
            'R' => SourceGrapheme::UpperR,
            'S' => SourceGrapheme::UpperS,
            'T' => SourceGrapheme::UpperT,
            'V' => SourceGrapheme::UpperV,
            'W' => SourceGrapheme::UpperW,
            'X' => SourceGrapheme::UpperX,
            'Y' => SourceGrapheme::UpperY,
            'Z' => SourceGrapheme::UpperZ,

            // Lowercase vowels
            'a' => SourceGrapheme::A,
            'e' => SourceGrapheme::E,
            'i' => SourceGrapheme::I,
            'o' => SourceGrapheme::O,
            'u' => SourceGrapheme::U,

            // Lowercase consonants
            'b' => SourceGrapheme::B,
            'c' => SourceGrapheme::C,
            'd' => SourceGrapheme::D,
            'f' => SourceGrapheme::F,
            'g' => SourceGrapheme::G,
            'h' => SourceGrapheme::H,
            'j' => SourceGrapheme::J,
            'k' => SourceGrapheme::K,
            'l' => SourceGrapheme::L,
            'm' => SourceGrapheme::M,
            'n' => SourceGrapheme::N,
            'p' => SourceGrapheme::P,
            'q' => SourceGrapheme::Q,
            'r' => SourceGrapheme::R,
            's' => SourceGrapheme::S,
            't' => SourceGrapheme::T,
            'v' => SourceGrapheme::V,
            'w' => SourceGrapheme::W,
            'x' => SourceGrapheme::X,
            'y' => SourceGrapheme::Y,
            'z' => SourceGrapheme::Z,

            // Spanish
            'ñ' | 'Ñ' => SourceGrapheme::Enye,

            // Whitespace
            ' ' => SourceGrapheme::Space,

            // ASCII non-alphanumeric passthrough
            c if c.is_ascii() && !c.is_ascii_alphabetic() => {
                SourceGrapheme::Passthrough(c.to_string())
            }

            // Non-ASCII or unknown
            _ => SourceGrapheme::Other,
        }
    }
}

/// Match a 3-character string to a trigraph
pub fn match_trigraph(s: &str) -> Option<SourceGrapheme> {
    match s.to_lowercase().as_str() {
        "ore" => Some(SourceGrapheme::ORE),

        _ => None,
    }
}

/// Match a 2-character string to a digraph grapheme
///
/// Recognizes digraphs (two-letter combinations that represent
/// a single sound or pattern).
///
/// # Arguments
///
/// * `s` - A 2-character string to match (case-insensitive)
///
/// # Returns
///
/// Returns `Some(SourceGrapheme)` if the string matches a known digraph.
/// Returns `None` if no match is found.
pub fn match_digraph(s: &str) -> Option<SourceGrapheme> {
    match s.to_lowercase().as_str() {
        "ph" => Some(SourceGrapheme::PH),
        "ch" => Some(SourceGrapheme::CH),
        "th" => Some(SourceGrapheme::TH),
        "sh" => Some(SourceGrapheme::SH),
        "ee" => Some(SourceGrapheme::EE),
        "oo" => Some(SourceGrapheme::OO),
        "ed" => Some(SourceGrapheme::ED),
        "gh" => Some(SourceGrapheme::GH),

        _ => None,
    }
}

impl std::fmt::Display for SourceGrapheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_rep())
    }
}
