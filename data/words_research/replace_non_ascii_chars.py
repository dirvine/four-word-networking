#!/usr/bin/env python3
"""
Replace non-ASCII characters with ASCII equivalents in dictionary words.

UV Dependencies:
# Run this script: uv run python scripts/replace_non_ascii_chars.py
"""

# /// script
# requires-python = ">=3.11"
# ///

from pathlib import Path
import unicodedata


def remove_accents(text: str) -> str:
    """Remove accents from characters and convert to ASCII equivalent."""
    # Normalize to NFD (decomposed form) then encode to ASCII ignoring non-ASCII
    nfd = unicodedata.normalize('NFD', text)
    return ''.join(char for char in nfd if unicodedata.category(char) != 'Mn')


def clean_dictionary():
    """Replace non-ASCII characters with ASCII equivalents."""
    data_dir = Path(__file__).parent.parent / "data"
    dict_file = data_dir / "human_readable_word_list_65k.txt"
    
    # Read all words
    with open(dict_file, 'r', encoding='utf-8') as f:
        words = [line.strip() for line in f]
    
    print(f"Original word count: {len(words)}")
    
    # Process words to replace non-ASCII characters
    cleaned_words = []
    modified_words = []
    
    for word in words:
        # Check if word has non-ASCII characters
        if all(ord(c) < 128 for c in word):
            cleaned_words.append(word)
        else:
            # Replace accented characters with ASCII equivalents
            cleaned = remove_accents(word)
            cleaned_words.append(cleaned)
            modified_words.append((word, cleaned))
    
    print(f"Modified {len(modified_words)} words with non-ASCII characters")
    
    # Check for duplicates after cleaning
    seen = set()
    duplicates = []
    final_words = []
    
    for word in cleaned_words:
        if word in seen:
            duplicates.append(word)
        else:
            seen.add(word)
            final_words.append(word)
    
    print(f"Found {len(duplicates)} duplicate words after ASCII conversion")
    
    # If we have duplicates, we need to handle them
    if duplicates:
        print(f"Final word count after removing duplicates: {len(final_words)}")
        
        # We need to add more words to maintain 65536 count
        words_needed = 65536 - len(final_words)
        if words_needed > 0:
            print(f"WARNING: Need {words_needed} additional words to reach 65536")
            # For now, just proceed with what we have
    
    # Write the cleaned dictionary
    with open(dict_file, 'w', encoding='utf-8') as f:
        for word in final_words:
            f.write(f"{word}\n")
    
    print(f"\nDictionary cleaned and saved to {dict_file}")
    print(f"Final word count: {len(final_words)}")
    
    # Show some of the modified words
    print("\nModified words (first 20):")
    for original, cleaned in modified_words[:20]:
        print(f"  {original} → {cleaned}")
    
    # Show duplicates if any
    if duplicates[:10]:
        print(f"\nDuplicate words after conversion (first 10):")
        for word in duplicates[:10]:
            print(f"  - {word}")


if __name__ == "__main__":
    clean_dictionary()