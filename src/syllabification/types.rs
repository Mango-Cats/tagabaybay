/// Syllable pattern element
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pat {
    /// Consonant (Katinig)
    K,
    /// Vowel (Patinig)
    P,
}

use Pat::{K, P};

/// All valid Filipino syllable patterns, grouped by length.
/// Patterns are checked from longest to shortest during validation.
pub const SYLLABLE_PATTERNS: &[&[Pat]] = &[
    // Length 6
    &[K, K, P, K, K, K], // KKPKKK
    // Length 5
    &[K, K, P, K, K], // KKPKK
    // Length 4
    &[K, K, P, K], // KKPK
    &[K, P, K, K], // KPKK
    // Length 3
    &[K, K, P], // KKP
    &[K, P, K], // KPK
    &[P, K, K], // PKK
    // Length 2
    &[K, P], // KP
    &[P, K], // PK
    // Length 1
    &[P], // P (lone vowel)
];
