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
pub enum Grapheme {
    // Bigraphs (English spelling patterns)
    BigraphPh,
    BigraphPs,
    BigraphCh,
    BigraphTh,
    BigraphSh,
    BigraphEe,
    BigraphOo,

    //ArpaBet
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

impl Grapheme {
    /// Convert the grapheme back to its original string representation
    pub fn as_str(&self) -> String {
        match self {
            // Bigraphs
            Grapheme::BigraphPh => "ph".to_string(),
            Grapheme::BigraphPs => "ps".to_string(),
            Grapheme::BigraphCh => "ch".to_string(),
            Grapheme::BigraphTh => "th".to_string(),
            Grapheme::BigraphSh => "sh".to_string(),
            Grapheme::BigraphEe => "ee".to_string(),
            Grapheme::BigraphOo => "oo".to_string(),

            //ArpaBet
            Grapheme::ArpaAA => "aa".to_string(),
            Grapheme::ArpaAE => "ae".to_string(),
            Grapheme::ArpaAH => "ah".to_string(),
            Grapheme::ArpaAO => "ao".to_string(),
            Grapheme::ArpaAW => "aw".to_string(),
            Grapheme::ArpaAY => "ay".to_string(),
            Grapheme::ArpaEH => "eh".to_string(),
            Grapheme::ArpaER => "er".to_string(),
            Grapheme::ArpaEY => "ey".to_string(),
            Grapheme::ArpaIH => "ih".to_string(),
            Grapheme::ArpaIY => "iy".to_string(),
            Grapheme::ArpaOW => "ow".to_string(),
            Grapheme::ArpaOY => "oy".to_string(),
            Grapheme::ArpaUH => "uh".to_string(),
            Grapheme::ArpaUW => "uw".to_string(),

            // Vowels
            Grapheme::A => "a".to_string(),
            Grapheme::E => "e".to_string(),
            Grapheme::I => "i".to_string(),
            Grapheme::O => "o".to_string(),
            Grapheme::U => "u".to_string(),

            // Consonants
            Grapheme::B => "b".to_string(),
            Grapheme::C => "c".to_string(),
            Grapheme::D => "d".to_string(),
            Grapheme::F => "f".to_string(),
            Grapheme::G => "g".to_string(),
            Grapheme::H => "h".to_string(),
            Grapheme::J => "j".to_string(),
            Grapheme::K => "k".to_string(),
            Grapheme::L => "l".to_string(),
            Grapheme::M => "m".to_string(),
            Grapheme::N => "n".to_string(),
            Grapheme::P => "p".to_string(),
            Grapheme::Q => "q".to_string(),
            Grapheme::R => "r".to_string(),
            Grapheme::S => "s".to_string(),
            Grapheme::T => "t".to_string(),
            Grapheme::V => "v".to_string(),
            Grapheme::W => "w".to_string(),
            Grapheme::X => "x".to_string(),
            Grapheme::Y => "y".to_string(),
            Grapheme::Z => "z".to_string(),

            // Uppercase
            Grapheme::UpperA => "A".to_string(),
            Grapheme::UpperB => "B".to_string(),
            Grapheme::UpperC => "C".to_string(),
            Grapheme::UpperD => "D".to_string(),
            Grapheme::UpperE => "E".to_string(),
            Grapheme::UpperF => "F".to_string(),
            Grapheme::UpperG => "G".to_string(),
            Grapheme::UpperH => "H".to_string(),
            Grapheme::UpperI => "I".to_string(),
            Grapheme::UpperJ => "J".to_string(),
            Grapheme::UpperK => "K".to_string(),
            Grapheme::UpperL => "L".to_string(),
            Grapheme::UpperM => "M".to_string(),
            Grapheme::UpperN => "N".to_string(),
            Grapheme::UpperO => "O".to_string(),
            Grapheme::UpperP => "P".to_string(),
            Grapheme::UpperQ => "Q".to_string(),
            Grapheme::UpperR => "R".to_string(),
            Grapheme::UpperS => "S".to_string(),
            Grapheme::UpperT => "T".to_string(),
            Grapheme::UpperU => "U".to_string(),
            Grapheme::UpperV => "V".to_string(),
            Grapheme::UpperW => "W".to_string(),
            Grapheme::UpperX => "X".to_string(),
            Grapheme::UpperY => "Y".to_string(),
            Grapheme::UpperZ => "Z".to_string(),

            // Spanish
            Grapheme::Enye => "ñ".to_string(),

            // Whitespace
            Grapheme::Space => " ".to_string(),

            // ASCII passthrough
            Grapheme::Passthrough(c) => c.to_string(),

            // Other
            Grapheme::Other => "#".to_string(),
        }
    }

