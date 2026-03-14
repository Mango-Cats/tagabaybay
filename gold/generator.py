import sys
import os

def clean_file(input_path, output_path="o.txt"):
    if not os.path.exists(input_path):
        print(f"Error: The file '{input_path}' was not found.")
        sys.exit(1)

    with open(input_path, 'r', encoding='utf-8') as file:
        text = file.read()

    raw_words = text.split()
    valid_words = set()

    for word in raw_words:
        if not word:
            continue

        letter_count = sum(char.isalpha() for char in word)
        density = letter_count / len(word)
        if density < 0.6:
            continue

        if not word.isalpha() or not word.isascii():
            continue

        if len(word) in {1,2}:
            continue

        valid_words.add(word.lower())

    sorted_unique_words = sorted(list(valid_words), key=str.lower)

    with open(output_path, 'w', encoding='utf-8') as file:
        for word in sorted_unique_words:
            file.write(word + '\n')

    print(f"Success! Cleaned and sorted words have been saved to '{output_path}'.")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python generator.py <input_file.txt>")
        sys.exit(1)

    input_file = sys.argv[1]
    clean_file(input_file)