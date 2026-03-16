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
use tagabaybay::alignment::alignment_adapter::adapt_aligned;
use tagabaybay::configs::AdapterConfig;
use tagabaybay::g2p::G2Py;

const GOLD_DIR: &str = "gold/data";
const GOLD_COUNT: usize = 5;
const GOLD_STANDARDS: [&str; GOLD_COUNT] = [
    "common_drugs.csv",
    "common_eng.csv",
    "ph_fda_human.csv",
    "ching_chua.csv",
    "wiki.csv",
];

const ACCEPT_TER: f64 = 20.;
const REPORT_DIR: &str = ".tests/report";

// Character equivalence mappings for toggle-agnostic evaluation
const CHAR_EQUIVALENCES: &[(&str, &str)] = &[
    ("sh", "s"), // allow_sh_letter
    ("z", "s"),  // allow_z_letter
    ("j", "dy"), // allow_j_letter
    ("v", "b"),  // allow_v_letter
];

#[derive(Clone, Debug)]
struct EvalConfig {
    equate_e_i: bool,
    equate_o_u: bool,
}

fn evaluate_csv(path: &str, eval_config: &EvalConfig) -> EvalReport {
    let config = AdapterConfig::new();
    let mut _adapter = Adapter::new_with_config(config.clone());
    let mut ipa_g2p = G2Py::new().ok();

    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut results: Vec<TestResult> = Vec::new();
    let mut total_token_edits: usize = 0;
    let mut total_tokens: usize = 0;
    let mut token_errors: HashMap<String, TokenError> = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            continue;
        }

        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            continue;
        }

        let input = parts[0].trim();
        let expected = parts[1].trim();
        // let actual = adapter
        //     .adaptation(input)
        //     .map(|phl_graphemes| graphemes_to_string(&phl_graphemes))
        //     .unwrap_or_default();

        let mut actual = String::new();

        if let Some(ref mut g2p) = ipa_g2p {
            actual = adapt_aligned(input, g2p, &config);
        }

        // Normalize both actual and expected for toggle-agnostic comparison
        let actual_normalized = normalize_for_comparison(&actual, eval_config);
        let expected_normalized = normalize_for_comparison(expected, eval_config);

        // Calculate token-level edit distance on normalized strings
        let edit_dist = levenshtein_distance(&actual_normalized, &expected_normalized);
        total_token_edits += edit_dist;
        total_tokens += expected_normalized.chars().count().max(1);

        // Track token errors for ranking (on normalized strings)
        if actual_normalized != expected_normalized {
            track_token_errors(
                &mut token_errors,
                &actual_normalized,
                &expected_normalized,
                input,
                3,
                eval_config,
            );
        }

        results.push(TestResult {
            input: input.to_string(),
            expected: expected.to_string(),
            actual: actual.clone(),
            passed: actual_normalized == expected_normalized,
        });
    }

    let total = results.len();
    let failures: Vec<TestResult> = results.into_iter().filter(|r| !r.passed).collect();

    let token_error_rate = if total_tokens > 0 {
        (total_token_edits as f64 / total_tokens as f64) * 100.0
    } else {
        0.0
    };

    EvalReport {
        total,
        token_error_rate,
        total_token_edits,
        total_tokens,
        failures,
        token_errors,
    }
}

struct TestResult {
    input: String,
    expected: String,
    actual: String,
    passed: bool,
}

#[derive(Clone, Debug)]
struct TokenError {
    expected: String,
    actual: String,
    count: usize,
    examples: Vec<String>,
}

struct EvalReport {
    total: usize,
    token_error_rate: f64,
    total_token_edits: usize,
    total_tokens: usize,
    failures: Vec<TestResult>,
    token_errors: HashMap<String, TokenError>,
}

struct OverallMetrics {
    cer_default: f64,
    cer_e_i_y: f64,
    cer_o_u: f64,
    cer_both: f64,
    words: usize,
    tokens: usize,
    edits: usize,
    ter: f64,
    worst_performer: String,
    worst_ter: f64,
    per_dataset: Vec<DatasetMetrics>,
    token_errors: HashMap<String, TokenError>,
    token_errors_both: HashMap<String, TokenError>,
}

struct DatasetMetrics {
    name: String,
    ter: f64,
    report_file: String,
}

