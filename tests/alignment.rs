//! Tests for correct mapping of ipa to filipino graphemes.

// to run: set CSV_NAME=<csv_name>.csv && cargo test --test alignment evaluate_csv -- --nocapture
//
// this will not make a report file, ensure that the csv is in /data
//
// CSV format: `input,expected`
// Example: `alphabet,alfabet`

use std::fs::File;
use std::io::{BufRead, BufReader};
use tagabaybay::configs::AdapterConfig;
use tagabaybay::g2p::G2Py;
use tagabaybay::grapheme::filipino::graphemes_to_string;
use tagabaybay::phoneme::tokenizer::ipa::tokenize_ipa;
use tagabaybay::grapheme::tokenize::source_tokenizer;
use tagabaybay::alignment::{
    alignment::phoneme_grapheme_alignment,
    aligned_string::ipa_to_filipino_graphemes
};

const CSV_PATH: &str = "tests/data/";

struct TestResult {
    input: String,
    expected: String,
    actual: String
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

#[test]
fn evaluate_csv() {
    let csv_name = std::env::var("CSV_NAME").expect("CSV_NAME not set");
    let csv_path = format!("{}{}", CSV_PATH, csv_name);

    let file = match File::open(&csv_path) {
        Ok(f) => f,
        Err(_) => {
            println!("No CSV file found at {}", csv_path);
            println!("Create a CSV with format: input, expected");
            return;
        }
    };

    let config = AdapterConfig::new();
    let mut ipa_g2p = G2Py::new().ok();
    let reader = BufReader::new(file);
    let mut failures: Vec<TestResult> = Vec::new();
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
        let mut actual = String::new();

        // align ipa and map to filipino graphemes
        if let Some(ref mut g2p) = ipa_g2p {
            if let Ok(phonemes) = g2p.phonemize_phrase(&input, None, None, &config) {
                println!("* {phonemes}");
                let aligned_string = phoneme_grapheme_alignment(tokenize_ipa(&phonemes), source_tokenizer(input));
                let ipa_to_fg = ipa_to_filipino_graphemes(&aligned_string);
                let mapped_string = graphemes_to_string(&ipa_to_fg);
                println!("-> {mapped_string} || {expected}\n");

                actual = mapped_string;
            }
        }

        total += 1;
        let matches = actual == expected;

        if matches {
            passed += 1;
        } else {
            failures.push(TestResult {
                input: input.to_string(),
                expected: expected.clone(),
                actual: actual.clone()
            });
        }
    }

    // Print report
    let failed = total - passed;
    // let num_invalid = failures.iter().filter(|f| !f.valid).count() + invalid_syllables.len();
    let accuracy = if total > 0 {
        (passed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    println!("\nALIGNMENT EVALUATION");
    println!("{}", "=".repeat(50));
    println!("├── Total           {}", total);
    println!("├── Passed          {}", passed);
    println!("├── Failed          {}", failed);
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
            println!(
                "  {:>3}. {:<width_in$}  {:<width_act$}  {:<width_exp$}  {}",
                i + 1,
                f.input,
                highlighted_actual,
                highlighted_expected,
                width_in = max_input,
                width_act = max_actual,
                width_exp = max_expected
            );
        }
    }

    // Don't fail the test, just report
    if total == 0 {
        println!("\nNo test cases found. Add entries to {}", CSV_PATH);
    }
}