#!/usr/bin/env python3
"""
Fix dictionary size to exactly 65536 words by adding words from frequency list.

UV Dependencies:
# Run this script: uv run python scripts/fix_dictionary_size.py
"""

# /// script
# requires-python = ">=3.11"
# ///

from pathlib import Path


def fix_dictionary_size():
    """Ensure dictionary has exactly 65536 words."""
    data_dir = Path(__file__).parent.parent / "data"
    dict_file = data_dir / "human_readable_word_list_65k.txt"
    
    # Read current dictionary
    with open(dict_file, 'r', encoding='utf-8') as f:
        current_words = [line.strip() for line in f if line.strip()]
    
    current_words_set = set(current_words)
    print(f"Current word count: {len(current_words)}")
    
    words_needed = 65536 - len(current_words)
    print(f"Need {words_needed} additional words")
    
    if words_needed <= 0:
        print("Dictionary already has enough words")
        return
    
    # Read from cleaned frequency list
    freq_file = data_dir / "cleaned_frequency_list.txt"
    replacement_words = []
    
    with open(freq_file, 'r', encoding='utf-8') as f:
        for line in f:
            word = line.strip()
            if (word and 
                word not in current_words_set and
                len(word) >= 2 and
                word.isalpha() and
                all(ord(c) < 128 for c in word)):
                replacement_words.append(word)
                if len(replacement_words) >= words_needed:
                    break
    
    print(f"Found {len(replacement_words)} replacement words")
    
    # Add replacement words
    final_words = current_words + replacement_words[:words_needed]
    
    # Verify no duplicates
    if len(set(final_words)) != len(final_words):
        print("ERROR: Duplicates found in final word list")
        return
    
    print(f"Final word count: {len(final_words)}")
    
    # Write the fixed dictionary
    with open(dict_file, 'w', encoding='utf-8') as f:
        for word in final_words:
            f.write(f"{word}\n")
    
    print(f"Dictionary fixed and saved to {dict_file}")
    
    # Show some of the added words
    print(f"\nAdded words (first 10):")
    for word in replacement_words[:10]:
        print(f"  - {word}")


if __name__ == "__main__":
    fix_dictionary_size()