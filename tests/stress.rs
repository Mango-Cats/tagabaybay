//! Tests for stress/prominence assignment (the stress rule).
//!
//! `stress_*` and `cmudict_*` and `espeak_parse_*` are pure unit tests (no
//! external processes). `prominence_end_to_end` exercises the full
//! `Adapter::prominence` pipeline via eSpeak and is skipped gracefully if
//! `uv`/eSpeak-NG aren't available, matching the pattern in
//! `tests/alignment.rs`.
//!
//! to run just this file: cargo test --test stress -- --nocapture

use tagabaybay::adaptation::adapter::Adapter;
use tagabaybay::configs::AdapterConfig;
use tagabaybay::grapheme::filipino::FilipinoGrapheme::*;
use tagabaybay::stress::StressPosition;
use tagabaybay::stress::apply_stress_rule;
use tagabaybay::stress::cmudict::CmuDict;
use tagabaybay::stress::espeak::stress_position_from_ipa;
use tagabaybay::tokens;

/// English stress reported as landing on the penult, for a word with
/// `syllable_count` syllables. Used by tests that exercise the Filipino
/// open/closed-penult logic and want the English-stress gate to pass through
/// as a no-op.
fn penult_stress(syllable_count: usize) -> StressPosition {
    StressPosition {
        syllable_count,
        stressed_index_from_end: 2,
    }
}

/// English stress reported as landing on a monosyllabic word's only
/// syllable, which trivially stands in for the penult.
fn monosyllabic_stress() -> StressPosition {
    StressPosition {
        syllable_count: 1,
        stressed_index_from_end: 1,
    }
}

// ---------------------------------------------------------------------
// stress rule: open penult -> mark its vowel nucleus; closed penult -> word
// emitted completely unmarked (no exemptions for liquids/nasals/rhotics)
// ---------------------------------------------------------------------

#[test]
fn stress_open_penult_marks_only_its_vowel_nucleus() {
    // "tso-ko-leyt" (chocolate): penult "ko" is open -> mark just the "o".
    let syllables = vec![tokens![T, S, O], tokens![K, O], tokens![L, E, Y, T]];

    let prominence = apply_stress_rule(&syllables, penult_stress(3)).unwrap();
    assert_eq!(prominence.syllable_index, 1);
    assert_eq!(prominence.syllable, "ko");
    assert!(prominence.is_long);
    assert!(prominence.english_stress_on_penult);

    assert_eq!(prominence.respelled, "tsokoleyt");
    assert_eq!(prominence.with_stress, "tsokOleyt");
    assert_eq!(prominence.with_stress_and_syllabified, "tso-kO-leyt");
}

#[test]
fn stress_amoxicillin_marks_penult_vowel() {
    // a-mok-si-si-lin: penult (index 3) is "si", open -> mark just the "i".
    let syllables = vec![
        tokens![A],
        tokens![M, O, K],
        tokens![S, I],
        tokens![S, I],
        tokens![L, I, N],
    ];

    let prominence = apply_stress_rule(&syllables, penult_stress(5)).unwrap();
    assert_eq!(prominence.syllable_index, 3);
    assert_eq!(prominence.syllable, "si");
    assert!(prominence.is_long);

    assert_eq!(prominence.respelled, "amoksisilin");
    assert_eq!(prominence.with_stress, "amoksisIlin");
    assert_eq!(prominence.with_stress_and_syllabified, "a-mok-si-sI-lin");
}

#[test]
fn stress_closed_penult_emits_word_completely_unmarked() {
    // met-for-min: penult "for" is closed by /r/ (no exemption) -> the whole
    // word is emitted unmarked, not shifted-and-marked onto "min". English
    // stress is (for this test) still on the penult, isolating the
    // open/closed check from the English-stress gate.
    let syllables = vec![tokens![M, E, T], tokens![F, O, R], tokens![M, I, N]];

    let prominence = apply_stress_rule(&syllables, penult_stress(3)).unwrap();
    assert_eq!(prominence.syllable_index, 1);
    assert_eq!(prominence.syllable, "for");
    assert!(!prominence.is_long);
    assert!(prominence.english_stress_on_penult);

    assert_eq!(prominence.respelled, "metformin");
    assert_eq!(prominence.with_stress, "metformin");
    assert_eq!(prominence.with_stress_and_syllabified, "met-for-min");
}

