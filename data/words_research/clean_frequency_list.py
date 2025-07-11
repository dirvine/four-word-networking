#!/usr/bin/env python3
"""
Clean the 100k frequency word list for use in three-word networking.

This script performs initial cleanup on the downloaded frequency list:
- Removes words with numbers or punctuation
- Keeps only words with 2+ characters (matching Dictionary65K requirement)
- Removes obvious non-English words
- Removes basic offensive words
- Preserves frequency ordering

Input: data/top_english_words_lower_100000.txt
Output: data/cleaned_frequency_list.txt
"""

import re
import sys
from pathlib import Path

# Basic offensive words to filter out
OFFENSIVE_WORDS = {
    'fuck', 'shit', 'bitch', 'cock', 'cunt', 'dick', 'piss', 'fag',
    'nigger', 'retard', 'whore', 'slut', 'bastard', 'damn', 'hell',
    'ass', 'arse', 'prick', 'wank', 'feck', 'bollocks', 'bugger',
    'crap', 'tits', 'twat', 'pussy', 'dildo', 'sperm', 'semen'
}

# Common non-English patterns to filter
NON_ENGLISH_PATTERNS = [
    re.compile(r'ü|ö|ä|ß|ñ|ç|æ|ø|å|œ|ł|ż|ś|ć|ń'),  # Non-English characters
    re.compile(r'^[xz]{3,}'),  # Unusual consonant clusters
    re.compile(r'[bcdfghjklmnpqrstvwxyz]{5,}'),  # Too many consonants
]

def is_valid_word(word):
    """Check if a word meets our criteria."""
    # Length check (minimum 2 characters)
    if len(word) < 2:
        return False
    
    # Must be purely alphabetic
    if not word.isalpha():
        return False
    
    # Check for offensive words
    if word.lower() in OFFENSIVE_WORDS:
        return False
    
    # Check for non-English patterns
    for pattern in NON_ENGLISH_PATTERNS:
        if pattern.search(word):
            return False
    
    # Filter out very rare/unusual letter combinations
    # But keep common 2-letter words
    if len(word) == 2:
        # Keep all 2-letter words for now (will be filtered by Claude later)
        return True
    
    return True

def main():
    # Set up paths
    project_root = Path(__file__).parent.parent
    input_file = project_root / "data" / "top_english_words_lower_100000.txt"
    output_file = project_root / "data" / "cleaned_frequency_list.txt"
    
    if not input_file.exists():
        print(f"Error: Input file not found: {input_file}")
        sys.exit(1)
    
    # Read and clean words
    cleaned_words = []
    removed_count = 0
    
    print("Reading word list...")
    with open(input_file, 'r', encoding='utf-8') as f:
        for line_num, line in enumerate(f, 1):
            word = line.strip().lower()
            
            if not word:
                continue
            
            if is_valid_word(word):
                cleaned_words.append(word)
            else:
                removed_count += 1
                # Show examples of removed words (first 20)
                if removed_count <= 20:
                    print(f"  Removed: {word}")
    
    # Write cleaned list
    print(f"\nWriting cleaned list to {output_file}...")
    with open(output_file, 'w', encoding='utf-8') as f:
        for word in cleaned_words:
            f.write(f"{word}\n")
    
    # Summary statistics
    print(f"\nCleaning complete!")
    print(f"  Input words: 100,000")
    print(f"  Output words: {len(cleaned_words):,}")
    print(f"  Removed: {removed_count:,}")
    print(f"  Removal rate: {removed_count/1000:.1f}%")
    
    # Word length distribution
    length_dist = {}
    for word in cleaned_words:
        length = len(word)
        length_dist[length] = length_dist.get(length, 0) + 1
    
    print(f"\nWord length distribution:")
    for length in sorted(length_dist.keys()):
        count = length_dist[length]
        print(f"  {length:2d} chars: {count:6,} words")
    
    # Show some example words from different frequency ranges
    print(f"\nExample words by frequency:")
    ranges = [(0, 10, "Most common"), (100, 110, "Common"), 
              (1000, 1010, "Moderate"), (10000, 10010, "Less common")]
    
    for start, end, label in ranges:
        if start < len(cleaned_words):
            examples = cleaned_words[start:min(end, len(cleaned_words))]
            print(f"  {label} ({start}-{end}): {', '.join(examples[:5])}")

if __name__ == "__main__":
    main()