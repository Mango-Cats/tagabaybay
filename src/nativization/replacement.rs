use std::vec;

use super::context::Context;
use crate::consts::NativizationConfig;
use crate::tokenization::graphemes::EnglishGrapheme;
use crate::tokenization::phoneme::FilipinoGrapheme;

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
pub fn free_replacement(
    ctx: &Context,
    config: &NativizationConfig,
) -> Option<(FilipinoGrapheme, usize)> {
    let g = ctx.current().to_lowercase();

    match g {
        // Digraph replacements (digraph count as 1 grapheme)
        EnglishGrapheme::PH => Some((FilipinoGrapheme::F, 1)),
        EnglishGrapheme::PS => Some((FilipinoGrapheme::S, 1)),
        EnglishGrapheme::TH => Some((FilipinoGrapheme::T, 1)),
        EnglishGrapheme::SH => {
            if config.allow_sh_sound {
                Some((FilipinoGrapheme::SH, 1))
            } else {
                Some((FilipinoGrapheme::S, 1))
            }
        }
        EnglishGrapheme::EE => Some((FilipinoGrapheme::I, 1)),
        EnglishGrapheme::OO => Some((FilipinoGrapheme::U, 1)),

        // Consonants
        EnglishGrapheme::B => Some((FilipinoGrapheme::B, 1)),
        EnglishGrapheme::D => Some((FilipinoGrapheme::D, 1)),
        EnglishGrapheme::F => Some((FilipinoGrapheme::F, 1)),
        EnglishGrapheme::G => Some((FilipinoGrapheme::G, 1)),
        EnglishGrapheme::H => Some((FilipinoGrapheme::H, 1)),
        EnglishGrapheme::K => Some((FilipinoGrapheme::K, 1)),
        EnglishGrapheme::L => Some((FilipinoGrapheme::L, 1)),
        EnglishGrapheme::M => Some((FilipinoGrapheme::M, 1)),
        EnglishGrapheme::N => Some((FilipinoGrapheme::N, 1)),
        EnglishGrapheme::P => Some((FilipinoGrapheme::P, 1)),
        EnglishGrapheme::R => Some((FilipinoGrapheme::R, 1)),
        EnglishGrapheme::S => Some((FilipinoGrapheme::S, 1)),
        EnglishGrapheme::T => Some((FilipinoGrapheme::T, 1)),
        EnglishGrapheme::V => Some((FilipinoGrapheme::B, 1)),
        EnglishGrapheme::W => Some((FilipinoGrapheme::W, 1)),
        EnglishGrapheme::Y => Some((FilipinoGrapheme::Y, 1)),
        EnglishGrapheme::Z => {
            if config.allow_z_sound {
                Some((FilipinoGrapheme::Z, 1))
            } else {
                Some((FilipinoGrapheme::S, 1))
            }
        }

        // Spanish
        EnglishGrapheme::Enye => Some((FilipinoGrapheme::N, 1)),

        // Whitespace
        EnglishGrapheme::Space => Some((FilipinoGrapheme::Space, 1)),

        // ASCII passthrough (digits, punctuation, etc.)
        EnglishGrapheme::Passthrough(c) => Some((FilipinoGrapheme::Passthrough(c.to_string()), 1)),

        // Context-sensitive letters (handled in sensitive_replacement)
        EnglishGrapheme::C
        | EnglishGrapheme::J
        | EnglishGrapheme::Q
        | EnglishGrapheme::X
        | EnglishGrapheme::CH => None,

        // Other characters (pass through as-is)
        EnglishGrapheme::Other => Some((FilipinoGrapheme::Other, 1)),

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
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
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
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current();

    // remove duplicates
    if let Some(x) = handle_duplicates(ctx, config) {
        return Some(x);
    }

    match curr {
        EnglishGrapheme::C => handle_consonant_c(&ctx),
        EnglishGrapheme::X => handle_consonant_x(&ctx),
        EnglishGrapheme::Y => handle_consonant_y(&ctx),
        EnglishGrapheme::T => handle_consonant_t(&ctx),
        EnglishGrapheme::D => handle_consonant_d(&ctx),
        EnglishGrapheme::G => handle_consonant_g(&ctx),
        EnglishGrapheme::S => handle_consonant_s(&ctx),
        EnglishGrapheme::J => Some((vec![FilipinoGrapheme::DY], 1)),
        EnglishGrapheme::Q => Some((vec![FilipinoGrapheme::K], 1)),
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
fn handle_consonant_c(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match ctx.lookahead(1) {
        // 'c' before ('e' | 'i' | 'y') becomes 's'
        Some(
            EnglishGrapheme::E | EnglishGrapheme::I | EnglishGrapheme::Y | EnglishGrapheme::EE,
        ) => Some((vec![FilipinoGrapheme::S], 1)),
        // default: 'k'
        _ => Some((vec![FilipinoGrapheme::K], 1)),
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
fn handle_consonant_x(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // 'x' at start becomes 's'
    if ctx.at_start() {
        Some((vec![FilipinoGrapheme::S], 1))
    } else {
        // otherwise 'ks'
        Some((vec![FilipinoGrapheme::K, FilipinoGrapheme::S], 1))
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
fn handle_consonant_y(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match (ctx.prev(), ctx.next()) {
        // 'y' after ('s' | 'l' | 'x') becomes 'i'
        (Some(EnglishGrapheme::S | EnglishGrapheme::L | EnglishGrapheme::X), _) => {
            Some((vec![FilipinoGrapheme::I], 1))
        }
        // 'y' before 's' or 'l' becomes 'i'
        (_, Some(EnglishGrapheme::S | EnglishGrapheme::L)) => Some((vec![FilipinoGrapheme::I], 1)),
        // 'y' not preceded by 'a' becomes 'i'
        (Some(g), _) if g != EnglishGrapheme::A => Some((vec![FilipinoGrapheme::I], 1)),
        (None, _) => Some((vec![FilipinoGrapheme::I], 1)), // 'y' at start becomes 'i'
        // 'y' preceded by 'a' - just emit 'y' (A already processed)
        (Some(EnglishGrapheme::A), _) => Some((vec![FilipinoGrapheme::Y], 1)),
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
fn handle_consonant_t(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match (ctx.prev(), ctx.next()) {
        // 'th' + (a|o) -> 'tay/toy'
        (Some(EnglishGrapheme::H), Some(EnglishGrapheme::A | EnglishGrapheme::O)) => Some((
            vec![
                FilipinoGrapheme::T,
                FilipinoGrapheme::A,
                FilipinoGrapheme::Y,
                match ctx.next() {
                    Some(EnglishGrapheme::A) => FilipinoGrapheme::A,
                    Some(EnglishGrapheme::O) => FilipinoGrapheme::O,
                    _ => FilipinoGrapheme::Other,
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
fn handle_consonant_d(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match (ctx.prev(), ctx.next()) {
        (Some(EnglishGrapheme::I), Some(EnglishGrapheme::E)) => Some((
            vec![
                FilipinoGrapheme::A,
                FilipinoGrapheme::Y,
                FilipinoGrapheme::D,
            ],
            1,
        )),
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
fn handle_consonant_g(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match ctx.next() {
        Some(
            EnglishGrapheme::E | EnglishGrapheme::I | EnglishGrapheme::Y | EnglishGrapheme::EE,
        ) => {
            // Check if NOT followed by s/c/k
            match ctx.lookahead(2) {
                Some(EnglishGrapheme::S | EnglishGrapheme::C | EnglishGrapheme::K) => None,
                _ => Some((
                    vec![
                        FilipinoGrapheme::DY,
                        match ctx.next() {
                            Some(EnglishGrapheme::E) => FilipinoGrapheme::E,
                            Some(EnglishGrapheme::I) => FilipinoGrapheme::I,
                            Some(EnglishGrapheme::Y) => FilipinoGrapheme::I,
                            _ => FilipinoGrapheme::Other,
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
fn handle_consonant_s(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match (ctx.prev(), ctx.next()) {
        (Some(EnglishGrapheme::EE | EnglishGrapheme::OO), Some(EnglishGrapheme::E)) => {
            match ctx.lookahead(2) {
                Some(EnglishGrapheme::B | EnglishGrapheme::D) => {
                    Some((vec![FilipinoGrapheme::S], 2))
                }
                _ => None,
            }
        }
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
fn sensitive_bigraph(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current();
    let next = ctx.next();

    match curr {
        EnglishGrapheme::CH => {
            if let Some(next) = next {
                if next.is_consonant() {
                    return Some((vec![FilipinoGrapheme::K], 1));
                }
            }

            Some((vec![FilipinoGrapheme::TS], 1))
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
pub fn handle_vowel(
    ctx: &Context,
    arpabet: &Vec<EnglishGrapheme>,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current();
    let idx = ctx.index;
    let letter = arpabet.get(idx)?.to_lowercase();

    match letter {
        EnglishGrapheme::A => handle_arpa_a(arpabet, idx),
        EnglishGrapheme::E => handle_arpa_e(arpabet, idx),
        EnglishGrapheme::I => handle_arpa_i(arpabet, idx),
        EnglishGrapheme::O => handle_arpa_o(arpabet, idx),
        EnglishGrapheme::U => handle_arpa_u(arpabet, idx),
        _ => None,
    }

    // match letter {
    //     EnglishGrapheme::ArpaAA | EnglishGrapheme::ArpaAO =>  Some((vec![FilipinoGrapheme::O], 1)),
    //     EnglishGrapheme::ArpaAE | EnglishGrapheme::ArpaAH => Some((vec![FilipinoGrapheme::A], 1)),
    //     EnglishGrapheme::ArpaAW => Some((vec![FilipinoGrapheme::A, FilipinoGrapheme::W], 1)),
    //     EnglishGrapheme::ArpaAY => Some((vec![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1)),
    //     EnglishGrapheme::ArpaEH => Some((vec![FilipinoGrapheme::E], 1)),
    //     EnglishGrapheme::ArpaER => Some((vec![FilipinoGrapheme::E, FilipinoGrapheme::R], 1)),
    //     EnglishGrapheme::ArpaEY => Some((vec![FilipinoGrapheme::E, FilipinoGrapheme::Y], 1)),
    //     EnglishGrapheme::ArpaIH | EnglishGrapheme::ArpaIY => Some((vec![FilipinoGrapheme::I], 1)),
    //     EnglishGrapheme::ArpaOW => Some((vec![FilipinoGrapheme::O], 1)),
    //     EnglishGrapheme::ArpaOY => Some((vec![FilipinoGrapheme::O, FilipinoGrapheme::Y], 1)),
    //     EnglishGrapheme::ArpaUH | EnglishGrapheme::ArpaUW => Some((vec![FilipinoGrapheme::U], 1)),
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
fn handle_arpa_a(
    arpabet: &Vec<EnglishGrapheme>,
    idx: usize,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(EnglishGrapheme::E | EnglishGrapheme::H) = next {
        return Some((vec![FilipinoGrapheme::A], 1));
    }

    if let Some(EnglishGrapheme::O | EnglishGrapheme::A) = next {
        return Some((vec![FilipinoGrapheme::O], 1));
    }

    if let Some(EnglishGrapheme::W) = next {
        return Some((vec![FilipinoGrapheme::A, FilipinoGrapheme::W], 1));
    }

    if let Some(EnglishGrapheme::Y) = next {
        return Some((vec![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1));
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
fn handle_arpa_e(
    arpabet: &Vec<EnglishGrapheme>,
    idx: usize,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(EnglishGrapheme::H) = next {
        return Some((vec![FilipinoGrapheme::E], 1));
    }

    if let Some(EnglishGrapheme::R) = next {
        return Some((vec![FilipinoGrapheme::E, FilipinoGrapheme::R], 1));
    }

    if let Some(EnglishGrapheme::Y) = next {
        return Some((vec![FilipinoGrapheme::E, FilipinoGrapheme::Y], 1));
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
fn handle_arpa_i(
    arpabet: &Vec<EnglishGrapheme>,
    idx: usize,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(EnglishGrapheme::H | EnglishGrapheme::Y) = next {
        return Some((vec![FilipinoGrapheme::I], 1));
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
fn handle_arpa_o(
    arpabet: &Vec<EnglishGrapheme>,
    idx: usize,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(EnglishGrapheme::W) = next {
        return Some((vec![FilipinoGrapheme::O], 1));
    }

    if let Some(EnglishGrapheme::Y) = next {
        return Some((vec![FilipinoGrapheme::O, FilipinoGrapheme::Y], 1));
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
fn handle_arpa_u(
    arpabet: &Vec<EnglishGrapheme>,
    idx: usize,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(EnglishGrapheme::H | EnglishGrapheme::W) = next {
        return Some((vec![FilipinoGrapheme::U], 1));
    }

    // if let Some(EnglishGrapheme::W) = next {
    //     return Some((vec![FilipinoGrapheme::Y, FilipinoGrapheme::U], 1))
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
fn handle_duplicates(
    ctx: &Context,
    config: &NativizationConfig,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current();
    if let Some(next) = ctx.next() {
        if next == curr
        // some symbol overload here: !matches!() is `NOT`matches!()
        // matches!() returns type bool.
        && !matches!(
                curr,
                EnglishGrapheme::Passthrough(_) | EnglishGrapheme::Space | EnglishGrapheme::Other
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
/// Returns `Some(Vec<FilipinoGrapheme>)` with the phonetic spelling, or `None` if
/// the grapheme is not a letter.
pub fn letter_to_phonetic(letter: EnglishGrapheme) -> Option<Vec<FilipinoGrapheme>> {
    let l = letter.to_lowercase();
    match l {
        EnglishGrapheme::A => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::Y]),
        EnglishGrapheme::B => Some(vec![FilipinoGrapheme::B, FilipinoGrapheme::I]),
        EnglishGrapheme::C => Some(vec![FilipinoGrapheme::S, FilipinoGrapheme::I]),
        EnglishGrapheme::D => Some(vec![FilipinoGrapheme::D, FilipinoGrapheme::I]),
        EnglishGrapheme::E => Some(vec![FilipinoGrapheme::I]),
        EnglishGrapheme::F => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::F]),
        EnglishGrapheme::G => Some(vec![
            FilipinoGrapheme::D,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::I,
        ]),
        EnglishGrapheme::H => Some(vec![
            FilipinoGrapheme::E,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::TS,
        ]),
        EnglishGrapheme::I => Some(vec![FilipinoGrapheme::A, FilipinoGrapheme::Y]),
        EnglishGrapheme::J => Some(vec![
            FilipinoGrapheme::DY,
            FilipinoGrapheme::E,
            FilipinoGrapheme::Y,
        ]),
        EnglishGrapheme::K => Some(vec![
            FilipinoGrapheme::K,
            FilipinoGrapheme::E,
            FilipinoGrapheme::Y,
        ]),
        EnglishGrapheme::L => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::L]),
        EnglishGrapheme::M => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::M]),
        EnglishGrapheme::N => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::N]),
        EnglishGrapheme::O => Some(vec![FilipinoGrapheme::O, FilipinoGrapheme::W]),
        EnglishGrapheme::P => Some(vec![FilipinoGrapheme::P, FilipinoGrapheme::I]),
        EnglishGrapheme::Q => Some(vec![
            FilipinoGrapheme::K,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::U,
        ]),
        EnglishGrapheme::R => Some(vec![FilipinoGrapheme::A, FilipinoGrapheme::R]),
        EnglishGrapheme::S => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::S]),
        EnglishGrapheme::T => Some(vec![FilipinoGrapheme::T, FilipinoGrapheme::I]),
        EnglishGrapheme::U => Some(vec![FilipinoGrapheme::Y, FilipinoGrapheme::U]),
        EnglishGrapheme::V => Some(vec![FilipinoGrapheme::B, FilipinoGrapheme::I]),
        EnglishGrapheme::W => Some(vec![
            FilipinoGrapheme::D,
            FilipinoGrapheme::A,
            FilipinoGrapheme::B,
            FilipinoGrapheme::O,
            FilipinoGrapheme::L,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::U,
        ]),
        EnglishGrapheme::X => Some(vec![
            FilipinoGrapheme::E,
            FilipinoGrapheme::K,
            FilipinoGrapheme::S,
        ]),
        EnglishGrapheme::Y => Some(vec![
            FilipinoGrapheme::W,
            FilipinoGrapheme::A,
            FilipinoGrapheme::Y,
        ]),
        EnglishGrapheme::Z => Some(vec![FilipinoGrapheme::S, FilipinoGrapheme::I]),
        _ => None,
    }
}
