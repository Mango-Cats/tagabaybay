"""
This is a script for validating the accuracy of the gold standards.

# How to

Simply run the script.
"""

import csv
from datetime import date, datetime
from io import TextIOWrapper
from os import listdir, makedirs, mkdir
from os.path import dirname, join
from random import sample
from typing import List, Optional, Tuple

""" this is a percentage that dicatates the maximum size of the validation dataset
    max = max_validation_sample * length_of_dataset

    that is, 1.0 is the entire dataset can be the validation dataset (do not do this!)
    and 0.0 is none

    by default, this should be 0.6
"""
max_validation_sample = 0.6

""" this is the directory where the script is located """""
script_dir: str = dirname(__file__)

""" this is the directory of the gold data """
gold_data_dir: str = join(script_dir, "data/")

""" this is the directory of the evaluation results """
eval_dir: str = join(script_dir, "eval/")

""" this is a list of the gold data in `gold_data_dir` """
gold_files: list[str] = listdir(gold_data_dir)

""" this will contain the selected file """
selected_file = ()


def choose_file():
    """
    Makes the user pick which gold file to evaluate.

    # Return
    The opened file.

    # Note
    Always ensure that the returned file is closed.
    """
    print("\n# Files available:")
    for i, f in enumerate(gold_files, start=1):
        print(f"{i}. {f}")

    while True:
        choice = input("\nChoose a file (1–{}): ".format(len(gold_files))).strip()

        if choice.isdigit():
            idx = int(choice) - 1
            if 0 <= idx < len(gold_files):
                selected_file = gold_files[idx]
                return selected_file, open(gold_data_dir + selected_file, "r")

        print(
            "\nInvalid choice. Enter a number between 1 and {}.".format(len(gold_files))
        )


def open_file(f: TextIOWrapper) -> dict[str, str]:
    """
    Open the file and returns a dict[string, string] with the
    left being the input and the right being the expected output

    # Params
    * f: the selected file.

    # Return
    A dictionary representation of the CSV.
    """
    reader = csv.reader(f)
    gold_map = dict()
    for row in reader:
        assert len(row) == 2
        gold_map[row[0]] = row[1]

    return gold_map


def get_validation_size(length: int, max_validation_sample: Optional[float]) -> int:
    """
    Gets the desired validation size (number of entries in the selected file).
    The validation size cannot be equal to the total number of entries in the
    gold standard. Furthermore, we may also introduce a maximum limit which is
    a percentage of the sample_size (see `max_validation_sample`).

    # Params
    * length: the size of the selected gold standard (len of dataset)
    * max_validation_sample: the maximum fraction of entries allowed for validation

    # Return
    A valid validation size as an integer
    """
    sample_size = -1

    # compute maximum allowed sample size if max_validation_sample is set
    max_val = (
        int(length * max_validation_sample)
        if max_validation_sample is not None
        else length - 1
    )

    def not_negative():
        return sample_size < 0

    def not_greater():
        return sample_size > length

    def not_equal():
        if max_validation_sample is None:
            # cannot pick full dataset
            return sample_size == length
        else:
            # cannot exceed max fraction
            return sample_size > max_val

    while not_negative() or not_greater() or not_equal():
        try:
            sample_size = int(
                input(f"\nEnter a number of entries to validate (max {max_val}): ")
            )
        except ValueError:
            print("Please enter a valid integer.")
            continue

    return sample_size


def random_sample(gold_map: dict[str, str], validation_size: int) -> dict[str, str]:
    """
    Gets a validation set from a random sample of the selected gold standard.

    # Params
    * gold_map: the selected gold standard
    * validation_size: the selected validation size

    # Return
    A random sample from the gold_map with size validation_size
    """
    random_sample = sample(list(gold_map.keys()), validation_size)
    validation_set = dict()

    for input in random_sample:
        validation_set[input] = gold_map[input]

    return validation_set


def validate(validation_set: dict[str, str], validation_size: int):
    """
    The validation function itself
    """
    correct = 0

    # an array of [question, expected, response]
    mislabels: list[Tuple[str, str, str]] = list()

    for i, (question, expected) in enumerate(validation_set.items(), start=1):
        print(f"\n{i}. {question}")
        response = input("$ ")
        if response.lower() == expected.lower():
            correct += 1
        else:
            mislabels.append((question, expected, response))

    # calculate metrics
    n_errors = validation_size - correct
    accuracy = correct / validation_size

    return mislabels, n_errors, accuracy


def print_result(
    mislabels: List[Tuple[str, str, str]],
    n_errors: int,
    accuracy: float,
    gold_filename: str,
    valset_size: int,
    validator: str,
) -> None:
    # Construct output filename
    now = datetime.now()
    timestamp = now.strftime("%Y-%m-%d_%H-%M")
    safe_validator = validator.replace(" ", "_").lower()
    safe_gold = gold_filename.replace(" ", "_")
    output_filename = f"{safe_gold}_{timestamp}_{safe_validator}.txt"

    # Prepare lines to print and write
    lines = []

    lines.append("===== VALIDATION TEST RESULT =====\n")
    lines.append("# METADATA")
    lines.append(f"ON    :   {now.strftime('%Y-%m-%d %H:%M')}")
    lines.append(f"BY    :   {validator}")
    lines.append(f"FOR   :   {gold_filename}")
    lines.append(f"SIZE  :   {valset_size}\n")
    lines.append("# METRICS")
    lines.append(f"ERR   :   {n_errors}")
    lines.append(f"ACC   :   {accuracy:.2%}\n")
    lines.append("# MISLABELS")
    lines.append("    > the `input` field is the data entry.")
    lines.append("    > the `gold` field is the reference label in the gold standard.")
    lines.append("    > the `annot` field is the annotators label.\n")

    for i, (inp, exp, ans) in enumerate(mislabels, start=1):
        lines.append(f"{i}\tinput | {inp}\n\tgold  | {exp}\n\tannot | {ans}\n\t      |")

    # Print to console
    print("\n\n\n")  # spacer for console
    for line in lines:
        print(line)

    makedirs(eval_dir, exist_ok=True)
    with open(eval_dir + output_filename, "w", encoding="utf-8") as f:
        for line in lines:
            f.write(line + "\n")

    print(f"\nValidation results saved to {output_filename}")
    
    print("\n\n\n===== VALIDATION TEST RESULT =====\n")
    print("# METADATA")
    print(f"ON    :   {date.today()}")
    print(f"BY    :   {validator}")
    print(f"FOR   :   {filename}")
    print(f"SIZE  :   {valset_size}\n")
    print("# METRICS")
    print(f"ERR   :   {n_errors}")
    print(f"ACC   :   {accuracy:.2%}")
    print("\n# MISLABELS")
    print("    > the `input` field is the data entry.")
    print("    > the `gold` field is the reference label in the gold standard.")
    print("    > the `annot` field is the annotators label.\n")

    for i, (inp, exp, ans) in enumerate(mislabels, start=1):
        print(f"{i}\tinput | {inp}\n\tgold  | {exp}\n\tannot | {ans}\n\t      |")


if __name__ == "__main__":
    validator_name = input("Enter your name: ")
    filename, selected_file = choose_file()
    gold_map = open_file(selected_file)
    valset_size = get_validation_size(len(gold_map), max_validation_sample)
    valset = random_sample(gold_map, valset_size)
    mislabels, n_errors, accuracy = validate(valset, valset_size)
    print_result(mislabels, n_errors, accuracy, filename, valset_size, validator_name)
    selected_file.close()
