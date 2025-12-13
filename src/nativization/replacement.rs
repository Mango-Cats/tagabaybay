use crate::nativization::error::printe;
use crate::tokenization::graphemes::Grapheme;
use crate::tokenization::phoneme::{Phoneme, phonemes_to_string};
use crate::tokenization::tokenize::tokenize;

use regex::Regex;

/// Convert an input grapheme to output phoneme(s) - context-free replacements
pub fn free_replacement(graphemes: &[Grapheme], index: usize) -> Option<Vec<Phoneme>> {
    match graphemes[index] {
        // Bigraph replacements
        Grapheme::Ph => Some(vec![Phoneme::F]),
        Grapheme::Ps => Some(vec![Phoneme::S]),
        // Ch
        Grapheme::Th => Some(vec![Phoneme::T]),
        Grapheme::Sh => Some(vec![Phoneme::S]),
        Grapheme::Ee => Some(vec![Phoneme::I]),
        Grapheme::Oo => Some(vec![Phoneme::U]),

        // Vowels
        Grapheme::A => Some(vec![Phoneme::A]),
        Grapheme::E => Some(vec![Phoneme::E]),
        Grapheme::I => Some(vec![Phoneme::I]),
        Grapheme::O => Some(vec![Phoneme::O]),
        Grapheme::U => Some(vec![Phoneme::U]),

        // Consonants
        Grapheme::B => Some(vec![Phoneme::B]),
        // C
        Grapheme::D => Some(vec![Phoneme::D]),
        Grapheme::F => Some(vec![Phoneme::F]),
        Grapheme::G => Some(vec![Phoneme::G]),
        Grapheme::H => Some(vec![Phoneme::H]),
        Grapheme::K => Some(vec![Phoneme::K]),
        // J
        // FIXME: this should be a context sensitive rule
        //         Grapheme::J => Some(vec![Phoneme::AFFDy]),
        Grapheme::L => Some(vec![Phoneme::L]),
        Grapheme::M => Some(vec![Phoneme::M]),
        Grapheme::N => Some(vec![Phoneme::N]),
        Grapheme::P => Some(vec![Phoneme::P]),
        // Q
        Grapheme::R => Some(vec![Phoneme::R]),
        Grapheme::S => Some(vec![Phoneme::S]),
        Grapheme::T => Some(vec![Phoneme::T]),
        Grapheme::V => Some(vec![Phoneme::B]),
        Grapheme::W => Some(vec![Phoneme::W]),
        Grapheme::Y => Some(vec![Phoneme::Y]),
        Grapheme::Z => Some(vec![Phoneme::S]),

        // Spanish
        Grapheme::Enye => Some(vec![Phoneme::Ny]),

        // Whitespace
        Grapheme::Space => Some(vec![Phoneme::Space]),

        // ASCII passthrough (digits, punctuation, etc.)
        Grapheme::Passthrough(c) => Some(vec![Phoneme::Passthrough(c)]),

        // Context-sensitive letters (handled in sensitive_replacement)
        // see `sensitive_replacement()`
        Grapheme::C | Grapheme::J | Grapheme::Q | Grapheme::X | Grapheme::Ch => None,

        // Other characters (pass through as-is)
        Grapheme::Other => Some(vec![Phoneme::Other]),
    }
}

/// Context-sensitive nativization (needs surrounding graphemes)
/// Returns None if no context-sensitive rule applies (use free_replacement instead)
pub fn sensitive_replacement(graphemes: &[Grapheme], index: usize) -> Option<Vec<Phoneme>> {
    let curr = &graphemes[index];

    match curr {
        // Vowels
        Grapheme::A | Grapheme::E | Grapheme::I | Grapheme::O | Grapheme::U => {
            sensitive_vowel(graphemes, index)
        }
        // Bigraphs
        Grapheme::Ch | Grapheme::Th | Grapheme::Sh => sensitive_bigraph(graphemes, index),
        // Consonants
        _ => sensitive_consonant(graphemes, index),
    }
}