    /// Check if this grapheme is an uppercase letter
    pub fn is_uppercase(&self) -> bool {
        matches!(
            self,
            Grapheme::UpperA
                | Grapheme::UpperB
                | Grapheme::UpperC
                | Grapheme::UpperD
                | Grapheme::UpperE
                | Grapheme::UpperF
                | Grapheme::UpperG
                | Grapheme::UpperH
                | Grapheme::UpperI
                | Grapheme::UpperJ
                | Grapheme::UpperK
                | Grapheme::UpperL
                | Grapheme::UpperM
                | Grapheme::UpperN
                | Grapheme::UpperO
                | Grapheme::UpperP
                | Grapheme::UpperQ
                | Grapheme::UpperR
                | Grapheme::UpperS
                | Grapheme::UpperT
                | Grapheme::UpperU
                | Grapheme::UpperV
                | Grapheme::UpperW
                | Grapheme::UpperX
                | Grapheme::UpperY
                | Grapheme::UpperZ
        )
    }

    /// Convert uppercase grapheme to lowercase, returns self if already lowercase or not a letter
    pub fn to_lowercase(&self) -> Grapheme {
        match self {
            Grapheme::UpperA => Grapheme::A,
            Grapheme::UpperB => Grapheme::B,
            Grapheme::UpperC => Grapheme::C,
            Grapheme::UpperD => Grapheme::D,
            Grapheme::UpperE => Grapheme::E,
            Grapheme::UpperF => Grapheme::F,
            Grapheme::UpperG => Grapheme::G,
            Grapheme::UpperH => Grapheme::H,
            Grapheme::UpperI => Grapheme::I,
            Grapheme::UpperJ => Grapheme::J,
            Grapheme::UpperK => Grapheme::K,
            Grapheme::UpperL => Grapheme::L,
            Grapheme::UpperM => Grapheme::M,
            Grapheme::UpperN => Grapheme::N,
            Grapheme::UpperO => Grapheme::O,
            Grapheme::UpperP => Grapheme::P,
            Grapheme::UpperQ => Grapheme::Q,
            Grapheme::UpperR => Grapheme::R,
            Grapheme::UpperS => Grapheme::S,
            Grapheme::UpperT => Grapheme::T,
            Grapheme::UpperU => Grapheme::U,
            Grapheme::UpperV => Grapheme::V,
            Grapheme::UpperW => Grapheme::W,
            Grapheme::UpperX => Grapheme::X,
            Grapheme::UpperY => Grapheme::Y,
            Grapheme::UpperZ => Grapheme::Z,
            _ => self.clone(),
        }
    }

    /// Check if this grapheme represents a vowel sound
    pub fn is_vowel(&self) -> bool {
        matches!(
            self.to_lowercase(),
            Grapheme::A
                | Grapheme::E
                | Grapheme::I
                | Grapheme::O
                | Grapheme::U
                | Grapheme::BigraphEe
                | Grapheme::BigraphOo
        )
    }

    /// Check if this grapheme represents a bigraph
    pub fn is_bigraph(&self) -> bool {
        matches!(
            self,
            Grapheme::BigraphPh
                | Grapheme::BigraphPs
                | Grapheme::BigraphCh
                | Grapheme::BigraphTh
                | Grapheme::BigraphSh
                | Grapheme::BigraphEe
                | Grapheme::BigraphOo
        )
    }

    /// Check if this grapheme represents a consonant sound
    pub fn is_consonant(&self) -> bool {
        matches!(
            self.to_lowercase(),
            Grapheme::B
                | Grapheme::C
                | Grapheme::D
                | Grapheme::F
                | Grapheme::G
                | Grapheme::H
                | Grapheme::J
                | Grapheme::K
                | Grapheme::L
                | Grapheme::M
                | Grapheme::N
                | Grapheme::P
                | Grapheme::Q
                | Grapheme::R
                | Grapheme::S
                | Grapheme::T
                | Grapheme::V
                | Grapheme::W
                | Grapheme::X
                | Grapheme::Y
                | Grapheme::Z
                | Grapheme::Enye
                | Grapheme::BigraphPh
                | Grapheme::BigraphPs
                | Grapheme::BigraphCh
                | Grapheme::BigraphTh
                | Grapheme::BigraphSh
        )
    }

