pub enum Arpabet {
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
}

impl Arpabet {
    pub fn as_str(&self) -> String {
        match self {
            Arpabet::ArpaAA => "aa".to_string(),
            Arpabet::ArpaAE => "ae".to_string(),
            Arpabet::ArpaAH => "ah".to_string(),
            Arpabet::ArpaAO => "ao".to_string(),
            Arpabet::ArpaAW => "aw".to_string(),
            Arpabet::ArpaAY => "ay".to_string(),
            Arpabet::ArpaEH => "eh".to_string(),
            Arpabet::ArpaER => "er".to_string(),
            Arpabet::ArpaEY => "ey".to_string(),
            Arpabet::ArpaIH => "ih".to_string(),
            Arpabet::ArpaIY => "iy".to_string(),
            Arpabet::ArpaOW => "ow".to_string(),
            Arpabet::ArpaOY => "oy".to_string(),
            Arpabet::ArpaUH => "uh".to_string(),
            Arpabet::ArpaUW => "uw".to_string(),
        }
    }
}

pub fn match_arpabet(s: &str) -> Option<Arpabet> {
    match s.to_lowercase().as_str() {
        "aa" => Some(Arpabet::ArpaAA),
        "ae" => Some(Arpabet::ArpaAE),
        "ah" => Some(Arpabet::ArpaAH),
        "ao" => Some(Arpabet::ArpaAO),
        "aw" => Some(Arpabet::ArpaAW),
        "ay" => Some(Arpabet::ArpaAY),
        "eh" => Some(Arpabet::ArpaEH),
        "er" => Some(Arpabet::ArpaER),
        "ey" => Some(Arpabet::ArpaEY),
        "ih" => Some(Arpabet::ArpaIH),
        "iy" => Some(Arpabet::ArpaIY),
        "ow" => Some(Arpabet::ArpaOW),
        "oy" => Some(Arpabet::ArpaOY),
        "uh" => Some(Arpabet::ArpaUH),
        "uw" => Some(Arpabet::ArpaUW),

        _ => None,
    }
}

impl std::fmt::Display for Arpabet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
