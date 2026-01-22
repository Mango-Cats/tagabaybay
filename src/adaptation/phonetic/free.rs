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
use crate::error::PhonetizationError;
use crate::grapheme::filipino::FilipinoGrapheme;
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

    // Alignment see section (### Solution) in top-level documentation
    let pre_vowels = vowels_before(ctx);
    let phoneme = find_nth_vowel_phoneme(&ctx.phonemes, pre_vowels)?;

    let next_is_unpredictable_variant = ctx
        .next_grapheme_low()
        .map(|g| g.is_unpredictable_variant())
        .unwrap_or(false);

    if let Some((result, is_diphthong)) = graphemize(&phoneme) {
        let consumed = if is_diphthong && next_is_unpredictable_variant {
            2
        } else {
            1
        };
        Some((result, consumed))
    } else {
        let err = PhonetizationError::new(ctx.input_pronunciation.clone(), None, None);
        err.print_error();
        if config.panic_at_error {
            panic!("Phonetization failed: {:?}", err);
        }
        None
    }
}

/// Count vowel graphemes before current position
fn vowels_before(ctx: &Cursor) -> usize {
    let mut count = 0;
    for i in 0..ctx.index {
        let g = ctx.graphemes[i].to_lowercase();
        if g.is_unpredictable_variant() {
            // Create a temporary "view" to check if this was silent
            count += 1;
        }
    }
    count
}

/// Find the nth vowel phoneme in the phoneme sequence
fn find_nth_vowel_phoneme(phonemes: &[IPASymbol], n: usize) -> Option<IPASymbol> {
    phonemes.iter().filter(|p| p.is_vowel()).nth(n).cloned()
}
