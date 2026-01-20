//! Tests for Filipino syllabification rules.
//!
//! Test cases derived from Filipino syllabification rules from 2001
//! Revisyon ng Alfabeto at Patnubay sa Ispeling ng Wikang Filipino
//! (2001 Revision Alphabet and Spelling Guide of the Filipino Language)

use tagabaybay::grapheme::filipino::FilipinoGrapheme::*;
use tagabaybay::grapheme::filipino::hyphenate;
use tagabaybay::syllabification::algorithm::*;
use tagabaybay::syllabification::pattern_builder::*;
use tagabaybay::syllabification::types::*;
use tagabaybay::syllabification::validation::*;
use tagabaybay::tokens;

mod two_consonants {
    use super::*;

    /// buksan → buk-san
    #[test]
    fn test_buksan() {
        let word = tokens![B, U, K, S, A, N];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "buk-san");
    }

    /// pinto → pin-to
    #[test]
    fn test_pinto() {
        let word = tokens![P, I, N, T, O];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "pin-to");
    }

    /// tuktok → tuk-tok
    #[test]
    fn test_tuktok() {
        let word = tokens![T, U, K, T, O, K];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "tuk-tok");
    }

    /// pantig → pan-tig
    #[test]
    fn test_pantig() {
        let word = tokens![P, A, N, T, I, G];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "pan-tig");
    }

    /// sobre → sob-re
    #[test]
    fn test_sobre() {
        let word = tokens![S, O, B, R, E];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "sob-re");
    }

    /// kopya → kop-ya
    #[test]
    fn test_kopya() {
        let word = tokens![K, O, P, Y, A];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "kop-ya");
    }

    /// kapre → kap-re
    #[test]
    fn test_kapre() {
        let word = tokens![K, A, P, R, E];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "kap-re");
    }

    /// tokwa → tok-wa
    #[test]
    fn test_tokwa() {
        let word = tokens![T, O, K, W, A];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "tok-wa");
    }
}

mod three_consonants {
    use super::*;

    /// eksperimento → eks-pe-ri-men-to
    #[test]
    fn test_eksperimento() {
        let word = tokens![E, K, S, P, E, R, I, M, E, N, T, O];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "eks-pe-ri-men-to");
    }

    /// transkripsyon → trans-krip-syon
    #[test]
    fn test_transkripsyon() {
        let word = tokens![T, R, A, N, S, K, R, I, P, S, Y, O, N];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "trans-krip-syon");
    }
}

mod mn_cluster_exception {
    use super::*;

    /// asambleya → a-sam-ble-ya
    #[test]
    fn test_asambleya() {
        let word = tokens![A, S, A, M, B, L, E, Y, A];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "a-sam-ble-ya");
    }

    /// alambre → a-lam-bre
    #[test]
    fn test_alambre() {
        let word = tokens![A, L, A, M, B, R, E];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "a-lam-bre");
    }

    /// balandra → ba-lan-dra
    #[test]
    fn test_balandra() {
        let word = tokens![B, A, L, A, N, D, R, A];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "ba-lan-dra");
    }

    /// simple → sim-ple
    #[test]
    fn test_simple() {
        let word = tokens![S, I, M, P, L, E];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "sim-ple");
    }

    /// sentro → sen-tro
    #[test]
    fn test_sentro() {
        let word = tokens![S, E, N, T, R, O];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "sen-tro");
    }

    /// kontra → kon-tra
    #[test]
    fn test_kontra() {
        let word = tokens![K, O, N, T, R, A];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "kon-tra");
    }
}

mod four_consonants {
    use super::*;

    /// ekstradisyon → eks-tra-di-syon
    #[test]
    fn test_ekstradisyon() {
        let word = tokens![E, K, S, T, R, A, D, I, S, Y, O, N];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "eks-tra-di-syon");
    }

    /// eksklusibo → eks-klu-si-bo
    #[test]
    fn test_eksklusibo() {
        let word = tokens![E, K, S, K, L, U, S, I, B, O];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "eks-klu-si-bo");
    }
}

mod vowel_initial {
    use super::*;

    /// alis → a-lis
    #[test]
    fn test_alis() {
        let word = tokens![A, L, I, S];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "a-lis");
    }

    /// iwan → i-wan
    #[test]
    fn test_iwan() {
        let word = tokens![I, W, A, N];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "i-wan");
    }

    /// ambon → am-bon
    #[test]
    fn test_ambon() {
        let word = tokens![A, M, B, O, N];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "am-bon");
    }

    /// ekstra → eks-tra
    #[test]
    fn test_ekstra() {
        let word = tokens![E, K, S, T, R, A];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "eks-tra");
    }
}

mod cv_initial {
    use super::*;

    /// basa → ba-sa
    #[test]
    fn test_basa() {
        let word = tokens![B, A, S, A];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "ba-sa");
    }

    /// lakad → la-kad
    #[test]
    fn test_lakad() {
        let word = tokens![L, A, K, A, D];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "la-kad");
    }

    /// takbo → tak-bo
    #[test]
    fn test_takbo() {
        let word = tokens![T, A, K, B, O];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "tak-bo");
    }

    /// lundag → lun-dag
    #[test]
    fn test_lundag() {
        let word = tokens![L, U, N, D, A, G];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "lun-dag");
    }

    /// nars → nars (single syllable)
    #[test]
    fn test_nars() {
        let word = tokens![N, A, R, S];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "nars");
    }
}

mod cc_initial {
    use super::*;

    /// plantsa → plan-tsa
    #[test]
    fn test_plantsa() {
        let word = tokens![P, L, A, N, TS, A];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "plan-tsa");
    }

    /// prito → pri-to
    #[test]
    fn test_prito() {
        let word = tokens![P, R, I, T, O];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "pri-to");
    }

    /// kwento → kwen-to
    #[test]
    fn test_kwento() {
        let word = tokens![K, W, E, N, T, O];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "kwen-to");
    }
}

mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_sequence() {
        assert!(validate_filipino_syllables_dp(vec![]));
        let (syllables, _) = syllabify(&[]).unwrap();
        assert!(syllables.is_empty());
    }

    #[test]
    fn test_invalid_consonant_only() {
        // Consonants alone cannot form syllables
        assert!(!validate_filipino_syllables_dp(vec![K, T, S]));
        assert!(syllabify(&[K, T, S]).is_none());
    }

    #[test]
    fn test_single_vowel_valid() {
        assert!(validate_filipino_syllables_dp(vec![A]));
        let (syllables, _) = syllabify(&[A]).unwrap();
        assert_eq!(hyphenate(&syllables), "a");
    }
}

mod pattern_builder {
    use super::*;

    #[test]
    fn test_builder_kpk() {
        let pattern = PatternBuilder::new().k().p().k().build();
        assert_eq!(pattern, vec![Pat::K, Pat::P, Pat::K]);

        let builder = PatternBuilder::new().k().p().k();
        assert!(builder.matches(&[B, A, S]));
        assert!(!builder.matches(&[A, B, A]));
    }

    #[test]
    fn test_builder_kkpkk() {
        let pattern = PatternBuilder::new().k().k().p().k().k().build();
        assert_eq!(pattern, vec![Pat::K, Pat::K, Pat::P, Pat::K, Pat::K]);
    }
}
