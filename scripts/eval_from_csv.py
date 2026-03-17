import csv
import argparse
from pathlib import Path
from collections import defaultdict
from datetime import datetime

E_I_Y = {"e", "i", "y"}
O_U = {"o", "u"}

def normalize(text: str, equate_eiy: bool = False, equate_ou: bool = False) -> str:
    text = text.lower().strip()
    if equate_eiy:
        text = "".join("e" if c in E_I_Y else c for c in text)
    if equate_ou:
        text = "".join("o" if c in O_U else c for c in text)
    return text

def align_and_highlight(actual: str, expected: str):
    """
    Computes Levenshtein distance with backtracing.
    Returns:
      - hl_actual: actual string with errors UPPERCASED
      - hl_expected: expected string with errors UPPERCASED
      - errors: list of error operations (e.g., 'a->b', '#->b', 'a->#')
      - edit_distance: integer
    """
    actual = actual.lower()
    expected = expected.lower()
    n, m = len(actual), len(expected)
    
    dp = [[0] * (m + 1) for _ in range(n + 1)]
    for i in range(1, n + 1): dp[i][0] = i
    for j in range(1, m + 1): dp[0][j] = j
    
    for i in range(1, n + 1):
        for j in range(1, m + 1):
            if actual[i-1] == expected[j-1]:
                dp[i][j] = dp[i-1][j-1]
            else:
                dp[i][j] = 1 + min(dp[i-1][j-1], dp[i-1][j], dp[i][j-1])
                
    i, j = n, m
    ops = []
    while i > 0 or j > 0:
        if i > 0 and j > 0 and actual[i-1] == expected[j-1]:
            ops.append(('match', actual[i-1], expected[j-1]))
            i -= 1; j -= 1
        elif i > 0 and j > 0 and dp[i][j] == dp[i-1][j-1] + 1:
            ops.append(('sub', actual[i-1], expected[j-1]))
            i -= 1; j -= 1
        elif i > 0 and dp[i][j] == dp[i-1][j] + 1:
            ops.append(('del', actual[i-1], '#'))
            i -= 1
        else:
            ops.append(('ins', '#', expected[j-1]))
            j -= 1
            
    ops.reverse()
    
    hl_actual = ""
    hl_expected = ""
    errors = []
    
    for op, a_char, e_char in ops:
        if op == 'match':
            hl_actual += a_char
            hl_expected += e_char
        elif op == 'sub':
            hl_actual += a_char.upper()
            hl_expected += e_char.upper()
            errors.append(f"{a_char}->{e_char}")
        elif op == 'del':
            hl_actual += a_char.upper()
            errors.append(f"{a_char}->#")
        elif op == 'ins':
            hl_expected += e_char.upper()
            errors.append(f"#->{e_char}")
            
    return hl_actual, hl_expected, errors, dp[n][m]

def format_error_ranking(error_map, limit=None):
    lines = [
        "  # = absent character",
        "  x->y = substitution  |  #->y = missing (insertion)  |  x-># = extra (deletion)",
        "  (actual->expected : count : examples)",
        "  " + "-" * 60
    ]
    sorted_errors = sorted(error_map.items(), key=lambda item: (-len(item[1]), item[0]))
    if limit:
        sorted_errors = sorted_errors[:limit]
        
    for idx, (err_op, words) in enumerate(sorted_errors):
        count = len(words)
        # Deduplicate examples but preserve order
        unique_examples = []
        for w in words:
            if w not in unique_examples:
                unique_examples.append(w)
        example_str = ", ".join(unique_examples[:3])
        lines.append(f"  {idx+1:>2}. {err_op:<10}: {count:>4} : {example_str}")
    return "\n".join(lines)

