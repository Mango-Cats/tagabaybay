//! Context-sensitive consonant and digraph rules
//!
//! Handles grapheme-to-grapheme conversions that depend on surrounding context.
//! This includes soft c (cent→sent), position-dependent transformations
//! (x at start→s, otherwise→ks), and digraph patterns.

use crate::adaptation::cursor::Cursor;
use crate::configs::AdapterConfig;
use crate::grapheme::filipino::FilipinoGrapheme;
use crate::grapheme::source::SourceGrapheme;
use crate::tokens;

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
    config: &AdapterConfig,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current_grapheme_low();

    if curr.is_digraph() {
        sensitive_digraph(ctx, config)
    } else if curr.is_consonant() {
        sensitive_consonant(ctx, config)
    } else if curr.is_vowel() {
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
    config: &AdapterConfig,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current_grapheme_low();
    let next = ctx.next_grapheme_low();

    // remove duplicates
    if let Some(next) = next {
        if next == curr {
            if let Some(replacement) = handle_duplicates(ctx, config) {
                return Some(replacement);
            }
        }
    }

    match curr {
        SourceGrapheme::B => handle_consonant_b(ctx),
        SourceGrapheme::C => handle_consonant_c(ctx),
        SourceGrapheme::D => handle_consonant_d(ctx),
        SourceGrapheme::F => handle_consonant_f(ctx),
        SourceGrapheme::G => handle_consonant_g(ctx),
        SourceGrapheme::H => handle_consonant_h(ctx),
        SourceGrapheme::K => handle_consonant_k(ctx),
        SourceGrapheme::L => handle_consonant_l(ctx),
        SourceGrapheme::M => handle_consonant_m(ctx),
        SourceGrapheme::N => handle_consonant_n(ctx),
        SourceGrapheme::P => handle_consonant_p(ctx),
        SourceGrapheme::R => handle_consonant_r(ctx),
        SourceGrapheme::S => handle_consonant_s(ctx),   
        SourceGrapheme::T => handle_consonant_t(ctx),
        SourceGrapheme::V => handle_consonant_v(ctx),
        SourceGrapheme::X => handle_consonant_x(ctx),
        SourceGrapheme::Y => handle_consonant_y(ctx),
        SourceGrapheme::Z => handle_consonant_z(ctx),
        SourceGrapheme::J => {
            if config.allow_j_letter {
                Some((tokens![FilipinoGrapheme::J], 1))
            } else {
                Some((tokens![FilipinoGrapheme::DY], 1))
            }
        }
        SourceGrapheme::Q => Some((tokens![FilipinoGrapheme::K, FilipinoGrapheme::W], 1)),
        _ => None,
    }
}

