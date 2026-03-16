//! Phonetic vowel rules
//!
//! Handles vowel adaptation based on IPA phonemes rather than
//! orthographic patterns. This is useful when spelling doesn't reflect
//! actual pronunciation.
//!
//! For example:
//! - "make" is spelled with 'a' but pronounced /eɪ/
//! - "women" is spelled with 'o' but pronounced /ɪ/
//!
//! ## The Alignment Problem
//!
//! Graphemes and phonemes don't have 1:1 correspondence:
//! - "make" → graphemes: [m, a, k, e] vs phonemes: [m, eɪ, k]
//! - The 'a' maps to eɪ, and 'e' is silent (no phoneme)
//! - "knight" → graphemes: [k, n, i, g, h, t] vs phonemes: [n, aɪ, t]
//!
//! ### Solution
//!
//! Since we only perform G2P on vowels, we can check how many vowels
//! are before the index we're processing (let this be N). And, skipping
//! N IPA vowel symbols, the first one MUST be the vowel we're looking at.
//!
//! ### Example
//!
//! Consider the example:
//! ```text
//! ctx.input_word =            eggplant
//! ctx.input_pronunciation=    ɛɡ.plænt
//! ctx.index =                 5   
//! ctx.current =               a
//! ```
//! then, N = 1 (there is only 1 vowel before 'a' -- 'e'). So,
//! skip the N = 1 vowel phonemes in `ctx.input_pronunciations`,
//! which skips the /ɛ/ symbol. So, we're left with /ɡ.plænt/.
//! The first (also, in this case, only) vowel pronunciation left must
//! be the IPA transcription of 'a'. In this case, /æ/.

use super::p2g::graphemize;
use crate::adaptation::cursor::Cursor;
use crate::configs::AdapterConfig;
use crate::error::{G2PError, G2PErrorKind};
use crate::grapheme::filipino::FilipinoGrapheme;
use crate::grapheme::source::SourceGrapheme;
use crate::phoneme::tokens::ipa::IPASymbol;

/// Handles phonetic replacement for vowels and Y based on G2P transcription.
///
/// # Arguments
///
/// * `ctx` - Cursor containing both graphemes and phonemes
///
/// # Returns
///
/// Returns `Some((Vec<FilipinoGrapheme>, consumed))` if the current grapheme
/// is a vowel/Y and we can find a corresponding phoneme.
/// Returns `None` if not applicable, alignment fails, or this is a silent vowel.
///
/// # Issues
///
/// - There is no proper alignment for phonetics.
/// - This relies on G2P. Question: what if we G2P(w) = p
///     and let w:=w1,w2,w3,...,wn and p:=p1,p2,p3,...pn
///     how do we check if the mapping wi -> pi is too far?
///     Example: crap -> crawp (isn't a -> aw too far given the context?)
pub fn phonetic_replacements(
    ctx: &Cursor,
    config: &AdapterConfig,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current_grapheme_low();

    if !curr.is_unpredictable_variant() || !config.g2p_unpredictable_variants {
        return None;
    }

    if is_secondary_vowel_in_cluster(ctx) {
        return Some((Vec::new(), 1));
    }

    // Alignment see section (### Solution) in top-level documentation
    let pre_vowels = vowel_clusters_before(ctx);
    let phoneme = find_nth_vowel_phoneme(&ctx.phonemes, pre_vowels)?;

    if is_medial_r_colored_vowel(ctx, &phoneme) {
        return Some((vec![preserve_written_vowel(curr)], 1));
    }

    if phoneme == IPASymbol::Schwa {
        if is_word_final_vowel(ctx) {
            let fg = match curr {
                SourceGrapheme::O | SourceGrapheme::U => FilipinoGrapheme::O,
                _ => FilipinoGrapheme::E,
            };
            return Some((vec![fg], 1));
        } else {
            return None;
        }
    }

    let next_is_unpredictable_variant = ctx
        .next_grapheme_low()
        .map(|g| g.is_unpredictable_variant())
        .unwrap_or(false);
    let vowel_cluster_len = vowel_cluster_len(ctx);

    if let Some((result, is_diphthong)) = graphemize(&phoneme) {
        let result = if result == [FilipinoGrapheme::A] {
            match curr {
                SourceGrapheme::O => vec![FilipinoGrapheme::O],
                SourceGrapheme::U => vec![FilipinoGrapheme::U],
                _ => result,
            }
        } else {
            result
        };

        let consumed = if curr.is_tetragraph() {
            1
        } else if vowel_cluster_len > 1 {
            vowel_cluster_len
        } else if is_diphthong && next_is_unpredictable_variant {
            2
        } else if next_is_unpredictable_variant
            && find_nth_vowel_phoneme(&ctx.phonemes, pre_vowels + 1).is_none()
        {
            2
        } else {
            1
        };
        Some((result, consumed))
    } else {
        let err = G2PError::with_input(
            G2PErrorKind::TranscriptionFailed {
                message: format!("no graphemization found for IPA symbol: {:?}", phoneme),
            },
            &ctx.input_pronunciation,
        );
        err.print_error();
        if config.panic_at_error {
            panic!("G2P failed: {:?}", err);
        }
        None
    }
}

