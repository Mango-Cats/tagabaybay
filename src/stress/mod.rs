//! Stress/prominence assignment for Filipino-adapted loanwords.
//!
//! This module implements the **stress rule**: English primary stress is
//! looked up for the source word (via CMUdict or eSpeak's English frontend),
//! then mapped onto the syllabification of the *Filipino-adapted* word to
//! decide which syllable is prominent and whether it is phonetically long.
//!
//! # Pipeline
//!
//! 1. Look up English primary stress -> [`StressPosition`] ([`cmudict`] or
//!    [`espeak`]). This gates the rule: if the source word isn't found (no
//!    stress info), there's nothing to adapt and the pipeline stops here.
//! 2. Syllabify the adapted Filipino word (see
//!    [`crate::syllabification::algorithm::syllabify`]).
//! 3. Apply [`rules::apply_stress_rule`]: the word's penult is open (ends in a
//!    vowel) -> length is retained, prominence stays on the penult. The
//!    penult is closed (ends in a consonant) -> prominence shifts to the
//!    final syllable instead, since Filipino cannot lengthen a checked
//!    syllable.
//!
//! This whole feature is opt-in: it only runs when
//! [`crate::configs::AdapterConfig::assign_prominence`] is enabled, via
//! [`crate::adaptation::adapter::Adapter::prominence`].

pub mod cmudict;
pub mod espeak;
pub mod rules;

pub use cmudict::CmuDict;
pub use espeak::stress_position_from_ipa;
pub use rules::{Prominence, apply_stress_rule};

/// Where primary stress falls in an English word, counted in syllables.
///
/// Positions are counted **from the end of the word** (1 = final syllable,
/// 2 = penult, 3 = antepenult, ...) rather than from the start, so that
/// syllable-count mismatches between the English source and the shorter or
/// longer Filipino-adapted form don't misalign the stress rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StressPosition {
    /// Total number of syllables (vowel nuclei) in the English word.
    pub syllable_count: usize,
    /// 1-based index of the primary-stressed syllable, counted from the end.
    pub stressed_index_from_end: usize,
}
