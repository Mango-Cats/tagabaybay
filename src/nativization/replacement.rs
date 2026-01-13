use std::vec;

use super::context::Context;
use crate::consts::NativizationConfig;
use crate::tokenization::phl_graphemes::FilipinoGrapheme;
use crate::tokenization::src_graphemes::SourceGrapheme;

/// Convert an input grapheme to output grapheme(s) - context-free replacements
///
/// Handles straightforward grapheme-to-grapheme conversions that don't require
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
/// Returns `Some((FilipinoGrapheme, consumed))` if a context-free rule matches, where
/// `consumed` is typically 1. Returns `None` for context-sensitive letters.
pub fn free_replacement(
    ctx: &Context,
    config: &NativizationConfig,
) -> Option<(FilipinoGrapheme, usize)> {
    let g = ctx.current().to_lowercase();

    match g {
        // Digraph replacements (digraph count as 1 grapheme)
        SourceGrapheme::PH => Some((FilipinoGrapheme::F, 1)),
        SourceGrapheme::PS => Some((FilipinoGrapheme::S, 1)),
        SourceGrapheme::TH => Some((FilipinoGrapheme::T, 1)),
        SourceGrapheme::SH => {
            if config.allow_sh_sound {
                Some((FilipinoGrapheme::SH, 1))
            } else {
                Some((FilipinoGrapheme::S, 1))
            }
        }
        SourceGrapheme::EE => Some((FilipinoGrapheme::I, 1)),
        SourceGrapheme::OO => Some((FilipinoGrapheme::U, 1)),

        // Consonants
        SourceGrapheme::B => Some((FilipinoGrapheme::B, 1)),
        SourceGrapheme::D => Some((FilipinoGrapheme::D, 1)),
        SourceGrapheme::F => Some((FilipinoGrapheme::F, 1)),
        SourceGrapheme::G => Some((FilipinoGrapheme::G, 1)),
        SourceGrapheme::H => Some((FilipinoGrapheme::H, 1)),
        SourceGrapheme::K => Some((FilipinoGrapheme::K, 1)),
        SourceGrapheme::L => Some((FilipinoGrapheme::L, 1)),
        SourceGrapheme::M => Some((FilipinoGrapheme::M, 1)),
        SourceGrapheme::N => Some((FilipinoGrapheme::N, 1)),
        SourceGrapheme::P => Some((FilipinoGrapheme::P, 1)),
        SourceGrapheme::R => Some((FilipinoGrapheme::R, 1)),
        SourceGrapheme::S => Some((FilipinoGrapheme::S, 1)),
        SourceGrapheme::T => Some((FilipinoGrapheme::T, 1)),
        SourceGrapheme::V => Some((FilipinoGrapheme::B, 1)),
        SourceGrapheme::W => Some((FilipinoGrapheme::W, 1)),
        SourceGrapheme::Y => Some((FilipinoGrapheme::Y, 1)),
        SourceGrapheme::Z => {
            if config.allow_z_sound {
                Some((FilipinoGrapheme::Z, 1))
            } else {
                Some((FilipinoGrapheme::S, 1))
            }
        }

        // Spanish
        SourceGrapheme::Enye => Some((FilipinoGrapheme::N, 1)),

        // Whitespace
        SourceGrapheme::Space => Some((FilipinoGrapheme::Space, 1)),

        // ASCII passthrough (digits, punctuation, etc.)
        SourceGrapheme::Passthrough(c) => Some((FilipinoGrapheme::Passthrough(c.to_string()), 1)),

        // Context-sensitive letters (handled in sensitive_replacement)
        SourceGrapheme::C
        | SourceGrapheme::J
        | SourceGrapheme::Q
        | SourceGrapheme::X
        | SourceGrapheme::CH => None,

        // Other characters (pass through as-is)
        SourceGrapheme::Other => Some((FilipinoGrapheme::Other, 1)),

        // Uppercase variants should not reach here (normalized by to_lowercase)
        _ => None,
    }
}