fn normalize_for_comparison(text: &str, cfg: &EvalConfig) -> String {
    let mut normalized = text.to_lowercase();

    let mut equivalences: Vec<_> = CHAR_EQUIVALENCES.iter().collect();
    equivalences.sort_by_key(|(a, b)| std::cmp::Reverse(a.len().max(b.len())));

    for &(variant1, variant2) in equivalences.iter() {
        let canonical = if variant1 < variant2 {
            variant1
        } else {
            variant2
        };
        let alternate = if variant1 < variant2 {
            variant2
        } else {
            variant1
        };

        normalized = normalized.replace(alternate, canonical);
    }

    let mut out = String::with_capacity(normalized.len());
    for c in normalized.chars() {
        let mapped = match c {
            'i' if cfg.equate_e_i => 'e',
            'y' if cfg.equate_e_i => 'e',
            'u' if cfg.equate_o_u => 'o',
            _ => c,
        };
        out.push(mapped);
    }

    out
}

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

fn get_edit_operations(
    actual: &str,
    expected: &str,
    cfg: &EvalConfig,
) -> Vec<(Option<char>, Option<char>)> {
    let actual_normalized = normalize_for_comparison(actual, cfg);
    let expected_normalized = normalize_for_comparison(expected, cfg);

    let a_chars: Vec<char> = actual_normalized.chars().collect();
    let e_chars: Vec<char> = expected_normalized.chars().collect();
    let m = a_chars.len();
    let n = e_chars.len();

    if m == 0 && n == 0 {
        return vec![];
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

    let mut ops = vec![];
    let mut i = m;
    let mut j = n;

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && a_chars[i - 1] == e_chars[j - 1] {
            i -= 1;
            j -= 1;
        } else if i > 0 && j > 0 && dp[i][j] == dp[i - 1][j - 1] + 1 {
            ops.push((Some(a_chars[i - 1]), Some(e_chars[j - 1])));
            i -= 1;
            j -= 1;
        } else if i > 0 && dp[i][j] == dp[i - 1][j] + 1 {
            ops.push((Some(a_chars[i - 1]), None));
            i -= 1;
        } else if j > 0 && dp[i][j] == dp[i][j - 1] + 1 {
            ops.push((None, Some(e_chars[j - 1])));
            j -= 1;
        } else {
            break;
        }
    }

    ops.reverse();
    ops
}

fn track_token_errors(
    token_errors: &mut HashMap<String, TokenError>,
    actual: &str,
    expected: &str,
    input: &str,
    max_examples: usize,
    cfg: &EvalConfig,
) {
    let ops = get_edit_operations(actual, expected, cfg);
    for (actual_char, expected_char) in ops {
        // Skip non-alphabetic characters (digits, spaces, punctuation) — they
        // distort the ranking and aren't phonemic targets we're diagnosing.
        let is_alpha = |c: Option<char>| c.map_or(true, |ch| ch.is_ascii_alphabetic());
        if !is_alpha(actual_char) || !is_alpha(expected_char) {
            continue;
        }
        let key = format!(
            "{}->{}",
            actual_char
                .map(|c| c.to_string())
                .unwrap_or_else(|| "#".to_string()),
            expected_char
                .map(|c| c.to_string())
                .unwrap_or_else(|| "#".to_string())
        );
        let entry = token_errors.entry(key).or_insert(TokenError {
            expected: expected_char
                .map(|c| c.to_string())
                .unwrap_or_else(|| "#".to_string()),
            actual: actual_char
                .map(|c| c.to_string())
                .unwrap_or_else(|| "#".to_string()),
            count: 0,
            examples: vec![],
        });
        entry.count += 1;
        if entry.examples.len() < max_examples && !entry.examples.iter().any(|e| e == input) {
            entry.examples.push(input.to_string());
        }
    }
}

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
            if entry.examples.len() < max_examples && !entry.examples.iter().any(|e| e == &example)
            {
                entry.examples.push(example);
            }
        }
    }
}

fn highlight_differences(actual: &str, expected: &str, cfg: &EvalConfig) -> (String, String) {
    let actual_normalized = normalize_for_comparison(actual, cfg);
    let expected_normalized = normalize_for_comparison(expected, cfg);

    let actual_chars: Vec<char> = actual_normalized.chars().collect();
    let expected_chars: Vec<char> = expected_normalized.chars().collect();
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
    title: &str,
    token_errors: &HashMap<String, TokenError>,
    limit: usize,
) -> String {
    let mut content = String::new();

    if token_errors.is_empty() {
        return content;
    }

    content.push_str(&format!("\n{}\n", title));
    content.push_str("  # = absent character\n");
    content.push_str(
        "  x->y = substitution  |  #->y = missing (insertion)  |  x-># = extra (deletion)\n",
    );
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
            examples_str
        ));
    }

    content
}

