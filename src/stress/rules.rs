//! Rules for Stress Assignment
//!
//! This rule only fires when the source word actually carries **penultimate**
//! stress in English (see [`english_stress_on_penult`]). When it does,
//! Filipino's realization of that stress on the adapted form depends purely
//! on the *weight* of its penult syllable:
//!
//! - **Open penult** (no coda at all) -> length is retained: the vowel
//!   nucleus of the penult is marked (the "malumay" pattern).
//! - **Closed penult** (any coda consonant, no exemptions for liquids,
//!   nasals, or rhotics) -> the word is emitted completely unmarked. Filipino
//!   cannot lengthen a checked syllable, and there is no established
//!   orthographic device for marking unlengthened final stress here, so
//!   nothing is capitalized anywhere.
//!
//! When English stress falls elsewhere (antepenult, final, ...), this rule
//! doesn't govern the word at all: it is emitted completely unmarked,
//! regardless of the Filipino penult's own weight. Concretely, pharmaceutical
//! names phonemized via eSpeak-NG carry non-penultimate primary stress a
//! majority of the time (e.g. "metformin", "losartan"), so this gate matters
//! in practice, not just at the margins.

use crate::grapheme::filipino::FilipinoGrapheme;
use crate::stress::StressPosition;

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
    /// Whether the source word's English primary stress actually falls on
    /// the penult (or, for a monosyllabic English word, on its only
    /// syllable). This is the gate: when `false`, `is_long` is always
    /// `false` and nothing is marked, regardless of the Filipino penult's
    /// own weight. Exposed here (rather than only used internally) so
    /// callers can distinguish "unmarked because the penult is closed" from
    /// "unmarked because English stress isn't even penultimate" - useful as
    /// a feature in its own right.
    pub english_stress_on_penult: bool,
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

/// Whether `english_stress` puts primary stress on the penult - or, for a
/// monosyllabic English word, trivially "on" its only syllable, which stands
/// in for the penult the same way it does on the Filipino side.
///
/// Positions are compared using [`StressPosition`]'s own from-the-end
/// counting, so a mismatch in syllable count between the English source and
/// the (possibly shorter or longer) Filipino-adapted form doesn't misalign
/// this check.
pub fn english_stress_on_penult(english_stress: StressPosition) -> bool {
    match english_stress.syllable_count {
        0 => false,
        1 => true,
        _ => english_stress.stressed_index_from_end == 2,
    }
}

/// Apply the Kaufman rule to a syllabified Filipino word, gated by whether
/// the source word's English stress is actually penultimate.
///
/// If [`english_stress_on_penult`] is `false` for `english_stress`, the word
/// is emitted completely unmarked and `is_long` is `false` - this rule only
/// ever marks a syllable that English also treats as prominent. Otherwise,
/// looks only at the word's penult (second-to-last syllable):
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
pub fn apply_stress_rule(
    syllables: &[Vec<FilipinoGrapheme>],
    english_stress: StressPosition,
) -> Option<Prominence> {
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

    let on_penult = english_stress_on_penult(english_stress);
    let is_long = on_penult && is_open_syllable(penult);

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
        // English stress isn't penultimate, or the penult is closed, or an
        // open penult's vowel nucleus couldn't be unambiguously identified:
        // emit completely unmarked.
        None => (respelled.clone(), rendered.join("-")),
    };

    Some(Prominence {
        syllable_index: penult_index,
        syllable: rendered[penult_index].clone(),
        is_long,
        english_stress_on_penult: on_penult,
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