/// Handle 'b' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_b(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // "-bed" → "-bd" (webbed, clubbed, scrubbed)
    match ctx.next_grapheme_low() {
        Some(SourceGrapheme::ED) => Some((
            tokens![
                FilipinoGrapheme::B,
                FilipinoGrapheme::D
            ],
            2,
        )),
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
    let next = ctx.next_grapheme_low();
    match next {
        // cc -> just k
        // handled at `handle_duplicates`

        // ck → just k
        // check, back, trick
        // consume both
        Some(SourceGrapheme::K) => Some((tokens![FilipinoGrapheme::K], 2)),

        // c(e|i|y) -> just s
        // cyst, cite, ceiling
        // consume self
        Some(SourceGrapheme::E | SourceGrapheme::I | SourceGrapheme::Y | SourceGrapheme::EE) => {
            Some((tokens![FilipinoGrapheme::S], 1))
        }

        // cch -> just k
        // saccharin
        // consume self
        Some(SourceGrapheme::CH) => Some((tokens![FilipinoGrapheme::K], 2)),

        // default 'k'
        // consume self
        _ => Some((tokens![FilipinoGrapheme::K], 1)),
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
    match (ctx.prev_grapheme_low(), ctx.next_grapheme_low()) {
        (Some(SourceGrapheme::I), Some(SourceGrapheme::E)) => Some((
            tokens![
                FilipinoGrapheme::A,
                FilipinoGrapheme::Y,
                FilipinoGrapheme::D,
            ],
            1,
        )),
        _ => None,
    }
}

/// Handle 'f' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_f(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // "-fed" → "-ft" (surfed, scoffed, bluffed)
    match ctx.next_grapheme_low() {
        Some(SourceGrapheme::ED) => Some((
            tokens![
                FilipinoGrapheme::F,
                FilipinoGrapheme::T
            ],
            2,
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
    // "-ged" → "-jd" (engaged, managed, damaged)
    if let Some(SourceGrapheme::ED) = ctx.next_grapheme_low() {
        return Some((tokens![FilipinoGrapheme::J, FilipinoGrapheme::D], 2));
    }

    // ge | gi | gy | gee
    match ctx.next_grapheme_low() {
        Some(SourceGrapheme::E | SourceGrapheme::I | SourceGrapheme::Y | SourceGrapheme::EE) => {
            match ctx.lookat_grapheme_low(2) {
                // not ge(s|c|k)
                Some(SourceGrapheme::S | SourceGrapheme::C | SourceGrapheme::K) => None,
                _ => Some((
                    tokens![
                        FilipinoGrapheme::DY,
                        match ctx.next_grapheme_low() {
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

/// Handle 'h' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_h(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // "heigh-" → "hay-" (height, heightened)
    match ctx.next_grapheme_low() {
        Some(SourceGrapheme::EIGH) => Some((
            tokens![
                FilipinoGrapheme::H,
                FilipinoGrapheme::A,
                FilipinoGrapheme::Y
            ],
            2,
        )),
        _ => None,
    }
}

/// Handle 'k' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_k(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // "-ked" → "-kt" (walked, baked, kicked)
    match ctx.next_grapheme_low() {
        Some(SourceGrapheme::ED) => Some((
            tokens![
                FilipinoGrapheme::K,
                FilipinoGrapheme::T
            ],
            2,
        )),
        _ => None,
    }
}

/// Handle 'l' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_l(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {

    // "laugh" → "laf" (laugh, laughter, laughing)
    // check if word starts with "l" since that's the only time "augh" will be "af"
    if ctx.position() == 0 {
        if let Some(SourceGrapheme::AUGH) = ctx.next_grapheme_low() {
            return Some((tokens![FilipinoGrapheme::L, FilipinoGrapheme::A, FilipinoGrapheme::F], 2));
        }

    }

    // "-led" → "-ld" (called, pulled, rolled)
    match ctx.next_grapheme_low() {
        Some(SourceGrapheme::ED) => Some((
            tokens![
                FilipinoGrapheme::L,
                FilipinoGrapheme::D
            ],
            2,
        )),
        _ => None,
    }
}

/// Handle 'm' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_m(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // "-med" → "-md" (claimed, named, framed)
    match ctx.next_grapheme_low() {
        Some(SourceGrapheme::ED) => Some((
            tokens![
                FilipinoGrapheme::M,
                FilipinoGrapheme::D
            ],
            2,
        )),
        _ => None,
    }
}

/// Handle 'n' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_n(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // "-ned" → "-nd" (turned, learned, earned)
    match ctx.next_grapheme_low() {
        Some(SourceGrapheme::ED) => Some((
            tokens![
                FilipinoGrapheme::N,
                FilipinoGrapheme::D
            ],
            2,
        )),
        _ => None,
    }
}

/// Handle 'p' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_p(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // "-ped" → "-pt" (helped, skipped, leaped)
    match ctx.next_grapheme_low() {
        Some(SourceGrapheme::ED) => Some((
            tokens![
                FilipinoGrapheme::P,
                FilipinoGrapheme::T
            ],
            2,
        )),
        _ => None,
    }
}

/// Handle 'r' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_r(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // "-red" → "-rd" (offered, entered, altered)
    match ctx.next_grapheme_low() {
        Some(SourceGrapheme::ED) => Some((
            tokens![
                FilipinoGrapheme::R,
                FilipinoGrapheme::D
            ],
            2,
        )),
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
    // "-sed" → "-st" (crossed, missed, passed)
    if let Some(SourceGrapheme::ED) = ctx.next_grapheme_low() {
        return Some((tokens![FilipinoGrapheme::S, FilipinoGrapheme::T], 2));
    }

    match (ctx.prev_grapheme_low(), ctx.next_grapheme_low()) {
        (Some(SourceGrapheme::EE | SourceGrapheme::OO), Some(SourceGrapheme::E)) => {
            match ctx.lookat_grapheme_low(2) {
                Some(SourceGrapheme::B | SourceGrapheme::D) => {
                    Some((tokens![FilipinoGrapheme::S], 2))
                }
                _ => None,
            }
        }
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
    match (ctx.prev_grapheme_low(), ctx.next_grapheme_low()) {
        // 'th' + (a|o) -> 'tay/toy'
        (Some(SourceGrapheme::H), Some(SourceGrapheme::A | SourceGrapheme::O)) => Some((
            tokens![
                FilipinoGrapheme::T,
                FilipinoGrapheme::A,
                FilipinoGrapheme::Y,
                match ctx.next_grapheme_low() {
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

/// Handle 'v' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_v(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // "-ved" → "-vd" (loved, saved, moved)
    match ctx.next_grapheme_low() {
        Some(SourceGrapheme::ED) => Some((
            tokens![
                FilipinoGrapheme::V,
                FilipinoGrapheme::D
            ],
            2,
        )),
        _ => None,
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
    // ^x -> 's'
    // consume self
    if ctx.at_start() {
        return Some((tokens![FilipinoGrapheme::S], 1));
    }

    // default 'ks'
    // consume self
    Some((tokens![FilipinoGrapheme::K, FilipinoGrapheme::S], 1))
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
    // "-yed" → "-yd" (played, stayed, prayed)
    if let Some(SourceGrapheme::ED) = ctx.next_grapheme_low() {
        return Some((tokens![FilipinoGrapheme::Y, FilipinoGrapheme::D], 2));
    }

    let prev = ctx.prev_grapheme_low();
    let next = ctx.next_grapheme_low();

    // Check for patterns where 'y' represents long I sound (AY)
    // Common in medical/scientific terms: cy-, hy-, my-, dy-, py-, ty- followed by consonant
    match (&prev, &next) {
        // "cy" + consonant = "say" (cyanide, cycle, doxycycline)
        // This pattern works for cy- anywhere in the word
        (Some(SourceGrapheme::C), Some(n)) if n.is_consonant() => {
            Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1))
        }
        // "hy" + consonant = "hay" (hydro, hydrogen)
        (Some(SourceGrapheme::H), Some(n)) if n.is_consonant() => {
            Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1))
        }
        // "my" + consonant = "may" (mycin compounds)
        (Some(SourceGrapheme::M), Some(n)) if n.is_consonant() => {
            Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1))
        }
        // "dy" + consonant (not after vowel) = "day" (dynamic)
        (Some(SourceGrapheme::D), Some(n)) if n.is_consonant() => {
            // Check if there's a vowel before 'd'
            let before_d = ctx.lookat_grapheme_low(-2);
            match before_d {
                Some(v) if v.is_vowel() => Some((tokens![FilipinoGrapheme::I], 1)),
                _ => Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1)),
            }
        }
        // "py" + consonant = "pay" (pyrantel, pyrazinamide)
        (Some(SourceGrapheme::P), Some(n)) if n.is_consonant() => {
            Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1))
        }
        // "ty" + consonant = "tay" (type)
        (Some(SourceGrapheme::T), Some(n)) if n.is_consonant() => {
            Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1))
        }
        // 'y' after ('s' | 'l' | 'x') becomes 'i'
        // Note: 'x' already produces 'ks', so 'y' after it is just 'i'
        (Some(SourceGrapheme::S | SourceGrapheme::L | SourceGrapheme::X), _) => {
            Some((tokens![FilipinoGrapheme::I], 1))
        }
        // 'y' before 's' or 'l' becomes 'i'
        (_, Some(SourceGrapheme::S | SourceGrapheme::L)) => Some((tokens![FilipinoGrapheme::I], 1)),
        // 'y' not preceded by 'a' becomes 'i'
        (Some(g), _) if *g != SourceGrapheme::A => Some((tokens![FilipinoGrapheme::I], 1)),
        (None, _) => Some((tokens![FilipinoGrapheme::I], 1)), // 'y' at start becomes 'i'
        // 'y' preceded by 'a' - just emit 'y' (A already processed)
        (Some(SourceGrapheme::A), _) => Some((tokens![FilipinoGrapheme::Y], 1)),
        _ => None,
    }
}

