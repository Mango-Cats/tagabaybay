# /// script
# requires-python = ">=3.11"
# dependencies = ["anthropic"]
# ///

"""
LLM Prompting for orthographic nativization.

Sends each English word to Claude and asks it to produce the Filipino
nativized spelling. Computes CER against the gold standard using the
same Levenshtein metric as the main evaluation harness.

Usage:
    export ANTHROPIC_API_KEY=""
    uv run BaybayinMoClaude.py

Outputs:
    - Console summary with per-dataset and overall CER
    - results/ directory with per-dataset CSV files (input, expected, actual)
    - results/overall_report.txt with full summary
"""

import csv
import os
import time
from pathlib import Path

import anthropic

GOLD_DIR = Path("gold/data")
GOLD_FILES = [
    "common_drugs.csv",
    "common_eng.csv",
    "ph_fda_human.csv",
    "ching_chua.csv",
    "wiki.csv",
]

MODEL = "claude-haiku-4-5-20251001"

RESULTS_DIR = Path(".tests/llm_baseline")

DELAY = 0.2

E_I_Y = {"e", "i", "y"}
O_U = {"o", "u"}

SYSTEM_PROMPT = """You are an expert in Filipino orthography. Your task is to
convert English loanwords into their standard Filipino nativized spelling,
following the conventions of the Komisyon sa Wikang Filipino.

Rules to follow:
- Use the Filipino five-vowel system (a, e, i, o, u).
- Map English sounds to their Filipino letter equivalents.
- Simplify doubled consonants (e.g., "addict" becomes "adik").
- Drop silent letters (e.g., word-final silent "e" in "make" becomes "meyk").
- Diphthongs: /eɪ/ becomes "ey", /oʊ/ becomes "ow", /aɪ/ becomes "ay", /aʊ/ becomes "aw".
- The digraph "ph" maps to "f", "th" maps to "t", "sh" maps to "s" or "sy".
- Insert epenthetic vowels to repair illegal consonant clusters where needed.

IMPORTANT:
- Preserve all numbers exactly as they appear (e.g., "50", "125", "5000").
- Preserve all symbols exactly as they appear (e.g., "+", "-", "/").
- Preserve hyphens and the structure around them (e.g., "Epribenz-50" becomes "Epribens-50").
- For multi-word inputs, nativize each word separately and keep the spacing.
- For abbreviations like "IV", "XR", "DS", "OB", spell each letter in Filipino

Respond with ONLY the nativized Filipino spelling. No explanation, no
extra punctuation, no quotes. Just the nativized form."""


def make_user_prompt(word: str) -> str:
    return f"{word}"


def levenshtein(s: str, t: str) -> int:
    n, m = len(s), len(t)
    if n == 0:
        return m
    if m == 0:
        return n

    dp = list(range(m + 1))
    for i in range(1, n + 1):
        prev = dp[0]
        dp[0] = i
        for j in range(1, m + 1):
            temp = dp[j]
            if s[i - 1] == t[j - 1]:
                dp[j] = prev
            else:
                dp[j] = 1 + min(prev, dp[j], dp[j - 1])
            prev = temp
    return dp[m]


def normalize(text: str, equate_eiy: bool = False, equate_ou: bool = False) -> str:
    text = text.lower().strip()
    if equate_eiy:
        text = "".join("e" if c in E_I_Y else c for c in text)
    if equate_ou:
        text = "".join("o" if c in O_U else c for c in text)
    return text


def clean_response(response: str) -> str:
    """Clean up common LLM formatting artifacts."""
    result = response.strip()
    # Remove surrounding quotes
    result = result.strip("\"'`")
    # Remove trailing periods or colons
    result = result.rstrip(".,;:!?")
    # If the model returned multiple lines, take the first
    if "\n" in result:
        result = result.split("\n")[0].strip()
    # If the model prefixed with something like "Answer: ", strip it
    prefixes = ["answer:", "nativized:", "filipino:", "result:"]
    lower = result.lower()
    for prefix in prefixes:
        if lower.startswith(prefix):
            result = result[len(prefix):].strip()
            break
    return result


def nativize_with_llm(client: anthropic.Anthropic, word: str) -> str:
    """Send a single word to Claude and return the nativized form."""
    try:
        response = client.messages.create(
            model=MODEL,
            max_tokens=150,
            system=SYSTEM_PROMPT,
            messages=[{"role": "user", "content": make_user_prompt(word)}],
        )
        raw = response.content[0].text
        return clean_response(raw)
    except Exception as e:
        print(f"  ERROR on '{word}': {e}")
        return ""


def evaluate_dataset(
    client: anthropic.Anthropic,
    csv_path: Path,
) -> dict:
    """Evaluate one gold standard CSV. Returns metrics and per-word results."""

    rows = []
    with open(csv_path, "r", encoding="utf-8") as f:
        reader = csv.reader(f)
        next(reader)  # skip header
        for row in reader:
            if len(row) >= 2:
                rows.append((row[0].strip(), row[1].strip()))

    results = []
    for i, (eng, expected) in enumerate(rows):
        actual = nativize_with_llm(client, eng)
        results.append({"input": eng, "expected": expected, "actual": actual})

        act_norm = normalize(actual)
        exp_norm = normalize(expected)
        status = "OK" if act_norm == exp_norm else "MISS"
        print(f"  [{i+1}/{len(rows)}] {status:4s}  {eng:25s}  {actual:25s}  {expected}")

        time.sleep(DELAY)

    metrics = {}
    for label, eiy, ou in [
        ("5-vowel", False, False),
        ("e=i=y", True, False),
        ("o=u", False, True),
        ("3-vowel", True, True),
    ]:
        total_edits = 0
        total_chars = 0
        for r in results:
            act = normalize(r["actual"], eiy, ou)
            exp = normalize(r["expected"], eiy, ou)
            total_edits += levenshtein(act, exp)
            total_chars += len(exp) if len(exp) > 0 else 1
        cer = (total_edits / total_chars) * 100 if total_chars > 0 else 0
        metrics[label] = cer

    return {
        "words": len(results),
        "metrics": metrics,
        "results": results,
    }


