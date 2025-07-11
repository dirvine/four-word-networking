#!/usr/bin/env python3
"""
Final cleanup to ensure all words are ASCII only.

UV Dependencies:
# Run this script: uv run python scripts/final_ascii_cleanup.py
"""

# /// script
# requires-python = ">=3.11"
# ///

from pathlib import Path


def final_cleanup():
    """Remove any remaining non-ASCII words and replace them."""
    data_dir = Path(__file__).parent.parent / "data"
    dict_file = data_dir / "human_readable_word_list_65k.txt"
    
    # Read current words
    with open(dict_file, 'r', encoding='utf-8') as f:
        words = [line.strip() for line in f if line.strip()]
    
    # Find non-ASCII words
    ascii_words = []
    non_ascii_words = []
    
    for word in words:
        if all(ord(c) < 128 for c in word):
            ascii_words.append(word)
        else:
            non_ascii_words.append(word)
            print(f"Removing non-ASCII word: {word}")
    
    print(f"\nFound {len(non_ascii_words)} non-ASCII words")
    print(f"ASCII words: {len(ascii_words)}")
    
    # Find replacements
    existing_words = set(ascii_words)
    replacements = []
    
    # Hardcode some common replacements for these specific cases
    manual_replacements = [
        "micrometer", "nanogram", "micron", "micro",
        "nano", "milli", "kilo", "mega", "giga", "tera",
        "alpha", "beta", "gamma", "delta", "epsilon",
        "zeta", "eta", "theta", "iota", "kappa",
        "lambda", "mu", "nu", "xi", "omicron",
        "pi", "rho", "sigma", "tau", "upsilon",
        "phi", "chi", "psi", "omega"
    ]
    
    for word in manual_replacements:
        if word not in existing_words and len(replacements) < len(non_ascii_words):
            replacements.append(word)
            existing_words.add(word)
    
    # If we still need more, get from frequency list
    if len(replacements) < len(non_ascii_words):
        freq_list = data_dir / "words_research" / "cleaned_frequency_list.txt"
        if freq_list.exists():
            with open(freq_list, 'r', encoding='utf-8') as f:
                for line in f:
                    word = line.strip().lower()
                    if (word and 
                        word.isalpha() and 
                        len(word) >= 2 and
                        all(ord(c) < 128 for c in word) and
                        word not in existing_words):
                        replacements.append(word)
                        existing_words.add(word)
                        if len(replacements) >= len(non_ascii_words):
                            break
    
    # Add replacements
    ascii_words.extend(replacements[:len(non_ascii_words)])
    
    print(f"Added {len(replacements[:len(non_ascii_words)])} replacement words")
    print(f"Final word count: {len(ascii_words)}")
    
    if len(ascii_words) != 65536:
        print(f"ERROR: Expected 65536 words, got {len(ascii_words)}")
        return
    
    # Write the cleaned dictionary
    with open(dict_file, 'w', encoding='utf-8') as f:
        for word in ascii_words:
            f.write(f"{word}\n")
    
    print("\nDictionary cleaned successfully!")
    print("Replacement words used:")
    for word in replacements[:len(non_ascii_words)]:
        print(f"  - {word}")


if __name__ == "__main__":
    final_cleanup()