use std::vec;

use crate::nativization::error::printe;
use crate::tokenization::graphemes::Grapheme;
use crate::tokenization::phoneme::{Phoneme, phonemes_to_string};

/// Convert an input grapheme to output phoneme(s) - context-free replacements
/// Returns (phonemes, graphemes_consumed)
pub fn free_replacement(graphemes: &[Grapheme], index: usize) -> Option<(Vec<Phoneme>, usize)> {
    let g = graphemes[index].to_lowercase();

    match g {
        // Bigraph replacements (bigraphs count as 1 grapheme)
        Grapheme::BigraphPh => Some((vec![Phoneme::F], 1)),
        Grapheme::BigraphPs => Some((vec![Phoneme::S], 1)),
        Grapheme::BigraphTh => Some((vec![Phoneme::T], 1)),
        Grapheme::BigraphSh => Some((vec![Phoneme::S], 1)),
        Grapheme::BigraphEe => Some((vec![Phoneme::I], 1)),
        Grapheme::BigraphOo => Some((vec![Phoneme::U], 1)),

        // Vowels
        Grapheme::A => Some((vec![Phoneme::A], 1)),
        Grapheme::E => Some((vec![Phoneme::E], 1)),
        Grapheme::I => Some((vec![Phoneme::I], 1)),
        Grapheme::O => Some((vec![Phoneme::O], 1)),
        Grapheme::U => Some((vec![Phoneme::U], 1)),

        // Consonants
        Grapheme::B => Some((vec![Phoneme::B], 1)),
        Grapheme::D => Some((vec![Phoneme::D], 1)),
        Grapheme::F => Some((vec![Phoneme::F], 1)),
        Grapheme::G => Some((vec![Phoneme::G], 1)),
        Grapheme::H => Some((vec![Phoneme::H], 1)),
        Grapheme::K => Some((vec![Phoneme::K], 1)),
        Grapheme::L => Some((vec![Phoneme::L], 1)),
        Grapheme::M => Some((vec![Phoneme::M], 1)),
        Grapheme::N => Some((vec![Phoneme::N], 1)),
        Grapheme::P => Some((vec![Phoneme::P], 1)),
        Grapheme::R => Some((vec![Phoneme::R], 1)),
        Grapheme::S => Some((vec![Phoneme::S], 1)),
        Grapheme::T => Some((vec![Phoneme::T], 1)),
        Grapheme::V => Some((vec![Phoneme::B], 1)),
        Grapheme::W => Some((vec![Phoneme::W], 1)),
        Grapheme::Y => Some((vec![Phoneme::Y], 1)),
        Grapheme::Z => Some((vec![Phoneme::S], 1)),

        // Spanish
        Grapheme::Enye => Some((vec![Phoneme::Ny], 1)),

        // Whitespace
        Grapheme::Space => Some((vec![Phoneme::Space], 1)),

        // ASCII passthrough (digits, punctuation, etc.)
        Grapheme::Passthrough(c) => Some((vec![Phoneme::Passthrough(c)], 1)),

        // Context-sensitive letters (handled in sensitive_replacement)
        Grapheme::C | Grapheme::J | Grapheme::Q | Grapheme::X | Grapheme::BigraphCh => None,

        // Other characters (pass through as-is)
        Grapheme::Other => Some((vec![Phoneme::Other], 1)),

        // Uppercase variants should not reach here (normalized by to_lowercase)
        _ => None,
    }
}

/// Context-sensitive nativization (needs surrounding graphemes)
/// Returns (phonemes, graphemes_consumed) or None if no context-sensitive rule applies
pub fn sensitive_replacement(
    graphemes: &[Grapheme],
    index: usize,
) -> Option<(Vec<Phoneme>, usize)> {
    let curr = graphemes[index].to_lowercase();

    if curr.is_vowel() {
        sensitive_vowel(graphemes, index)
    } else if curr.is_bigraph() {
        sensitive_bigraph(graphemes, index)
    } else if curr.is_consonant() {
        sensitive_consonant(graphemes, index)
    } else {
        printe(graphemes, index, None, None);
        None
    }
}

