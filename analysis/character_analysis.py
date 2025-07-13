#!/usr/bin/env python3
"""
Character Pattern Analysis

Analyzes character frequencies, patterns, and linguistic properties
of the GOLD_WORDLIST.txt dictionary.

Usage:
    python3 character_analysis.py

Requirements:
    - GOLD_WORDLIST.txt in parent directory
"""

from collections import Counter
import re
import math

def main():
    # Read the wordlist
    try:
        with open('../GOLD_WORDLIST.txt', 'r') as f:
            words = [line.strip().lower() for line in f if line.strip()]
    except FileNotFoundError:
        print("Error: GOLD_WORDLIST.txt not found in parent directory")
        return

    print("Character Pattern Analysis")
    print("="*50)

    # Count total characters
    total_chars = sum(len(word) for word in words)
    print(f"\nTotal characters in dictionary: {total_chars:,}")
    print(f"Average characters per word: {total_chars/len(words):.2f}")

    # Character frequency
    all_chars = ''.join(words)
    char_freq = Counter(all_chars)

    print("\nMost common characters:")
    for char, count in char_freq.most_common(10):
        percentage = (count / total_chars) * 100
        print(f"  '{char}': {count:,} ({percentage:.2f}%)")

    # Calculate character entropy
    char_entropy = 0
    for count in char_freq.values():
        if count > 0:
            p = count / total_chars
            char_entropy -= p * math.log2(p)

    print(f"\nCharacter-level entropy: {char_entropy:.2f} bits")
    print(f"Alphabet size used: {len(char_freq)} unique characters")

    # Special character analysis
    special_patterns = {
        'numbers': r'\d',
        'hyphens': r'-',
        'apostrophes': r"'",
        'spaces': r' ',
        'non-alphanumeric': r'[^a-zA-Z0-9]'
    }

    print("\nSpecial character analysis:")
    for pattern_name, pattern in special_patterns.items():
        matching_words = [w for w in words if re.search(pattern, w)]
        count = len(matching_words)
        percentage = (count / len(words)) * 100
        if count > 0:
            print(f"  Words with {pattern_name}: {count} ({percentage:.2f}%)")
            print(f"    Examples: {', '.join(matching_words[:5])}")

    # Check for problematic patterns
    print("\nQuality checks:")

    # Single-letter words
    single_letter = [w for w in words if len(w) == 1]
    print(f"  Single-letter words: {len(single_letter)} - {', '.join(sorted(set(single_letter)))}")

    # Two-letter words
    two_letter = [w for w in words if len(w) == 2]
    print(f"  Two-letter words: {len(two_letter)} (examples: {', '.join(two_letter[:10])})")

    # Very long words
    long_words = [w for w in words if len(w) >= 14]
    print(f"  Words with 14+ characters: {len(long_words)}")
    for w in sorted(long_words, key=len, reverse=True)[:10]:
        print(f"    {w} ({len(w)} chars)")

    # Words starting with capital letters (checking original case)
    try:
        with open('../GOLD_WORDLIST.txt', 'r') as f:
            original_words = [line.strip() for line in f if line.strip()]
    except FileNotFoundError:
        original_words = words
        
    capitalized = [w for w in original_words if w and w[0].isupper()]
    print(f"\n  Capitalized words: {len(capitalized)} ({len(capitalized)/len(words)*100:.2f}%)")
    if capitalized:
        print(f"    Examples: {', '.join(capitalized[:10])}")

    # Phonetic analysis - check for common endings
    print("\nCommon word endings (suffixes):")
    endings = Counter()
    for word in words:
        if len(word) >= 3:
            endings[word[-3:]] += 1
            
    for ending, count in endings.most_common(15):
        percentage = (count / len(words)) * 100
        print(f"  -{ending}: {count} words ({percentage:.2f}%)")

    # Check for homophones indicators (common patterns)
    print("\nPotential homophone patterns:")
    patterns = {
        'tion': 0, 'sion': 0, 'ght': 0, 'ough': 0, 
        'eigh': 0, 'augh': 0, 'ite': 0, 'ight': 0,
        'ate': 0, 'ait': 0, 'eight': 0
    }

    for word in words:
        for pattern in patterns:
            if pattern in word:
                patterns[pattern] += 1

    for pattern, count in sorted(patterns.items(), key=lambda x: x[1], reverse=True):
        if count > 0:
            percentage = (count / len(words)) * 100
            print(f"  Contains '{pattern}': {count} words ({percentage:.2f}%)")

    # Vowel/consonant ratio
    vowels = set('aeiou')
    vowel_count = sum(1 for c in all_chars if c in vowels)
    consonant_count = total_chars - vowel_count

    print(f"\nVowel/Consonant analysis:")
    print(f"  Vowels: {vowel_count:,} ({vowel_count/total_chars*100:.1f}%)")
    print(f"  Consonants: {consonant_count:,} ({consonant_count/total_chars*100:.1f}%)")
    print(f"  Vowel/Consonant ratio: 1:{consonant_count/vowel_count:.2f}")

if __name__ == "__main__":
    main()