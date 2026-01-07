use crate::consts::NativizationConfig;
use crate::nativization::context::Context;
use crate::tokenization::{graphemes::Grapheme, phoneme::Phoneme};

pub fn handle_vowel(ctx: &Context) {
    let curr = ctx.current();
}

/// Vowel-specific context-sensitive rules
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
fn sensitive_vowel(ctx: &Context, config: &NativizationConfig) -> Option<(Vec<Phoneme>, usize)> {
    match &ctx.current() {
        Grapheme::A => handle_vowel_a(&ctx),
        Grapheme::E => handle_vowel_e(&ctx),
        Grapheme::I => handle_vowel_i(&ctx),
        Grapheme::O => handle_vowel_o(&ctx),
        Grapheme::U => handle_vowel_u(&ctx),
        _ => None,
    }
}

/// Handle 'a' vowel patterns
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_vowel_a(ctx: &Context) -> Option<(Vec<Phoneme>, usize)> {
    // Check for "ate" pattern (a-t-e at end) → "eyt"
    if let Some(Grapheme::T) = ctx.next() {
        if let Some(Grapheme::E) = ctx.lookahead(2) {
            if ctx.position() + 2 == ctx.graphemes.len() - 1 {
                return Some((vec![Phoneme::E, Phoneme::Y, Phoneme::T], 3));
            }
        }
    }
    None
}

/// Handle 'e' vowel patterns
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_vowel_e(ctx: &Context) -> Option<(Vec<Phoneme>, usize)> {
    // remove trailing 'e'
    if ctx.at_end() {
        return Some((vec![], 1));
    }

    // ei -> i (consume both e and i)
    match ctx.next() {
        Some(Grapheme::I) => Some((vec![Phoneme::I], 2)),
        _ => None,
    }
}

/// Handle 'i' vowel patterns
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_vowel_i(ctx: &Context) -> Option<(Vec<Phoneme>, usize)> {
    // Check for "ide" pattern (i-d-e at end) → "ayd"
    if let Some(Grapheme::D) = ctx.next() {
        if let Some(Grapheme::E) = ctx.lookahead(2) {
            if ctx.position() + 2 == ctx.graphemes.len() - 1 {
                return Some((vec![Phoneme::A, Phoneme::Y, Phoneme::D], 3));
            }
        }
    }

    // Regular i + vowel patterns
    match ctx.next() {
        Some(Grapheme::A) => Some((vec![Phoneme::I, Phoneme::Y, Phoneme::A], 2)),
        Some(Grapheme::E) => Some((vec![Phoneme::I, Phoneme::Y, Phoneme::E], 2)),
        Some(Grapheme::O) => Some((vec![Phoneme::I, Phoneme::Y, Phoneme::O], 2)),
        Some(Grapheme::U) => Some((vec![Phoneme::I, Phoneme::Y, Phoneme::U], 2)),
        _ => None,
    }
}

/// Handle 'o' vowel patterns
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_vowel_o(ctx: &Context) -> Option<(Vec<Phoneme>, usize)> {
    // Check for "one" pattern (o-n-e at end) → "own"
    if let Some(Grapheme::N) = ctx.next() {
        if let Some(Grapheme::E) = ctx.lookahead(2) {
            if ctx.position() + 2 == ctx.graphemes.len() - 1 {
                return Some((vec![Phoneme::O, Phoneme::W, Phoneme::N], 3));
            }
        }
    }

    match ctx.next() {
        Some(vowel) if vowel.is_vowel() => {
            // o + vowel -> oy + vowel (unless next is also a vowel)
            match ctx.lookahead(2) {
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

/// Handle 'u' vowel patterns
///
/// # Arguments
///
/// * `ctx` - Context containing the grapheme sequence and current position
///
/// # Returns
///
/// Returns `Some((phonemes, consumed))` if a pattern matches, `None` otherwise.
fn handle_vowel_u(ctx: &Context) -> Option<(Vec<Phoneme>, usize)> {
    match ctx.next() {
        Some(Grapheme::A) => Some((vec![Phoneme::U, Phoneme::W, Phoneme::A], 2)),
        Some(Grapheme::E) => Some((vec![Phoneme::U, Phoneme::W, Phoneme::E], 2)),
        Some(Grapheme::I) => Some((vec![Phoneme::U, Phoneme::W, Phoneme::I], 2)),
        Some(Grapheme::O) => Some((vec![Phoneme::U, Phoneme::W, Phoneme::O], 2)),
        Some(Grapheme::U) => Some((vec![Phoneme::U, Phoneme::W, Phoneme::U], 2)),
        _ => match ctx.prev() {
            Some(Grapheme::E) => Some((vec![Phoneme::Y, Phoneme::U], 1)),
            _ => None,
        },
    }
}
