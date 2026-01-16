// to run: cargo test --test eval
//
// then, this will put test files in
//      `target/tests/report/`
// the file naming convention is as follows
//      `<YY><MM><DD>_<HH><MM>_<GOLD_STANDARD>`

use chrono::Local;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use tagabaybay::adaptation::adapter::Adapter;
use tagabaybay::configs::AdaptationConfig;
use tagabaybay::grapheme::filipino::phl_graphemes_to_string;

const GOLD_DIR: &str = "gold/data";
const GOLD_COUNT: usize = 3;
const GOLD_STANDARDS: [&str; GOLD_COUNT] =
    ["common_drugs.csv", "common_eng.csv", "ph_fda_human.csv"];

const ACCEPT: f64 = 70.;

const REPORT_DIR: &str = ".tests/report";

struct TestResult {
    input: String,
    expected: String,
    actual: String,
    passed: bool,
}

struct EvalReport {
    total: usize,
    passed: usize,
    #[allow(dead_code)]
    failed: usize,
    accuracy: f64,
    failures: Vec<TestResult>,
}

fn evaluate_csv(path: &str) -> EvalReport {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let adapter = Adapter::new();
    let config = AdaptationConfig::new();

    let mut results: Vec<TestResult> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            continue;
        } // Skip header

        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            continue;
        }

        let input = parts[0].trim();
        let expected = parts[1].trim();
        let actual = adapter
            .adaptation(input, &config)
            .map(|phl_graphemes| phl_graphemes_to_string(&phl_graphemes))
            .unwrap_or_default();

        results.push(TestResult {
            input: input.to_string(),
            expected: expected.to_string(),
            actual: actual.clone(),
            passed: actual.to_lowercase() == expected.to_lowercase(),
        });
    }

    let total = results.len();
    let passed = results.iter().filter(|r| r.passed).count();
    let failures: Vec<TestResult> = results.into_iter().filter(|r| !r.passed).collect();

    EvalReport {
        total,
        passed,
        failed: total - passed,
        accuracy: (passed as f64 / total as f64) * 100.0,
        failures,
    }
}

fn highlight_differences(actual: &str, expected: &str) -> (String, String) {
    let actual_chars: Vec<char> = actual.to_lowercase().chars().collect();
    let expected_chars: Vec<char> = expected.to_lowercase().chars().collect();
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
                // Different characters - capitalize both
                highlighted_actual.push_str(&a.to_uppercase().to_string());
                highlighted_expected.push_str(&e.to_uppercase().to_string());
            }
            (Some(a), None) => {
                // Actual is longer - capitalize extra chars
                highlighted_actual.push_str(&a.to_uppercase().to_string());
            }
            (None, Some(e)) => {
                // Expected is longer - capitalize extra chars
                highlighted_expected.push_str(&e.to_uppercase().to_string());
            }
            (None, None) => break,
        }
    }

    (highlighted_actual, highlighted_expected)
}

fn write_report(name: &str, report: &EvalReport, timestamp: &str) -> String {
    let csv_name = name.replace(".csv", "");
    let filename = format!("{}/{}_{}.txt", REPORT_DIR, timestamp, csv_name);

    let max_width = |extract: fn(&TestResult) -> usize| -> usize {
        report
            .failures
            .iter()
            .map(extract)
            .max()
            .unwrap_or(10)
            .max(10)
    };

    let max_input = max_width(|f| f.input.len());
    let max_actual = max_width(|f| f.actual.len());
    let max_expected = max_width(|f| f.expected.len());

    // Report Content
    let mut content = String::new();
    content.push_str(&format!("{}\n", name));
    content.push_str(&format!("├── Total           {}\n", report.total));
    content.push_str(&format!("├── Passed          {}\n", report.passed));
    content.push_str(&format!(
        "├── Failed          {}\n",
        report.total - report.passed
    ));
    content.push_str(&format!("├── Accuracy        {:.2}%\n", report.accuracy));
    content.push_str(&format!(
        "└── Accept@{:<3}      {}\n",
        ACCEPT,
        report.accuracy > ACCEPT
    ));

    if !report.failures.is_empty() {
        content.push_str("\nFailures:\n");
        content.push_str(&format!(
            "  {}  {:<width_in$}  {:<width_act$}  {:<width_exp$}\n",
            " ~ ",
            "INPUT",
            "ACTUAL",
            "EXPECTED",
            width_in = max_input,
            width_act = max_actual,
            width_exp = max_expected
        ));
        content.push_str(&format!(
            "  {}  {}  {}  {}\n",
            "-".repeat(4),
            "-".repeat(max_input),
            "-".repeat(max_actual),
            "-".repeat(max_expected)
        ));

        for (i, f) in report.failures.iter().enumerate() {
            let (highlighted_actual, highlighted_expected) =
                highlight_differences(&f.actual, &f.expected);
            content.push_str(&format!(
                "  {:>3}. {:<width_in$}  {:<width_act$}  {:<width_exp$}\n",
                i + 1,
                f.input,
                highlighted_actual,
                highlighted_expected,
                width_in = max_input,
                width_act = max_actual,
                width_exp = max_expected
            ));
        }
    }

    let mut file = File::create(&filename).expect("Failed to create report file");
    file.write_all(content.as_bytes())
        .expect("Failed to write report");

    filename
}

#[test]
fn compare() {
    fs::create_dir_all(REPORT_DIR).expect("Failed to create report directory");

    let timestamp = Local::now().format("%m%d%y_%H%M").to_string();

    let mut total_all = 0;
    let mut passed_all = 0;
    let mut reports_info: Vec<(&str, f64, String)> = Vec::new();

    for fname in GOLD_STANDARDS {
        let report = evaluate_csv(format!("{}/{}", GOLD_DIR, fname).as_str());
        let report_file = write_report(fname, &report, &timestamp);
        reports_info.push((fname, report.accuracy, report_file));
        total_all += report.total;
        passed_all += report.passed;
    }

    // Find worst performing gold standard
    let worst = reports_info
        .iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();

    let overall = (passed_all as f64 / total_all as f64) * 100.0;
    println!("\nOVERALL");
    println!("=======");
    println!("├── Total           {}", total_all);
    println!("├── Passed          {}", passed_all);
    println!("├── Failed          {}", total_all - passed_all);
    println!("├── Accuracy        {:.2}%", overall);
    println!("├── Accept?         {}", overall > ACCEPT);
    println!(
        "└── Worst Performer {} ({:.2}%)",
        worst.0.replace(".csv", ""),
        worst.1
    );

    println!("\nReports written to:");
    for (name, accuracy, file) in reports_info {
        println!(
            "  - {} ({:.2}%): {}",
            name.replace(".csv", ""),
            accuracy,
            file
        );
    }
}
