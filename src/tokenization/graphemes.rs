/// Represents an input grapheme from English orthography
/// These are patterns we recognize when tokenizing English text
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grapheme {
    // Bigraphs (English spelling patterns)
    Ph,
    Ps,
    Ch,
    Th,
    Sh,
    Ee,
    Oo,

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

    // Spanish
    Enye,

    // Whitespace
    Space,

    // ASCII non-alphanumeric passthrough (digits, punctuation, etc.)
    Passthrough(char),

    // Non-ASCII or unknown
    Other,
}

impl Grapheme {
    /// Convert the grapheme back to its original string representation
    pub fn as_str(&self) -> String {
        match self {
            // Bigraphs
            Grapheme::Ph => "ph".to_string(),
            Grapheme::Ps => "ps".to_string(),
            Grapheme::Ch => "ch".to_string(),
            Grapheme::Th => "th".to_string(),
            Grapheme::Sh => "sh".to_string(),
            Grapheme::Ee => "ee".to_string(),
            Grapheme::Oo => "oo".to_string(),

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

    /// Check if this grapheme represents a vowel sound
    pub fn is_vowel(&self) -> bool {
        matches!(
            self,
            Grapheme::A
                | Grapheme::E
                | Grapheme::I
                | Grapheme::O
                | Grapheme::U
                | Grapheme::Ee
                | Grapheme::Oo
        )
    }

    /// Check if this grapheme represents a consonant sound
    pub fn is_consonant(&self) -> bool {
        !self.is_vowel()
    }

    /// Create a Grapheme from a single character
    pub fn from_char(c: char) -> Grapheme {
        match c.to_ascii_lowercase() {
            // Vowels
            'a' => Grapheme::A,
            'e' => Grapheme::E,
            'i' => Grapheme::I,
            'o' => Grapheme::O,
            'u' => Grapheme::U,

            // Consonants
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
            'ñ' => Grapheme::Enye,

            // Whitespace
            ' ' => Grapheme::Space,

            // ASCII non-alphanumeric passthrough
            c if c.is_ascii() && !c.is_ascii_alphabetic() => Grapheme::Passthrough(c),

            // Non-ASCII or unknown
            _ => Grapheme::Other,
        }
    }
}

/// Match a 3-character string to a trigraph
pub fn match_trigraph(s: &str) -> Option<Grapheme> {
    match s.to_lowercase().as_str() {
        _ => None,
    }
}

/// Match a 2-character string to a bigraph
pub fn match_bigraph(s: &str) -> Option<Grapheme> {
    match s.to_lowercase().as_str() {
        "ph" => Some(Grapheme::Ph),
        "ch" => Some(Grapheme::Ch),
        "th" => Some(Grapheme::Th),
        "sh" => Some(Grapheme::Sh),
        "ee" => Some(Grapheme::Ee),
        "oo" => Some(Grapheme::Oo),
        _ => None,
    }
}

impl std::fmt::Display for Grapheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
