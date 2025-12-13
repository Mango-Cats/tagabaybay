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

    // Non-alphabetic or unknown
    Other,
}

impl Grapheme {
    /// Convert the grapheme back to its original string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            // Bigraphs
            Grapheme::Ph => "ph",
            Grapheme::Ps => "ps",
            Grapheme::Ch => "ch",
            Grapheme::Th => "th",
            Grapheme::Sh => "sh",
            Grapheme::Ee => "ee",
            Grapheme::Oo => "oo",

            // Vowels
            Grapheme::A => "a",
            Grapheme::E => "e",
            Grapheme::I => "i",
            Grapheme::O => "o",
            Grapheme::U => "u",

            // Consonants
            Grapheme::B => "b",
            Grapheme::C => "c",
            Grapheme::D => "d",
            Grapheme::F => "f",
            Grapheme::G => "g",
            Grapheme::H => "h",
            Grapheme::J => "j",
            Grapheme::K => "k",
            Grapheme::L => "l",
            Grapheme::M => "m",
            Grapheme::N => "n",
            Grapheme::P => "p",
            Grapheme::Q => "q",
            Grapheme::R => "r",
            Grapheme::S => "s",
            Grapheme::T => "t",
            Grapheme::V => "v",
            Grapheme::W => "w",
            Grapheme::X => "x",
            Grapheme::Y => "y",
            Grapheme::Z => "z",

            // Spanish
            Grapheme::Enye => "ñ",

            // Other
            Grapheme::Other => "#",
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

            // Other
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