import sys
import os
import csv

def compare_csvs(file1_path, file2_path, output_path="mismatches.csv"):
    if not os.path.exists(file1_path):
        print(f"Error: The file '{file1_path}' was not found.")
        sys.exit(1)
    if not os.path.exists(file2_path):
        print(f"Error: The file '{file2_path}' was not found.")
        sys.exit(1)

    def load_csv_data(filepath):
        data = {}
        with open(filepath, 'r', encoding='utf-8') as file:
            reader = csv.DictReader(file)
            for row in reader:
                if 'INPUT' in row and 'ACTUAL' in row:
                    data[row['INPUT']] = row['ACTUAL']
        return data

    data1 = load_csv_data(file1_path)
    data2 = load_csv_data(file2_path)

    mismatches = []
    
    all_inputs = set(data1.keys()).union(set(data2.keys()))

    for input_word in all_inputs:
        actual1 = data1.get(input_word, "MISSING")
        actual2 = data2.get(input_word, "MISSING")

        if actual1 != actual2:
            mismatches.append([input_word, actual1, actual2])

    with open(output_path, 'w', newline='', encoding='utf-8') as file:
        writer = csv.writer(file)
        
        header1 = f'ACTUAL_{os.path.basename(file1_path)}'
        header2 = f'ACTUAL_{os.path.basename(file2_path)}'
        
        writer.writerow(['INPUT', header1, header2])
        writer.writerows(mismatches)

    print(f"Success! Found {len(mismatches)} mismatches. Saved to '{output_path}'.")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python comparer.py <file1.csv> <file2.csv>")
        sys.exit(1)

    file1 = sys.argv[1]
    file2 = sys.argv[2]
    compare_csvs(file1, file2)