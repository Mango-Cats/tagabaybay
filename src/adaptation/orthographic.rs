use std::vec;

use super::cursor::Cursor;
use crate::consts::AdaptationConfig;
use crate::grapheme::filipino::FilipinoGrapheme;
use crate::grapheme::source::SourceGrapheme;

/// Convert an input grapheme to output grapheme(s) - context-free replacements
///
/// Handles straightforward grapheme-to-grapheme conversions that don't require
/// context analysis.
///
/// # Arguments
///
/// * `graphemes` - The full sequence of graphemes
/// * `index` - Current position in the sequence
/// * `config` - Adaptation configuration (affects sh/z sounds)
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` if a context-free rule matches, where
/// `consumed` is typically 1. Returns `None` for context-sensitive letters.
pub fn free_replacement(
    ctx: &Cursor,
    config: &AdaptationConfig,
) -> Option<(FilipinoGrapheme, usize)> {
    let g = ctx.current_grapheme().to_lowercase();

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

        // Cursor-sensitive letters (handled in sensitive_replacement)
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

/// Cursor-sensitive adaptation (needs surrounding graphemes)
///
/// Handles grapheme-to-grapheme conversions that depend on surrounding context.
/// This includes soft c (cent→sent) and position-dependent
/// transformations (x at start→s, otherwise→ks).
///
/// # Arguments
///
/// * `graphemes` - The full sequence of graphemes
/// * `index` - Current position in the sequence
/// * `config` - Adaptation configuration
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` if a context-sensitive rule matches.
/// Returns `None` if no rule applies (will print error).
pub fn sensitive_replacement(
    ctx: &Cursor,
    config: &AdaptationConfig,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current_grapheme();

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
/// * `ctx` - Cursor containing the grapheme sequence and current position
/// * `config` - Adaptation configuration
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` if a rule matches, where `consumed` is the number
/// of graphemes processed. Returns `None` if no context-sensitive rule applies.
fn sensitive_consonant(
    ctx: &Cursor,
    config: &AdaptationConfig,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current_grapheme();

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
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_c(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match ctx.lookahead_grapheme(1) {
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
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_x(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
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
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_y(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match (ctx.prev_grapheme(), ctx.next_grapheme()) {
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
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_t(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match (ctx.prev_grapheme(), ctx.next_grapheme()) {
        // 'th' + (a|o) -> 'tay/toy'
        (Some(SourceGrapheme::H), Some(SourceGrapheme::A | SourceGrapheme::O)) => Some((
            vec![
                FilipinoGrapheme::T,
                FilipinoGrapheme::A,
                FilipinoGrapheme::Y,
                match ctx.next_grapheme() {
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
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_d(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match (ctx.prev_grapheme(), ctx.next_grapheme()) {
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
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_g(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match ctx.next_grapheme() {
        Some(SourceGrapheme::E | SourceGrapheme::I | SourceGrapheme::Y | SourceGrapheme::EE) => {
            // Check if NOT followed by s/c/k
            match ctx.lookahead_grapheme(2) {
                Some(SourceGrapheme::S | SourceGrapheme::C | SourceGrapheme::K) => None,
                _ => Some((
                    vec![
                        FilipinoGrapheme::DY,
                        match ctx.next_grapheme() {
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
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` if a pattern matches, `None` otherwise.
fn handle_consonant_s(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match (ctx.prev_grapheme(), ctx.next_grapheme()) {
        (Some(SourceGrapheme::EE | SourceGrapheme::OO), Some(SourceGrapheme::E)) => {
            match ctx.lookahead_grapheme(2) {
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
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` if a rule matches, `None` otherwise.
fn sensitive_digraph(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current_grapheme();
    let next = ctx.next_grapheme();

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
/// adaptation
///
/// Issue: Doesn't properly output vowels, could be with my logic here or
/// how im accessing it through adaptation.rs
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
/// * `arpabet` - Contains the grapheme vector for the ARPAbet
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))`
pub fn handle_vowel(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match ctx.current_grapheme() {
        SourceGrapheme::A => handle_vowel_a(&ctx),
        SourceGrapheme::E => handle_vowel_e(&ctx),
        SourceGrapheme::I => handle_vowel_i(&ctx),
        SourceGrapheme::O => handle_vowel_o(&ctx),
        SourceGrapheme::U => handle_vowel_u(&ctx),
        _ => None,
    }
}

/// handle 'a' vowel patterns
///
/// # arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # returns
///
/// returns `Some((FilipinoGraphemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_vowel_a(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // check for "ate" pattern (a-t-e at end) → "eyt"
    if let Some(SourceGrapheme::T) = ctx.next_grapheme() {
        if let Some(SourceGrapheme::E) = ctx.lookahead_grapheme(2) {
            if ctx.position() + 2 == ctx.graphemes.len() - 1 {
                return Some((
                    vec![
                        FilipinoGrapheme::E,
                        FilipinoGrapheme::Y,
                        FilipinoGrapheme::T,
                    ],
                    3,
                ));
            }
        }
    }
    None
}

/// handle 'e' vowel patterns
///
/// # arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # returns
///
/// returns `Some((FilipinoGraphemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_vowel_e(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // remove trailing 'e'
    if ctx.at_end() {
        return Some((vec![], 1));
    }

    // ei -> i (consume both e and i)
    match ctx.next_grapheme() {
        Some(SourceGrapheme::I) => Some((vec![FilipinoGrapheme::I], 2)),
        _ => None,
    }
}

/// handle 'i' vowel patterns
///
/// # arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # returns
///
/// returns `Some((FilipinoGraphemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_vowel_i(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // check for "ide" pattern (i-d-e at end) → "ayd"
    if let Some(SourceGrapheme::D) = ctx.next_grapheme() {
        if let Some(SourceGrapheme::E) = ctx.lookahead_grapheme(2) {
            if ctx.position() + 2 == ctx.graphemes.len() - 1 {
                return Some((
                    vec![
                        FilipinoGrapheme::A,
                        FilipinoGrapheme::Y,
                        FilipinoGrapheme::D,
                    ],
                    3,
                ));
            }
        }
    }

    // regular i + vowel patterns
    match ctx.next_grapheme() {
        Some(SourceGrapheme::A) => Some((
            vec![
                FilipinoGrapheme::I,
                FilipinoGrapheme::Y,
                FilipinoGrapheme::A,
            ],
            2,
        )),
        Some(SourceGrapheme::E) => Some((
            vec![
                FilipinoGrapheme::I,
                FilipinoGrapheme::Y,
                FilipinoGrapheme::E,
            ],
            2,
        )),
        Some(SourceGrapheme::O) => Some((
            vec![
                FilipinoGrapheme::I,
                FilipinoGrapheme::Y,
                FilipinoGrapheme::O,
            ],
            2,
        )),
        Some(SourceGrapheme::U) => Some((
            vec![
                FilipinoGrapheme::I,
                FilipinoGrapheme::Y,
                FilipinoGrapheme::U,
            ],
            2,
        )),
        _ => None,
    }
}

/// handle 'o' vowel patterns
///
/// # arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # returns
///
/// returns `Some((FilipinoGraphemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_vowel_o(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // check for "one" pattern (o-n-e at end) → "own"
    if let Some(SourceGrapheme::N) = ctx.next_grapheme() {
        if let Some(SourceGrapheme::E) = ctx.lookahead_grapheme(2) {
            if ctx.position() + 2 == ctx.graphemes.len() - 1 {
                return Some((
                    vec![
                        FilipinoGrapheme::O,
                        FilipinoGrapheme::W,
                        FilipinoGrapheme::N,
                    ],
                    3,
                ));
            }
        }
    }

    match ctx.next_grapheme() {
        Some(vowel) if vowel.is_vowel() => {
            // o + vowel -> oy + vowel (unless next is also a vowel)
            match ctx.lookahead_grapheme(2) {
                Some(v) if v.is_vowel() => None,
                _ => Some((
                    vec![
                        FilipinoGrapheme::O,
                        FilipinoGrapheme::Y,
                        match vowel {
                            SourceGrapheme::A => FilipinoGrapheme::A,
                            SourceGrapheme::E => FilipinoGrapheme::E,
                            SourceGrapheme::I => FilipinoGrapheme::I,
                            SourceGrapheme::O => FilipinoGrapheme::O,
                            SourceGrapheme::U => FilipinoGrapheme::U,
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

/// handle 'u' vowel patterns
///
/// # arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # returns
///
/// returns `Some((FilipinoGraphemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_vowel_u(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match ctx.next_grapheme() {
        Some(SourceGrapheme::A) => Some((
            vec![
                FilipinoGrapheme::U,
                FilipinoGrapheme::W,
                FilipinoGrapheme::A,
            ],
            2,
        )),
        Some(SourceGrapheme::E) => Some((
            vec![
                FilipinoGrapheme::U,
                FilipinoGrapheme::W,
                FilipinoGrapheme::E,
            ],
            2,
        )),
        Some(SourceGrapheme::I) => Some((
            vec![
                FilipinoGrapheme::U,
                FilipinoGrapheme::W,
                FilipinoGrapheme::I,
            ],
            2,
        )),
        Some(SourceGrapheme::O) => Some((
            vec![
                FilipinoGrapheme::U,
                FilipinoGrapheme::W,
                FilipinoGrapheme::O,
            ],
            2,
        )),
        Some(SourceGrapheme::U) => Some((
            vec![
                FilipinoGrapheme::U,
                FilipinoGrapheme::W,
                FilipinoGrapheme::U,
            ],
            2,
        )),
        _ => match ctx.prev_grapheme() {
            Some(SourceGrapheme::E) => Some((vec![FilipinoGrapheme::Y, FilipinoGrapheme::U], 1)),
            _ => None,
        },
    }
}

/// Handle duplicate graphemes
///
/// Collapses repeated letters into single grapheme.
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
/// * `config` - Adaptation configuration
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, 2))` if a duplicate is found (consuming 2 graphemes),
/// `None` otherwise.
fn handle_duplicates(
    ctx: &Cursor,
    config: &AdaptationConfig,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current_grapheme();
    if let Some(next) = ctx.next_grapheme() {
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