fn write_dataset_report(
    name: &str,
    report: &EvalReport,
    timestamp: &str,
    eval_config: &EvalConfig,
) -> String {
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
    content.push_str(&format!("├── Words           {}\n", report.total));
    content.push_str(&format!("├── Tokens          {}\n", report.total_tokens));
    content.push_str(&format!(
        "├── Edits           {}\n",
        report.total_token_edits
    ));
    content.push_str(&format!(
        "├── CER             {:.2}%\n",
        report.token_error_rate
    ));
    content.push_str(&format!(
        "└── Accept@CER<{:<2}   {}\n",
        ACCEPT_TER,
        report.token_error_rate < ACCEPT_TER
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
                highlight_differences(&f.actual, &f.expected, eval_config);
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

    // Character Error Ranking
    content.push_str(&format_token_errors(
        "Character Error Ranking",
        &report.token_errors,
        usize::MAX,
    ));

    let mut file = File::create(&filename).expect("Failed to create report file");
    file.write_all(content.as_bytes())
        .expect("Failed to write report");

    filename
}

fn write_overall_report(metrics: &OverallMetrics, timestamp: &str) -> String {
    let filename = format!("{}/{}_overall.txt", REPORT_DIR, timestamp);

    let mut content = String::new();

    content.push_str("OVERALL EVALUATION REPORT\n");
    content.push_str(&format!(
        "Generated: {}\n",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    ));
    content.push_str(&format!("{}\n\n", "=".repeat(50)));

    content.push_str("Performance (CER)\n");
    content.push_str(&format!("{}\n", "-".repeat(50)));
    content.push_str(&format!(
        "  {:<8} {:>11} {:>11} {:>11} {:>11}\n",
        "", "default", "e==i==y", "o==u", "both"
    ));
    content.push_str(&format!(
        "  {:<8} {:>10.2}% {:>10.2}% {:>10.2}% {:>10.2}%\n\n",
        "CER", metrics.cer_default, metrics.cer_e_i_y, metrics.cer_o_u, metrics.cer_both
    ));

    content.push_str("Summary\n");
    content.push_str(&format!("{}\n", "-".repeat(50)));
    content.push_str(&format!("├── Words           {}\n", metrics.words));
    content.push_str(&format!("├── Tokens          {}\n", metrics.tokens));
    content.push_str(&format!("├── Edits           {}\n", metrics.edits));
    content.push_str(&format!("├── CER             {:.2}%\n", metrics.ter));
    content.push_str(&format!(
        "├── Accept@CER<{:<2}   {}\n",
        ACCEPT_TER,
        metrics.ter < ACCEPT_TER
    ));
    content.push_str(&format!(
        "└── Worst Performer {} (CER {:.2}%)\n",
        metrics.worst_performer, metrics.worst_ter
    ));

    content.push_str("\nPer-Dataset Metrics\n");
    content.push_str(&format!("{}\n", "-".repeat(50)));
    content.push_str(&format!("  {:<15} {:>10}\n", "Dataset", "CER"));
    content.push_str(&format!("  {}\n", "-".repeat(28)));

    for dm in &metrics.per_dataset {
        content.push_str(&format!("  {:<15} {:>9.2}%\n", dm.name, dm.ter));
    }

    content.push_str(&format_token_errors(
        "Character Error Ranking (5-vowel, default)",
        &metrics.token_errors,
        usize::MAX,
    ));
    content.push_str(&format_token_errors(
        "Character Error Ranking (3-vowel, e==i==y and o==u)",
        &metrics.token_errors_both,
        usize::MAX,
    ));

    content.push_str("\nIndividual Reports\n");
    content.push_str(&format!("{}\n", "-".repeat(50)));
    for dm in &metrics.per_dataset {
        content.push_str(&format!(
            "  - {} (CER {:.2}%): {}\n",
            dm.name, dm.ter, dm.report_file
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
    println!("\nPerformance (CER):");
    println!(
        "  {:<8} {:>11} {:>11} {:>11} {:>11}",
        "", "default", "e==i==y", "o==u", "both"
    );
    println!(
        "  {:<8} {:>10.2}% {:>10.2}% {:>10.2}% {:>10.2}%",
        "CER", metrics.cer_default, metrics.cer_e_i_y, metrics.cer_o_u, metrics.cer_both
    );

    println!("├── Words           {}", metrics.words);
    println!("├── Tokens          {}", metrics.tokens);
    println!("├── Edits           {}", metrics.edits);
    println!("├── CER             {:.2}%", metrics.ter);
    println!("├── Accept?         {}", metrics.ter < ACCEPT_TER);
    println!(
        "└── Worst Performer {} (CER {:.2}%)",
        metrics.worst_performer, metrics.worst_ter
    );

    println!("\nPer-Dataset Metrics:");
    println!("  {:<15} {:>10}", "Dataset", "CER");
    println!("  {}", "-".repeat(28));
    for dm in &metrics.per_dataset {
        println!("  {:<15} {:>9.2}%", dm.name, dm.ter);
    }

    print!(
        "{}",
        format_token_errors("Token Errors (5-vowel, default)", &metrics.token_errors, 20,)
    );
    print!(
        "{}",
        format_token_errors(
            "Token Errors (3-vowel, e==i==y and o==u)",
            &metrics.token_errors_both,
            20,
        )
    );
}

fn print_report_locations(metrics: &OverallMetrics, overall_file: &str) {
    println!("\nReports written to:");
    println!("  - overall: {}", overall_file);
    for dm in &metrics.per_dataset {
        println!("  - {} (CER {:.2}%): {}", dm.name, dm.ter, dm.report_file);
    }
}

fn compute_overall_cer(eval_config: &EvalConfig) -> f64 {
    let mut total_token_edits_all = 0usize;
    let mut total_tokens_all = 0usize;

    for fname in GOLD_STANDARDS {
        let report = evaluate_csv(format!("{}/{}", GOLD_DIR, fname).as_str(), eval_config);
        total_token_edits_all += report.total_token_edits;
        total_tokens_all += report.total_tokens;
    }

    if total_tokens_all == 0 {
        0.0
    } else {
        (total_token_edits_all as f64 / total_tokens_all as f64) * 100.0
    }
}

#[test]
fn compare() {
    fs::create_dir_all(REPORT_DIR).expect("Failed to create report directory");

    let timestamp = Local::now().format("%m%d%y_%H%M").to_string();

    let eval_config = EvalConfig {
        equate_e_i: false,
        equate_o_u: false,
    };
    let cer_default = compute_overall_cer(&EvalConfig {
        equate_e_i: false,
        equate_o_u: false,
    });
    let cer_e_i_y = compute_overall_cer(&EvalConfig {
        equate_e_i: true,
        equate_o_u: false,
    });
    let cer_o_u = compute_overall_cer(&EvalConfig {
        equate_e_i: false,
        equate_o_u: true,
    });
    let cer_both = compute_overall_cer(&EvalConfig {
        equate_e_i: true,
        equate_o_u: true,
    });

    let mut words_all = 0;
    let mut total_token_edits_all = 0;
    let mut total_tokens_all = 0;
    let mut all_token_errors: HashMap<String, TokenError> = HashMap::new();
    let mut all_token_errors_both: HashMap<String, TokenError> = HashMap::new();
    let mut per_dataset: Vec<DatasetMetrics> = Vec::new();

    // Evaluate each dataset using default config for detailed reports
    for fname in GOLD_STANDARDS {
        let report = evaluate_csv(format!("{}/{}", GOLD_DIR, fname).as_str(), &eval_config);
        let report_file = write_dataset_report(fname, &report, &timestamp, &eval_config);

        per_dataset.push(DatasetMetrics {
            name: fname.replace(".csv", ""),
            ter: report.token_error_rate,
            report_file,
        });

        words_all += report.total;
        total_token_edits_all += report.total_token_edits;
        total_tokens_all += report.total_tokens;

        merge_token_errors(&mut all_token_errors, report.token_errors, 5);
    }

    // Collect token errors under the 3-vowel system (e==i==y and o==u)
    let config_both = EvalConfig {
        equate_e_i: true,
        equate_o_u: true,
    };
    for fname in GOLD_STANDARDS {
        let report_both = evaluate_csv(format!("{}/{}", GOLD_DIR, fname).as_str(), &config_both);
        merge_token_errors(&mut all_token_errors_both, report_both.token_errors, 5);
    }

    let worst = per_dataset
        .iter()
        .max_by(|a, b| a.ter.partial_cmp(&b.ter).unwrap())
        .unwrap();

    let metrics = OverallMetrics {
        cer_default,
        cer_e_i_y,
        cer_o_u,
        cer_both,
        words: words_all,
        tokens: total_tokens_all,
        edits: total_token_edits_all,
        ter: (total_token_edits_all as f64 / total_tokens_all as f64) * 100.0,
        worst_performer: worst.name.clone(),
        worst_ter: worst.ter,
        per_dataset,
        token_errors: all_token_errors,
        token_errors_both: all_token_errors_both,
    };

    let overall_file = write_overall_report(&metrics, &timestamp);

    print_overall_metrics(&metrics);
    print_report_locations(&metrics, &overall_file);
}
