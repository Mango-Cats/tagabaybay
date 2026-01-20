// to run: cargo test --test eval
//
// then, this will put test files in
//      `.tests/report/`
// the file naming convention is as follows
//      `<MMDDYY>_<HHMM>_<GOLD_STANDARD>.txt`
//      `<MMDDYY>_<HHMM>_overall.txt`

use chrono::Local;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use tagabaybay::adaptation::adapter::Adapter;
use tagabaybay::configs::AdaptationConfig;
use tagabaybay::grapheme::filipino::graphemes_to_string;

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

/// Token error information for ranking
#[derive(Clone, Debug)]
struct TokenError {
    expected: String,
    actual: String,
    count: usize,
    examples: Vec<String>,
}

struct EvalReport {
    total: usize,
    passed: usize,
    #[allow(dead_code)]
    failed: usize,
    accuracy: f64,
    word_error_rate: f64,
    token_error_rate: f64,
    total_token_edits: usize,
    total_tokens: usize,
    failures: Vec<TestResult>,
    token_errors: HashMap<String, TokenError>,
}

struct OverallMetrics {
    total: usize,
    passed: usize,
    failed: usize,
    accuracy: f64,
    wer: f64,
    ter: f64,
    total_token_edits: usize,
    total_tokens: usize,
    worst_performer: String,
    worst_accuracy: f64,
    per_dataset: Vec<DatasetMetrics>,
    token_errors: HashMap<String, TokenError>,
}

struct DatasetMetrics {
    name: String,
    accuracy: f64,
    wer: f64,
    ter: f64,
    report_file: String,
}

/// Calculate Levenshtein edit distance between two strings
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[m][n]
}

/// Get the edit operations (alignment) between two strings for token error tracking
fn get_edit_operations(actual: &str, expected: &str) -> Vec<(Option<char>, Option<char>)> {
    let a_chars: Vec<char> = actual.to_lowercase().chars().collect();
    let e_chars: Vec<char> = expected.to_lowercase().chars().collect();
    let m = a_chars.len();
    let n = e_chars.len();

    if m == 0 && n == 0 {
        return vec![];
    }

    // Build DP table
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1] == e_chars[j - 1] {
                0
            } else {
                1
            };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    // Backtrack to find operations
    let mut ops = vec![];
    let mut i = m;
    let mut j = n;

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && a_chars[i - 1] == e_chars[j - 1] {
            // Match - no error
            i -= 1;
            j -= 1;
        } else if i > 0 && j > 0 && dp[i][j] == dp[i - 1][j - 1] + 1 {
            // Substitution
            ops.push((Some(a_chars[i - 1]), Some(e_chars[j - 1])));
            i -= 1;
            j -= 1;
        } else if i > 0 && dp[i][j] == dp[i - 1][j] + 1 {
            // Deletion (extra char in actual)
            ops.push((Some(a_chars[i - 1]), None));
            i -= 1;
        } else if j > 0 && dp[i][j] == dp[i][j - 1] + 1 {
            // Insertion (missing char in actual)
            ops.push((None, Some(e_chars[j - 1])));
            j -= 1;
        } else {
            break;
        }
    }

    ops.reverse();
    ops
}

/// Track token-level errors from edit operations
fn track_token_errors(
    token_errors: &mut HashMap<String, TokenError>,
    actual: &str,
    expected: &str,
    input: &str,
    max_examples: usize,
) {
    let ops = get_edit_operations(actual, expected);
    for (actual_char, expected_char) in ops {
        let key = format!(
            "{}->{}",
            actual_char
                .map(|c| c.to_string())
                .unwrap_or_else(|| "∅".to_string()),
            expected_char
                .map(|c| c.to_string())
                .unwrap_or_else(|| "∅".to_string())
        );
        let entry = token_errors.entry(key).or_insert(TokenError {
            expected: expected_char
                .map(|c| c.to_string())
                .unwrap_or_else(|| "∅".to_string()),
            actual: actual_char
                .map(|c| c.to_string())
                .unwrap_or_else(|| "∅".to_string()),
            count: 0,
            examples: vec![],
        });
        entry.count += 1;
        if entry.examples.len() < max_examples {
            entry.examples.push(input.to_string());
        }
    }
}

