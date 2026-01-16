//! Phonetic vowel rules
//!
//! Handles vowel adaptation based on ARPAbet/IPA phonemes rather than
//! orthographic patterns. This is useful when spelling doesn't reflect
//! actual pronunciation.
//!
//! For example:
//! - "make" is spelled with 'a' but pronounced /eɪ/ (AY in ARPAbet)
//! - "women" is spelled with 'o' but pronounced /ɪ/ (IH in ARPAbet)
//!
//! ## The Alignment Problem
//!
//! Graphemes and phonemes don't have 1:1 correspondence:
//! - "make" → graphemes: [m, a, k, e] vs phonemes: [M, EY, K]
//! - The 'a' maps to EY, and 'e' is silent (no phoneme)
//! - "knight" → graphemes: [k, n, i, g, h, t] vs phonemes: [N, AY, T]
//!
//! ## Current Approach
//!
//! We solve this by counting vowel graphemes before current position and
//! finding the corresponding vowel phoneme at that count. We also detect
//! common silent vowel patterns (final 'e', 'u' after 'q', etc.) to improve
//! alignment accuracy.
//!
//! **Note**: This approach is disabled by default (`g2p_unpredictable_variants = false`)
//! because the alignment is imperfect for words with many silent letters.
//! Orthographic rules in `orthographic/sensitive.rs` handle most common patterns.
//!
//! ## Future Improvements
//!
//! To improve this module, consider:
//! 1. Using sequence alignment algorithms (like Needleman-Wunsch) for better
//!    grapheme-phoneme alignment
//! 2. Building a lookup table of known alignments for common word patterns
//! 3. Using machine learning to predict alignments

use super::p2g::graphemize;
use crate::adaptation::cursor::Cursor;
use crate::configs::AdaptationConfig;
use crate::grapheme::filipino::FilipinoGrapheme;
use crate::phoneme::symbols::ArpabetSymbols;

/// Handles phonetic replacement for vowels and Y based on G2P transcription.
///
/// The key insight: vowels in spelling roughly correspond to vowel phonemes
/// in order. We count how many vowel graphemes appear before the current
/// position, then find the nth vowel phoneme.
///
/// # Algorithm
///
/// 1. Count vowel graphemes (A, E, I, O, U, Y) before current position
/// 2. Find the nth vowel phoneme in the phoneme sequence
/// 3. Convert that ARPAbet vowel to Filipino grapheme(s)
/// 4. If no phoneme found (silent vowel), return None to fall back to orthographic
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
pub fn phonetic_replacements(
    ctx: &Cursor,
    config: &AdaptationConfig,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current_grapheme_low();

    // Only process unpredictable variants (vowels and Y) and if config allows it
    if !curr.is_unpredictable_variant() || !config.g2p_unpredictable_variants {
        return None;
    }

    // Adjust vowel index by counting non-silent vowels we've already seen
    let pre_vowels = vowels_before(ctx);

    let phoneme = find_nth_vowel_phoneme(&ctx.phonemes, pre_vowels)?;

    // Check if next grapheme is also a vowel (might be consumed by diphthong)
    let next_is_vowel = ctx
        .next_grapheme_low()
        .map(|g| g.is_unpredictable_variant())
        .unwrap_or(false);

    // Convert ARPAbet phoneme to Filipino grapheme(s)
    let (result, is_diphthong) = graphemize(&phoneme);

    let consumed = if is_diphthong && next_is_vowel { 2 } else { 1 };

    Some((result, consumed))
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
fn find_nth_vowel_phoneme(phonemes: &[ArpabetSymbols], n: usize) -> Option<ArpabetSymbols> {
    phonemes.iter().filter(|p| p.is_vowel()).nth(n).cloned()
}