#[test]
fn stress_closed_penult_nasal_coda_also_unmarked() {
    // No exemption for nasal codas either: penult "tam" closed by /m/.
    let syllables = vec![tokens![A], tokens![S, I], tokens![T, A, M], tokens![N, O]];

    let prominence = apply_stress_rule(&syllables, penult_stress(4)).unwrap();
    assert_eq!(prominence.syllable_index, 2);
    assert_eq!(prominence.syllable, "tam");
    assert!(!prominence.is_long);

    assert_eq!(prominence.respelled, "asitamno");
    assert_eq!(prominence.with_stress, "asitamno");
    assert_eq!(prominence.with_stress_and_syllabified, "a-si-tam-no");
}

#[test]
fn stress_closed_penult_liquid_l_coda_also_unmarked() {
    // No exemption for /l/ either, unlike /r/ which happens to look similar
    // in some analyses - this rule draws no distinction between coda types.
    let syllables = vec![tokens![S, A, L], tokens![B, U, T], tokens![A, M, O, L]];

    let prominence = apply_stress_rule(&syllables, penult_stress(3)).unwrap();
    assert_eq!(prominence.syllable_index, 1);
    assert_eq!(prominence.syllable, "but");
    assert!(!prominence.is_long);
    assert_eq!(prominence.with_stress, prominence.respelled);
}

#[test]
fn stress_two_syllable_open_penult() {
    let syllables = vec![tokens![B, A], tokens![S, A]];

    let prominence = apply_stress_rule(&syllables, penult_stress(2)).unwrap();
    assert_eq!(prominence.syllable_index, 0);
    assert_eq!(prominence.syllable, "ba");
    assert!(prominence.is_long);
    assert_eq!(prominence.with_stress, "bAsa");
}

#[test]
fn stress_monosyllabic_word_open() {
    let syllables = vec![tokens![K, A]];

    let prominence = apply_stress_rule(&syllables, monosyllabic_stress()).unwrap();
    assert_eq!(prominence.syllable_index, 0);
    assert_eq!(prominence.syllable, "ka");
    assert!(prominence.is_long);
    assert_eq!(prominence.with_stress, "kA");
}

#[test]
fn stress_monosyllabic_word_closed() {
    // No distinct penult: the only syllable stands in for both. Closed, so
    // completely unmarked.
    let syllables = vec![tokens![K, A, T]];

    let prominence = apply_stress_rule(&syllables, monosyllabic_stress()).unwrap();
    assert_eq!(prominence.syllable_index, 0);
    assert_eq!(prominence.syllable, "kat");
    assert!(!prominence.is_long);
    assert_eq!(prominence.with_stress, "kat");
}

#[test]
fn stress_ambiguous_nucleus_left_unmarked() {
    // Penult "ai" has two vowel graphemes (malformed/diphthong-like nucleus):
    // structurally open (ends in a vowel), but there's no single vowel to
    // mark, so it's left unmarked rather than guessed at.
    let syllables = vec![tokens![A, I], tokens![N]];

    let prominence = apply_stress_rule(&syllables, penult_stress(2)).unwrap();
    assert_eq!(prominence.syllable_index, 0);
    assert_eq!(prominence.syllable, "ai");
    assert!(prominence.is_long); // structurally open ("ai" ends in a vowel)
    assert_eq!(prominence.with_stress, prominence.respelled); // but left unmarked
}

