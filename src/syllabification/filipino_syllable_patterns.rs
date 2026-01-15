//! Syllabification layer for Filipino.
//!
//! These functions perform the post-adaptation validation layer.
//! This checks whether the phonetic adaptation follows the
//! syllabification rules of Filipino.

use crate::grapheme::phl_graphemes::FilipinoGrapheme;

/// Check if a 6-Filipino grapheme sequence matches a `kkpkkk` pattern.
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
/// * `string` - A vector of Filipino graphemes to check
///
/// # Returns
///
/// Returns `true` if all positions match the expected consonant/vowel
/// types; otherwise returns `false`.
pub fn match_6_syllable(string: Vec<FilipinoGrapheme>) -> bool {
    if string.len() != 6 {
        return false;
    }

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

/// Check if a 5-Filipino grapheme sequence matches a `kkpkk` pattern.
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
/// * `string` - A vector of Filipino graphemes to check
///
/// # Returns
///
/// Returns `true` if the sequence matches the `kkpkk` pattern, `false` otherwise.
pub fn match_5_syllable(string: Vec<FilipinoGrapheme>) -> bool {
    if string.len() != 5 {
        return false;
    }

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

/// Check if a 4-Filipino grapheme sequence matches one of two patterns:
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
/// * `string` - A vector of Filipino graphemes to check
///
/// # Returns
///
/// Returns `true` if the sequence matches either `kkpk` or `kpkk` pattern,
/// `false` otherwise.
pub fn match_4_syllable(string: Vec<FilipinoGrapheme>) -> bool {
    if string.len() != 4 {
        return false;
    }

    let k0 = string.get(0).is_some_and(|x| x.is_consonant());

    let x1 = string.get(1).is_some_and(|x| x.is_consonant());
    let x2 = string.get(2).is_some_and(|x| x.is_vowel());

    let k3 = string.get(3).is_some_and(|x| x.is_consonant());

    // kkpk | kpkk
    if k0 && x1 == x2 && k3 { true } else { false }
}

/// Check if a 3-Filipino grapheme sequence matches one of the following:
///
/// - `kkp` (C C V)
/// - `kpk` (C V C)
/// - `pkk` (V C C)
///
/// Pattern notation: `k` = consonant, `p` = vowel (patinig)
///
/// The function first checks if the initial Filipino grapheme is a consonant, then
/// compares the remaining positions to ensure one vowel and one consonant
/// in the last two slots, or handles the `pkk` case when the first is a vowel.
///
/// # Arguments
///
/// * `string` - A vector of Filipino graphemes to check
///
/// # Returns
///
/// Returns `true` if the sequence matches `kkp`, `kpk`, or `pkk` pattern,
/// `false` otherwise.
pub fn match_3_syllable(string: Vec<FilipinoGrapheme>) -> bool {
    if string.len() != 3 {
        return false;
    }

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

/// Check if a 2-Filipino grapheme sequence matches either a `kp` or `pk` pattern.
///
/// Pattern notation: `k` = consonant, `p` = vowel (patinig)
///
/// This accepts:
///
/// - `kp` (C V)
/// - `pk` (V C)
///
/// by comparing whether the consonant/vowel status of the two Filipino graphemes differs.
///
/// # Arguments
///
/// * `string` - A vector of Filipino graphemes to check
///
/// # Returns
///
/// Returns `true` if the sequence matches `kp` or `pk` pattern, `false` otherwise.
pub fn match_2_syllable(string: Vec<FilipinoGrapheme>) -> bool {
    if string.len() != 2 {
        return false;
    }

    let x0 = string.get(0).is_some_and(|x| x.is_consonant());
    let x1 = string.get(1).is_some_and(|x| x.is_vowel());

    // kp | pk
    if x0 == x1 {
        return true;
    }

    false
}

/// Check if a 1-Filipino grapheme sequence is a vowel (`p`).
///
/// Pattern notation: `p` = vowel (patinig)
///
/// This is essentially a thin wrapper around `FilipinoGrapheme::is_vowel` and
/// checks only the first element in the vector.
///
/// # Arguments
///
/// * `string` - A vector of Filipino graphemes to check
///
/// # Returns
///
/// Returns `true` if the first Filipino grapheme is a vowel, `false` otherwise.
pub fn match_1_syllable(string: Vec<FilipinoGrapheme>) -> bool {
    string.len() == 1 && string.get(0).is_some_and(|x| x.is_vowel())
}

/// Validate that a Filipino grapheme sequence can be segmented into valid Filipino syllable patterns.
///
/// This function attempts to parse the entire Filipino grapheme sequence by greedily matching
/// the longest possible syllable pattern at each position. It tries patterns from
/// 6 Filipino graphemes down to 1 Filipino grapheme.
///
/// **Note**: This greedy approach may fail to find valid segmentations in some cases.
/// For guaranteed correctness, use [`validate_filipino_syllables_dp`] instead.
///
/// # Arguments
///
/// * `adapted` - A vector of Filipino graphemes to validate
///
/// # Returns
///
/// Returns `true` if the entire sequence can be segmented into valid syllable patterns
/// using a greedy approach, `false` otherwise.
pub fn validate_filipino_syllables_greedy(adapted: Vec<FilipinoGrapheme>) -> bool {
    let mut i = 0;

    while i < adapted.len() {
        let remaining = adapted.len() - i;
        let mut matched = false;

        // Try to match the longest syllable pattern possible
        if remaining >= 6 && match_6_syllable(adapted[i..i + 6].to_vec()) {
            i += 6;
            matched = true;
        } else if remaining >= 5 && match_5_syllable(adapted[i..i + 5].to_vec()) {
            i += 5;
            matched = true;
        } else if remaining >= 4 && match_4_syllable(adapted[i..i + 4].to_vec()) {
            i += 4;
            matched = true;
        } else if remaining >= 3 && match_3_syllable(adapted[i..i + 3].to_vec()) {
            i += 3;
            matched = true;
        } else if remaining >= 2 && match_2_syllable(adapted[i..i + 2].to_vec()) {
            i += 2;
            matched = true;
        } else if remaining >= 1 && match_1_syllable(adapted[i..i + 1].to_vec()) {
            i += 1;
            matched = true;
        }

        if !matched {
            return false;
        }
    }

    true
}

/// Validate that a Filipino grapheme sequence can be segmented into valid Filipino syllable patterns.
///
/// This function uses dynamic programming to determine if the entire Filipino grapheme sequence
/// can be segmented into valid syllable patterns. It builds up solutions for prefixes
/// of the sequence, trying all possible syllable lengths (1-6) at each position.
///
/// # Algorithm
///
/// - `dp[i]` = `true` if Filipino graphemes `[0..i]` can be validly segmented
/// - Base case: `dp[0]` = `true` (empty sequence)
/// - For each position `i`, try syllable lengths 1-6 and check if:
///   - `dp[i - len]` is `true` (prefix is valid), AND
///   - `[i-len..i]` matches a valid syllable pattern
///
/// # Arguments
///
/// * `adapted` - A vector of Filipino graphemes to validate
///
/// # Returns
///
/// Returns `true` if the entire sequence can be segmented into valid syllable patterns,
/// `false` otherwise.
pub fn validate_filipino_syllables_dp(adapted: Vec<FilipinoGrapheme>) -> bool {
    let n = adapted.len();
    if n == 0 {
        return true;
    }

    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 1..=n {
        // Try all possible syllable lengths ending at position i
        for len in 1..=6 {
            if len > i {
                break;
            }

            // Check if prefix [0..i-len] is valid and current syllable [i-len..i] matches
            if dp[i - len] {
                let syllable = adapted[i - len..i].to_vec();

                let is_valid = match len {
                    6 => match_6_syllable(syllable),
                    5 => match_5_syllable(syllable),
                    4 => match_4_syllable(syllable),
                    3 => match_3_syllable(syllable),
                    2 => match_2_syllable(syllable),
                    1 => match_1_syllable(syllable),
                    _ => false,
                };

                if is_valid {
                    dp[i] = true;
                    break;
                }
            }
        }
    }

    dp[n]
}
