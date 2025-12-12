use std::vec;

use crate::nativization::error::printe;
use crate::tokenization::graphemes::Grapheme;
use crate::tokenization::phoneme::Phoneme;
use crate::tokenization::tokenize::tokenize;

/// Convert an input grapheme to output phoneme(s) - context-free replacements
pub fn free_replacement(graphemes: &[Grapheme], index: usize) -> Option<Vec<Phoneme>> {
    match graphemes[index] {
        // Bigraph replacements
        Grapheme::Ph => Some(vec![Phoneme::F]),
        Grapheme::Th => Some(vec![Phoneme::T]),
        Grapheme::Sh => Some(vec![Phoneme::S]),
        Grapheme::Ee => Some(vec![Phoneme::I]),
        Grapheme::Oo => Some(vec![Phoneme::U]),
        Grapheme::Qu => Some(vec![Phoneme::K, Phoneme::U, Phoneme::W]),

        // Non-native consonant replacements
        Grapheme::J => Some(vec![Phoneme::Dy]),
        Grapheme::V => Some(vec![Phoneme::B]),
        Grapheme::Z => Some(vec![Phoneme::S]),
        Grapheme::F => Some(vec![Phoneme::F]),

        // Native consonants (pass through)
        Grapheme::B => Some(vec![Phoneme::B]),
        Grapheme::D => Some(vec![Phoneme::D]),
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
        Grapheme::W => Some(vec![Phoneme::W]),
        Grapheme::Y => Some(vec![Phoneme::Y]),

        // Vowels (pass through)
        Grapheme::A => Some(vec![Phoneme::A]),
        Grapheme::E => Some(vec![Phoneme::E]),
        Grapheme::I => Some(vec![Phoneme::I]),
        Grapheme::O => Some(vec![Phoneme::O]),
        Grapheme::U => Some(vec![Phoneme::U]),

        // Context-sensitive letters (handled in sensitive_replacement)
        // see `sensitive_replacement()`
        Grapheme::C | Grapheme::X | Grapheme::Ch => None,

        // Other characters (pass through as-is)
        Grapheme::Other => Some(vec![Phoneme::Other]),
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
        Grapheme::C => match next {
            Some(Grapheme::E | Grapheme::I | Grapheme::Y) => Some(vec![Phoneme::S]),
            _ => Some(vec![Phoneme::K]),
        },

        // 'x' at start becomes 's', otherwise 'ks'
        Grapheme::X => {
            if index == 0 {
                Some(vec![Phoneme::S])
            } else {
                Some(vec![Phoneme::K, Phoneme::S])
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
pub fn nativize_word(word: &str) -> Vec<Phoneme> {
    nativize(word, None, None)
}

pub fn nativize_word_set(word_list: &[&str], dataset_name: &str) -> Vec<Vec<Phoneme>> {
    let mut res: Vec<Vec<Phoneme>> = Vec::new();

    for (i, word) in word_list.iter().enumerate() {
        res.push(nativize(&word, Some(i), Some(dataset_name)));
    }

    res
}

fn nativize(
    word: &str,
    word_number: Option<usize>,
    dataset_name: Option<&str>,
) -> Vec<Phoneme> {
    let mut res: Vec<Phoneme> = Vec::new();
    let graphemes = tokenize(word);

    for (i, _) in graphemes.iter().enumerate() {
        // Try context-sensitive replacement first
        if let Some(sens_res) = sensitive_replacement(&graphemes, i) {
            res.extend(sens_res);
        } else {
            // Fall back to context-free replacement
            if let Some(free_res) = free_replacement(&graphemes, i) {
                res.extend(free_res);
            } else {
                printe(&graphemes, i, word_number, dataset_name);
            }
        }
    }

    res
}
