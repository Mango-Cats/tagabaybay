use tagabaybay::syllabification::filipino_syllable_patterns::*;
use tagabaybay::tokenization::phoneme::FilipinoGrapheme;

#[test]
fn test_match_1_syllable_vowel() {
    // Single vowels should match
    assert!(match_1_syllable(vec![FilipinoGrapheme::A]));
    assert!(match_1_syllable(vec![FilipinoGrapheme::E]));
    assert!(match_1_syllable(vec![FilipinoGrapheme::I]));
    assert!(match_1_syllable(vec![FilipinoGrapheme::O]));
    assert!(match_1_syllable(vec![FilipinoGrapheme::U]));
}

#[test]
fn test_match_1_syllable_consonant() {
    // Single consonants should not match
    assert!(!match_1_syllable(vec![FilipinoGrapheme::K]));
    assert!(!match_1_syllable(vec![FilipinoGrapheme::B]));
    assert!(!match_1_syllable(vec![FilipinoGrapheme::T]));
}

#[test]
fn test_match_1_syllable_empty() {
    // Empty vector should not match
    assert!(!match_1_syllable(vec![]));
}

#[test]
fn test_match_2_syllable_cv_pattern() {
    // Consonant-vowel (kp) should match
    assert!(match_2_syllable(vec![
        FilipinoGrapheme::B,
        FilipinoGrapheme::A
    ]));
    assert!(match_2_syllable(vec![
        FilipinoGrapheme::K,
        FilipinoGrapheme::E
    ]));
    assert!(match_2_syllable(vec![
        FilipinoGrapheme::T,
        FilipinoGrapheme::O
    ]));
}

#[test]
fn test_match_2_syllable_vc_pattern() {
    // Vowel-consonant (pk) should match
    assert!(match_2_syllable(vec![
        FilipinoGrapheme::A,
        FilipinoGrapheme::N
    ]));
    assert!(match_2_syllable(vec![
        FilipinoGrapheme::I,
        FilipinoGrapheme::T
    ]));
    assert!(match_2_syllable(vec![
        FilipinoGrapheme::O,
        FilipinoGrapheme::K
    ]));
}

#[test]
fn test_match_2_syllable_invalid() {
    // Consonant-consonant (kk) should not match
    assert!(!match_2_syllable(vec![
        FilipinoGrapheme::B,
        FilipinoGrapheme::K
    ]));
    // Vowel-vowel (pp) should not match
    assert!(!match_2_syllable(vec![
        FilipinoGrapheme::A,
        FilipinoGrapheme::E
    ]));
}

#[test]
fn test_match_3_syllable_ccv_pattern() {
    // Consonant-consonant-vowel (kkp) should match
    assert!(match_3_syllable(vec![
        FilipinoGrapheme::B,
        FilipinoGrapheme::L,
        FilipinoGrapheme::A
    ]));
    assert!(match_3_syllable(vec![
        FilipinoGrapheme::K,
        FilipinoGrapheme::R,
        FilipinoGrapheme::E
    ]));
}

#[test]
fn test_match_3_syllable_cvc_pattern() {
    // Consonant-vowel-consonant (kpk) should match
    assert!(match_3_syllable(vec![
        FilipinoGrapheme::B,
        FilipinoGrapheme::A,
        FilipinoGrapheme::Y
    ]));
    assert!(match_3_syllable(vec![
        FilipinoGrapheme::T,
        FilipinoGrapheme::A,
        FilipinoGrapheme::K
    ]));
}

#[test]
fn test_match_3_syllable_vcc_pattern() {
    // Vowel-consonant-consonant (pkk) should match
    assert!(match_3_syllable(vec![
        FilipinoGrapheme::A,
        FilipinoGrapheme::N,
        FilipinoGrapheme::G
    ]));
    assert!(match_3_syllable(vec![
        FilipinoGrapheme::I,
        FilipinoGrapheme::T,
        FilipinoGrapheme::S
    ]));
}

#[test]
fn test_match_3_syllable_invalid() {
    // Other patterns should not match
    assert!(!match_3_syllable(vec![
        FilipinoGrapheme::A,
        FilipinoGrapheme::E,
        FilipinoGrapheme::I
    ]));
    assert!(!match_3_syllable(vec![
        FilipinoGrapheme::K,
        FilipinoGrapheme::T,
        FilipinoGrapheme::P
    ]));
}