#[test]
fn stress_empty_syllabification_returns_none() {
    let syllables: Vec<Vec<tagabaybay::grapheme::filipino::FilipinoGrapheme>> = vec![];
    assert!(apply_stress_rule(&syllables, monosyllabic_stress()).is_none());
}

// ---------------------------------------------------------------------
// the English-stress gate: this rule must never mark a Filipino syllable
// that English itself doesn't treat as prominent, regardless of whether
// that Filipino penult happens to be open. Regression coverage for a real
// bug: `Adapter::prominence` used to look up English stress only to check
// presence/absence, discarding *where* it fell, so it would mark based on
// Filipino penult weight alone. Verified against real espeak-ng output that
// this is not a rare edge case for pharmaceutical names (e.g. "metformin",
// "losartan" both carry non-penultimate English primary stress).
// ---------------------------------------------------------------------

#[test]
fn stress_gate_blocks_marking_when_english_stress_is_antepenultimate() {
    // "ba-sa": penult "ba" is open (would be marked if the gate were
    // missing), but English stress here is reported on the antepenult, not
    // the penult -> must come back completely unmarked.
    let syllables = vec![tokens![B, A], tokens![S, A]];
    let antepenult_stress = StressPosition {
        syllable_count: 3,
        stressed_index_from_end: 3,
    };

    let prominence = apply_stress_rule(&syllables, antepenult_stress).unwrap();
    assert!(!prominence.english_stress_on_penult);
    assert!(!prominence.is_long);
    assert_eq!(prominence.with_stress, prominence.respelled);
}

#[test]
fn stress_gate_blocks_marking_when_english_stress_is_final() {
    let syllables = vec![tokens![B, A], tokens![S, A]];
    let final_stress = StressPosition {
        syllable_count: 2,
        stressed_index_from_end: 1,
    };

    let prominence = apply_stress_rule(&syllables, final_stress).unwrap();
    assert!(!prominence.english_stress_on_penult);
    assert!(!prominence.is_long);
    assert_eq!(prominence.with_stress, prominence.respelled);
}

#[test]
fn english_stress_on_penult_helper_matches_stressed_index_two() {
    use tagabaybay::stress::rules::english_stress_on_penult;

    assert!(english_stress_on_penult(penult_stress(3)));
    assert!(english_stress_on_penult(monosyllabic_stress()));
    assert!(!english_stress_on_penult(StressPosition {
        syllable_count: 3,
        stressed_index_from_end: 3,
    }));
    assert!(!english_stress_on_penult(StressPosition {
        syllable_count: 3,
        stressed_index_from_end: 1,
    }));
}

// ---------------------------------------------------------------------
// CMUdict backend
// ---------------------------------------------------------------------

const SAMPLE_CMUDICT: &str = "\
;;; comment lines should be skipped
ACETAMINOPHEN  AH2 S IY1 T AH0 M IH2 N AH0 F EH0 N
CAT  K AE1 T
READ  R IY1 D
READ(2)  R EH1 D
ACTION  AE1 K SH AH0 N
";

#[test]
fn cmudict_parses_primary_stress_from_end() {
    let dict = CmuDict::parse(SAMPLE_CMUDICT);

    // ACETAMINOPHEN has 6 syllables, primary stress on the 2nd (IY1) ->
    // 6 - 1 = 5 syllables from the end.
    let stress = dict.primary_stress("acetaminophen").unwrap();
    assert_eq!(stress.syllable_count, 6);
    assert_eq!(stress.stressed_index_from_end, 5);
}

#[test]
fn cmudict_monosyllable() {
    let dict = CmuDict::parse(SAMPLE_CMUDICT);
    let stress = dict.primary_stress("cat").unwrap();
    assert_eq!(stress.syllable_count, 1);
    assert_eq!(stress.stressed_index_from_end, 1);
}

#[test]
fn cmudict_lookup_is_case_insensitive() {
    let dict = CmuDict::parse(SAMPLE_CMUDICT);
    assert!(dict.primary_stress("CAT").is_some());
    assert!(dict.primary_stress("Cat").is_some());
}

