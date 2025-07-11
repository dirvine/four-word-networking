#!/usr/bin/env python3
"""
Remove non-ASCII words from the dictionary and replace with ASCII alternatives.

UV Dependencies:
# Run this script: uv run python scripts/remove_non_ascii_words.py
"""

# /// script
# requires-python = ">=3.11"
# ///

from pathlib import Path


def is_ascii(word: str) -> bool:
    """Check if a word contains only ASCII characters."""
    return all(ord(c) < 128 for c in word)


def clean_dictionary():
    """Remove non-ASCII words and replace with ASCII alternatives."""
    data_dir = Path(__file__).parent.parent / "data"
    dict_file = data_dir / "human_readable_word_list_65k.txt"
    
    # Read all words
    with open(dict_file, 'r', encoding='utf-8') as f:
        words = [line.strip() for line in f]
    
    print(f"Original word count: {len(words)}")
    
    # Filter out non-ASCII words
    ascii_words = []
    non_ascii_words = []
    
    for word in words:
        if is_ascii(word):
            ascii_words.append(word)
        else:
            non_ascii_words.append(word)
    
    print(f"ASCII words: {len(ascii_words)}")
    print(f"Non-ASCII words: {len(non_ascii_words)}")
    
    # Read the original 100k list to find replacements
    freq_list = data_dir / "google-10000-english-usa-no-swears.txt"
    if not freq_list.exists():
        # Try the downloaded file
        freq_list = data_dir / "count_1w.txt"
    
    replacement_candidates = []
    if freq_list.exists():
        with open(freq_list, 'r', encoding='utf-8') as f:
            for line in f:
                parts = line.strip().split()
                if parts:
                    word = parts[0].lower()
                    if (is_ascii(word) and 
                        len(word) >= 2 and 
                        word.isalpha() and
                        word not in ascii_words):
                        replacement_candidates.append(word)
    
    print(f"Found {len(replacement_candidates)} potential replacements")
    
    # Add replacements to reach exactly 65536 words
    words_needed = 65536 - len(ascii_words)
    print(f"Need {words_needed} replacement words")
    
    if words_needed > 0:
        replacements = replacement_candidates[:words_needed]
        ascii_words.extend(replacements)
        print(f"Added {len(replacements)} replacement words")
    
    # Verify final count
    print(f"Final word count: {len(ascii_words)}")
    
    if len(ascii_words) != 65536:
        print(f"WARNING: Expected 65536 words, got {len(ascii_words)}")
        return
    
    # Write the cleaned dictionary
    with open(dict_file, 'w', encoding='utf-8') as f:
        for word in ascii_words:
            f.write(f"{word}\n")
    
    print(f"Dictionary cleaned and saved to {dict_file}")
    
    # Show some of the removed words
    print("\nRemoved non-ASCII words (first 10):")
    for word in non_ascii_words[:10]:
        print(f"  - {word}")


if __name__ == "__main__":
    clean_dictionary()