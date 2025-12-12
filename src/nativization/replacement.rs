use std::vec;

use crate::tokenization::graphemes::Grapheme;
use crate::tokenization::phoneme::{Phoneme, phonemes_to_string};
use crate::tokenization::tokenize::tokenize;

/// Convert an input grapheme to output phoneme(s) - context-free replacements
pub fn free_replacement(g: &Grapheme) -> Vec<Phoneme> {
    match g {
        // Bigraph replacements
        Grapheme::Ph => vec![Phoneme::F],
        Grapheme::Th => vec![Phoneme::T],
        Grapheme::Sh => vec![Phoneme::S],
        Grapheme::Ch => vec![Phoneme::Ts],
        Grapheme::Ee => vec![Phoneme::I],
        Grapheme::Oo => vec![Phoneme::U],
        Grapheme::Qu=> vec![Phoneme::K, Phoneme::U, Phoneme::W],

        // Non-native consonant replacements
        Grapheme::J => vec![Phoneme::Dy],
        Grapheme::V => vec![Phoneme::B],
        Grapheme::Z => vec![Phoneme::S],
        Grapheme::F => vec![Phoneme::F],

        // Native consonants (pass through)
        Grapheme::B => vec![Phoneme::B],
        Grapheme::D => vec![Phoneme::D],
        Grapheme::G => vec![Phoneme::G],
        Grapheme::H => vec![Phoneme::H],
        Grapheme::K => vec![Phoneme::K],
        Grapheme::L => vec![Phoneme::L],
        Grapheme::M => vec![Phoneme::M],
        Grapheme::N => vec![Phoneme::N],
        Grapheme::P => vec![Phoneme::P],
        Grapheme::R => vec![Phoneme::R],
        Grapheme::S => vec![Phoneme::S],
        Grapheme::T => vec![Phoneme::T],
        Grapheme::W => vec![Phoneme::W],
        Grapheme::Y => vec![Phoneme::Y],

        // Vowels (pass through)
        Grapheme::A => vec![Phoneme::A],
        Grapheme::E => vec![Phoneme::E],
        Grapheme::I => vec![Phoneme::I],
        Grapheme::O => vec![Phoneme::O],
        Grapheme::U => vec![Phoneme::U],

        // Context-sensitive letters (handled in sensitive_replacement)
        Grapheme::C | Grapheme::X => vec![], // Will be handled by sensitive_replacement

        // Other characters (pass through as-is)
        Grapheme::Other => vec![Phoneme::Other]
    }
}

/// Context-sensitive nativization (needs surrounding graphemes)
/// Returns None if no context-sensitive rule applies (use free_replacement instead)
pub fn sensitive_replacement(graphemes: &[Grapheme], index: usize) -> Option<Vec<Phoneme>> {
    let curr = &graphemes[index];
    let prev = if index > 0 {
        Some(&graphemes[index - 1])
    } else {
        None
    };
    let next = graphemes.get(index + 1);

    match curr {
        // 'c' before 'e', 'i', or 'y' becomes 's', otherwise 'k'
        Grapheme::C => {
            match next {
                Some(Grapheme::E | Grapheme::I | Grapheme::Y) => {
                    Some(vec![Phoneme::S])
                }
                _ => Some(vec![Phoneme::K]),
            }
        }

        // 'x' at start becomes 's', otherwise 'ks'
        Grapheme::X => {
            if index == 0 {
                Some(vec![Phoneme::S])
            } else {
                Some(vec![Phoneme::K, Phoneme::S])
            }
        }

        // 'qu' becomes 'kw' before vowel, 'ku' otherwise
        Grapheme::Qu => {
            if let Some(next_g) = next {
                if next_g.is_vowel() {
                    Some(vec![Phoneme::K, Phoneme::W])
                } else {
                    Some(vec![Phoneme::K, Phoneme::U, Phoneme::W])
                }
            } else {
                Some(vec![Phoneme::K, Phoneme::U])
            }
        }

        // 'y' after 's' or 'l' becomes 'i'
        Grapheme::Y => {
            match prev {
                Some(Grapheme::S | Grapheme::L) => Some(vec![Phoneme::I]),
                _ => None, // Use default 'y'
            }
        }

        // 'g' before 'e' or 'i' could become 'h' (Spanish loanwords)
        // For now, no change - this is highly context-dependent
        Grapheme::G => None,

        _ => None,
    }
}

/// Nativize an entire word: String -> Vec<Grapheme> -> Vec<Phoneme> -> String
pub fn nativize_word(word: &str) -> String {
    let graphemes = tokenize(word);
    let mut phonemes: Vec<Phoneme> = Vec::new();

    for (i, g) in graphemes.iter().enumerate() {
        // Try context-sensitive replacement first
        if let Some(replacement) = sensitive_replacement(&graphemes, i) {
            phonemes.extend(replacement);
        } else {
            // Fall back to context-free replacement
            phonemes.extend(free_replacement(g));
        }
    }

    phonemes_to_string(&phonemes)
}