#[test]
fn cmudict_ignores_alternate_pronunciations() {
    let dict = CmuDict::parse(SAMPLE_CMUDICT);
    // The first "READ" entry (R IY1 D) should win over "READ(2)" (R EH1 D);
    // both are monosyllabic so this mostly checks the (2) variant is never
    // treated as its own headword.
    assert!(dict.primary_stress("read").is_some());
    assert!(dict.primary_stress("read(2)").is_none());
}

#[test]
fn cmudict_unknown_word_returns_none() {
    let dict = CmuDict::parse(SAMPLE_CMUDICT);
    assert!(
        dict.primary_stress("floccinaucinihilipilification")
            .is_none()
    );
}

// ---------------------------------------------------------------------
// eSpeak stress-marked IPA parsing
// ---------------------------------------------------------------------

#[test]
fn espeak_parse_primary_stress_on_first_syllable() {
    // Actual phonemizer/eSpeak-NG output (with_stress=True) for "acetaminophen".
    let stress = stress_position_from_ipa("ˈæsɪtˌæmɪnˌɑːfən").unwrap();
    assert_eq!(stress.syllable_count, 6);
    assert_eq!(stress.stressed_index_from_end, 6);
}

#[test]
fn espeak_parse_primary_stress_mid_word() {
    // "computer" -> k ə m p j ˈuː ɾ ɚ (stress on penult).
    let stress = stress_position_from_ipa("kəmpjˈuːɾɚ").unwrap();
    assert_eq!(stress.syllable_count, 3);
    assert_eq!(stress.stressed_index_from_end, 2);
}

#[test]
fn espeak_parse_monosyllable() {
    let stress = stress_position_from_ipa("kˈæt").unwrap();
    assert_eq!(stress.syllable_count, 1);
    assert_eq!(stress.stressed_index_from_end, 1);
}

#[test]
fn espeak_parse_no_vowels_returns_none() {
    assert!(stress_position_from_ipa("").is_none());
}

#[test]
fn espeak_parse_missing_primary_mark_defaults_to_first_syllable() {
    // No ˈ present at all: falls back to treating the first syllable as stressed.
    let stress = stress_position_from_ipa("æsɪt").unwrap();
    assert_eq!(stress.syllable_count, 2);
    assert_eq!(stress.stressed_index_from_end, 2);
}

// ---------------------------------------------------------------------
// End-to-end (requires uv + eSpeak-NG; skipped gracefully if unavailable)
// ---------------------------------------------------------------------

#[test]
fn prominence_end_to_end() {
    let config = AdapterConfig::new().set_assign_prominence(true);
    let mut adapter = Adapter::new_with_config(config);

    let drug_names = [
        "Acetaminophen",
        "Ibuprofen",
        "Amoxicillin",
        "Chocolate",
        "Metformin",
    ];

    for name in drug_names {
        let Ok(adapted) = adapter.adaptation(name) else {
            println!("skipping '{name}': eSpeak-NG/uv not available");
            return;
        };

        match adapter.prominence(name, &adapted) {
            Ok(Some(prominence)) => {
                println!(
                    "{name}: {} -> {} -> {} (english stress on penult: {})",
                    prominence.respelled,
                    prominence.with_stress,
                    prominence.with_stress_and_syllabified,
                    prominence.english_stress_on_penult
                );
            }
            Ok(None) => println!("{name}: no prominence result"),
            Err(e) => {
                println!("skipping '{name}': {e:?}");
                return;
            }
        }
    }
}

#[test]
fn prominence_returns_none_when_disabled() {
    let config = AdapterConfig::new().set_assign_prominence(false);
    let mut adapter = Adapter::new_with_config(config);
    let adapted = adapter.adaptation("cat").unwrap();
    let prominence = adapter.prominence("cat", &adapted).unwrap();
    assert!(prominence.is_none());
}
