use crate::adaptation::cursor::Cursor;
use crate::tokenization::phl_graphemes::FilipinoGrapheme;
use crate::tokenization::src_graphemes::SourceGrapheme;

pub fn handle_vowel(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current();

    // dbg!(&ctx.ipa);
    // look for the specific ipa transcription the vowel/curr is referring to
    None
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
    if let Some(SourceGrapheme::T) = ctx.next() {
        if let Some(SourceGrapheme::E) = ctx.lookahead(2) {
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
    match ctx.next() {
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
    if let Some(SourceGrapheme::D) = ctx.next() {
        if let Some(SourceGrapheme::E) = ctx.lookahead(2) {
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
    match ctx.next() {
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
    if let Some(SourceGrapheme::N) = ctx.next() {
        if let Some(SourceGrapheme::E) = ctx.lookahead(2) {
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

    match ctx.next() {
        Some(vowel) if vowel.is_vowel() => {
            // o + vowel -> oy + vowel (unless next is also a vowel)
            match ctx.lookahead(2) {
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
    match ctx.next() {
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
        _ => match ctx.prev() {
            Some(SourceGrapheme::E) => Some((vec![FilipinoGrapheme::Y, FilipinoGrapheme::U], 1)),
            _ => None,
        },
    }
}