/// Vowel-specific context-sensitive rules
/// Returns (phonemes, graphemes_consumed)
fn sensitive_vowel(graphemes: &[Grapheme], index: usize) -> Option<(Vec<Phoneme>, usize)> {
    let curr = graphemes[index].to_lowercase();
    let prev = if index > 0 {
        Some(graphemes[index - 1].to_lowercase())
    } else {
        None
    };
    let next = graphemes.get(index + 1).map(|g| g.to_lowercase());

    // remove duplicates
    if let Some(x) = sensitive_duplicates(curr, next, graphemes, index) {
        return Some(x);
    }

    match curr {
        Grapheme::A => {
            // Check for "ate" pattern (a-t-e at end) → "eyt"
            if let Some(Grapheme::T) = next {
                if let Some(e) = graphemes.get(index + 2).map(|g| g.to_lowercase()) {
                    if e == Grapheme::E && index + 2 == graphemes.len() - 1 {
                        // "ate" at end of word → "eyt"
                        return Some((vec![Phoneme::E, Phoneme::Y, Phoneme::T], 3));
                    }
                }
            }
            None
        }

        Grapheme::E => {
            // remove trailing 'e'
            // FIXME: we may need to check the character before
            if index == graphemes.len() - 1 {
                return Some((vec![], 1));
            }

            // ei -> i (consume both e and i)
            match next {
                Some(Grapheme::I) => Some((vec![Phoneme::I], 2)),
                _ => None,
            }
        }

        Grapheme::I => {
            // Check for "ide" pattern (i-d-e at end) → "ayd"
            if let Some(Grapheme::D) = next {
                if let Some(e) = graphemes.get(index + 2).map(|g| g.to_lowercase()) {
                    if e == Grapheme::E && index + 2 == graphemes.len() - 1 {
                        // "ide" at end of word → "ayd"
                        return Some((vec![Phoneme::A, Phoneme::Y, Phoneme::D], 3));
                    }
                }
            }

            // Regular i + vowel patterns
            match next {
                Some(Grapheme::A) => Some((vec![Phoneme::I, Phoneme::Y, Phoneme::A], 2)),
                Some(Grapheme::E) => Some((vec![Phoneme::I, Phoneme::Y, Phoneme::E], 2)),
                // FIXME: Some(Grapheme::I) => Some((vec![Phoneme::I, Phoneme::Y, Phoneme::I], 2)),
                Some(Grapheme::O) => Some((vec![Phoneme::I, Phoneme::Y, Phoneme::O], 2)),
                Some(Grapheme::U) => Some((vec![Phoneme::I, Phoneme::Y, Phoneme::U], 2)),
                _ => None,
            }
        }

        Grapheme::O => {
            // Check for "one" pattern (o-n-e at end) → "own"
            if let Some(Grapheme::N) = next {
                if let Some(e) = graphemes.get(index + 2).map(|g| g.to_lowercase()) {
                    if e == Grapheme::E && index + 2 == graphemes.len() - 1 {
                        // "one" at end of word → "own"
                        return Some((vec![Phoneme::O, Phoneme::W, Phoneme::N], 3));
                    }
                }
            }

            match next {
                Some(vowel) if vowel.is_vowel() => {
                    // o + vowel -> oy + vowel (unless next is also a vowel)
                    match graphemes.get(index + 2).map(|g| g.to_lowercase()) {
                        Some(v) if v.is_vowel() => None,
                        _ => Some((
                            vec![
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
                            ],
                            2,
                        )),
                    }
                }
                _ => None,
            }
        }

        Grapheme::U => match next {
            Some(Grapheme::A) => Some((vec![Phoneme::U, Phoneme::W, Phoneme::A], 2)),
            Some(Grapheme::E) => Some((vec![Phoneme::U, Phoneme::W, Phoneme::E], 2)),
            Some(Grapheme::I) => Some((vec![Phoneme::U, Phoneme::W, Phoneme::I], 2)),
            Some(Grapheme::O) => Some((vec![Phoneme::U, Phoneme::W, Phoneme::O], 2)),
            Some(Grapheme::U) => Some((vec![Phoneme::U, Phoneme::W, Phoneme::U], 2)),
            _ => match prev {
                Some(Grapheme::E) => Some((vec![Phoneme::Y, Phoneme::U], 1)),
                _ => None,
            },
        },
        _ => None,
    }
}

