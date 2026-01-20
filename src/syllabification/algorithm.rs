//! Syllabification layer for Filipino.
//!
//! These functions perform the post-adaptation validation layer.
//! This checks whether the phonetic adaptation follows the
//! syllabification rules of Filipino.
//!
//! # Pattern Notation
//!
//! - `K` = consonant (katinig)
//! - `P` = vowel (patinig)
//!
//! Valid syllable patterns in Filipino:
//! - Length 1: `P`
//! - Length 2: `KP`, `PK`
//! - Length 3: `KKP`, `KPK`, `PKK`
//! - Length 4: `KKPK`, `KPKK`
//! - Length 5: `KKPKK`
//! - Length 6: `KKPKKK`

use crate::grapheme::filipino::FilipinoGrapheme;
use crate::syllabification::types::*;
use crate::tokens;

/// Syllabify a sequence of Filipino graphemes into syllables.
///
/// Returns `Some(Vec<Vec<FilipinoGrapheme>>)` with the syllables if valid,
/// or `None` if the sequence cannot be syllabified.
///
/// # Filipino Syllabification Rules
/// From 2001 Revisyon ng Alfabeto at Patnubay sa Ispeling ng Wikang Filipino
///
/// 1. When two different consonants are adjacent within a word, the first
///    belongs to the preceding vowel, the second to the following vowel.
///    - buksan → buk-san, pinto → pin-to
///
/// 2. When three or more different consonants are adjacent, the first two
///    belong to the preceding vowel, the last to the following vowel.
///    - eksperimento → eks-pe-ri-men-to
///
/// 3. Exception: If the first of three consonants is M or N, and the next
///    two form a cluster (bl, br, dr, pl, tr), then M/N goes with preceding
///    vowel and the cluster goes with following vowel.
///    - asambleya → a-sam-ble-ya, sentro → sen-tro
///
/// 4. When four consonants are adjacent, the first two go with preceding
///    vowel, the last two with following vowel.
///    - ekstradisyon → eks-tra-di-syon
///
/// # Arguments
///
/// * `graphemes` - the tokenized Filipino word to syllabify
///
/// # Returns
///
/// Returns the syllabification of `graphemes` and `true` if possible and valid.
/// If a syllabification is possible but INVALID, return the syllabification
/// and `false`. If impossible, then otherwise.
pub fn syllabify(graphemes: &[FilipinoGrapheme]) -> Option<(Vec<Vec<FilipinoGrapheme>>, bool)> {
    if graphemes.is_empty() {
        return Some((tokens![], true));
    }

    let vowel_positions: Vec<usize> = graphemes
        .iter()
        .enumerate()
        .filter(|(_, g)| g.is_vowel())
        .map(|(i, _)| i)
        .collect();

    if vowel_positions.is_empty() {
        return None;
    }

    let mut syllables: Vec<Vec<FilipinoGrapheme>> = Vec::new();
    let mut current_start = 0;

    for (i, &vowel_pos) in vowel_positions.iter().enumerate() {
        if i == vowel_positions.len() - 1 {
            // Last syllable: take everything from current_start to end
            syllables.push(graphemes[current_start..].to_vec());
        } else {
            // Find where to split between this vowel and the next
            let next_vowel_pos = vowel_positions[i + 1];
            let consonants_between = next_vowel_pos - vowel_pos - 1;

            // split depending on number of consonants
            let split_point = if consonants_between == 0 {
                vowel_pos + 1
            } else if consonants_between == 1 {
                vowel_pos + 1
            } else {
                let consonant_start = vowel_pos + 1;
                let consonant_end = next_vowel_pos;

                // Check for special clusters (bl, br, dr, pl, tr, kl, kr, etc.)
                // that should stay together with the following vowel
                // that is, multigraphs stay together
                let split = find_consonant_split(graphemes, consonant_start, consonant_end);
                split
            };

            syllables.push(graphemes[current_start..split_point].to_vec());
            current_start = split_point;
        }
    }

    // Validity check
    if syllables.iter().all(|s| matches_any_pattern(s)) {
        Some((syllables, true))
    } else {
        Some((syllables, false))
    }
}

/// Check if a grapheme matches a pattern element
#[inline]
fn matches_element(grapheme: &FilipinoGrapheme, pat: Pat) -> bool {
    match pat {
        Pat::K => grapheme.is_consonant(),
        Pat::P => grapheme.is_vowel(),
    }
}

/// Check if a slice of graphemes matches a pattern exactly
pub(crate) fn matches_pattern(graphemes: &[FilipinoGrapheme], pattern: &[Pat]) -> bool {
    graphemes.len() == pattern.len()
        && graphemes
            .iter()
            .zip(pattern.iter())
            .all(|(g, p)| matches_element(g, *p))
}

/// Try to match any pattern of a specific length
pub(crate) fn matches_any_pattern_of_length(graphemes: &[FilipinoGrapheme], len: usize) -> bool {
    if graphemes.len() != len {
        return false;
    }
    SYLLABLE_PATTERNS
        .iter()
        .filter(|p| p.len() == len)
        .any(|p| matches_pattern(graphemes, p))
}

