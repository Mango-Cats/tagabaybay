use std::vec;

use super::context::Context;
use crate::consts::NativizationConfig;
use crate::tokenization::graphemes::Grapheme;
use crate::tokenization::phoneme::Phoneme;

/// Convert an input grapheme to output phoneme(s) - context-free replacements
///
/// Handles straightforward grapheme-to-phoneme conversions that don't require
/// context analysis.
///
/// # Arguments
///
/// * `graphemes` - The full sequence of graphemes
/// * `index` - Current position in the sequence
/// * `config` - Nativization configuration (affects sh/z sounds)
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a context-free rule matches, where
/// `consumed` is typically 1. Returns `None` for context-sensitive letters.
pub fn free_replacement(ctx: &Context, config: &NativizationConfig) -> Option<(Phoneme, usize)> {
    let g = ctx.current().to_lowercase();

    match g {
        // Bigraph replacements (bigraphs count as 1 grapheme)
        Grapheme::PH => Some((Phoneme::F, 1)),
        Grapheme::PS => Some((Phoneme::S, 1)),
        Grapheme::TH => Some((Phoneme::T, 1)),
        Grapheme::SH => {
            if config.allow_sh_sound {
                Some((Phoneme::SH, 1))
            } else {
                Some((Phoneme::S, 1))
            }
        }
        Grapheme::EE => Some((Phoneme::I, 1)),
        Grapheme::OO => Some((Phoneme::U, 1)),

        // Consonants
        Grapheme::B => Some((Phoneme::B, 1)),
        Grapheme::D => Some((Phoneme::D, 1)),
        Grapheme::F => Some((Phoneme::F, 1)),
        Grapheme::G => Some((Phoneme::G, 1)),
        Grapheme::H => Some((Phoneme::H, 1)),
        Grapheme::K => Some((Phoneme::K, 1)),
        Grapheme::L => Some((Phoneme::L, 1)),
        Grapheme::M => Some((Phoneme::M, 1)),
        Grapheme::N => Some((Phoneme::N, 1)),
        Grapheme::P => Some((Phoneme::P, 1)),
        Grapheme::R => Some((Phoneme::R, 1)),
        Grapheme::S => Some((Phoneme::S, 1)),
        Grapheme::T => Some((Phoneme::T, 1)),
        Grapheme::V => Some((Phoneme::B, 1)),
        Grapheme::W => Some((Phoneme::W, 1)),
        Grapheme::Y => Some((Phoneme::Y, 1)),
        Grapheme::Z => {
            if config.allow_z_sound {
                Some((Phoneme::Z, 1))
            } else {
                Some((Phoneme::S, 1))
            }
        }

        // Spanish
        Grapheme::Enye => Some((Phoneme::N, 1)),

        // Whitespace
        Grapheme::Space => Some((Phoneme::Space, 1)),

        // ASCII passthrough (digits, punctuation, etc.)
        Grapheme::Passthrough(c) => Some((Phoneme::Passthrough(c.to_string()), 1)),

        // Context-sensitive letters (handled in sensitive_replacement)
        Grapheme::C | Grapheme::J | Grapheme::Q | Grapheme::X | Grapheme::CH => None,

        // Other characters (pass through as-is)
        Grapheme::Other => Some((Phoneme::Other, 1)),

        // Uppercase variants should not reach here (normalized by to_lowercase)
        _ => None,
    }
}

/// Context-sensitive nativization (needs surrounding graphemes)
///
/// Handles grapheme-to-phoneme conversions that depend on surrounding context.
/// This includes soft c (cent→sent) and position-dependent
/// transformations (x at start→s, otherwise→ks).
///
/// # Arguments
///
/// * `graphemes` - The full sequence of graphemes
/// * `index` - Current position in the sequence
/// * `config` - Nativization configuration
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a context-sensitive rule matches.
/// Returns `None` if no rule applies (will print error).
pub fn sensitive_replacement(
    ctx: &Context,
    config: &NativizationConfig,
) -> Option<(Vec<Phoneme>, usize)> {
    let curr = ctx.current();

    if curr.is_bigraph() {
        sensitive_bigraph(&ctx)
    } else if curr.is_consonant() {
        sensitive_consonant(&ctx, config)
    } else {
        None
    }
}