/// Consonant-specific context-sensitive rules
/// Returns (phonemes, graphemes_consumed)
fn sensitive_consonant(graphemes: &[Grapheme], index: usize) -> Option<(Vec<Phoneme>, usize)> {
    let curr = graphemes[index].to_lowercase();
    let prev = if index > 0 {
        Some(graphemes[index - 1].to_lowercase())
    } else {
        None
    };
    let next = graphemes.get(index + 1).map(|g| g.to_lowercase());

    // remove duplicates
    if let Some(x) = sensitive_duplicates(curr, next, graphemes, index) {
        return Some(x);
    }

    match curr {
        Grapheme::C => match next {
            // 'c' before ('e' | 'i' | 'y') becomes 's'
            Some(Grapheme::E | Grapheme::I | Grapheme::Y) => Some((vec![Phoneme::S], 1)),
            // default: 'k'
            _ => Some((vec![Phoneme::K], 1)),
        },

        Grapheme::X => {
            // 'x' at start becomes 's'
            if index == 0 {
                Some((vec![Phoneme::S], 1))
            } else {
                // otherwise 'ks'
                Some((vec![Phoneme::K, Phoneme::S], 1))
            }
        }

        Grapheme::Y => {
            match (prev, next) {
                // 'y' after ('s' | 'l' | 'x') becomes 'i'
                (Some(Grapheme::S | Grapheme::L | Grapheme::X), _) => Some((vec![Phoneme::I], 1)),
                // 'y' before 's' becomes 'i'
                (_, Some(Grapheme::S)) => Some((vec![Phoneme::I], 1)),
                // 'y' before 'l' becomes 'i'
                (_, Some(Grapheme::L)) => Some((vec![Phoneme::I], 1)),
                // 'y' not preceded by 'a' becomes 'i'
                (Some(g), _) if g != Grapheme::A => Some((vec![Phoneme::I], 1)),
                (None, _) => Some((vec![Phoneme::I], 1)), // 'y' at start becomes 'i'
                // 'y' preceded by 'a' - just emit 'y' (A already processed)
                (Some(Grapheme::A), _) => Some((vec![Phoneme::Y], 1)),
                _ => None,
            }
        }

        Grapheme::T => match (prev, next) {
            // 'th' + (a|o) -> 'tay/toy'
            (Some(Grapheme::H), Some(Grapheme::A | Grapheme::O)) => Some((
                vec![
                    Phoneme::T,
                    Phoneme::A,
                    Phoneme::Y,
                    match next {
                        Some(Grapheme::A) => Phoneme::A,
                        Some(Grapheme::O) => Phoneme::O,
                        _ => Phoneme::Other,
                    },
                ],
                2,
            )),
            (_, _) => None,
        },

        Grapheme::D => match (prev, next) {
            (Some(Grapheme::I), Some(Grapheme::E)) => Some((vec![Phoneme::DIPAy, Phoneme::D], 1)),
            (_, _) => None,
        },

        Grapheme::G => {
            match next {
                Some(Grapheme::E | Grapheme::I | Grapheme::Y) => {
                    // Check if NOT followed by s/c/k
                    match graphemes.get(index + 2).map(|g| g.to_lowercase()) {
                        Some(Grapheme::S | Grapheme::C | Grapheme::K) => None,
                        _ => Some((
                            vec![
                                Phoneme::AFFDy,
                                match next {
                                    Some(Grapheme::E) => Phoneme::E,
                                    Some(Grapheme::I) => Phoneme::I,
                                    Some(Grapheme::Y) => Phoneme::Y,
                                    _ => Phoneme::Other,
                                },
                            ],
                            2,
                        )),
                    }
                }
                _ => None,
            }
        }

        // 'j' always becomes 'dy' (affricate)
        Grapheme::J => Some((vec![Phoneme::AFFDy], 1)),

        // 'q' becomes 'k' (usually followed by 'u' which handles the 'w' sound)
        Grapheme::Q => Some((vec![Phoneme::K], 1)),

        _ => None,
    }
}

/// Bigraph-specific context-sensitive rules
/// Returns (phonemes, graphemes_consumed)
fn sensitive_bigraph(graphemes: &[Grapheme], index: usize) -> Option<(Vec<Phoneme>, usize)> {
    let curr = &graphemes[index];
    let prev = if index > 0 {
        Some(graphemes[index - 1].to_lowercase())
    } else {
        None
    };
    let next = graphemes.get(index + 1).map(|g| g.to_lowercase());

    match curr {
        Grapheme::BigraphCh => {
            if let Some(next) = next {
                if next.is_consonant() {
                    return Some((vec![Phoneme::K], 1));
                }
            }

            Some((vec![Phoneme::AFFTs], 1))
        }

        // Th and Sh are handled in free_replacement
        _ => None,
    }
}

fn sensitive_duplicates(
    curr: Grapheme,
    next: Option<Grapheme>,
    graphemes: &[Grapheme],
    index: usize,
) -> Option<(Vec<Phoneme>, usize)> {
    if let Some(next_grapheme) = next {
        if curr == next_grapheme && !matches!(curr, Grapheme::Passthrough(_) | Grapheme::Space | Grapheme::Other) {
            if let Some((phonemes, _)) = free_replacement(graphemes, index) {
                return Some((phonemes, 2));
            }
        }
    }

    None
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