/// Try to match any valid syllable pattern
fn matches_any_pattern(graphemes: &[FilipinoGrapheme]) -> bool {
    SYLLABLE_PATTERNS
        .iter()
        .any(|p| matches_pattern(graphemes, p))
}

/// Determine where to split a sequence of consonants between two vowels.
///
/// Filipino rules:
/// - 2 consonants: split after first (C|C) - each consonant goes to its nearest vowel
/// - 3 consonants:
///   - If first is M/N and last two form cluster (bl, br, dr, pl, tr): (C|CC)
///   - Otherwise: (CC|C) - first two with preceding
/// - 4+ consonants: split after second (CC|CC)
fn find_consonant_split(
    graphemes: &[FilipinoGrapheme],
    consonant_start: usize,
    consonant_end: usize,
) -> usize {
    let num_consonants = consonant_end - consonant_start;

    if num_consonants <= 1 {
        return consonant_start;
    }

    if num_consonants == 2 {
        let c1 = &graphemes[consonant_start];
        let c2 = &graphemes[consonant_start + 1];

        // SY cluster
        if matches!((c1, c2), (FilipinoGrapheme::S, FilipinoGrapheme::Y)) {
            return consonant_start;
        }

        return consonant_start + 1;
    }

    if num_consonants == 3 {
        let first = &graphemes[consonant_start];

        // General cluster
        if matches!(first, FilipinoGrapheme::M | FilipinoGrapheme::N) {
            if is_valid_onset_cluster(graphemes, consonant_start + 1) {
                return consonant_start + 1;
            }
        }

        // SY cluster
        let c2 = &graphemes[consonant_start + 1];
        let c3 = &graphemes[consonant_start + 2];
        if matches!((c2, c3), (FilipinoGrapheme::S, FilipinoGrapheme::Y)) {
            return consonant_start + 1;
        }

        return consonant_start + 2;
    }

    consonant_start + 2
}

/// Check if two consonants at the given position form a valid consonant cluster.
fn is_valid_onset_cluster(graphemes: &[FilipinoGrapheme], pos: usize) -> bool {
    if pos + 1 >= graphemes.len() {
        return false;
    }

    let c1 = &graphemes[pos];
    let c2 = &graphemes[pos + 1];

    // Common consonant clusters in Filipino (especially loanwords)
    matches!(
        (c1, c2),
        // Liquid clusters (C + L/R)
        (FilipinoGrapheme::B, FilipinoGrapheme::L)
            | (FilipinoGrapheme::B, FilipinoGrapheme::R)
            | (FilipinoGrapheme::K, FilipinoGrapheme::L)
            | (FilipinoGrapheme::K, FilipinoGrapheme::R)
            | (FilipinoGrapheme::D, FilipinoGrapheme::R)
            | (FilipinoGrapheme::F, FilipinoGrapheme::L)
            | (FilipinoGrapheme::F, FilipinoGrapheme::R)
            | (FilipinoGrapheme::G, FilipinoGrapheme::L)
            | (FilipinoGrapheme::G, FilipinoGrapheme::R)
            | (FilipinoGrapheme::P, FilipinoGrapheme::L)
            | (FilipinoGrapheme::P, FilipinoGrapheme::R)
            | (FilipinoGrapheme::T, FilipinoGrapheme::R)
            // S clusters
            | (FilipinoGrapheme::S, FilipinoGrapheme::K)
            | (FilipinoGrapheme::S, FilipinoGrapheme::L)
            | (FilipinoGrapheme::S, FilipinoGrapheme::M)
            | (FilipinoGrapheme::S, FilipinoGrapheme::N)
            | (FilipinoGrapheme::S, FilipinoGrapheme::P)
            | (FilipinoGrapheme::S, FilipinoGrapheme::T)
            | (FilipinoGrapheme::S, FilipinoGrapheme::W)
            // W clusters
            | (FilipinoGrapheme::K, FilipinoGrapheme::W)
            | (FilipinoGrapheme::T, FilipinoGrapheme::W)
            // Y clusters (glides)
            | (FilipinoGrapheme::P, FilipinoGrapheme::Y)
            | (FilipinoGrapheme::B, FilipinoGrapheme::Y)
            | (FilipinoGrapheme::K, FilipinoGrapheme::Y)
            | (FilipinoGrapheme::D, FilipinoGrapheme::Y)
            | (FilipinoGrapheme::G, FilipinoGrapheme::Y)
            | (FilipinoGrapheme::M, FilipinoGrapheme::Y)
            | (FilipinoGrapheme::N, FilipinoGrapheme::Y)
            | (FilipinoGrapheme::S, FilipinoGrapheme::Y)
            | (FilipinoGrapheme::T, FilipinoGrapheme::Y)
            | (FilipinoGrapheme::L, FilipinoGrapheme::Y)
    )
}
