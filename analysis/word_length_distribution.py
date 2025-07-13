#!/usr/bin/env python3
"""
Word Length Distribution Analysis

Analyzes the GOLD_WORDLIST.txt to show distribution of words by character length
and calculates entropy for each length group.

Usage:
    python3 word_length_distribution.py

Requirements:
    - GOLD_WORDLIST.txt in parent directory
"""

import math
from collections import Counter, defaultdict

def calculate_entropy(word_list):
    """Calculate entropy in bits for a list of words"""
    if len(word_list) <= 1:
        return 0.0
    return math.log2(len(word_list))

def main():
    # Read the wordlist
    try:
        with open('../GOLD_WORDLIST.txt', 'r') as f:
            words = [line.strip() for line in f if line.strip()]
    except FileNotFoundError:
        print("Error: GOLD_WORDLIST.txt not found in parent directory")
        return

    # Count words by length
    length_counts = defaultdict(int)
    words_by_length = defaultdict(list)

    for word in words:
        length = len(word)
        length_counts[length] += 1
        words_by_length[length].append(word)

    # Print header
    print("Word Length Distribution Analysis")
    print("="*80)
    print()
    print("Length | Count  | Percentage | Entropy (bits) | Example Words")
    print("-------|--------|------------|----------------|---------------")

    total_words = len(words)
    total_entropy = math.log2(total_words)

    # Sort by length and display
    for length in sorted(length_counts.keys()):
        count = length_counts[length]
        percentage = (count / total_words) * 100
        entropy = calculate_entropy(words_by_length[length])
        
        # Get a few example words
        examples = words_by_length[length][:3]
        examples_str = ", ".join(examples)
        if len(words_by_length[length]) > 3:
            examples_str += ", ..."
        
        print(f"{length:^7}| {count:^7}| {percentage:^10.2f}% | {entropy:^14.2f} | {examples_str}")

    # Print summary statistics
    print("\n" + "="*80)
    print(f"Total words: {total_words:,}")
    print(f"Total entropy: {total_entropy:.2f} bits")
    print(f"Minimum word length: {min(length_counts.keys())} characters")
    print(f"Maximum word length: {max(length_counts.keys())} characters")
    print(f"Average word length: {sum(length * count for length, count in length_counts.items()) / total_words:.2f} characters")

    # Find the most common length
    most_common_length = max(length_counts.items(), key=lambda x: x[1])
    print(f"Most common word length: {most_common_length[0]} characters ({most_common_length[1]:,} words, {most_common_length[1]/total_words*100:.1f}%)")

    # Calculate weighted average entropy (by word count)
    weighted_entropy = sum(count * calculate_entropy(words_by_length[length]) for length, count in length_counts.items()) / total_words
    print(f"Weighted average entropy per length group: {weighted_entropy:.2f} bits")

if __name__ == "__main__":
    main()