def evaluate_file(csv_path: Path, reports_dir: Path):
    rows = []
    with open(csv_path, "r", encoding="utf-8") as f:
        reader = csv.reader(f)
        header = next(reader)
        for row in reader:
            if len(row) >= 3:
                rows.append((row[0].strip(), row[1].strip(), row[2].strip()))

    results = []
    global_errors_5v = defaultdict(list)
    global_errors_3v = defaultdict(list)
    
    total_tokens = sum(max(1, len(normalize(exp))) for _, exp, _ in rows)
    total_edits_5v = 0
    failures = []

    for eng, expected, actual in rows:
        
        hl_act, hl_exp, errs_5v, edits_5v = align_and_highlight(actual, expected)
        total_edits_5v += edits_5v
        for err in errs_5v:
            global_errors_5v[err].append(eng)
            
        if edits_5v > 0:
            failures.append((eng, hl_act, hl_exp))
            
        act_3v = normalize(actual, equate_eiy=True, equate_ou=True)
        exp_3v = normalize(expected, equate_eiy=True, equate_ou=True)
        _, _, errs_3v, _ = align_and_highlight(act_3v, exp_3v)
        for err in errs_3v:
            global_errors_3v[err].append(eng)

        results.append({
            "input": eng,
            "actual": actual,
            "expected": expected
        })

    metrics = {}
    for label, eiy, ou in [
        ("5-vowel", False, False),
        ("e=i=y", True, False),
        ("o=u", False, True),
        ("3-vowel", True, True),
    ]:
        edits = sum(align_and_highlight(normalize(r["actual"], eiy, ou), normalize(r["expected"], eiy, ou))[3] for r in results)
        tokens = sum(max(1, len(normalize(r["expected"], eiy, ou))) for r in results)
        metrics[label] = (edits / tokens * 100) if tokens else 0

    dataset_name = csv_path.stem
    accept = metrics["5-vowel"] < 20
    
    out = []
    out.append(f"{csv_path.name}")
    out.append(f"├── Words           {len(rows)}")
    out.append(f"├── Tokens          {total_tokens}")
    out.append(f"├── Edits           {total_edits_5v}")
    out.append(f"├── CER (5-vowel)   {metrics['5-vowel']:.2f}%")
    out.append(f"├── CER (e=i=y)     {metrics['e=i=y']:.2f}%")
    out.append(f"├── CER (o=u)       {metrics['o=u']:.2f}%")
    out.append(f"├── CER (3-vowel)   {metrics['3-vowel']:.2f}%")
    out.append(f"└── Accept@CER<20   {str(accept).lower()}\n")

    if failures:
        out.append("Failures:")
        out.append(f"   {'#':>3s}  {'INPUT':14s}  {'ACTUAL':14s}  {'EXPECTED':14s}")
        out.append(f"  ----  {'-'*13}  {'-'*14}  {'-'*14}")
        for i, (eng, act, exp) in enumerate(failures[:200]): # Limit to 200 display failures
            out.append(f"  {i+1:3d}. {eng:14s}  {act:14s}  {exp:14s}")
            
    out.append("\nCharacter Error Ranking")
    out.append(format_error_ranking(global_errors_5v))
    out.append("\n")
    
    report_str = "\n".join(out)
    print(report_str)
    
    timestamp = datetime.now().strftime("%m%d%y_%H%M")
    report_file = reports_dir / f"{timestamp}_{dataset_name}.txt"
    with open(report_file, "w", encoding="utf-8") as f:
        f.write(report_str)
        
    return {
        "name": dataset_name,
        "words": len(rows),
        "tokens": total_tokens,
        "edits_5v": total_edits_5v,
        "metrics": metrics,
        "global_errors_5v": global_errors_5v,
        "global_errors_3v": global_errors_3v,
        "report_path": str(report_file)
    }

