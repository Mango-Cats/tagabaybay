use std::vec;

use crate::nativization::error::printe;
use crate::tokenization::graphemes::Grapheme;
use crate::tokenization::phoneme::{Phoneme, phonemes_to_string};

use regex::Regex;

/// Convert an input grapheme to output phoneme(s) - context-free replacements
pub fn free_replacement(graphemes: &[Grapheme], index: usize) -> Option<Vec<Phoneme>> {
    let g = graphemes[index].to_lowercase();

    match g {
        // Bigraph replacements
        Grapheme::BigraphPh => Some(vec![Phoneme::F]),
        Grapheme::BigraphPs => Some(vec![Phoneme::S]),
        Grapheme::BigraphTh => Some(vec![Phoneme::T]),
        Grapheme::BigraphSh => Some(vec![Phoneme::S]),
        Grapheme::BigraphEe => Some(vec![Phoneme::I]),
        Grapheme::BigraphOo => Some(vec![Phoneme::U]),

        // Vowels
        Grapheme::A => Some(vec![Phoneme::A]),
        Grapheme::E => Some(vec![Phoneme::E]),
        Grapheme::I => Some(vec![Phoneme::I]),
        Grapheme::O => Some(vec![Phoneme::O]),
        Grapheme::U => Some(vec![Phoneme::U]),

        // Consonants
        Grapheme::B => Some(vec![Phoneme::B]),
        Grapheme::D => Some(vec![Phoneme::D]),
        Grapheme::F => Some(vec![Phoneme::F]),
        Grapheme::G => Some(vec![Phoneme::G]),
        Grapheme::H => Some(vec![Phoneme::H]),
        Grapheme::K => Some(vec![Phoneme::K]),
        Grapheme::L => Some(vec![Phoneme::L]),
        Grapheme::M => Some(vec![Phoneme::M]),
        Grapheme::N => Some(vec![Phoneme::N]),
        Grapheme::P => Some(vec![Phoneme::P]),
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
        Grapheme::C | Grapheme::J | Grapheme::Q | Grapheme::X | Grapheme::BigraphCh => None,

        // Other characters (pass through as-is)
        Grapheme::Other => Some(vec![Phoneme::Other]),

        // Uppercase variants should not reach here (normalized by to_lowercase)
        _ => None,
    }
}

/// Context-sensitive nativization (needs surrounding graphemes)
/// Returns None if no context-sensitive rule applies (use free_replacement instead)
pub fn sensitive_replacement(graphemes: &[Grapheme], index: usize) -> Option<Vec<Phoneme>> {
    let curr = graphemes[index].to_lowercase();

    if curr.is_vowel() {
        sensitive_vowel(graphemes, index)
    } else if curr.is_bigraph() {
        sensitive_bigraph(graphemes, index)
    } else if curr.is_consonant(){
        sensitive_consonant(graphemes, index)
    } else {
        printe(graphemes, index, None, None);
        None
    }
}

/// Vowel-specific context-sensitive rules
fn sensitive_vowel(graphemes: &[Grapheme], index: usize) -> Option<Vec<Phoneme>> {
    let curr = graphemes[index].to_lowercase();
    let prev = if index > 0 {
        Some(graphemes[index - 1].to_lowercase())
    } else {
        None
    };
    let next = graphemes.get(index + 1).map(|g| g.to_lowercase());

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
                    match graphemes.get(index + 2).map(|g| g.to_lowercase()) {
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
    let curr = graphemes[index].to_lowercase();
    let prev = if index > 0 {
        Some(graphemes[index - 1].to_lowercase())
    } else {
        None
    };
    let next = graphemes.get(index + 1).map(|g| g.to_lowercase());

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
                (Some(g), _) if g != Grapheme::A => Some(vec![Phoneme::I]),
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
                    match graphemes.get(index + 2).map(|g| g.to_lowercase()) {
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

        // 'j' always becomes 'dy' (affricate)
        Grapheme::J => Some(vec![Phoneme::AFFDy]),

        // 'q' becomes 'k' (usually followed by 'u' which handles the 'w' sound)
        Grapheme::Q => Some(vec![Phoneme::K]),

        _ => None,
    }
}

/// Bigraph-specific context-sensitive rules
fn sensitive_bigraph(graphemes: &[Grapheme], index: usize) -> Option<Vec<Phoneme>> {
    let curr = &graphemes[index];

    match curr {
        Grapheme::BigraphCh => {
            Some(vec![Phoneme::AFFTs])
            // FIXME: doesn't handle "chasm"
        }

        // Th and Sh are handled in free_replacement
        _ => None,
    }
}

/// Convert a single letter to its Filipino phonetic alphabet name
pub fn letter_to_phonetic(letter: Grapheme) -> Option<Vec<Phoneme>> {
    let l = letter.to_lowercase();
    match l {
        Grapheme::A => Some(vec![Phoneme::E, Phoneme::Y]),
        Grapheme::B => Some(vec![Phoneme::B, Phoneme::I]),
        Grapheme::C => Some(vec![Phoneme::S, Phoneme::I]),
        Grapheme::D => Some(vec![Phoneme::D, Phoneme::I]),
        Grapheme::E => Some(vec![Phoneme::I]),
        Grapheme::F => Some(vec![Phoneme::E, Phoneme::F]),
        Grapheme::G => Some(vec![Phoneme::D, Phoneme::Y, Phoneme::I]),
        Grapheme::H => Some(vec![Phoneme::E, Phoneme::Y, Phoneme::AFFTs]),
        Grapheme::I => Some(vec![Phoneme::A, Phoneme::Y]),
        Grapheme::J => Some(vec![Phoneme::AFFDy, Phoneme::E, Phoneme::Y]),
        Grapheme::K => Some(vec![Phoneme::K, Phoneme::E, Phoneme::Y]),
        Grapheme::L => Some(vec![Phoneme::E, Phoneme::L]),
        Grapheme::M => Some(vec![Phoneme::E, Phoneme::M]),
        Grapheme::N => Some(vec![Phoneme::E, Phoneme::N]),
        Grapheme::O => Some(vec![Phoneme::O, Phoneme::W]),
        Grapheme::P => Some(vec![Phoneme::P, Phoneme::I]),
        Grapheme::Q => Some(vec![Phoneme::K, Phoneme::Y, Phoneme::U]),
        Grapheme::R => Some(vec![Phoneme::A, Phoneme::R]),
        Grapheme::S => Some(vec![Phoneme::E, Phoneme::S]),
        Grapheme::T => Some(vec![Phoneme::T, Phoneme::I]),
        Grapheme::U => Some(vec![Phoneme::Y, Phoneme::U]),
        Grapheme::V => Some(vec![Phoneme::B, Phoneme::I]),
        Grapheme::W => Some(vec![
            Phoneme::D,
            Phoneme::A,
            Phoneme::B,
            Phoneme::O,
            Phoneme::L,
            Phoneme::Y,
            Phoneme::U,
        ]),
        Grapheme::X => Some(vec![Phoneme::E, Phoneme::K, Phoneme::S]),
        Grapheme::Y => Some(vec![Phoneme::W, Phoneme::A, Phoneme::Y]),
        Grapheme::Z => Some(vec![Phoneme::S, Phoneme::I]),
        _ => None,
    }
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
