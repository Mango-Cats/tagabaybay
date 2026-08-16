//! CMUdict-backed English primary-stress lookup.
//!
//! Parses the standard CMUdict text format (as distributed at
//! <https://github.com/cmusphinx/cmudict>): one entry per line,
//!
//! ```text
//! WORD  P1 P2 P3 ...
//! ```
//!
//! where vowel phones carry a trailing stress digit (`0` = unstressed,
//! `1` = primary, `2` = secondary), e.g. `ACETAMINOPHEN  AH2 S IY1 T AH0 M IH2
//! N AH0 F EH0 N`. Alternate pronunciations are suffixed on the headword as
//! `WORD(2)`, `WORD(3)`, ... and are ignored in favor of the first
//! pronunciation encountered.
//!
//! This is a from-scratch parser (not the `cmudict` crate) so the dictionary
//! file stays a user-supplied path rather than a bundled/downloaded asset,
//! matching how the rest of tagabaybay avoids network calls at runtime.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::error::{G2PError, G2PErrorKind};
use crate::stress::StressPosition;

/// A loaded CMUdict pronunciation dictionary, indexed by lowercased headword.
#[derive(Debug, Clone)]
pub struct CmuDict {
    entries: HashMap<String, StressPosition>,
}

impl CmuDict {
    /// Load and parse a CMUdict-format file from `path`.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, G2PError> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path).map_err(|e| {
            G2PError::new(G2PErrorKind::ServerUnavailable {
                reason: format!("failed to read cmudict file {}: {}", path.display(), e),
            })
        })?;

        Ok(Self::parse(&contents))
    }

    /// Parse CMUdict-format text directly (used by [`CmuDict::load`] and by
    /// tests that don't want to touch the filesystem).
    pub fn parse(contents: &str) -> Self {
        let mut entries = HashMap::new();

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(";;;") {
                continue;
            }

            let mut parts = line.split_whitespace();
            let Some(raw_word) = parts.next() else {
                continue;
            };

            // Skip alternate pronunciations, e.g. "READ(2)"; keep whichever
            // pronunciation (usually the primary one) was seen first.
            if raw_word.ends_with(')') && raw_word.contains('(') {
                continue;
            }

            let phones: Vec<&str> = parts.collect();
            if let Some(stress) = stress_position_from_phones(&phones) {
                entries.entry(raw_word.to_lowercase()).or_insert(stress);
            }
        }

        Self { entries }
    }

    /// Look up the primary-stress position for `word` (case-insensitive).
    pub fn primary_stress(&self, word: &str) -> Option<StressPosition> {
        self.entries.get(&word.to_lowercase()).copied()
    }
}

/// Compute syllable count and primary-stress-from-end from ARPABET phones
/// with trailing stress digits, e.g. `["AH0", "S", "IY1", "T", ...]`.
fn stress_position_from_phones(phones: &[&str]) -> Option<StressPosition> {
    let mut syllable_count = 0usize;
    let mut primary_index = None;

    for phone in phones {
        let Some(last) = phone.chars().last() else {
            continue;
        };
        if !last.is_ascii_digit() {
            continue; // consonant phone, carries no stress digit
        }

        if last == '1' {
            primary_index = Some(syllable_count);
        }
        syllable_count += 1;
    }

    if syllable_count == 0 {
        return None;
    }

    // No phone marked primary (rare, e.g. some function-word entries): fall
    // back to the first syllable rather than dropping the entry.
    let idx = primary_index.unwrap_or(0);

    Some(StressPosition {
        syllable_count,
        stressed_index_from_end: syllable_count - idx,
    })
}