/// Merge token errors from one map into another
fn merge_token_errors(
    target: &mut HashMap<String, TokenError>,
    source: HashMap<String, TokenError>,
    max_examples: usize,
) {
    for (key, error) in source {
        let entry = target.entry(key).or_insert(TokenError {
            expected: error.expected.clone(),
            actual: error.actual.clone(),
            count: 0,
            examples: vec![],
        });
        entry.count += error.count;
        for example in error.examples {
            if entry.examples.len() < max_examples {
                entry.examples.push(example);
            }
        }
    }
}

fn evaluate_csv(path: &str) -> EvalReport {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let adapter = Adapter::new();
    let config = AdaptationConfig::new();

    let mut results: Vec<TestResult> = Vec::new();
    let mut total_token_edits: usize = 0;
    let mut total_tokens: usize = 0;
    let mut token_errors: HashMap<String, TokenError> = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            continue; // Skip header
        }

        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            continue;
        }

        let input = parts[0].trim();
        let expected = parts[1].trim();
        let actual = adapter
            .adaptation(input, &config)
            .map(|phl_graphemes| graphemes_to_string(&phl_graphemes))
            .unwrap_or_default();

        let actual_lower = actual.to_lowercase();
        let expected_lower = expected.to_lowercase();

        // Calculate token-level edit distance
        let edit_dist = levenshtein_distance(&actual_lower, &expected_lower);
        total_token_edits += edit_dist;
        total_tokens += expected_lower.chars().count().max(1);

        // Track token errors for ranking
        if actual_lower != expected_lower {
            track_token_errors(&mut token_errors, &actual, expected, input, 3);
        }

        results.push(TestResult {
            input: input.to_string(),
            expected: expected.to_string(),
            actual: actual.clone(),
            passed: actual_lower == expected_lower,
        });
    }

    let total = results.len();
    let passed = results.iter().filter(|r| r.passed).count();
    let failures: Vec<TestResult> = results.into_iter().filter(|r| !r.passed).collect();

    let word_error_rate = if total > 0 {
        ((total - passed) as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let token_error_rate = if total_tokens > 0 {
        (total_token_edits as f64 / total_tokens as f64) * 100.0
    } else {
        0.0
    };

    EvalReport {
        total,
        passed,
        failed: total - passed,
        accuracy: (passed as f64 / total as f64) * 100.0,
        word_error_rate,
        token_error_rate,
        total_token_edits,
        total_tokens,
        failures,
        token_errors,
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

fn format_token_errors(
    token_errors: &HashMap<String, TokenError>,
    limit: usize,
    max_example_len: usize,
) -> String {
    let mut content = String::new();

    if token_errors.is_empty() {
        return content;
    }

    content.push_str("\nToken Error Ranking:\n");
    content.push_str("  (actual->expected : count : examples)\n");
    content.push_str(&format!("  {}\n", "-".repeat(60)));

    let mut errors: Vec<_> = token_errors.iter().collect();
    errors.sort_by(|a, b| b.1.count.cmp(&a.1.count));

    for (i, (key, error)) in errors.iter().take(limit).enumerate() {
        let examples_str = error.examples.join(", ");
        content.push_str(&format!(
            "  {:>3}. {:<10} : {:>4} : {}\n",
            i + 1,
            key,
            error.count,
            if examples_str.len() > max_example_len {
                format!("{}...", &examples_str[..max_example_len])
            } else {
                examples_str
            }
        ));
    }

    content
}

fn write_dataset_report(name: &str, report: &EvalReport, timestamp: &str) -> String {
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

    let mut content = String::new();

    // Header metrics
    content.push_str(&format!("{}\n", name));
    content.push_str(&format!("├── Total           {}\n", report.total));
    content.push_str(&format!("├── Passed          {}\n", report.passed));
    content.push_str(&format!(
        "├── Failed          {}\n",
        report.total - report.passed
    ));
    content.push_str(&format!("├── Accuracy        {:.2}%\n", report.accuracy));
    content.push_str(&format!(
        "├── WER             {:.2}%\n",
        report.word_error_rate
    ));
    content.push_str(&format!(
        "├── TER             {:.2}% ({}/{})\n",
        report.token_error_rate, report.total_token_edits, report.total_tokens
    ));
    content.push_str(&format!(
        "└── Accept@{:<3}      {}\n",
        ACCEPT,
        report.accuracy > ACCEPT
    ));

    // Failures list
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

    // Token error ranking
    content.push_str(&format_token_errors(&report.token_errors, 20, 40));

    let mut file = File::create(&filename).expect("Failed to create report file");
    file.write_all(content.as_bytes())
        .expect("Failed to write report");

    filename
}

fn write_overall_report(metrics: &OverallMetrics, timestamp: &str) -> String {
    let filename = format!("{}/{}_overall.txt", REPORT_DIR, timestamp);

    let mut content = String::new();

    // Header
    content.push_str("OVERALL EVALUATION REPORT\n");
    content.push_str(&format!(
        "Generated: {}\n",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    ));
    content.push_str(&format!("{}\n\n", "=".repeat(50)));

    // Summary metrics
    content.push_str("Summary\n");
    content.push_str(&format!("{}\n", "-".repeat(50)));
    content.push_str(&format!("├── Total           {}\n", metrics.total));
    content.push_str(&format!("├── Passed          {}\n", metrics.passed));
    content.push_str(&format!("├── Failed          {}\n", metrics.failed));
    content.push_str(&format!("├── Accuracy        {:.2}%\n", metrics.accuracy));
    content.push_str(&format!("├── WER             {:.2}%\n", metrics.wer));
    content.push_str(&format!(
        "├── TER             {:.2}% ({}/{})\n",
        metrics.ter, metrics.total_token_edits, metrics.total_tokens
    ));
    content.push_str(&format!(
        "├── Accept@{:<3}      {}\n",
        ACCEPT,
        metrics.accuracy > ACCEPT
    ));
    content.push_str(&format!(
        "└── Worst Performer {} ({:.2}%)\n",
        metrics.worst_performer, metrics.worst_accuracy
    ));

    // Per-dataset breakdown
    content.push_str("\nPer-Dataset Metrics\n");
    content.push_str(&format!("{}\n", "-".repeat(50)));
    content.push_str(&format!(
        "  {:<15} {:>10} {:>10} {:>10}\n",
        "Dataset", "Accuracy", "WER", "TER"
    ));
    content.push_str(&format!("  {}\n", "-".repeat(47)));

    for dm in &metrics.per_dataset {
        content.push_str(&format!(
            "  {:<15} {:>9.2}% {:>9.2}% {:>9.2}%\n",
            dm.name, dm.accuracy, dm.wer, dm.ter
        ));
    }

    // Token error ranking
    content.push_str(&format_token_errors(&metrics.token_errors, 25, 50));

    // Report file locations
    content.push_str("\nIndividual Reports\n");
    content.push_str(&format!("{}\n", "-".repeat(50)));
    for dm in &metrics.per_dataset {
        content.push_str(&format!(
            "  - {} ({:.2}%): {}\n",
            dm.name, dm.accuracy, dm.report_file
        ));
    }

    let mut file = File::create(&filename).expect("Failed to create overall report file");
    file.write_all(content.as_bytes())
        .expect("Failed to write overall report");

    filename
}

fn print_overall_metrics(metrics: &OverallMetrics) {
    println!("\nOVERALL");
    println!("=======");
    println!("├── Total           {}", metrics.total);
    println!("├── Passed          {}", metrics.passed);
    println!("├── Failed          {}", metrics.failed);
    println!("├── Accuracy        {:.2}%", metrics.accuracy);
    println!("├── WER             {:.2}%", metrics.wer);
    println!(
        "├── TER             {:.2}% ({}/{})",
        metrics.ter, metrics.total_token_edits, metrics.total_tokens
    );
    println!("├── Accept?         {}", metrics.accuracy > ACCEPT);
    println!(
        "└── Worst Performer {} ({:.2}%)",
        metrics.worst_performer, metrics.worst_accuracy
    );

    println!("\nPer-Dataset Metrics:");
    println!(
        "  {:<15} {:>10} {:>10} {:>10}",
        "Dataset", "Accuracy", "WER", "TER"
    );
    println!("  {}", "-".repeat(50));
    for dm in &metrics.per_dataset {
        println!(
            "  {:<15} {:>9.2}% {:>9.2}% {:>9.2}%",
            dm.name, dm.accuracy, dm.wer, dm.ter
        );
    }

    // Top token errors
    println!("\nTop Token Errors (across all datasets):");
    println!("  (actual->expected : count : examples)");
    println!("  {}", "-".repeat(70));

    let mut errors: Vec<_> = metrics.token_errors.iter().collect();
    errors.sort_by(|a, b| b.1.count.cmp(&a.1.count));

    for (i, (key, error)) in errors.iter().take(15).enumerate() {
        let examples_str = error.examples.join(", ");
        println!(
            "  {:>3}. {:<10} : {:>4} : {}",
            i + 1,
            key,
            error.count,
            if examples_str.len() > 50 {
                format!("{}...", &examples_str[..50])
            } else {
                examples_str
            }
        );
    }
}

fn print_report_locations(metrics: &OverallMetrics, overall_file: &str) {
    println!("\nReports written to:");
    println!("  - overall: {}", overall_file);
    for dm in &metrics.per_dataset {
        println!("  - {} ({:.2}%): {}", dm.name, dm.accuracy, dm.report_file);
    }
}

#[test]
fn compare() {
    fs::create_dir_all(REPORT_DIR).expect("Failed to create report directory");

    let timestamp = Local::now().format("%m%d%y_%H%M").to_string();

    let mut total_all = 0;
    let mut passed_all = 0;
    let mut total_token_edits_all = 0;
    let mut total_tokens_all = 0;
    let mut all_token_errors: HashMap<String, TokenError> = HashMap::new();
    let mut per_dataset: Vec<DatasetMetrics> = Vec::new();

    // Evaluate each dataset
    for fname in GOLD_STANDARDS {
        let report = evaluate_csv(format!("{}/{}", GOLD_DIR, fname).as_str());
        let report_file = write_dataset_report(fname, &report, &timestamp);

        per_dataset.push(DatasetMetrics {
            name: fname.replace(".csv", ""),
            accuracy: report.accuracy,
            wer: report.word_error_rate,
            ter: report.token_error_rate,
            report_file,
        });

        total_all += report.total;
        passed_all += report.passed;
        total_token_edits_all += report.total_token_edits;
        total_tokens_all += report.total_tokens;

        merge_token_errors(&mut all_token_errors, report.token_errors, 5);
    }

    // Find worst performer
    let worst = per_dataset
        .iter()
        .min_by(|a, b| a.accuracy.partial_cmp(&b.accuracy).unwrap())
        .unwrap();

    // Build overall metrics
    let metrics = OverallMetrics {
        total: total_all,
        passed: passed_all,
        failed: total_all - passed_all,
        accuracy: (passed_all as f64 / total_all as f64) * 100.0,
        wer: ((total_all - passed_all) as f64 / total_all as f64) * 100.0,
        ter: (total_token_edits_all as f64 / total_tokens_all as f64) * 100.0,
        total_token_edits: total_token_edits_all,
        total_tokens: total_tokens_all,
        worst_performer: worst.name.clone(),
        worst_accuracy: worst.accuracy,
        per_dataset,
        token_errors: all_token_errors,
    };

    // Write overall report file
    let overall_file = write_overall_report(&metrics, &timestamp);

    // Print to console
    print_overall_metrics(&metrics);
    print_report_locations(&metrics, &overall_file);
}
