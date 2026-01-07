use tagabaybay::syllabification::filipino_syllable_patterns::*;
use tagabaybay::tokenization::phoneme::Phoneme;

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