/// Consonant-specific context-sensitive rules
///
/// Handles context-dependent consonant transformations based on surrounding graphemes.
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
/// * `config` - Nativization configuration
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a rule matches, where `consumed` is the number
/// of graphemes processed. Returns `None` if no context-sensitive rule applies.
fn sensitive_consonant(
    ctx: &Context,
    config: &NativizationConfig,
) -> Option<(Vec<Phoneme>, usize)> {
    let curr = ctx.current();

    // remove duplicates
    if let Some(x) = handle_duplicates(ctx, config) {
        return Some(x);
    }

    match curr {
        Grapheme::C => handle_consonant_c(&ctx),
        Grapheme::X => handle_consonant_x(&ctx),
        Grapheme::Y => handle_consonant_y(&ctx),
        Grapheme::T => handle_consonant_t(&ctx),
        Grapheme::D => handle_consonant_d(&ctx),
        Grapheme::G => handle_consonant_g(&ctx),
        Grapheme::S => handle_consonant_s(&ctx),
        Grapheme::J => Some((vec![Phoneme::DY], 1)),
        Grapheme::Q => Some((vec![Phoneme::K], 1)),
        _ => None,
    }
}

/// Handle 'c' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` with the appropriate conversion.
fn handle_consonant_c(ctx: &Context) -> Option<(Vec<Phoneme>, usize)> {
    match ctx.lookahead(1) {
        // 'c' before ('e' | 'i' | 'y') becomes 's'
        Some(Grapheme::E | Grapheme::I | Grapheme::Y | Grapheme::EE) => Some((vec![Phoneme::S], 1)),
        // default: 'k'
        _ => Some((vec![Phoneme::K], 1)),
    }
}

/// Handle 'x' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` with the appropriate conversion.
fn handle_consonant_x(ctx: &Context) -> Option<(Vec<Phoneme>, usize)> {
    // 'x' at start becomes 's'
    if ctx.at_start() {
        Some((vec![Phoneme::S], 1))
    } else {
        // otherwise 'ks'
        Some((vec![Phoneme::K, Phoneme::S], 1))
    }
}

/// Handle 'y' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_y(ctx: &Context) -> Option<(Vec<Phoneme>, usize)> {
    match (ctx.prev(), ctx.next()) {
        // 'y' after ('s' | 'l' | 'x') becomes 'i'
        (Some(Grapheme::S | Grapheme::L | Grapheme::X), _) => Some((vec![Phoneme::I], 1)),
        // 'y' before 's' or 'l' becomes 'i'
        (_, Some(Grapheme::S | Grapheme::L)) => Some((vec![Phoneme::I], 1)),
        // 'y' not preceded by 'a' becomes 'i'
        (Some(g), _) if g != Grapheme::A => Some((vec![Phoneme::I], 1)),
        (None, _) => Some((vec![Phoneme::I], 1)), // 'y' at start becomes 'i'
        // 'y' preceded by 'a' - just emit 'y' (A already processed)
        (Some(Grapheme::A), _) => Some((vec![Phoneme::Y], 1)),
        _ => None,
    }
}

/// Handle 't' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_t(ctx: &Context) -> Option<(Vec<Phoneme>, usize)> {
    match (ctx.prev(), ctx.next()) {
        // 'th' + (a|o) -> 'tay/toy'
        (Some(Grapheme::H), Some(Grapheme::A | Grapheme::O)) => Some((
            vec![
                Phoneme::T,
                Phoneme::A,
                Phoneme::Y,
                match ctx.next() {
                    Some(Grapheme::A) => Phoneme::A,
                    Some(Grapheme::O) => Phoneme::O,
                    _ => Phoneme::Other,
                },
            ],
            2,
        )),
        _ => None,
    }
}

/// Handle 'd' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_d(ctx: &Context) -> Option<(Vec<Phoneme>, usize)> {
    match (ctx.prev(), ctx.next()) {
        (Some(Grapheme::I), Some(Grapheme::E)) => {
            Some((vec![Phoneme::A, Phoneme::Y, Phoneme::D], 1))
        }
        _ => None,
    }
}

