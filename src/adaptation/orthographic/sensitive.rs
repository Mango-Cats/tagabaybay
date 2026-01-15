//! Context-sensitive consonant and digraph rules
//!
//! Handles grapheme-to-grapheme conversions that depend on surrounding context.
//! This includes soft c (cent→sent), position-dependent transformations
//! (x at start→s, otherwise→ks), and digraph patterns.

use crate::adaptation::cursor::Cursor;
use crate::configs::AdaptationConfig;
use crate::grapheme::filipino::FilipinoGrapheme;
use crate::grapheme::source::SourceGrapheme;

use super::free::free_replacement;

/// Context-sensitive adaptation (needs surrounding graphemes)
///
/// Handles grapheme-to-grapheme conversions that depend on surrounding context.
/// This includes soft c (cent→sent) and position-dependent
/// transformations (x at start→s, otherwise→ks).
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
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
        sensitive_digraph(ctx)
    } else if curr.is_consonant() {
        sensitive_consonant(ctx, config)
    } else if curr.is_vowel()
        && !(config.g2p_unpredictable_variants && curr.is_unpredictable_variant())
    {
        // keep the condition here explicit!
        sensitive_vowel(ctx)
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
        SourceGrapheme::C => handle_consonant_c(ctx),
        SourceGrapheme::X => handle_consonant_x(ctx),
        SourceGrapheme::Y => handle_consonant_y(ctx),
        SourceGrapheme::T => handle_consonant_t(ctx),
        SourceGrapheme::D => handle_consonant_d(ctx),
        SourceGrapheme::G => handle_consonant_g(ctx),
        SourceGrapheme::S => handle_consonant_s(ctx),
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
    let prev = ctx.prev_grapheme();
    let next = ctx.next_grapheme();

    // Check for patterns where 'y' represents long I sound (AY)
    // Common in medical/scientific terms: cy-, hy-, my-, dy-, py-, ty- followed by consonant
    match (&prev, &next) {
        // "cy" + consonant = "say" (cyanide, cycle, doxycycline)
        // This pattern works for cy- anywhere in the word
        (Some(SourceGrapheme::C), Some(n)) if n.is_consonant() => {
            Some((vec![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1))
        }
        // "hy" + consonant = "hay" (hydro, hydrogen)
        (Some(SourceGrapheme::H), Some(n)) if n.is_consonant() => {
            Some((vec![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1))
        }
        // "my" + consonant = "may" (mycin compounds)
        (Some(SourceGrapheme::M), Some(n)) if n.is_consonant() => {
            Some((vec![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1))
        }
        // "dy" + consonant (not after vowel) = "day" (dynamic)
        (Some(SourceGrapheme::D), Some(n)) if n.is_consonant() => {
            // Check if there's a vowel before 'd'
            let before_d = ctx.lookahead_grapheme(-2);
            match before_d {
                Some(v) if v.is_vowel() => Some((vec![FilipinoGrapheme::I], 1)),
                _ => Some((vec![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1)),
            }
        }
        // "py" + consonant = "pay" (pyrantel, pyrazinamide)
        (Some(SourceGrapheme::P), Some(n)) if n.is_consonant() => {
            Some((vec![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1))
        }
        // "ty" + consonant = "tay" (type)
        (Some(SourceGrapheme::T), Some(n)) if n.is_consonant() => {
            Some((vec![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1))
        }
        // 'y' after ('s' | 'l' | 'x') becomes 'i'
        // Note: 'x' already produces 'ks', so 'y' after it is just 'i'
        (Some(SourceGrapheme::S | SourceGrapheme::L | SourceGrapheme::X), _) => {
            Some((vec![FilipinoGrapheme::I], 1))
        }
        // 'y' before 's' or 'l' becomes 'i'
        (_, Some(SourceGrapheme::S | SourceGrapheme::L)) => Some((vec![FilipinoGrapheme::I], 1)),
        // 'y' not preceded by 'a' becomes 'i'
        (Some(g), _) if *g != SourceGrapheme::A => Some((vec![FilipinoGrapheme::I], 1)),
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

/// Digraph-specific context-sensitive rules
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

/// Handles vowel replacement based on context
///
/// Processes vowel graphemes and applies appropriate transformation rules
/// based on surrounding context and position.
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((Vec<FilipinoGrapheme>, consumed))` if a vowel rule matches,
/// `None` otherwise.
pub fn sensitive_vowel(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    match ctx.current_grapheme() {
        SourceGrapheme::A => handle_vowel_a(ctx),
        SourceGrapheme::E => handle_vowel_e(ctx),
        SourceGrapheme::I => handle_vowel_i(ctx),
        SourceGrapheme::O => handle_vowel_o(ctx),
        SourceGrapheme::U => handle_vowel_u(ctx),
        _ => None,
    }
}

/// Handle 'a' vowel patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGraphemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_vowel_a(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let next = ctx.next_grapheme();

    // check for "ate" pattern (a-t-e at end) → "eyt"
    if let Some(SourceGrapheme::T) = next {
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

    // "ai" → "ey" (wait, mail, paid)
    if let Some(SourceGrapheme::I) = next {
        return Some((vec![FilipinoGrapheme::E, FilipinoGrapheme::Y], 2));
    }

    // "ay" at end → "ey" (okay, pay, day)
    if let Some(SourceGrapheme::Y) = next {
        if ctx.position() + 1 == ctx.graphemes.len() - 1 {
            return Some((vec![FilipinoGrapheme::E, FilipinoGrapheme::Y], 2));
        }
    }

    // Magic-e pattern: a + consonant + e at end = "ey" (make, cake, save)
    if let Some(n) = &next {
        if n.is_consonant() && !n.is_digraph() {
            if let Some(SourceGrapheme::E) = ctx.lookahead_grapheme(2) {
                if ctx.position() + 2 == ctx.graphemes.len() - 1 {
                    // Return "ey" + the consonant (will consume 3: a, consonant, e)
                    return Some((
                        vec![FilipinoGrapheme::E, FilipinoGrapheme::Y],
                        1, // Just consume 'a', let consonant be processed next, then silent-e
                    ));
                }
            }
        }
    }

    None
}

/// Handle 'e' vowel patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGraphemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_vowel_e(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let next = ctx.next_grapheme();

    // remove trailing 'e' (silent e)
    if ctx.at_end() {
        return Some((vec![], 1));
    }

    // "ei" → "i" (receive, ceiling)
    if let Some(SourceGrapheme::I) = next {
        return Some((vec![FilipinoGrapheme::I], 2));
    }

    // "ea" + consonant → "i" (team, cheap, meat, read)
    if let Some(SourceGrapheme::A) = next {
        if let Some(after) = ctx.lookahead_grapheme(2) {
            if after.is_consonant() {
                return Some((vec![FilipinoGrapheme::I], 2));
            }
        }
        // "ea" at end → "iya" (idea) - but only if preceded by 'd' or 'r'
        // to avoid breaking "tea" which should be "ti"
        if ctx.position() + 1 == ctx.graphemes.len() - 1 {
            if let Some(prev) = ctx.prev_grapheme() {
                if prev == SourceGrapheme::D || prev == SourceGrapheme::R {
                    return Some((
                        vec![
                            FilipinoGrapheme::I,
                            FilipinoGrapheme::Y,
                            FilipinoGrapheme::A,
                        ],
                        2,
                    ));
                }
            }
            // Default: "ea" at end → "i" (tea → ti)
            return Some((vec![FilipinoGrapheme::I], 2));
        }
    }

    None
}

/// Handle 'i' vowel patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGraphemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_vowel_i(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let next = ctx.next_grapheme();

    // check for "ide" pattern (i-d-e at end) → "ayd"
    if let Some(SourceGrapheme::D) = next {
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

    // "igh" → "ay" (high, light, night, flight)
    if let Some(SourceGrapheme::G) = next {
        if let Some(SourceGrapheme::H) = ctx.lookahead_grapheme(2) {
            return Some((
                vec![FilipinoGrapheme::A, FilipinoGrapheme::Y],
                3, // Consume i, g, h
            ));
        }
    }

    // regular i + vowel patterns
    match next {
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

/// Handle 'o' vowel patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGraphemes, consumed))` if a pattern matches, `None` otherwise.
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

/// Handle 'u' vowel patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGraphemes, consumed))` if a pattern matches, `None` otherwise.
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