    /// Create a Grapheme from a single character
    pub fn from_char(c: char) -> Grapheme {
        match c {
            // Uppercase vowels
            'A' => Grapheme::UpperA,
            'E' => Grapheme::UpperE,
            'I' => Grapheme::UpperI,
            'O' => Grapheme::UpperO,
            'U' => Grapheme::UpperU,

            // Uppercase consonants
            'B' => Grapheme::UpperB,
            'C' => Grapheme::UpperC,
            'D' => Grapheme::UpperD,
            'F' => Grapheme::UpperF,
            'G' => Grapheme::UpperG,
            'H' => Grapheme::UpperH,
            'J' => Grapheme::UpperJ,
            'K' => Grapheme::UpperK,
            'L' => Grapheme::UpperL,
            'M' => Grapheme::UpperM,
            'N' => Grapheme::UpperN,
            'P' => Grapheme::UpperP,
            'Q' => Grapheme::UpperQ,
            'R' => Grapheme::UpperR,
            'S' => Grapheme::UpperS,
            'T' => Grapheme::UpperT,
            'V' => Grapheme::UpperV,
            'W' => Grapheme::UpperW,
            'X' => Grapheme::UpperX,
            'Y' => Grapheme::UpperY,
            'Z' => Grapheme::UpperZ,

            // Lowercase vowels
            'a' => Grapheme::A,
            'e' => Grapheme::E,
            'i' => Grapheme::I,
            'o' => Grapheme::O,
            'u' => Grapheme::U,

            // Lowercase consonants
            'b' => Grapheme::B,
            'c' => Grapheme::C,
            'd' => Grapheme::D,
            'f' => Grapheme::F,
            'g' => Grapheme::G,
            'h' => Grapheme::H,
            'j' => Grapheme::J,
            'k' => Grapheme::K,
            'l' => Grapheme::L,
            'm' => Grapheme::M,
            'n' => Grapheme::N,
            'p' => Grapheme::P,
            'q' => Grapheme::Q,
            'r' => Grapheme::R,
            's' => Grapheme::S,
            't' => Grapheme::T,
            'v' => Grapheme::V,
            'w' => Grapheme::W,
            'x' => Grapheme::X,
            'y' => Grapheme::Y,
            'z' => Grapheme::Z,

            // Spanish
            'ñ' | 'Ñ' => Grapheme::Enye,

            // Whitespace
            ' ' => Grapheme::Space,

            // ASCII non-alphanumeric passthrough
            c if c.is_ascii() && !c.is_ascii_alphabetic() => Grapheme::Passthrough(c.to_string()),

            // Non-ASCII or unknown
            _ => Grapheme::Other,
        }
    }
}

/// Match a 3-character string to a trigraph
// pub fn match_trigraph(s: &str) -> Option<Grapheme> {
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
/// Returns `Some(Grapheme)` if the string matches a known bigraph.
/// Returns `None` if no match is found.
pub fn match_bigraph(s: &str) -> Option<Grapheme> {
    match s.to_lowercase().as_str() {
        "ph" => Some(Grapheme::BigraphPh),
        "ch" => Some(Grapheme::BigraphCh),
        "th" => Some(Grapheme::BigraphTh),
        "sh" => Some(Grapheme::BigraphSh),
        "ee" => Some(Grapheme::BigraphEe),
        "oo" => Some(Grapheme::BigraphOo),

        //ARPAbet
        "aa" => Some(Grapheme::ArpaAA),
        "ae" => Some(Grapheme::ArpaAE),
        "ah" => Some(Grapheme::ArpaAH),
        "ao" => Some(Grapheme::ArpaAO),
        "aw" => Some(Grapheme::ArpaAW),
        "ay" => Some(Grapheme::ArpaAY),
        "eh" => Some(Grapheme::ArpaEH),
        "er" => Some(Grapheme::ArpaER),
        "ey" => Some(Grapheme::ArpaEY),
        "ih" => Some(Grapheme::ArpaIH),
        "iy" => Some(Grapheme::ArpaIY),
        "ow" => Some(Grapheme::ArpaOW),
        "oy" => Some(Grapheme::ArpaOY),
        "uh" => Some(Grapheme::ArpaUH),
        "uw" => Some(Grapheme::ArpaUW),

        _ => None,
    }
}

impl std::fmt::Display for Grapheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
