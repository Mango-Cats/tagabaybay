//! Tests for Filipino syllabification rules.
//!
//! Test cases derived from Filipino syllabification rules from 2001
//! Revisyon ng Alfabeto at Patnubay sa Ispeling ng Wikang Filipino
//! (2001 Revision Alphabet and Spelling Guide of the Filipino Language)

// to run: cargo test --test syllabification csv_eval -- --nocapture
//
// this will not make a report file
//
// CSV format: `input,expected_hyphenated`
// Example: `buksan,buk-san`

use std::fs::File;
use std::io::{BufRead, BufReader};
use tagabaybay::grapheme::filipino::FilipinoGrapheme::*;
use tagabaybay::grapheme::filipino::hyphenate;
use tagabaybay::grapheme::tokenize::filipino_tokenizer;
use tagabaybay::syllabification::algorithm::*;
use tagabaybay::syllabification::pattern_builder::*;
use tagabaybay::syllabification::types::*;
use tagabaybay::syllabification::validation::*;
use tagabaybay::tokens;

const CSV_PATH: &str = "tests/data/syllabification.csv";

struct TestResult {
    input: String,
    expected: String,
    actual: String,
    valid: bool,
}

fn highlight_differences(actual: &str, expected: &str) -> (String, String) {
    let actual_chars: Vec<char> = actual.chars().collect();
    let expected_chars: Vec<char> = expected.chars().collect();
    let max_len = actual_chars.len().max(expected_chars.len());

    let mut highlighted_actual = String::new();
    let mut highlighted_expected = String::new();

    for i in 0..max_len {
        let a_char = actual_chars.get(i).copied();
        let e_char = expected_chars.get(i).copied();

        match (a_char, e_char) {
            (Some(a), Some(e)) if a == e => {
                highlighted_actual.push(a);
                highlighted_expected.push(e);
            }
            (Some(a), Some(e)) => {
                highlighted_actual.push_str(&a.to_uppercase().to_string());
                highlighted_expected.push_str(&e.to_uppercase().to_string());
            }
            (Some(a), None) => {
                highlighted_actual.push_str(&a.to_uppercase().to_string());
            }
            (None, Some(e)) => {
                highlighted_expected.push_str(&e.to_uppercase().to_string());
            }
            (None, None) => break,
        }
    }

    (highlighted_actual, highlighted_expected)
}

/// Evaluate syllabification against a CSV file.
#[test]
fn from_csv() {
    let file = match File::open(CSV_PATH) {
        Ok(f) => f,
        Err(_) => {
            println!("No CSV file found at {}", CSV_PATH);
            println!("Create a CSV with format: input,expected_hyphenated");
            println!("Example: buksan,buk-san");
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut failures: Vec<TestResult> = Vec::new();
    let mut invalid_syllables: Vec<TestResult> = Vec::new();
    let mut total = 0;
    let mut passed = 0;

    for (i, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        // Skip header and empty lines
        if i == 0 || line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            continue;
        }

        let input = parts[0].trim();
        let expected = parts[1].trim().to_lowercase();

        // Tokenize and syllabify
        let graphemes = filipino_tokenizer(input);
        let (actual, valid) = match syllabify(graphemes.as_slice()) {
            Some((syllables, is_valid)) => (hyphenate(&syllables), is_valid),
            None => (String::from("<failed>"), false),
        };

        total += 1;
        let matches = actual == expected;

        if matches {
            passed += 1;
        } else {
            failures.push(TestResult {
                input: input.to_string(),
                expected: expected.clone(),
                actual: actual.clone(),
                valid,
            });
        }

        // Track invalid syllabifications separately (even if they match expected)
        if !valid && matches {
            invalid_syllables.push(TestResult {
                input: input.to_string(),
                expected: expected.clone(),
                actual,
                valid,
            });
        }
    }

    // Print report
    let failed = total - passed;
    let num_invalid = failures.iter().filter(|f| !f.valid).count() + invalid_syllables.len();
    let accuracy = if total > 0 {
        (passed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    println!("\nSYLLABIFICATION EVALUATION");
    println!("{}", "=".repeat(50));
    println!("├── Total           {}", total);
    println!("├── Passed          {}", passed);
    println!("├── Failed          {}", failed);
    println!("├── Invalid         {}", num_invalid);
    println!("└── Accuracy        {:.2}%", accuracy);

    if !failures.is_empty() {
        let max_input = failures
            .iter()
            .map(|f| f.input.len())
            .max()
            .unwrap_or(10)
            .max(10);
        let max_actual = failures
            .iter()
            .map(|f| f.actual.len())
            .max()
            .unwrap_or(10)
            .max(10);
        let max_expected = failures
            .iter()
            .map(|f| f.expected.len())
            .max()
            .unwrap_or(10)
            .max(10);

        println!("\nFailures:");
        println!(
            "  {}  {:<width_in$}  {:<width_act$}  {:<width_exp$}  {}",
            " ~ ",
            "INPUT",
            "ACTUAL",
            "EXPECTED",
            "VALID",
            width_in = max_input,
            width_act = max_actual,
            width_exp = max_expected
        );
        println!(
            "  {}  {}  {}  {}  {}",
            "-".repeat(4),
            "-".repeat(max_input),
            "-".repeat(max_actual),
            "-".repeat(max_expected),
            "-".repeat(5)
        );

        for (i, f) in failures.iter().enumerate() {
            let (highlighted_actual, highlighted_expected) =
                highlight_differences(&f.actual, &f.expected);
            let valid_mark = if f.valid { "O" } else { "X" };
            println!(
                "  {:>3}. {:<width_in$}  {:<width_act$}  {:<width_exp$}  {}",
                i + 1,
                f.input,
                highlighted_actual,
                highlighted_expected,
                valid_mark,
                width_in = max_input,
                width_act = max_actual,
                width_exp = max_expected
            );
        }
    }

    // Show words that matched but had invalid syllable patterns
    if !invalid_syllables.is_empty() {
        println!("\nInvalid Syllable Patterns (matched but pattern invalid):");
        for f in &invalid_syllables {
            println!("  - {} → {}", f.input, f.actual);
        }
    }

    // Don't fail the test, just report
    if total == 0 {
        println!("\nNo test cases found. Add entries to {}", CSV_PATH);
    }
}

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

    /// transkripsyon → trans-krips-yon
    /// (P-S splits, then S-Y splits: S with preceding, Y with following)
    #[test]
    fn test_transkripsyon() {
        let word = tokens![T, R, A, N, S, K, R, I, P, S, Y, O, N];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "trans-krips-yon");
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

    /// ekstradisyon → eks-tra-dis-yon
    /// (S-Y splits: S with preceding, Y with following - matches telebisyon pattern)
    #[test]
    fn test_ekstradisyon() {
        let word = tokens![E, K, S, T, R, A, D, I, S, Y, O, N];
        assert!(validate_filipino_syllables_dp(word.clone()));

        let (syllables, _) = syllabify(&word).unwrap();
        assert_eq!(hyphenate(&syllables), "eks-tra-dis-yon");
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
