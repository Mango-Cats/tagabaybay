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

use super::p2g::graphemize;
use crate::adaptation::cursor::Cursor;
use crate::configs::AdapterConfig;
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

    // Only process unpredictable variants (vowels and Y) and if config allows it
    if !curr.is_unpredictable_variant() || !config.g2p_unpredictable_variants {
        #[cfg(feature = "debug-trace")]
        println!("    [phon] not unpredictable variant or g2p disabled");
        return None;
    }

    // Adjust vowel index by counting non-silent vowels we've already seen
    let pre_vowels = vowels_before(ctx);
    #[cfg(feature = "debug-trace")]
    println!("    [phon] vowels_before={}", pre_vowels);

    let phoneme = find_nth_vowel_phoneme(&ctx.phonemes, pre_vowels)?;
    #[cfg(feature = "debug-trace")]
    println!("    [phon] matched phoneme={:?}", phoneme);

    // Check if next grapheme is also a vowel (might be consumed by diphthong)
    let next_is_vowel = ctx
        .next_grapheme_low()
        .map(|g| g.is_unpredictable_variant())
        .unwrap_or(false);

    // Convert ARPAbet phoneme to Filipino grapheme(s)
    if let Some((result, is_diphthong)) = graphemize(&phoneme) {
        let consumed = if is_diphthong && next_is_vowel { 2 } else { 1 };
        #[cfg(feature = "debug-trace")]
        println!(
            "    [phon] result={:?} diphthong={} consumed={}",
            result, is_diphthong, consumed
        );

        Some((result, consumed))
    } else {
        panic!("cant convert!!!")
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
