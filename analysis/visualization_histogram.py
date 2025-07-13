#!/usr/bin/env python3
"""
Word Length Distribution Visualization

Creates a visual histogram of word length distribution and analyzes
the efficiency of the distribution.

Usage:
    python3 visualization_histogram.py

Requirements:
    - GOLD_WORDLIST.txt in parent directory
"""

import math
from collections import defaultdict

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
    for word in words:
        length_counts[len(word)] += 1

    # Create histogram
    print("Word Length Distribution Histogram")
    print("="*60)

    max_count = max(length_counts.values())
    scale = 50 / max_count  # Scale to fit in 50 characters

    for length in sorted(length_counts.keys()):
        count = length_counts[length]
        bar_length = int(count * scale)
        bar = '█' * bar_length
        entropy = math.log2(count) if count > 1 else 0
        print(f"{length:2d} chars: {bar:<50} {count:5d} ({entropy:5.2f} bits)")

    print(f"\nLegend: Each █ represents approximately {int(max_count/50)} words")

    # Calculate information about the distribution
    print("\nDistribution Analysis:")
    print("-"*40)

    # Calculate the theoretical maximum entropy for uniform distribution
    theoretical_max = math.log2(65536)
    print(f"Theoretical maximum entropy (uniform): {theoretical_max:.2f} bits")

    # Calculate actual entropy of the length distribution
    total = sum(length_counts.values())
    length_entropy = 0
    for count in length_counts.values():
        if count > 0:
            p = count / total
            length_entropy -= p * math.log2(p)

    print(f"Entropy of length distribution: {length_entropy:.2f} bits")
    print(f"Efficiency vs uniform: {length_entropy/math.log2(len(length_counts))*100:.1f}%")

    # Find optimal length range
    sorted_lengths = sorted(length_counts.items(), key=lambda x: x[1], reverse=True)
    top_5_lengths = sorted_lengths[:5]
    top_5_count = sum(count for _, count in top_5_lengths)
    top_5_percentage = top_5_count / total * 100

    print(f"\nTop 5 most common lengths: {[l for l, _ in top_5_lengths]}")
    print(f"These contain {top_5_count:,} words ({top_5_percentage:.1f}% of dictionary)")

if __name__ == "__main__":
    main()