/// Vowel-specific context-sensitive rules
fn sensitive_vowel(graphemes: &[Grapheme], index: usize) -> Option<Vec<Phoneme>> {
    let curr = &graphemes[index];
    let prev = if index > 0 {
        Some(&graphemes[index - 1])
    } else {
        None
    };
    let next = graphemes.get(index + 1);

    match curr {
        Grapheme::I => match next {
            Some(Grapheme::A) => Some(vec![Phoneme::I, Phoneme::Y, Phoneme::A]),
            Some(Grapheme::E) => Some(vec![Phoneme::I, Phoneme::Y, Phoneme::E]),
            Some(Grapheme::I) => Some(vec![Phoneme::I, Phoneme::Y, Phoneme::I]),
            Some(Grapheme::O) => Some(vec![Phoneme::I, Phoneme::Y, Phoneme::O]),
            Some(Grapheme::U) => Some(vec![Phoneme::I, Phoneme::Y, Phoneme::U]),
            _ => None,
        },

        Grapheme::U => match next {
            Some(Grapheme::A) => Some(vec![Phoneme::U, Phoneme::W, Phoneme::A]),
            Some(Grapheme::E) => Some(vec![Phoneme::U, Phoneme::W, Phoneme::E]),
            Some(Grapheme::I) => Some(vec![Phoneme::U, Phoneme::W, Phoneme::I]),
            Some(Grapheme::O) => Some(vec![Phoneme::U, Phoneme::W, Phoneme::O]),
            Some(Grapheme::U) => Some(vec![Phoneme::U, Phoneme::W, Phoneme::U]),
            _ => match prev {
                Some(Grapheme::E) => Some(vec![Phoneme::Y, Phoneme::U]),
                _ => None,
            },
        },

        Grapheme::O => {
            match next {
                Some(vowel) if vowel.is_vowel() => {
                    // o + vowel -> oy + vowel (unless next is also a vowel)
                    match graphemes.get(index + 2) {
                        Some(v) if v.is_vowel() => None,
                        _ => Some(vec![
                            Phoneme::O,
                            Phoneme::Y,
                            match vowel {
                                Grapheme::A => Phoneme::A,
                                Grapheme::E => Phoneme::E,
                                Grapheme::I => Phoneme::I,
                                Grapheme::O => Phoneme::O,
                                Grapheme::U => Phoneme::U,
                                _ => Phoneme::Other,
                            },
                        ]),
                    }
                }
                _ => None,
            }
        }

        Grapheme::A => match next {
            Some(vowel) if vowel.is_vowel() => Some(vec![
                Phoneme::A,
                match vowel {
                    Grapheme::A => Phoneme::A,
                    Grapheme::E => Phoneme::E,
                    Grapheme::I => Phoneme::I,
                    Grapheme::O => Phoneme::O,
                    Grapheme::U => Phoneme::U,
                    _ => Phoneme::Other,
                },
            ]),
            _ => None,
        },

        Grapheme::E => {
            if index == graphemes.len() - 1 {
                // Remove final 'e'
                Some(vec![])
            } else {
                None
            }
        }

        _ => None,
    }
}

/// Consonant-specific context-sensitive rules
fn sensitive_consonant(graphemes: &[Grapheme], index: usize) -> Option<Vec<Phoneme>> {
    let curr = &graphemes[index];
    let prev = if index > 0 {
        Some(&graphemes[index - 1])
    } else {
        None
    };
    let next = graphemes.get(index + 1);

    match curr {
        Grapheme::C => match next {
            // 'c' before ('e' | 'i' | 'y') becomes 's'
            Some(Grapheme::E | Grapheme::I | Grapheme::Y) => Some(vec![Phoneme::S]),
            // default: 'k'
            _ => Some(vec![Phoneme::K]),
        },

        Grapheme::X => {
            // 'x' at start becomes 's'
            if index == 0 {
                Some(vec![Phoneme::S])
            } else {
                // otherwise 'ks'
                Some(vec![Phoneme::K, Phoneme::S])
            }
        }

        Grapheme::Y => {
            match (prev, next) {
                // 'y' after ('s' | 'l' | 'x') becomes 'i'
                (Some(Grapheme::S | Grapheme::L | Grapheme::X), _) => Some(vec![Phoneme::I]),
                // 'y' before 's' becomes 'i'
                (_, Some(Grapheme::S)) => Some(vec![Phoneme::I]),
                // 'y' before 'l' becomes 'i'
                (_, Some(Grapheme::L)) => Some(vec![Phoneme::I]),
                // 'y' not preceded by 'a' becomes 'i'
                (Some(g), _) if *g != Grapheme::A => Some(vec![Phoneme::I]),
                (None, _) => Some(vec![Phoneme::I]), // 'y' at start becomes 'i'
                // 'y' preceded by 'a' becomes 'ay'
                (Some(Grapheme::A), _) => Some(vec![Phoneme::A, Phoneme::Y]),
                _ => None,
            }
        }

        Grapheme::T => match (prev, next) {
            (Some(Grapheme::A), Some(Grapheme::E)) => {
                Some(vec![Phoneme::E, Phoneme::Y, Phoneme::T])
            }
            // 'th' + (a|o) -> 'tay/toy'
            (Some(Grapheme::H), Some(Grapheme::A | Grapheme::O)) => Some(vec![
                Phoneme::T,
                Phoneme::A,
                Phoneme::Y,
                match next {
                    Some(Grapheme::A) => Phoneme::A,
                    Some(Grapheme::O) => Phoneme::O,
                    _ => Phoneme::Other,
                },
            ]),
            (_, _) => None,
        },

        Grapheme::D => match (prev, next) {
            (Some(Grapheme::I), Some(Grapheme::E)) => Some(vec![Phoneme::DIPAy, Phoneme::D]),
            (_, _) => None,
        },

        Grapheme::W => match (prev, next) {
            (Some(Grapheme::O), Some(Grapheme::N)) => {
                Some(vec![Phoneme::O, Phoneme::W, Phoneme::N])
            }
            (_, _) => None,
        },

        Grapheme::G => {
            match next {
                Some(Grapheme::E | Grapheme::I | Grapheme::Y) => {
                    // Check if NOT followed by s/c/k
                    match graphemes.get(index + 2) {
                        Some(Grapheme::S | Grapheme::C | Grapheme::K) => None,
                        _ => Some(vec![
                            Phoneme::AFFDy,
                            match next {
                                Some(Grapheme::E) => Phoneme::E,
                                Some(Grapheme::I) => Phoneme::I,
                                Some(Grapheme::Y) => Phoneme::Y,
                                _ => Phoneme::Other,
                            },
                        ]),
                    }
                }
                _ => None,
            }
        }

        _ => None,
    }
}