def main():
    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("ERROR: Set ANTHROPIC_API_KEY environment variable.")
        print("  export ANTHROPIC_API_KEY='sk-ant-...'")
        return

    client = anthropic.Anthropic(api_key=api_key)
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)

    all_results = []
    dataset_metrics = []

    print(f"Model: {MODEL}")
    print(f"Gold standard: {GOLD_DIR}")
    print(f"Datasets: {len(GOLD_FILES)}")
    print("=" * 70)

    for fname in GOLD_FILES:
        csv_path = GOLD_DIR / fname
        if not csv_path.exists():
            print(f"SKIP: {csv_path} not found")
            continue

        print(f"\n{fname}")
        print("-" * 50)

        data = evaluate_dataset(client, csv_path)

        dataset_metrics.append({"name": fname, **data})
        all_results.extend(data["results"])

        # Write per-dataset results CSV
        out_csv = RESULTS_DIR / fname
        with open(out_csv, "w", encoding="utf-8", newline="") as f:
            writer = csv.writer(f)
            writer.writerow(["input", "expected", "actual"])
            for r in data["results"]:
                writer.writerow([r["input"], r["expected"], r["actual"]])

        m = data["metrics"]
        print(f"  5-vowel: {m['5-vowel']:.2f}%  e=i=y: {m['e=i=y']:.2f}%  "
              f"o=u: {m['o=u']:.2f}%  3-vowel: {m['3-vowel']:.2f}%")

    # Overall CER
    print("\n" + "=" * 70)
    print("OVERALL")
    print("=" * 70)

    overall = {}
    for label, eiy, ou in [
        ("5-vowel", False, False),
        ("e=i=y", True, False),
        ("o=u", False, True),
        ("3-vowel", True, True),
    ]:
        total_edits = 0
        total_chars = 0
        for r in all_results:
            act = normalize(r["actual"], eiy, ou)
            exp = normalize(r["expected"], eiy, ou)
            total_edits += levenshtein(act, exp)
            total_chars += len(exp) if len(exp) > 0 else 1
        cer = (total_edits / total_chars) * 100 if total_chars > 0 else 0
        overall[label] = cer

    total_words = sum(d["words"] for d in dataset_metrics)
    total_edits_5v = sum(
        levenshtein(normalize(r["actual"]), normalize(r["expected"]))
        for r in all_results
    )
    total_chars_5v = sum(
        len(normalize(r["expected"])) for r in all_results
    )

    print(f"Words: {total_words}")
    print(f"Edits (5-vowel): {total_edits_5v}")
    print(f"Tokens: {total_chars_5v}")
    for label, cer in overall.items():
        print(f"  CER ({label}): {cer:.2f}%")

    # Write report
    report_path = RESULTS_DIR / "overall_report.txt"
    with open(report_path, "w", encoding="utf-8") as f:
        f.write("LLM BASELINE EVALUATION REPORT\n")
        f.write(f"Model: {MODEL}\n")
        f.write(f"{'=' * 50}\n\n")

        f.write("Overall CER\n")
        f.write(f"{'-' * 50}\n")
        f.write(f"  {'':10s} {'5-vowel':>10s} {'e=i=y':>10s} {'o=u':>10s} {'3-vowel':>10s}\n")
        f.write(f"  {'CER':10s} {overall['5-vowel']:>9.2f}% {overall['e=i=y']:>9.2f}% "
                f"{overall['o=u']:>9.2f}% {overall['3-vowel']:>9.2f}%\n\n")

        f.write(f"Words: {total_words}\n")
        f.write(f"Edits (5-vowel): {total_edits_5v}\n")
        f.write(f"Tokens: {total_chars_5v}\n\n")

        f.write("Per-Dataset CER\n")
        f.write(f"{'-' * 50}\n")
        f.write(f"  {'Dataset':20s} {'5-vowel':>10s} {'e=i=y':>10s} {'o=u':>10s} {'3-vowel':>10s}\n")
        for d in dataset_metrics:
            m = d["metrics"]
            f.write(f"  {d['name']:20s} {m['5-vowel']:>9.2f}% {m['e=i=y']:>9.2f}% "
                    f"{m['o=u']:>9.2f}% {m['3-vowel']:>9.2f}%\n")

        f.write(f"\nFailures (5-vowel, first 100)\n")
        f.write(f"{'-' * 70}\n")
        f.write(f"  {'#':>4s}  {'INPUT':25s}  {'ACTUAL':25s}  EXPECTED\n")
        f.write(f"  {'':->4s}  {'':->25s}  {'':->25s}  {'':->25s}\n")
        failures = [
            r for r in all_results
            if normalize(r["actual"]) != normalize(r["expected"])
        ]
        for i, r in enumerate(failures[:100]):
            f.write(f"  {i+1:4d}. {r['input']:25s}  {r['actual']:25s}  {r['expected']}\n")

    print(f"\nResults written to {RESULTS_DIR}/")
    print(f"Report: {report_path}")


if __name__ == "__main__":
    main()