/// Context-sensitive nativization (needs surrounding graphemes)
///
/// Handles grapheme-to-grapheme conversions that depend on surrounding context.
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
/// Returns `Some((FilipinoGrapheme, consumed))` if a context-sensitive rule matches.
/// Returns `None` if no rule applies (will print error).
pub fn sensitive_replacement(
    ctx: &Context,
    config: &NativizationConfig,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current();

    if curr.is_digraph() {
        sensitive_digraph(&ctx)
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
/// Returns `Some((FilipinoGrapheme, consumed))` if a rule matches, where `consumed` is the number
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
        SourceGrapheme::C => handle_consonant_c(&ctx),
        SourceGrapheme::X => handle_consonant_x(&ctx),
        SourceGrapheme::Y => handle_consonant_y(&ctx),
        SourceGrapheme::T => handle_consonant_t(&ctx),
        SourceGrapheme::D => handle_consonant_d(&ctx),
        SourceGrapheme::G => handle_consonant_g(&ctx),
        SourceGrapheme::S => handle_consonant_s(&ctx),
        SourceGrapheme::J => Some((vec![FilipinoGrapheme::DY], 1)),
        SourceGrapheme::Q => Some((vec![FilipinoGrapheme::K], 1)),
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
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_c(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match ctx.lookahead(1) {
        // 'c' before ('e' | 'i' | 'y') becomes 's'
        Some(SourceGrapheme::E | SourceGrapheme::I | SourceGrapheme::Y | SourceGrapheme::EE) => {
            Some((vec![FilipinoGrapheme::S], 1))
        }
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
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
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
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_y(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match (ctx.prev(), ctx.next()) {
        // 'y' after ('s' | 'l' | 'x') becomes 'i'
        (Some(SourceGrapheme::S | SourceGrapheme::L | SourceGrapheme::X), _) => {
            Some((vec![FilipinoGrapheme::I], 1))
        }
        // 'y' before 's' or 'l' becomes 'i'
        (_, Some(SourceGrapheme::S | SourceGrapheme::L)) => Some((vec![FilipinoGrapheme::I], 1)),
        // 'y' not preceded by 'a' becomes 'i'
        (Some(g), _) if g != SourceGrapheme::A => Some((vec![FilipinoGrapheme::I], 1)),
        (None, _) => Some((vec![FilipinoGrapheme::I], 1)), // 'y' at start becomes 'i'
        // 'y' preceded by 'a' - just emit 'y' (A already processed)
        (Some(SourceGrapheme::A), _) => Some((vec![FilipinoGrapheme::Y], 1)),
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
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_t(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match (ctx.prev(), ctx.next()) {
        // 'th' + (a|o) -> 'tay/toy'
        (Some(SourceGrapheme::H), Some(SourceGrapheme::A | SourceGrapheme::O)) => Some((
            vec![
                FilipinoGrapheme::T,
                FilipinoGrapheme::A,
                FilipinoGrapheme::Y,
                match ctx.next() {
                    Some(SourceGrapheme::A) => FilipinoGrapheme::A,
                    Some(SourceGrapheme::O) => FilipinoGrapheme::O,
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
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_d(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match (ctx.prev(), ctx.next()) {
        (Some(SourceGrapheme::I), Some(SourceGrapheme::E)) => Some((
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
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_g(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match ctx.next() {
        Some(SourceGrapheme::E | SourceGrapheme::I | SourceGrapheme::Y | SourceGrapheme::EE) => {
            // Check if NOT followed by s/c/k
            match ctx.lookahead(2) {
                Some(SourceGrapheme::S | SourceGrapheme::C | SourceGrapheme::K) => None,
                _ => Some((
                    vec![
                        FilipinoGrapheme::DY,
                        match ctx.next() {
                            Some(SourceGrapheme::E) => FilipinoGrapheme::E,
                            Some(SourceGrapheme::I) => FilipinoGrapheme::I,
                            Some(SourceGrapheme::Y) => FilipinoGrapheme::I,
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
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_s(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match (ctx.prev(), ctx.next()) {
        (Some(SourceGrapheme::EE | SourceGrapheme::OO), Some(SourceGrapheme::E)) => {
            match ctx.lookahead(2) {
                Some(SourceGrapheme::B | SourceGrapheme::D) => Some((vec![FilipinoGrapheme::S], 2)),
                _ => None,
            }
        }
        _ => None,
    }
}

/// digraph-specific context-sensitive rules
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` if a rule matches, `None` otherwise.
fn sensitive_digraph(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current();
    let next = ctx.next();

    match curr {
        SourceGrapheme::CH => {
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
/// Based on the current index of an loanword, get the corresponding
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
/// Returns `Some((FilipinoGrapheme, consumed))`
pub fn handle_vowel(
    ctx: &Context,
    arpabet: &Vec<SourceGrapheme>,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current();
    let idx = ctx.index;
    let letter = arpabet.get(idx)?.to_lowercase();

    match letter {
        SourceGrapheme::A => handle_arpa_a(arpabet, idx),
        SourceGrapheme::E => handle_arpa_e(arpabet, idx),
        SourceGrapheme::I => handle_arpa_i(arpabet, idx),
        SourceGrapheme::O => handle_arpa_o(arpabet, idx),
        SourceGrapheme::U => handle_arpa_u(arpabet, idx),
        _ => None,
    }

    // match letter {
    //     SourceGrapheme::ArpaAA | SourceGrapheme::ArpaAO =>  Some((vec![FilipinoGrapheme::O], 1)),
    //     SourceGrapheme::ArpaAE | SourceGrapheme::ArpaAH => Some((vec![FilipinoGrapheme::A], 1)),
    //     SourceGrapheme::ArpaAW => Some((vec![FilipinoGrapheme::A, FilipinoGrapheme::W], 1)),
    //     SourceGrapheme::ArpaAY => Some((vec![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1)),
    //     SourceGrapheme::ArpaEH => Some((vec![FilipinoGrapheme::E], 1)),
    //     SourceGrapheme::ArpaER => Some((vec![FilipinoGrapheme::E, FilipinoGrapheme::R], 1)),
    //     SourceGrapheme::ArpaEY => Some((vec![FilipinoGrapheme::E, FilipinoGrapheme::Y], 1)),
    //     SourceGrapheme::ArpaIH | SourceGrapheme::ArpaIY => Some((vec![FilipinoGrapheme::I], 1)),
    //     SourceGrapheme::ArpaOW => Some((vec![FilipinoGrapheme::O], 1)),
    //     SourceGrapheme::ArpaOY => Some((vec![FilipinoGrapheme::O, FilipinoGrapheme::Y], 1)),
    //     SourceGrapheme::ArpaUH | SourceGrapheme::ArpaUW => Some((vec![FilipinoGrapheme::U], 1)),
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
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_arpa_a(
    arpabet: &Vec<SourceGrapheme>,
    idx: usize,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(SourceGrapheme::E | SourceGrapheme::H) = next {
        return Some((vec![FilipinoGrapheme::A], 1));
    }

    if let Some(SourceGrapheme::O | SourceGrapheme::A) = next {
        return Some((vec![FilipinoGrapheme::O], 1));
    }

    if let Some(SourceGrapheme::W) = next {
        return Some((vec![FilipinoGrapheme::A, FilipinoGrapheme::W], 1));
    }

    if let Some(SourceGrapheme::Y) = next {
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
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_arpa_e(
    arpabet: &Vec<SourceGrapheme>,
    idx: usize,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(SourceGrapheme::H) = next {
        return Some((vec![FilipinoGrapheme::E], 1));
    }

    if let Some(SourceGrapheme::R) = next {
        return Some((vec![FilipinoGrapheme::E, FilipinoGrapheme::R], 1));
    }

    if let Some(SourceGrapheme::Y) = next {
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
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_arpa_i(
    arpabet: &Vec<SourceGrapheme>,
    idx: usize,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(SourceGrapheme::H | SourceGrapheme::Y) = next {
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
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_arpa_o(
    arpabet: &Vec<SourceGrapheme>,
    idx: usize,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(SourceGrapheme::W) = next {
        return Some((vec![FilipinoGrapheme::O], 1));
    }

    if let Some(SourceGrapheme::Y) = next {
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
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_arpa_u(
    arpabet: &Vec<SourceGrapheme>,
    idx: usize,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let next = arpabet.get(idx + 1);

    if let Some(SourceGrapheme::H | SourceGrapheme::W) = next {
        return Some((vec![FilipinoGrapheme::U], 1));
    }

    // if let Some(SourceGrapheme::W) = next {
    //     return Some((vec![FilipinoGrapheme::Y, FilipinoGrapheme::U], 1))
    // }

    None
}

/// Handle duplicate graphemes
///
/// Collapses repeated letters into single grapheme.
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
/// * `config` - Nativization configuration
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, 2))` if a duplicate is found (consuming 2 graphemes),
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
                SourceGrapheme::Passthrough(_) | SourceGrapheme::Space | SourceGrapheme::Other
            )
        {
            if let Some((graphemes, _)) = free_replacement(ctx, config) {
                return Some((Vec::from(vec![graphemes]), 2));
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
pub fn letter_to_phonetic(letter: SourceGrapheme) -> Option<Vec<FilipinoGrapheme>> {
    let l = letter.to_lowercase();
    match l {
        SourceGrapheme::A => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::Y]),
        SourceGrapheme::B => Some(vec![FilipinoGrapheme::B, FilipinoGrapheme::I]),
        SourceGrapheme::C => Some(vec![FilipinoGrapheme::S, FilipinoGrapheme::I]),
        SourceGrapheme::D => Some(vec![FilipinoGrapheme::D, FilipinoGrapheme::I]),
        SourceGrapheme::E => Some(vec![FilipinoGrapheme::I]),
        SourceGrapheme::F => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::F]),
        SourceGrapheme::G => Some(vec![
            FilipinoGrapheme::D,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::I,
        ]),
        SourceGrapheme::H => Some(vec![
            FilipinoGrapheme::E,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::TS,
        ]),
        SourceGrapheme::I => Some(vec![FilipinoGrapheme::A, FilipinoGrapheme::Y]),
        SourceGrapheme::J => Some(vec![
            FilipinoGrapheme::DY,
            FilipinoGrapheme::E,
            FilipinoGrapheme::Y,
        ]),
        SourceGrapheme::K => Some(vec![
            FilipinoGrapheme::K,
            FilipinoGrapheme::E,
            FilipinoGrapheme::Y,
        ]),
        SourceGrapheme::L => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::L]),
        SourceGrapheme::M => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::M]),
        SourceGrapheme::N => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::N]),
        SourceGrapheme::O => Some(vec![FilipinoGrapheme::O, FilipinoGrapheme::W]),
        SourceGrapheme::P => Some(vec![FilipinoGrapheme::P, FilipinoGrapheme::I]),
        SourceGrapheme::Q => Some(vec![
            FilipinoGrapheme::K,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::U,
        ]),
        SourceGrapheme::R => Some(vec![FilipinoGrapheme::A, FilipinoGrapheme::R]),
        SourceGrapheme::S => Some(vec![FilipinoGrapheme::E, FilipinoGrapheme::S]),
        SourceGrapheme::T => Some(vec![FilipinoGrapheme::T, FilipinoGrapheme::I]),
        SourceGrapheme::U => Some(vec![FilipinoGrapheme::Y, FilipinoGrapheme::U]),
        SourceGrapheme::V => Some(vec![FilipinoGrapheme::B, FilipinoGrapheme::I]),
        SourceGrapheme::W => Some(vec![
            FilipinoGrapheme::D,
            FilipinoGrapheme::A,
            FilipinoGrapheme::B,
            FilipinoGrapheme::O,
            FilipinoGrapheme::L,
            FilipinoGrapheme::Y,
            FilipinoGrapheme::U,
        ]),
        SourceGrapheme::X => Some(vec![
            FilipinoGrapheme::E,
            FilipinoGrapheme::K,
            FilipinoGrapheme::S,
        ]),
        SourceGrapheme::Y => Some(vec![
            FilipinoGrapheme::W,
            FilipinoGrapheme::A,
            FilipinoGrapheme::Y,
        ]),
        SourceGrapheme::Z => Some(vec![FilipinoGrapheme::S, FilipinoGrapheme::I]),
        _ => None,
    }
}