/// Handle 'z' consonant patterns
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` with the appropriate conversion.
fn handle_consonant_z(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // "-zed" → "-zd" (realized, legalized, recognized)
    match ctx.next_grapheme_low() {
        Some(SourceGrapheme::ED) => Some((
            tokens![
                FilipinoGrapheme::Z,
                FilipinoGrapheme::D
            ],
            2,
        )),
        _ => None,
    }
}

/// Digraph-specific context-sensitive rules
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
/// * `config` - Adaptation configuration

/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` if a rule matches, `None` otherwise.
fn sensitive_digraph(
    ctx: &Cursor,
    config: &AdapterConfig,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current_grapheme_low();
    let next = ctx.next_grapheme_low();
    let prev = ctx.prev_grapheme_low();

    match curr {
        SourceGrapheme::CH => {
            if let Some(next) = next {
                if next.is_consonant() {
                    return Some((tokens![FilipinoGrapheme::K], 1));
                }
            }

            match prev {
                // sch- as in schedule
                Some(SourceGrapheme::S) => return Some((tokens![FilipinoGrapheme::K], 1)),
                _ => {}
            }

            Some((tokens![FilipinoGrapheme::TS], 1))
        }

        SourceGrapheme::SH => {
            if config.allow_sh_letter {
                Some((tokens![FilipinoGrapheme::SH], 1))
            } else if ctx.at_end() {
                Some((tokens![FilipinoGrapheme::SY], 1))
            } else {
                Some((tokens![FilipinoGrapheme::S], 1))
            }
        }

        SourceGrapheme::WH => {
            match next {
                Some(SourceGrapheme::O) => return Some((tokens![FilipinoGrapheme::H], 1)),
                _ => {}
            }

            Some((tokens![FilipinoGrapheme::W], 1))
        }

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
    match ctx.current_grapheme_low() {
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
    let next = ctx.next_grapheme_low();

    // check for "ate" pattern (a-t-e at end) → "eyt"
    if let Some(SourceGrapheme::T) = next {
        if let Some(SourceGrapheme::E) = ctx.lookat_grapheme_low(2) {
            if ctx.position() + 2 == ctx.graphemes.len() - 1 {
                return Some((
                    tokens![
                        FilipinoGrapheme::E,
                        FilipinoGrapheme::Y,
                        FilipinoGrapheme::T,
                    ],
                    3,
                ));
            }
        }
    }

    // "a" before "ll" → "o" (call, mall, ball, all)
    if let Some(SourceGrapheme::L) = next {
        if let Some(SourceGrapheme::L) = ctx.lookat_grapheme_low(2) {
            return Some((tokens![FilipinoGrapheme::O], 1));
        }
    }

    // "ai" → "ey" (wait, mail, paid)
    if let Some(SourceGrapheme::I) = next {
        return Some((tokens![FilipinoGrapheme::E, FilipinoGrapheme::Y], 2));
    }

    // "ay" at end → "ey" (okay, pay, day)
    if let Some(SourceGrapheme::Y) = next {
        if ctx.position() + 1 == ctx.graphemes.len() - 1 {
            return Some((tokens![FilipinoGrapheme::E, FilipinoGrapheme::Y], 2));
        }
    }

    if let Some(SourceGrapheme::U) = next {
        if let Some(SourceGrapheme::GH) = ctx.lookat_grapheme_low(2) {
            // "aught" → "ot" (caught, naught, daughter)
            if let Some(SourceGrapheme::T) = ctx.lookat_grapheme_low(3) {
                return Some((tokens![FilipinoGrapheme::O, FilipinoGrapheme::T], 4));
            }
            // "augh" → "af"
            return Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::F], 3));
        }
    }

    // Magic-e pattern: a + consonant + e at end = "ey" (make, cake, save)
    if let Some(n) = &next {
        if n.is_consonant() && !n.is_digraph() {
            if let Some(SourceGrapheme::E) = ctx.lookat_grapheme_low(2) {
                if ctx.position() + 2 == ctx.graphemes.len() - 1 {
                    // Return "ey" + the consonant (will consume 3: a, consonant, e)
                    return Some((
                        tokens![FilipinoGrapheme::E, FilipinoGrapheme::Y],
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
    let next = ctx.next_grapheme_low();

    // remove trailing 'e' (silent e)
    if ctx.at_end() {
        return Some((tokens![], 1));
    }
    
    // "ei" → "i" (receive, ceiling)
    if let Some(SourceGrapheme::I) = next {
        return Some((tokens![FilipinoGrapheme::I], 2));
    }

    // "ea" + consonant → "i" (team, cheap, meat, read)
    if let Some(SourceGrapheme::A) = next {
        if let Some(after) = ctx.lookat_grapheme_low(2) {
            if after.is_consonant() {
                return Some((tokens![FilipinoGrapheme::I], 2));
            }
        }
        // "ea" at end → "iya" (idea) - but only if preceded by 'd' or 'r'
        // to avoid breaking "tea" which should be "ti"
        if ctx.position() + 1 == ctx.graphemes.len() - 1 {
            if let Some(prev) = ctx.prev_grapheme_low() {
                if prev == SourceGrapheme::D || prev == SourceGrapheme::R {
                    return Some((
                        tokens![
                            FilipinoGrapheme::I,
                            FilipinoGrapheme::Y,
                            FilipinoGrapheme::A,
                        ],
                        2,
                    ));
                }
            }
            // Default: "ea" at end → "i" (tea → ti)
            return Some((tokens![FilipinoGrapheme::I], 2));
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
    // "-ied" → "-ayd" (cried, tried, fried)
    if let Some(SourceGrapheme::ED) = ctx.next_grapheme_low() {
        return Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::Y, FilipinoGrapheme::D], 2));
    }

    let next = ctx.next_grapheme_low();

    // check for "ide" pattern (i-d-e at end) → "ayd"
    if let Some(SourceGrapheme::D) = next {
        if let Some(SourceGrapheme::E) = ctx.lookat_grapheme_low(2) {
            if ctx.position() + 2 == ctx.graphemes.len() - 1 {
                return Some((
                    tokens![
                        FilipinoGrapheme::A,
                        FilipinoGrapheme::Y,
                        FilipinoGrapheme::D,
                    ],
                    3,
                ));
            }
        }
    }

    // "ise|ize" -> "ays"
    if let Some(SourceGrapheme::S | SourceGrapheme::Z) = next {
        if let Some(SourceGrapheme::E) = ctx.lookat_grapheme_low(2) {
            if ctx.position() + 2 == ctx.graphemes.len() - 1 {
                return Some((
                    tokens![
                        FilipinoGrapheme::A,
                        FilipinoGrapheme::Y,
                        FilipinoGrapheme::S,
                    ],
                    3,
                ));
            }
        }
    }

    // "igh" → "ay" (high, light, night, flight)
    if let Some(SourceGrapheme::GH) = next {
            return Some((
                tokens![FilipinoGrapheme::A, FilipinoGrapheme::Y],
                2, // Consume i, gh
            ));
    }

    // regular i + vowel patterns
    match next {
        Some(SourceGrapheme::A) => Some((
            tokens![
                FilipinoGrapheme::I,
                FilipinoGrapheme::Y,
                FilipinoGrapheme::A,
            ],
            2,
        )),
        Some(SourceGrapheme::E) => Some((
            tokens![
                FilipinoGrapheme::I,
                FilipinoGrapheme::Y,
                FilipinoGrapheme::E,
            ],
            2,
        )),
        Some(SourceGrapheme::O) => Some((
            tokens![
                FilipinoGrapheme::I,
                FilipinoGrapheme::Y,
                FilipinoGrapheme::O,
            ],
            2,
        )),
        Some(SourceGrapheme::U) => Some((
            tokens![
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
    let next = ctx.next_grapheme_low();
    match next {
        // check for "one" pattern (o-n-e at end) → "own"
        Some(SourceGrapheme::N) => {
            if let Some(SourceGrapheme::E) = ctx.lookat_grapheme_low(2) {
                if ctx.position() + 2 == ctx.graphemes.len() - 1 {
                    return Some((
                        tokens![
                            FilipinoGrapheme::O,
                            FilipinoGrapheme::W,
                            FilipinoGrapheme::N,
                        ],
                        3,
                    ));
                }
            }

            None
        }

        // "ou" before consonant → "aw" (count, out, account, discount)
        Some(SourceGrapheme::U) => {
            if let Some(after) = ctx.lookat_grapheme_low(2) {
                if after.is_consonant() {
                    return Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::W], 2));
                }
            }
            // "ou" at end → "aw" (you)
            if ctx.position() + 1 == ctx.graphemes.len() - 1 {
                return Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::W], 2));
            }

            None
        }
        // "oa" + consonant → "ow" (loan, road, coat)
        Some(SourceGrapheme::A) => {
            if let Some(after) = ctx.lookat_grapheme_low(2) {
                if after.is_consonant() {
                    return Some((tokens![FilipinoGrapheme::O, FilipinoGrapheme::W], 2));
                }
            }

            None
        }

        // "oi" -> "oy" (oil -> oyl)
        Some(SourceGrapheme::I) => {
            return Some((tokens![FilipinoGrapheme::O, FilipinoGrapheme::Y], 2));
        }

        // o + vowel -> oy + vowel (unless next is also a vowel)
        Some(vowel) if vowel.is_vowel() => match ctx.lookat_grapheme_low(2) {
            Some(v) if v.is_vowel() => None,
            _ => Some((
                tokens![
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
        },

        None | _ => return None,
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
    match ctx.next_grapheme_low() {
        Some(SourceGrapheme::A) => Some((
            tokens![
                FilipinoGrapheme::U,
                FilipinoGrapheme::W,
                FilipinoGrapheme::A,
            ],
            2,
        )),
        Some(SourceGrapheme::E) => Some((
            tokens![
                FilipinoGrapheme::U,
                FilipinoGrapheme::W,
                FilipinoGrapheme::E,
            ],
            2,
        )),
        Some(SourceGrapheme::I) => Some((
            tokens![
                FilipinoGrapheme::U,
                FilipinoGrapheme::W,
                FilipinoGrapheme::I,
            ],
            2,
        )),
        Some(SourceGrapheme::O) => Some((
            tokens![
                FilipinoGrapheme::U,
                FilipinoGrapheme::W,
                FilipinoGrapheme::O,
            ],
            2,
        )),
        Some(SourceGrapheme::U) => Some((
            tokens![
                FilipinoGrapheme::U,
                FilipinoGrapheme::W,
                FilipinoGrapheme::U,
            ],
            2,
        )),
        _ => match ctx.prev_grapheme_low() {
            Some(SourceGrapheme::E) => Some((tokens![FilipinoGrapheme::Y, FilipinoGrapheme::U], 1)),
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
    config: &AdapterConfig,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current_grapheme_low();
    let mut consumed = 1;

    while let Some(next) = ctx.lookat_grapheme_low(consumed as isize) {
        if next == curr {
            consumed += 1;
        } else {
            break;
        }
    }

    // don't care about the consumed count here
    if let Some((repl, _)) = free_replacement(ctx, config) {
        return Some((repl, consumed));
    }

    None
}