#[test]
fn test_match_4_syllable_ccvc_pattern() {
    // Consonant-consonant-vowel-consonant (kkpk) should match
    assert!(match_4_syllable(vec![
        FilipinoGrapheme::B,
        FilipinoGrapheme::R,
        FilipinoGrapheme::A,
        FilipinoGrapheme::S
    ]));
    assert!(match_4_syllable(vec![
        FilipinoGrapheme::K,
        FilipinoGrapheme::L,
        FilipinoGrapheme::A,
        FilipinoGrapheme::N
    ]));
}

#[test]
fn test_match_4_syllable_cvcc_pattern() {
    // Consonant-vowel-consonant-consonant (kpkk) should match
    assert!(match_4_syllable(vec![
        FilipinoGrapheme::B,
        FilipinoGrapheme::A,
        FilipinoGrapheme::N,
        FilipinoGrapheme::G
    ]));
    assert!(match_4_syllable(vec![
        FilipinoGrapheme::T,
        FilipinoGrapheme::A,
        FilipinoGrapheme::N,
        FilipinoGrapheme::S
    ]));
}

#[test]
fn test_match_4_syllable_invalid() {
    // Other patterns should not match
    assert!(!match_4_syllable(vec![
        FilipinoGrapheme::A,
        FilipinoGrapheme::B,
        FilipinoGrapheme::I,
        FilipinoGrapheme::O
    ]));
    assert!(!match_4_syllable(vec![
        FilipinoGrapheme::K,
        FilipinoGrapheme::K,
        FilipinoGrapheme::K,
        FilipinoGrapheme::K
    ]));
}

#[test]
fn test_match_5_syllable_valid() {
    // Consonant-consonant-vowel-consonant-consonant (kkpkk) should match
    assert!(match_5_syllable(vec![
        FilipinoGrapheme::B,
        FilipinoGrapheme::L,
        FilipinoGrapheme::A,
        FilipinoGrapheme::N,
        FilipinoGrapheme::K
    ]));
    assert!(match_5_syllable(vec![
        FilipinoGrapheme::K,
        FilipinoGrapheme::R,
        FilipinoGrapheme::O,
        FilipinoGrapheme::S,
        FilipinoGrapheme::T
    ]));
}

#[test]
fn test_match_5_syllable_invalid() {
    // Wrong patterns should not match
    assert!(!match_5_syllable(vec![
        FilipinoGrapheme::A,
        FilipinoGrapheme::B,
        FilipinoGrapheme::I,
        FilipinoGrapheme::O,
        FilipinoGrapheme::U
    ]));
    assert!(!match_5_syllable(vec![
        FilipinoGrapheme::B,
        FilipinoGrapheme::A,
        FilipinoGrapheme::T,
        FilipinoGrapheme::A,
        FilipinoGrapheme::N
    ]));
}

#[test]
fn test_match_6_syllable_valid() {
    // Consonant-consonant-vowel-consonant-consonant-consonant (kkpkkk) should match
    assert!(match_6_syllable(vec![
        FilipinoGrapheme::K,
        FilipinoGrapheme::L,
        FilipinoGrapheme::A,
        FilipinoGrapheme::S,
        FilipinoGrapheme::K,
        FilipinoGrapheme::T
    ]));
    assert!(match_6_syllable(vec![
        FilipinoGrapheme::B,
        FilipinoGrapheme::R,
        FilipinoGrapheme::A,
        FilipinoGrapheme::N,
        FilipinoGrapheme::G,
        FilipinoGrapheme::S
    ]));
}

#[test]
fn test_match_6_syllable_invalid() {
    // Wrong patterns should not match
    assert!(!match_6_syllable(vec![
        FilipinoGrapheme::A,
        FilipinoGrapheme::B,
        FilipinoGrapheme::I,
        FilipinoGrapheme::O,
        FilipinoGrapheme::U,
        FilipinoGrapheme::E
    ]));
    assert!(!match_6_syllable(vec![
        FilipinoGrapheme::B,
        FilipinoGrapheme::A,
        FilipinoGrapheme::T,
        FilipinoGrapheme::A,
        FilipinoGrapheme::N,
        FilipinoGrapheme::G
    ]));
}