/// Count written vowel clusters before the current position.
fn vowel_clusters_before(ctx: &Cursor) -> usize {
    let mut count = 0;
    for i in 0..ctx.index {
        let g = ctx.graphemes[i].to_lowercase();
        let prev_is_variant = i > 0
            && ctx.graphemes[i - 1]
                .to_lowercase()
                .is_unpredictable_variant();

        if g.is_unpredictable_variant() && !prev_is_variant {
            count += 1;
        } else if g == SourceGrapheme::ED && i < ctx.graphemes.len() - 1 {
            count += 1;
        } else if g == SourceGrapheme::TI && ti_behaves_like_vowel_cluster(ctx, i) {
            count += 1;
        }
    }
    count
}

fn ti_behaves_like_vowel_cluster(ctx: &Cursor, ti_index: usize) -> bool {
    let next = ctx.graphemes.get(ti_index + 1).map(|g| g.to_lowercase());

    !matches!(
        next,
        Some(SourceGrapheme::A | SourceGrapheme::E | SourceGrapheme::O | SourceGrapheme::U)
    )
}

fn is_secondary_vowel_in_cluster(ctx: &Cursor) -> bool {
    ctx.prev_grapheme_low()
        .map(|prev| prev.is_unpredictable_variant())
        .unwrap_or(false)
}

fn vowel_cluster_len(ctx: &Cursor) -> usize {
    let mut consumed = 1;

    while let Some(next) = ctx.lookat_grapheme_low(consumed as isize) {
        if next.is_unpredictable_variant() {
            consumed += 1;
        } else {
            break;
        }
    }

    consumed
}

fn preserve_written_vowel(curr: SourceGrapheme) -> FilipinoGrapheme {
    match curr {
        SourceGrapheme::A => FilipinoGrapheme::A,
        SourceGrapheme::E => FilipinoGrapheme::E,
        SourceGrapheme::I => FilipinoGrapheme::I,
        SourceGrapheme::O => FilipinoGrapheme::O,
        SourceGrapheme::U => FilipinoGrapheme::U,
        SourceGrapheme::Y => FilipinoGrapheme::I,
        _ => FilipinoGrapheme::A,
    }
}

fn is_medial_r_colored_vowel(ctx: &Cursor, phoneme: &IPASymbol) -> bool {
    matches!(phoneme, IPASymbol::RColoredSchwa | IPASymbol::RColoredMid)
        && ctx.next_grapheme_low() == Some(SourceGrapheme::R)
        && ctx.lookat_grapheme_low(2).is_some()
}

fn is_word_final_vowel(ctx: &Cursor) -> bool {
    for i in (ctx.index + 1)..ctx.graphemes.len() {
        let g = ctx.graphemes[i].to_lowercase();
        match g {
            SourceGrapheme::Space => return true,
            SourceGrapheme::Passthrough(ref s) if s == "-" => return true,
            g if g.is_unpredictable_variant() => return false,
            _ => {}
        }
    }
    true
}

/// Find the nth vowel phoneme in the phoneme sequence
fn find_nth_vowel_phoneme(phonemes: &[IPASymbol], n: usize) -> Option<IPASymbol> {
    phonemes.iter().filter(|p| p.is_vowel()).nth(n).cloned()
}
