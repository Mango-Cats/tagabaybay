import sys
import os
import re
import csv

def extract_mappings(input_path, output_path="output.csv"):
    if not os.path.exists(input_path):
        print(f"Error: The file '{input_path}' was not found.")
        sys.exit(1)

    with open(input_path, 'r', encoding='utf-8') as file:
        lines = file.readlines()

    extracted_data = []
    # Pattern to capture lines matching: * INPUT -> ACTUAL
    pattern = re.compile(r'^\*\s+(.+?)\s+->\s+(.+)$')

    for line in lines:
        clean_line = line.strip()
        if not clean_line:
            continue

        match = pattern.match(clean_line)
        if match:
            input_word = match.group(1).strip()
            actual_word = match.group(2).strip()
            extracted_data.append([input_word, actual_word])

    with open(output_path, 'w', newline='', encoding='utf-8') as file:
        writer = csv.writer(file)
        writer.writerow(['INPUT', 'ACTUAL'])
        writer.writerows(extracted_data)

    print(f"Success! Extracted mappings have been saved to '{output_path}'.")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python extractor.py <input_file.txt>")
        sys.exit(1)

    input_file = sys.argv[1]
    extract_mappings(input_file)