def main():
    parser = argparse.ArgumentParser(description="Evaluate predicted actuals against expected targets in CSVs.")
    parser.add_argument("csv_files", nargs='+', type=Path, help="Paths to one or more CSV files")
    args = parser.parse_args()
    
    reports_dir = Path(".tests/report")
    reports_dir.mkdir(parents=True, exist_ok=True)

    dataset_summaries = []
    
    total_words = 0
    total_tokens = 0
    total_edits_5v = 0
    overall_errors_5v = defaultdict(list)
    overall_errors_3v = defaultdict(list)

    for csv_path in args.csv_files:
        if not csv_path.exists():
            print(f"Error: {csv_path} not found.")
            continue
            
        summary = evaluate_file(csv_path, reports_dir)
        dataset_summaries.append(summary)
        
        total_words += summary["words"]
        total_tokens += summary["tokens"]
        total_edits_5v += summary["edits_5v"]
        
        for k, v in summary["global_errors_5v"].items():
            overall_errors_5v[k].extend(v)
        for k, v in summary["global_errors_3v"].items():
            overall_errors_3v[k].extend(v)

    if not dataset_summaries:
        return

    overall_metrics = {"5-vowel": 0, "e=i=y": 0, "o=u": 0, "3-vowel": 0}
    for metric in overall_metrics.keys():
        total_metric_edits = sum(s["metrics"][metric] * s["tokens"] / 100 for s in dataset_summaries)
        overall_metrics[metric] = (total_metric_edits / total_tokens * 100) if total_tokens else 0

    worst_dataset = max(dataset_summaries, key=lambda x: x["metrics"]["5-vowel"])
    overall_accept = overall_metrics["5-vowel"] < 20

    timestamp_full = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    timestamp_file = datetime.now().strftime("%m%d%y_%H%M")
    
    overall_out = []
    overall_out.append("\noverall\n")
    overall_out.append("OVERALL EVALUATION REPORT")
    overall_out.append(f"Generated: {timestamp_full}")
    overall_out.append("=" * 50)
    
    overall_out.append("\nPerformance (CER)")
    overall_out.append("-" * 50)
    overall_out.append(f"               {'5-vowel':>10s}  {'e=i=y':>10s}  {'o=u':>10s}  {'3-vowel':>10s}")
    overall_out.append(f" CER           {overall_metrics['5-vowel']:>9.2f}%  {overall_metrics['e=i=y']:>9.2f}%  {overall_metrics['o=u']:>9.2f}%  {overall_metrics['3-vowel']:>9.2f}%")

    overall_out.append("\nSummary")
    overall_out.append("-" * 50)
    overall_out.append(f"├── Words           {total_words}")
    overall_out.append(f"├── Tokens          {total_tokens}")
    overall_out.append(f"├── Edits           {total_edits_5v}")
    overall_out.append(f"├── CER             {overall_metrics['5-vowel']:.2f}%")
    overall_out.append(f"├── Accept@CER<20   {str(overall_accept).lower()}")
    overall_out.append(f"└── Worst Performer {worst_dataset['name']} (CER {worst_dataset['metrics']['5-vowel']:.2f}%)")

    overall_out.append("\nPer-Dataset Metrics")
    overall_out.append("-" * 50)
    overall_out.append(f"  {'Dataset':<18s} {'5-vowel':>10s}  {'e=i=y':>10s}  {'o=u':>10s}  {'3-vowel':>10s}")
    overall_out.append("  " + "-" * 60)
    for ds in dataset_summaries:
        m = ds["metrics"]
        overall_out.append(f"  {ds['name']:<18s} {m['5-vowel']:>9.2f}%  {m['e=i=y']:>9.2f}%  {m['o=u']:>9.2f}%  {m['3-vowel']:>9.2f}%")

    overall_out.append("\nCharacter Error Ranking (5-vowel)")
    overall_out.append(format_error_ranking(overall_errors_5v, limit=112))

    overall_out.append("\nCharacter Error Ranking (3-vowel, e=i=y and o=u)")
    overall_out.append(format_error_ranking(overall_errors_3v, limit=73))

    overall_out.append("\nIndividual Reports")
    overall_out.append("-" * 50)
    for ds in dataset_summaries:
        overall_out.append(f"  - {ds['name']} (CER {ds['metrics']['5-vowel']:.2f}%): {ds['report_path']}")

    overall_report_str = "\n".join(overall_out)
    print(overall_report_str)
    
    overall_report_file = reports_dir / f"{timestamp_file}_overall.txt"
    with open(overall_report_file, "w", encoding="utf-8") as f:
        f.write(overall_report_str)
        
    print(f"\n[+] Overall report successfully saved to: {overall_report_file}")

if __name__ == "__main__":
    main()