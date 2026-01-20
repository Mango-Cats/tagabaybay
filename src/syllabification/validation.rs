use super::algorithm::matches_any_pattern_of_length;
use crate::grapheme::filipino::FilipinoGrapheme;

/// Validate that a Filipino grapheme sequence can be segmented into valid Filipino syllable patterns.
///
/// This function attempts to parse the entire Filipino grapheme sequence by greedily matching
/// the longest possible syllable pattern at each position. It tries patterns from
/// 6 Filipino graphemes down to 1 Filipino grapheme.
///
/// **Note**: This greedy approach may fail to find valid segmentations in some cases.
/// For guaranteed correctness, use [`validate_filipino_syllables_dp`] instead.
pub fn validate_filipino_syllables_greedy(adapted: Vec<FilipinoGrapheme>) -> bool {
    let mut i = 0;

    while i < adapted.len() {
        let remaining = adapted.len() - i;
        let mut matched = false;

        // Try to match the longest syllable pattern possible (6 down to 1)
        for len in (1..=6.min(remaining)).rev() {
            if matches_any_pattern_of_length(&adapted[i..i + len], len) {
                i += len;
                matched = true;
                break;
            }
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
/// - `dp[i]` = `true` if graphemes `[0..i]` can be validly segmented
/// - Base case: `dp[0]` = `true` (empty sequence)
/// - For each position `i`, try syllable lengths 1-6 and check if:
///   - `dp[i - len]` is `true` (prefix is valid), AND
///   - `[i-len..i]` matches a valid syllable pattern
pub fn validate_filipino_syllables_dp(adapted: Vec<FilipinoGrapheme>) -> bool {
    let n = adapted.len();
    if n == 0 {
        return true;
    }

    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 1..=n {
        // Try all possible syllable lengths ending at position i
        for len in 1..=6.min(i) {
            if dp[i - len] && matches_any_pattern_of_length(&adapted[i - len..i], len) {
                dp[i] = true;
                break;
            }
        }
    }

    dp[n]
}