/// Bigraph-specific context-sensitive rules
fn sensitive_bigraph(graphemes: &[Grapheme], index: usize) -> Option<Vec<Phoneme>> {
    let curr = &graphemes[index];

    match curr {
        Grapheme::Ch => {
            Some(vec![Phoneme::AFFTs])
            // FIXME: doesn't handle "chasm"
        }

        // Th and Sh are handled in free_replacement
        _ => None,
    }
}

/// Convert a single letter to its Filipino phonetic alphabet name
fn letter_to_phonetic(letter: char) -> &'static str {
    match letter.to_ascii_uppercase() {
        'A' => "ey",
        'B' => "bi",
        'C' => "si",
        'D' => "di",
        'E' => "i",
        'F' => "ef",
        'G' => "dyi",
        'H' => "eyts",
        'I' => "ay",
        'J' => "dyey",
        'K' => "key",
        'L' => "el",
        'M' => "em",
        'N' => "en",
        'O' => "o",
        'P' => "pi",
        'Q' => "kyu",
        'R' => "ar",
        'S' => "es",
        'T' => "ti",
        'U' => "yu",
        'V' => "vi",
        'W' => "dobolyu",
        'X' => "eks",
        'Y' => "way",
        'Z' => "zi",
        _ => "",
    }
}

/// Check if a word is an abbreviation (all uppercase letters)
pub fn is_abbreviation(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| c.is_ascii_uppercase())
}

/// Check if a single character is an uppercase letter (for prefix handling like "L-", "C")
pub fn is_single_letter(s: &str) -> bool {
    s.len() == 1
        && s.chars()
            .next()
            .map(|c| c.is_ascii_uppercase())
            .unwrap_or(false)
}

/// Convert an abbreviation to Filipino phonetic transcription
/// E.g., "XR" -> "eks ar", "IV" -> "ay vi"
pub fn transcribe_abbreviation(abbr: &str) -> String {
    abbr.chars()
        .map(letter_to_phonetic)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn preprocess(word: &str) -> String {
    let mut res = word.to_string();

    // remove vowels at the end
    let rem_vowel = Regex::new(r"[aeiou]$").expect("Invalid regex: rem_vowel");
    res = rem_vowel.replace(&res, "").to_string();

    // remove duplicate consecutive characters
    let mut result = String::new();
    let mut prev = ' ';
    for c in res.chars() {
        if c != prev {
            result.push(c);
            prev = c;
        }
    }

    result
}

pub fn postprocess(phonemes: &mut Vec<Phoneme>) -> Vec<Phoneme> {
    let word = phonemes_to_string(phonemes);
    let mut result = word.clone();

    // Handle ending patterns
    result = result.replace("ate", "eyt");
    result = result.replace("ide", "ayd");
    result = result.replace("one", "own");
    result = result.replace("ein", "in");
    result = result.replace("eu", "u");

    // Remove final 'e'
    if result.ends_with('e') && result.len() > 1 {
        result.pop();
    }

    // TODO: Convert back to phonemes if needed
    // For now, return original
    phonemes.clone()
}