/// Handle 'g' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_g(ctx: &Context) -> Option<(Vec<Phoneme>, usize)> {
    match ctx.next() {
        Some(Grapheme::E | Grapheme::I | Grapheme::Y | Grapheme::EE) => {
            // Check if NOT followed by s/c/k
            match ctx.lookahead(2) {
                Some(Grapheme::S | Grapheme::C | Grapheme::K) => None,
                _ => Some((
                    vec![
                        Phoneme::DY,
                        match ctx.next() {
                            Some(Grapheme::E) => Phoneme::E,
                            Some(Grapheme::I) => Phoneme::I,
                            Some(Grapheme::Y) => Phoneme::I,
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

/// Handle 's' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_s(ctx: &Context) -> Option<(Vec<Phoneme>, usize)> {
    match (ctx.prev(), ctx.next()) {
        (Some(Grapheme::EE | Grapheme::OO), Some(Grapheme::E)) => match ctx.lookahead(2) {
            Some(Grapheme::B | Grapheme::D) => Some((vec![Phoneme::S], 2)),
            _ => None,
        },
        _ => None,
    }
}

/// Bigraph-specific context-sensitive rules
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a rule matches, `None` otherwise.
fn sensitive_bigraph(ctx: &Context) -> Option<(Vec<Phoneme>, usize)> {
    let curr = ctx.current();
    let next = ctx.next();

    match curr {
        Grapheme::CH => {
            if let Some(next) = next {
                if next.is_consonant() {
                    return Some((vec![Phoneme::K], 1));
                }
            }

            Some((vec![Phoneme::TS], 1))
        }

        // Th and Sh are handled in free_replacement
        _ => None,
    }
}

/// Handles G2P vowel replacement
/// Based on the current index of an english word, get the corresponding
/// bigram matching the index in the ARPAbet(ipa) string, then add rules to
/// nativize
///
/// Issue: Doesn't properly output vowels, could be with my logic here or
/// how im accessing it through nativize.rs
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
/// * `arpabet` - Contains the grapheme vector for the ARPAbet
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))`
pub fn handle_vowel(ctx: &Context, arpabet: &Vec<Grapheme>) -> Option<(Vec<Phoneme>, usize)> {
    let curr = ctx.current();
    let idx = ctx.index;
    let letter = arpabet.get(idx)?.to_lowercase();

    match letter {
        Grapheme::A => handle_arpa_a(arpabet, idx),
        Grapheme::E => handle_arpa_e(arpabet, idx),
        Grapheme::I => handle_arpa_i(arpabet, idx),
        Grapheme::O => handle_arpa_o(arpabet, idx),
        Grapheme::U => handle_arpa_u(arpabet, idx),
        _ => None,
    }

    // match letter {
    //     Grapheme::ArpaAA | Grapheme::ArpaAO =>  Some((vec![Phoneme::O], 1)),
    //     Grapheme::ArpaAE | Grapheme::ArpaAH => Some((vec![Phoneme::A], 1)),
    //     Grapheme::ArpaAW => Some((vec![Phoneme::A, Phoneme::W], 1)),
    //     Grapheme::ArpaAY => Some((vec![Phoneme::A, Phoneme::Y], 1)),
    //     Grapheme::ArpaEH => Some((vec![Phoneme::E], 1)),
    //     Grapheme::ArpaER => Some((vec![Phoneme::E, Phoneme::R], 1)),
    //     Grapheme::ArpaEY => Some((vec![Phoneme::E, Phoneme::Y], 1)),
    //     Grapheme::ArpaIH | Grapheme::ArpaIY => Some((vec![Phoneme::I], 1)),
    //     Grapheme::ArpaOW => Some((vec![Phoneme::O], 1)),
    //     Grapheme::ArpaOY => Some((vec![Phoneme::O, Phoneme::Y], 1)),
    //     Grapheme::ArpaUH | Grapheme::ArpaUW => Some((vec![Phoneme::U], 1)),
    //     _ => None
    // }
}

/// Handle 'A' ARPAbet patterns
/// (AA, AE, AH, AO, AW, AY)
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_arpa_a(arpabet: &Vec<Grapheme>, idx: usize) -> Option<(Vec<Phoneme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(Grapheme::E | Grapheme::H) = next {
        return Some((vec![Phoneme::A], 1));
    }

    if let Some(Grapheme::O | Grapheme::A) = next {
        return Some((vec![Phoneme::O], 1));
    }

    if let Some(Grapheme::W) = next {
        return Some((vec![Phoneme::A, Phoneme::W], 1));
    }

    if let Some(Grapheme::Y) = next {
        return Some((vec![Phoneme::A, Phoneme::Y], 1));
    }

    None
}

/// Handle 'E' ARPAbet patterns
/// (EH, ER, EY)
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_arpa_e(arpabet: &Vec<Grapheme>, idx: usize) -> Option<(Vec<Phoneme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(Grapheme::H) = next {
        return Some((vec![Phoneme::E], 1));
    }

    if let Some(Grapheme::R) = next {
        return Some((vec![Phoneme::E, Phoneme::R], 1));
    }

    if let Some(Grapheme::Y) = next {
        return Some((vec![Phoneme::E, Phoneme::Y], 1));
    }

    None
}

/// Handle 'I' ARPAbet patterns
/// (IH, IY)
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_arpa_i(arpabet: &Vec<Grapheme>, idx: usize) -> Option<(Vec<Phoneme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(Grapheme::H | Grapheme::Y) = next {
        return Some((vec![Phoneme::I], 1));
    }

    None
}

/// Handle 'O' ARPAbet patterns
/// (OW, OY)
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_arpa_o(arpabet: &Vec<Grapheme>, idx: usize) -> Option<(Vec<Phoneme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(Grapheme::W) = next {
        return Some((vec![Phoneme::O], 1));
    }

    if let Some(Grapheme::Y) = next {
        return Some((vec![Phoneme::O, Phoneme::Y], 1));
    }

    None
}

/// Handle 'U' ARPAbet patterns
/// (UH, UW)
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_arpa_u(arpabet: &Vec<Grapheme>, idx: usize) -> Option<(Vec<Phoneme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(Grapheme::H | Grapheme::W) = next {
        return Some((vec![Phoneme::U], 1));
    }

    // if let Some(Grapheme::W) = next {
    //     return Some((vec![Phoneme::Y, Phoneme::U], 1))
    // }

    None
}

/// Handle duplicate graphemes
///
/// Collapses repeated letters into single phonemes.
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
/// * `config` - Nativization configuration
///
/// # Returns
///
/// Returns `Some((phonemes, 2))` if a duplicate is found (consuming 2 graphemes),
/// `None` otherwise.
fn handle_duplicates(ctx: &Context, config: &NativizationConfig) -> Option<(Vec<Phoneme>, usize)> {
    let curr = ctx.current();
    if let Some(next) = ctx.next() {
        if next == curr
        // some symbol overload here: !matches!() is `NOT`matches!()
        // matches!() returns type bool.
        && !matches!(
                curr,
                Grapheme::Passthrough(_) | Grapheme::Space | Grapheme::Other
            )
        {
            if let Some((phonemes, _)) = free_replacement(ctx, config) {
                return Some((Vec::from(vec![phonemes]), 2));
            }
        }
    }

    None
}

/// Convert a single letter to its Filipino phonetic alphabet name
///
/// Used for spelling out abbreviations and single letters.
///
/// # Arguments
///
/// * `letter` - The grapheme to spell out
///
/// # Returns
///
/// Returns `Some(Vec<Phoneme>)` with the phonetic spelling, or `None` if
/// the grapheme is not a letter.
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
        Grapheme::H => Some(vec![Phoneme::E, Phoneme::Y, Phoneme::TS]),
        Grapheme::I => Some(vec![Phoneme::A, Phoneme::Y]),
        Grapheme::J => Some(vec![Phoneme::DY, Phoneme::E, Phoneme::Y]),
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
