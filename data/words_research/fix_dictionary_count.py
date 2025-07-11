#!/usr/bin/env python3
"""
Fix dictionary to have exactly 65,536 words by adding missing words.

UV Dependencies:
# Run this script: uv run python scripts/fix_dictionary_count.py
"""

# /// script
# requires-python = ">=3.11"
# ///

from pathlib import Path


def fix_dictionary_count():
    """Add missing words to reach exactly 65,536 words."""
    data_dir = Path(__file__).parent.parent / "data"
    dict_file = data_dir / "human_readable_word_list_65k.txt"
    
    # Read current words
    with open(dict_file, 'r', encoding='utf-8') as f:
        words = [line.strip() for line in f if line.strip()]
    
    current_count = len(words)
    print(f"Current word count: {current_count}")
    
    if current_count == 65536:
        print("Dictionary already has exactly 65,536 words!")
        return
    
    words_needed = 65536 - current_count
    print(f"Need to add {words_needed} words")
    
    # Convert to set for faster lookup
    existing_words = set(words)
    
    # Read from the cleaned frequency list to find additional words
    freq_list = data_dir / "words_research" / "cleaned_frequency_list.txt"
    
    replacement_words = []
    if freq_list.exists():
        with open(freq_list, 'r', encoding='utf-8') as f:
            for line in f:
                word = line.strip().lower()
                if (word and 
                    word.isalpha() and 
                    len(word) >= 2 and
                    all(ord(c) < 128 for c in word) and  # ASCII only
                    word not in existing_words):
                    replacement_words.append(word)
                    if len(replacement_words) >= words_needed:
                        break
    
    if len(replacement_words) < words_needed:
        print(f"WARNING: Only found {len(replacement_words)} replacement words")
        # Try the top 100k list
        alt_list = data_dir / "words_research" / "top_english_words_lower_100000.txt"
        if alt_list.exists():
            with open(alt_list, 'r', encoding='utf-8') as f:
                for line in f:
                    word = line.strip().lower()
                    if (word and 
                        word.isalpha() and 
                        len(word) >= 2 and
                        all(ord(c) < 128 for c in word) and  # ASCII only
                        word not in existing_words and
                        word not in replacement_words):
                        replacement_words.append(word)
                        if len(replacement_words) >= words_needed:
                            break
    
    # Add the replacement words
    words.extend(replacement_words[:words_needed])
    
    final_count = len(words)
    print(f"Final word count: {final_count}")
    
    if final_count != 65536:
        print(f"ERROR: Expected 65536 words, got {final_count}")
        return
    
    # Write the updated dictionary
    with open(dict_file, 'w', encoding='utf-8') as f:
        for word in words:
            f.write(f"{word}\n")
    
    print(f"Dictionary updated successfully!")
    print(f"Added words (first 10):")
    for word in replacement_words[:10]:
        print(f"  - {word}")


if __name__ == "__main__":
    fix_dictionary_count()