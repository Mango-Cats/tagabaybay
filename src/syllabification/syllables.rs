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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_1_syllable_vowel() {
        // Single vowels should match
        assert!(match_1_syllable(vec![Phoneme::A]));
        assert!(match_1_syllable(vec![Phoneme::E]));
        assert!(match_1_syllable(vec![Phoneme::I]));
        assert!(match_1_syllable(vec![Phoneme::O]));
        assert!(match_1_syllable(vec![Phoneme::U]));
    }

    #[test]
    fn test_match_1_syllable_consonant() {
        // Single consonants should not match
        assert!(!match_1_syllable(vec![Phoneme::K]));
        assert!(!match_1_syllable(vec![Phoneme::B]));
        assert!(!match_1_syllable(vec![Phoneme::T]));
    }

    #[test]
    fn test_match_1_syllable_empty() {
        // Empty vector should not match
        assert!(!match_1_syllable(vec![]));
    }

    #[test]
    fn test_match_2_syllable_cv_pattern() {
        // Consonant-vowel (kp) should match
        assert!(match_2_syllable(vec![Phoneme::B, Phoneme::A]));
        assert!(match_2_syllable(vec![Phoneme::K, Phoneme::E]));
        assert!(match_2_syllable(vec![Phoneme::T, Phoneme::O]));
    }

    #[test]
    fn test_match_2_syllable_vc_pattern() {
        // Vowel-consonant (pk) should match
        assert!(match_2_syllable(vec![Phoneme::A, Phoneme::N]));
        assert!(match_2_syllable(vec![Phoneme::I, Phoneme::T]));
        assert!(match_2_syllable(vec![Phoneme::O, Phoneme::K]));
    }

    #[test]
    fn test_match_2_syllable_invalid() {
        // Consonant-consonant (kk) should not match
        assert!(!match_2_syllable(vec![Phoneme::B, Phoneme::K]));
        // Vowel-vowel (pp) should not match
        assert!(!match_2_syllable(vec![Phoneme::A, Phoneme::E]));
    }

    #[test]
    fn test_match_3_syllable_ccv_pattern() {
        // Consonant-consonant-vowel (kkp) should match
        assert!(match_3_syllable(vec![Phoneme::B, Phoneme::L, Phoneme::A]));
        assert!(match_3_syllable(vec![Phoneme::K, Phoneme::R, Phoneme::E]));
    }

    #[test]
    fn test_match_3_syllable_cvc_pattern() {
        // Consonant-vowel-consonant (kpk) should match
        assert!(match_3_syllable(vec![Phoneme::B, Phoneme::A, Phoneme::Y]));
        assert!(match_3_syllable(vec![Phoneme::T, Phoneme::A, Phoneme::K]));
    }

    #[test]
    fn test_match_3_syllable_vcc_pattern() {
        // Vowel-consonant-consonant (pkk) should match
        assert!(match_3_syllable(vec![Phoneme::A, Phoneme::N, Phoneme::G]));
        assert!(match_3_syllable(vec![Phoneme::I, Phoneme::T, Phoneme::S]));
    }

    #[test]
    fn test_match_3_syllable_invalid() {
        // Other patterns should not match
        assert!(!match_3_syllable(vec![Phoneme::A, Phoneme::E, Phoneme::I]));
        assert!(!match_3_syllable(vec![Phoneme::K, Phoneme::T, Phoneme::P]));
    }

    #[test]
    fn test_match_4_syllable_ccvc_pattern() {
        // Consonant-consonant-vowel-consonant (kkpk) should match
        assert!(match_4_syllable(vec![
            Phoneme::B,
            Phoneme::R,
            Phoneme::A,
            Phoneme::S
        ]));
        assert!(match_4_syllable(vec![
            Phoneme::K,
            Phoneme::L,
            Phoneme::A,
            Phoneme::N
        ]));
    }

    #[test]
    fn test_match_4_syllable_cvcc_pattern() {
        // Consonant-vowel-consonant-consonant (kpkk) should match
        assert!(match_4_syllable(vec![
            Phoneme::B,
            Phoneme::A,
            Phoneme::N,
            Phoneme::G
        ]));
        assert!(match_4_syllable(vec![
            Phoneme::T,
            Phoneme::A,
            Phoneme::N,
            Phoneme::S
        ]));
    }

    #[test]
    fn test_match_4_syllable_invalid() {
        // Other patterns should not match
        assert!(!match_4_syllable(vec![
            Phoneme::A,
            Phoneme::B,
            Phoneme::I,
            Phoneme::O
        ]));
        assert!(!match_4_syllable(vec![
            Phoneme::K,
            Phoneme::K,
            Phoneme::K,
            Phoneme::K
        ]));
    }

    #[test]
    fn test_match_5_syllable_valid() {
        // Consonant-consonant-vowel-consonant-consonant (kkpkk) should match
        assert!(match_5_syllable(vec![
            Phoneme::B,
            Phoneme::L,
            Phoneme::A,
            Phoneme::N,
            Phoneme::K
        ]));
        assert!(match_5_syllable(vec![
            Phoneme::K,
            Phoneme::R,
            Phoneme::O,
            Phoneme::S,
            Phoneme::T
        ]));
    }

    #[test]
    fn test_match_5_syllable_invalid() {
        // Wrong patterns should not match
        assert!(!match_5_syllable(vec![
            Phoneme::A,
            Phoneme::B,
            Phoneme::I,
            Phoneme::O,
            Phoneme::U
        ]));
        assert!(!match_5_syllable(vec![
            Phoneme::B,
            Phoneme::A,
            Phoneme::T,
            Phoneme::A,
            Phoneme::N
        ]));
    }

    #[test]
    fn test_match_6_syllable_valid() {
        // Consonant-consonant-vowel-consonant-consonant-consonant (kkpkkk) should match
        assert!(match_6_syllable(vec![
            Phoneme::K,
            Phoneme::L,
            Phoneme::A,
            Phoneme::S,
            Phoneme::K,
            Phoneme::T
        ]));
        assert!(match_6_syllable(vec![
            Phoneme::B,
            Phoneme::R,
            Phoneme::A,
            Phoneme::N,
            Phoneme::G,
            Phoneme::S
        ]));
    }

    #[test]
    fn test_match_6_syllable_invalid() {
        // Wrong patterns should not match
        assert!(!match_6_syllable(vec![
            Phoneme::A,
            Phoneme::B,
            Phoneme::I,
            Phoneme::O,
            Phoneme::U,
            Phoneme::E
        ]));
        assert!(!match_6_syllable(vec![
            Phoneme::B,
            Phoneme::A,
            Phoneme::T,
            Phoneme::A,
            Phoneme::N,
            Phoneme::G
        ]));
    }

    #[test]
    fn test_edge_cases() {
        // Test with longer sequences - should still check only the needed positions
        assert!(match_1_syllable(vec![Phoneme::A, Phoneme::B, Phoneme::T]));
        assert!(match_2_syllable(vec![Phoneme::K, Phoneme::A, Phoneme::N]));
    }
}
