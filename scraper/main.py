#!/usr/bin/env python3
"""
Scrape IPA transcriptions from Wiktionary and format output.
Reads words from .data/loan_words.txt and outputs formatted data with:
  english_word, tagalog_word
  english_ipa1 | english_ipa2 | ...
  tagalog_ipa1 | tagalog_ipa2 | ...
"""

import requests
from bs4 import BeautifulSoup
import re
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
from threading import Lock
from tqdm import tqdm


def get_tagalog_data(word: str) -> tuple[list[str], str | None]:
    """
    Fetch both IPA transcriptions and English etymology from a Tagalog word's Wiktionary page.
    Combines two requests into one since both are from the same page.

    Args:
        word: The Tagalog word to look up

    Returns:
        Tuple of (ipa_list, english_etymology)
    """
    url = f"https://en.wiktionary.org/wiki/{word}#Tagalog"
    try:
        headers = {
            "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
        }
        response = requests.get(url, headers=headers, timeout=10)
        response.raise_for_status()

        soup = BeautifulSoup(response.text, "html.parser")

        tagalog_heading = soup.find("h2", id="Tagalog")
        if not tagalog_heading:
            return [], None

        ipa_list = []
        english_etymology = None
        current = tagalog_heading.parent.find_next_sibling()

        while current:
            # Stop at next language section
            if current.name == "div" and current.find("h2"):
                next_h2 = current.find("h2")
                if next_h2 and next_h2.get("id") != "Tagalog":
                    break

            # Extract IPAs
            ipa_spans = current.find_all("span", class_="IPA")
            for span in ipa_spans:
                ipa_text = span.get_text().strip()
                ipa_text = re.sub(r"^[\[\]/]+|[\[\]/]+$", "", ipa_text)
                if ipa_text and ipa_text not in ipa_list:
                    ipa_list.append(ipa_text)

            # Extract English etymology
            if current.name == "div" and current.find("h3"):
                h3 = current.find("h3")
                if h3 and "Etymology" in h3.get_text() and not english_etymology:
                    etym_content = current.find_next_sibling("p")
                    if etym_content:
                        english_span = etym_content.find("span", class_="etyl")
                        if english_span:
                            english_link = english_span.find("a")
                            if english_link and "English" in english_link.get_text():
                                borrowed_word = etym_content.find("i", {"lang": "en"})
                                if borrowed_word:
                                    word_link = borrowed_word.find("a")
                                    if word_link:
                                        english_etymology = word_link.get_text().lower()

            current = current.find_next_sibling()

        return ipa_list, english_etymology

    except requests.exceptions.RequestException:
        return [], None


def get_english_ipa_transcriptions(word: str) -> list[str]:
    """
    Fetch IPA transcriptions for an English word from Wiktionary's English section.

    Args:
        word: The English word to look up

    Returns:
        List of IPA transcriptions found
    """
    url = f"https://en.wiktionary.org/wiki/{word}#English"
    try:
        headers = {
            "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
        }
        response = requests.get(url, headers=headers, timeout=10)
        response.raise_for_status()

        soup = BeautifulSoup(response.text, "html.parser")

        english_heading = soup.find("h2", id="English")
        if not english_heading:
            return []

        ipa_list = []
        current = english_heading.parent.find_next_sibling()

        while current:
            if current.name == "div" and current.find("h2"):
                next_h2 = current.find("h2")
                if next_h2 and next_h2.get("id") != "English":
                    break

            ipa_spans = current.find_all("span", class_="IPA")
            for span in ipa_spans:
                ipa_text = span.get_text().strip()
                ipa_text = re.sub(r"^[\[\]/]+|[\[\]/]+$", "", ipa_text)
                if ipa_text and ipa_text not in ipa_list:
                    ipa_list.append(ipa_text)

            current = current.find_next_sibling()

        return ipa_list

    except requests.exceptions.RequestException:
        return []


def scrape_and_format(
    tagalog_words: list[str], output_file: Path, num_workers: int = 100
):
    """
    Scrape all data and output in formatted structure.

    Format:
        english_word, tagalog_word
        english_ipa1 | english_ipa2 | ...
        tagalog_ipa1 | tagalog_ipa2 | ...

    Args:
        tagalog_words: List of Tagalog loan words to process
        output_file: Path to write formatted output
        num_workers: Number of parallel workers
    """
    print(f"\n{'='*60}")
    print("Scraping and formatting data...")
    print(f"Processing {len(tagalog_words)} Tagalog loan words\n")

    results = {}
    results_lock = Lock()

    def process_word(tagalog_word):
        # Get Tagalog data (IPAs + etymology) in a single request
        tagalog_ipas, english_word = get_tagalog_data(tagalog_word.lower())

        if english_word:
            english_ipas = get_english_ipa_transcriptions(english_word)

            with results_lock:
                results[tagalog_word] = {
                    "english_word": english_word,
                    "english_ipas": english_ipas,
                    "tagalog_ipas": tagalog_ipas,
                }

    # Process all words in parallel
    with ThreadPoolExecutor(max_workers=num_workers) as executor:
        futures = {executor.submit(process_word, word): word for word in tagalog_words}

        # Progress bar
        with tqdm(total=len(tagalog_words), desc="Scraping", unit="word") as pbar:
            for future in as_completed(futures):
                word = futures[future]
                try:
                    future.result()
                except Exception as e:
                    print(f"\nError processing '{word}': {e}")
                pbar.update(1)

    # Write formatted output
    print(f"\n{'='*60}")
    print(f"Writing formatted output to: {output_file}")

    with open(output_file, "w", encoding="utf-8") as f:
        for tagalog_word in tagalog_words:
            if tagalog_word in results:
                data = results[tagalog_word]

                # Line 1: english_word, tagalog_word
                f.write(f"{data['english_word']}, {tagalog_word}\n")

                # Line 2: English IPAs separated by |
                if data["english_ipas"]:
                    f.write(" | ".join(data["english_ipas"]) + "\n")

                # Line 3: Tagalog IPAs separated by |
                if data["tagalog_ipas"]:
                    f.write(" | ".join(data["tagalog_ipas"]) + "\n")

                f.write("\n")  # Blank line between entries

    print(f"Done! Processed {len(results)} word pairs")


def main():
    """Main function."""
    # Setup paths
    script_dir = Path(__file__).parent / ".data"
    input_file = script_dir / "loan_words.txt"
    output_file = script_dir / "scraped_ipas.txt"

    # Create output directory if it doesn't exist
    output_file.parent.mkdir(parents=True, exist_ok=True)

    # Read words from data
    print(f"Reading Tagalog loan words from: {input_file}")
    with open(input_file, "r", encoding="utf-8") as f:
        tagalog_words = [line.strip().lower() for line in f if line.strip()]

    # Scrape and format everything
    scrape_and_format(tagalog_words, output_file, num_workers=100)

    print(f"\n{'='*60}")
    print("All operations completed successfully!")


if __name__ == "__main__":
    main()
