//! Rules for Stress Assignment
//!
//! Once a loanword is known to carry (penultimate)
//! stress in English, Filipino's realization of that stress on the adapted
//! form depends purely on the *weight* of its penult syllable:
//!
//! - **Open penult** (no coda at all) -> length is retained: the vowel
//!   nucleus of the penult is marked (the "malumay" pattern).
//! - **Closed penult** (any coda consonant, no exemptions for liquids,
//!   nasals, or rhotics) -> the word is emitted completely unmarked. Filipino
//!   cannot lengthen a checked syllable, and there is no established
//!   orthographic device for marking unlengthened final stress here, so
//!   nothing is capitalized anywhere.

use crate::grapheme::filipino::FilipinoGrapheme;

/// The result of applying stress to a Filipino-adapted word.
///
/// Carries the analysis (`syllable_index`, `syllable`, `is_long`) alongside
/// three progressively-annotated renderings of the word, mirroring the
/// respell -> stress -> syllabify pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prominence {
    /// 0-based index of the penult syllable within the syllabification (the
    /// syllable examined by the rule, whether or not it ends up marked).
    pub syllable_index: usize,
    /// The penult syllable, spelled out (e.g. `"leyt"`), unmarked.
    pub syllable: String,
    /// Whether the penult is open (and thus long / gets its vowel marked).
    pub is_long: bool,
    /// The adapted word, respelled with no stress marking, e.g. `"tsokoleyt"`.
    pub respelled: String,
    /// `respelled` with the penult's vowel nucleus capitalized when the
    /// penult is open, e.g. `"amoksisIlin"`. Identical to `respelled`
    /// (nothing marked) when the penult is closed, or when the penult's
    /// nucleus can't be unambiguously identified (e.g. a diphthong-like
    /// syllable with zero or more than one vowel grapheme).
    pub with_stress: String,
    /// `with_stress`, hyphenated into syllables, e.g. `"a-mok-si-sI-lin"`.
    pub with_stress_and_syllabified: String,
}

/// Apply the Kaufman rule to a syllabified Filipino word.
///
/// Looks only at the word's penult (second-to-last syllable):
///
/// - **Open penult** (no coda) -> its vowel nucleus is capitalized via an
///   explicit lookup (`a`->`A`, `e`->`E`, `i`->`I`, `o`->`O`, `u`->`U`).
///   Never the whole syllable, and never a generic case-conversion:
///   onset/coda consonants have no uppercase form in this scheme, and
///   accidentally uppercasing one would silently corrupt downstream
///   ALINE-based scoring. If the penult doesn't have exactly one vowel
///   grapheme (e.g. a malformed or diphthong-like nucleus), it is left
///   unmarked rather than guessed at.
/// - **Closed penult** (any coda consonant - liquids, nasals, and rhotics
///   included, no exemptions) -> the word is emitted completely unmarked:
///   no capitals anywhere.
///
/// A monosyllabic word has no distinct penult, so its only syllable stands
/// in for both.
///
/// Returns `None` for an empty syllabification.
///
/// Reference: Daniel Kaufman (2025). PHILIPPINE PROSODY AND CONTRAST PRESERVATION
pub fn apply_stress_rule(syllables: &[Vec<FilipinoGrapheme>]) -> Option<Prominence> {
    if syllables.is_empty() {
        return None;
    }

    let last_index = syllables.len() - 1;
    let penult_index = if syllables.len() >= 2 {
        last_index - 1
    } else {
        last_index
    };
    let penult = &syllables[penult_index];

    let rendered: Vec<String> = syllables
        .iter()
        .map(|syl| syl.iter().map(FilipinoGrapheme::to_string_rep).collect())
        .collect();
    let respelled = rendered.concat();

    let is_long = is_open_syllable(penult);

    let marked_penult = if is_long {
        mark_vowel_nucleus(penult)
    } else {
        None
    };

    let (with_stress, with_stress_and_syllabified) = match marked_penult {
        Some(marked) => {
            let mut syllables_out = rendered.clone();
            syllables_out[penult_index] = marked;
            (syllables_out.concat(), syllables_out.join("-"))
        }
        // Closed penult, or an open penult whose vowel nucleus couldn't be
        // unambiguously identified: emit completely unmarked.
        None => (respelled.clone(), rendered.join("-")),
    };

    Some(Prominence {
        syllable_index: penult_index,
        syllable: rendered[penult_index].clone(),
        is_long,
        respelled,
        with_stress,
        with_stress_and_syllabified,
    })
}

/// A syllable is open if it ends in a vowel, i.e. it has no coda consonant.
fn is_open_syllable(syllable: &[FilipinoGrapheme]) -> bool {
    syllable.last().is_some_and(FilipinoGrapheme::is_vowel)
}

/// Render `syllable` with its vowel nucleus capitalized via an explicit
/// lookup, leaving every consonant untouched.
///
/// Returns `None` (rather than guessing) when the syllable doesn't have
/// exactly one vowel grapheme - a diphthong-like or malformed nucleus has no
/// uppercase form in this scheme.
fn mark_vowel_nucleus(syllable: &[FilipinoGrapheme]) -> Option<String> {
    let mut vowel_positions = syllable.iter().enumerate().filter(|(_, g)| g.is_vowel());
    let (nucleus_index, nucleus) = vowel_positions.next()?;
    if vowel_positions.next().is_some() {
        return None; // more than one vowel grapheme: ambiguous nucleus
    }

    let uppercase_nucleus = uppercase_vowel(nucleus)?;

    let mut result = String::with_capacity(syllable.len() * 2);
    for (i, g) in syllable.iter().enumerate() {
        if i == nucleus_index {
            result.push_str(uppercase_nucleus);
        } else {
            // to_string_rep() intentionally, never a case-conversion:
            // consonants have no uppercase entry in this scheme.
            result.push_str(&g.to_string_rep());
        }
    }
    Some(result)
}

/// Explicit vowel-nucleus uppercase lookup. Deliberately exhaustive over
/// only the five vowel graphemes; consonants have no entry here and must
/// never be case-converted (their uppercase form is meaningless downstream).
fn uppercase_vowel(g: &FilipinoGrapheme) -> Option<&'static str> {
    match g {
        FilipinoGrapheme::A => Some("A"),
        FilipinoGrapheme::E => Some("E"),
        FilipinoGrapheme::I => Some("I"),
        FilipinoGrapheme::O => Some("O"),
        FilipinoGrapheme::U => Some("U"),
        _ => None,
    }
}
