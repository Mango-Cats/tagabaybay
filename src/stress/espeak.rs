//! eSpeak-backed English primary-stress lookup.
//!
//! eSpeak-NG (via `phonemizer`'s `EspeakBackend` with `with_stress=True`)
//! marks the primary-stressed syllable with a leading IPA stress mark `ˈ`
//! (U+02C8) immediately before that syllable's onset, and secondary stress
//! with `ˌ` (U+02CC). For example `"acetaminophen"` phonemizes to
//! `ˈæsɪtˌæmɪnˌɑːfən`.
//!
//! [`stress_position_from_ipa`] scans that string with the same
//! longest-match-first strategy as
//! [`crate::phoneme::tokenizer::ipa::tokenize_ipa`], additionally tracking
//! stress marks to find which vowel nucleus the primary mark precedes.

use crate::phoneme::tokens::map::IPA_STR_TO_SYMBOL;
use crate::stress::StressPosition;

const PRIMARY_STRESS_MARK: char = '\u{02C8}'; // ˈ
const SECONDARY_STRESS_MARK: char = '\u{02CC}'; // ˌ

/// Parse a stress-marked IPA string into a [`StressPosition`].
///
/// Returns `None` if the string contains no recognizable vowel nucleus.
/// If no primary-stress mark is found (eSpeak omits it for some
/// monosyllables), the first syllable is treated as stressed.
pub fn stress_position_from_ipa(ipa: &str) -> Option<StressPosition> {
    let chars: Vec<char> = ipa.chars().collect();
    let mut i = 0;
    let mut syllable_count = 0usize;
    let mut primary_index: Option<usize> = None;
    let mut pending_primary = false;

    while i < chars.len() {
        let c = chars[i];

        if c == PRIMARY_STRESS_MARK {
            pending_primary = true;
            i += 1;
            continue;
        }
        if c == SECONDARY_STRESS_MARK {
            i += 1;
            continue;
        }

        let (matched_len, is_vowel) = match_symbol_at(&chars, i);

        if matched_len == 0 {
            i += 1; // unrecognized character (e.g. a length mark we don't track); skip
            continue;
        }

        if is_vowel {
            if pending_primary {
                primary_index = Some(syllable_count);
                pending_primary = false;
            }
            syllable_count += 1;
        }

        i += matched_len;
    }

    if syllable_count == 0 {
        return None;
    }

    let idx = primary_index.unwrap_or(0);
    Some(StressPosition {
        syllable_count,
        stressed_index_from_end: syllable_count - idx,
    })
}

/// Try to match an IPA symbol at `chars[i]`, preferring a 2-character match
/// (diphthongs, affricates) over a 1-character one. Returns `(matched_len,
/// is_vowel)`, where `matched_len == 0` means nothing matched.
fn match_symbol_at(chars: &[char], i: usize) -> (usize, bool) {
    if i + 1 < chars.len() {
        let two: String = chars[i..=i + 1].iter().collect();
        if let Some(sym) = IPA_STR_TO_SYMBOL.get(two.as_str()) {
            return (2, sym.is_vowel());
        }
    }

    let one: String = chars[i..=i].iter().collect();
    if let Some(sym) = IPA_STR_TO_SYMBOL.get(one.as_str()) {
        return (1, sym.is_vowel());
    }

    (0, false)
}
