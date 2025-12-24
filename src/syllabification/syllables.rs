//! Syllable pattern helpers for phoneme sequences.
//!
//! These functions operate on `Vec<Phoneme>` and check whether the
//! sequence matches a consonant/vowel pattern such as `kp`, `kkp`, or
//! `kkpkkk`, where `k` stands for a consonant and `p` (patinig) stands
//! for a vowel.
//!
//! The helpers are building blocks for higher-level syllabification and
//! are intended to be simple, predicate-style checks.

use crate::tokenization::phoneme::Phoneme;

/// Check if a 6-phoneme sequence matches a `kkpkkk` pattern.
///
/// Pattern notation: `k` = consonant, `p` = vowel (patinig)
///
/// - index 0: consonant (`k`)
/// - index 1: consonant (`k`)
/// - index 2: vowel     (`p`)
/// - index 3: consonant (`k`)
/// - index 4: consonant (`k`)
/// - index 5: consonant (`k`)
///
/// # Arguments
///
/// * `string` - A vector of phonemes to check
///
/// # Returns
///
/// Returns `true` if all positions match the expected consonant/vowel
/// types; otherwise returns `false`.
pub fn match_6_syllable(string: Vec<Phoneme>) -> bool {
    let k0 = string.get(0).is_some_and(|x| x.is_consonant());
    let k1 = string.get(1).is_some_and(|x| x.is_consonant());
    let k3 = string.get(3).is_some_and(|x| x.is_consonant());
    let k4 = string.get(4).is_some_and(|x| x.is_consonant());
    let k5 = string.get(5).is_some_and(|x| x.is_consonant());

    let v2 = string.get(2).is_some_and(|x| x.is_vowel());

    // kkpkkk
    if k0 && k1 && v2 && k3 && k4 && k5 {
        true
    } else {
        false
    }
}

/// Check if a 5-phoneme sequence matches a `kkpkk` pattern.
///
/// Pattern notation: `k` = consonant, `p` = vowel (patinig)
///
/// - index 0: consonant (`k`)
/// - index 1: consonant (`k`)
/// - index 2: vowel     (`p`)
/// - index 3: consonant (`k`)
/// - index 4: consonant (`k`)
///
/// # Arguments
///
/// * `string` - A vector of phonemes to check
///
/// # Returns
///
/// Returns `true` if the sequence matches the `kkpkk` pattern, `false` otherwise.
pub fn match_5_syllable(string: Vec<Phoneme>) -> bool {
    let k0 = string.get(0).is_some_and(|x| x.is_consonant());
    let k1 = string.get(1).is_some_and(|x| x.is_consonant());
    let k3 = string.get(3).is_some_and(|x| x.is_consonant());
    let k4 = string.get(4).is_some_and(|x| x.is_consonant());

    let v2 = string.get(2).is_some_and(|x| x.is_vowel());

    // kkpkk
    if k0 && k1 && v2 && k3 && k4 {
        true
    } else {
        false
    }
}

/// Check if a 4-phoneme sequence matches one of two patterns:
///
/// - `kkpk` (C C V C)
/// - `kpkk` (C V C C)
///
/// Pattern notation: `k` = consonant, `p` = vowel (patinig)
///
/// The function uses a compact comparison of positions 1 and 2 to allow
/// either consonant/vowel or vowel/consonant pairing.
///
/// # Arguments
///
/// * `string` - A vector of phonemes to check
///
/// # Returns
///
/// Returns `true` if the sequence matches either `kkpk` or `kpkk` pattern,
/// `false` otherwise.
pub fn match_4_syllable(string: Vec<Phoneme>) -> bool {
    let k0 = string.get(0).is_some_and(|x| x.is_consonant());

    let x1 = string.get(1).is_some_and(|x| x.is_consonant());
    let x2 = string.get(2).is_some_and(|x| x.is_vowel());

    let k3 = string.get(3).is_some_and(|x| x.is_consonant());

    // kkpk | kpkk
    if k0 && x1 == x2 && k3 { true } else { false }
}

/// Check if a 3-phoneme sequence matches one of the following:
///
/// - `kkp` (C C V)
/// - `kpk` (C V C)
/// - `pkk` (V C C)
///
/// Pattern notation: `k` = consonant, `p` = vowel (patinig)
///
/// The function first checks if the initial phoneme is a consonant, then
/// compares the remaining positions to ensure one vowel and one consonant
/// in the last two slots, or handles the `pkk` case when the first is a vowel.
///
/// # Arguments
///
/// * `string` - A vector of phonemes to check
///
/// # Returns
///
/// Returns `true` if the sequence matches `kkp`, `kpk`, or `pkk` pattern,
/// `false` otherwise.
pub fn match_3_syllable(string: Vec<Phoneme>) -> bool {
    let x0 = string.get(0).is_some_and(|x| x.is_consonant());
    if x0 == true {
        // kkp | kpk
        let x1 = string.get(1).is_some_and(|x| x.is_consonant());
        let x2 = string.get(2).is_some_and(|x| x.is_vowel());

        if x1 == x2 {
            return true;
        }
    } else {
        // pkk
        if string.get(1).is_some_and(|x| x.is_consonant())
            && string.get(2).is_some_and(|x| x.is_consonant())
        {
            return true;
        }
    }

    false
}

/// Check if a 2-phoneme sequence matches either a `kp` or `pk` pattern.
///
/// Pattern notation: `k` = consonant, `p` = vowel (patinig)
///
/// This accepts:
///
/// - `kp` (C V)
/// - `pk` (V C)
///
/// by comparing whether the consonant/vowel status of the two phonemes differs.
///
/// # Arguments
///
/// * `string` - A vector of phonemes to check
///
/// # Returns
///
/// Returns `true` if the sequence matches `kp` or `pk` pattern, `false` otherwise.
pub fn match_2_syllable(string: Vec<Phoneme>) -> bool {
    let x0 = string.get(0).is_some_and(|x| x.is_consonant());
    let x1 = string.get(1).is_some_and(|x| x.is_vowel());

    // kp | pk
    if x0 == x1 {
        return true;
    }

    false
}

/// Check if a 1-phoneme sequence is a vowel (`p`).
///
/// Pattern notation: `p` = vowel (patinig)
///
/// This is essentially a thin wrapper around `Phoneme::is_vowel` and
/// checks only the first element in the vector.
///
/// # Arguments
///
/// * `string` - A vector of phonemes to check
///
/// # Returns
///
/// Returns `true` if the first phoneme is a vowel, `false` otherwise.
pub fn match_1_syllable(string: Vec<Phoneme>) -> bool {
    string.get(0).is_some_and(|x| x.is_vowel())
}
