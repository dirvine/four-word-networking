#!/usr/bin/env python3
"""
Comprehensive Dictionary Summary

Creates a comprehensive summary table with all key metrics for the
GOLD_WORDLIST.txt dictionary analysis.

Usage:
    python3 comprehensive_summary.py

Requirements:
    - GOLD_WORDLIST.txt in parent directory
"""

import math
from collections import defaultdict

def voice_score(length):
    """Calculate voice-friendliness score based on word length"""
    if length == 1:
        return "Poor"
    elif length in [2, 3]:
        return "Fair"
    elif length in [4, 5, 6, 7]:
        return "Excellent"
    elif length in [8, 9]:
        return "Good"
    elif length in [10, 11]:
        return "Fair"
    else:
        return "Poor"

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

    print("GOLD_WORDLIST.txt Analysis Summary")
    print("="*80)
    print()
    print("| Length | Word Count | % of Dict | Entropy (bits) | Bits/Char | Voice Score |")
    print("|--------|------------|-----------|----------------|-----------|-------------|")

    total_words = len(words)

    for length in sorted(length_counts.keys()):
        count = length_counts[length]
        percentage = (count / total_words) * 100
        entropy = math.log2(count) if count > 1 else 0
        bits_per_char = entropy / length if length > 0 else 0
        
        print(f"| {length:^6} | {count:^10,} | {percentage:^9.2f}% | {entropy:^14.2f} | {bits_per_char:^9.2f} | {voice_score(length):^11} |")

    print("|" + "-"*78 + "|")

    # Summary statistics
    total_entropy = math.log2(total_words)
    avg_length = sum(length * count for length, count in length_counts.items()) / total_words
    weighted_entropy = sum(count * math.log2(count) for count in length_counts.values() if count > 1) / total_words

    print(f"| {'TOTAL':^6} | {total_words:^10,} | {100.0:^9.2f}% | {total_entropy:^14.2f} | {total_entropy/avg_length:^9.2f} | {'N/A':^11} |")
    print("="*80)

    print("\nKey Metrics:")
    print(f"• Total dictionary size: {total_words:,} words (2^16)")
    print(f"• Total entropy: {total_entropy:.2f} bits")
    print(f"• Average word length: {avg_length:.2f} characters")
    print(f"• Most common length: {max(length_counts.items(), key=lambda x: x[1])[0]} characters")
    print(f"• Optimal length range (4-9 chars): {sum(length_counts[i] for i in range(4, 10)):,} words ({sum(length_counts[i] for i in range(4, 10))/total_words*100:.1f}%)")

    # Calculate encoding efficiency
    print("\nEncoding Efficiency:")
    print(f"• Bits per word: 16.00 (fixed)")
    print(f"• Average bits per character: {16.0/avg_length:.2f}")
    print(f"• Character-level entropy: ~4.21 bits")
    print(f"• Efficiency ratio: {(16.0/avg_length)/4.21*100:.1f}% of theoretical maximum")

    # Voice-friendliness analysis
    voice_optimal = sum(count for length, count in length_counts.items() if 4 <= length <= 7)
    print(f"\nVoice-Friendliness Analysis:")
    print(f"• Optimal length words (4-7 chars): {voice_optimal:,} ({voice_optimal/total_words*100:.1f}%)")
    print(f"• Short words (1-3 chars): {sum(length_counts[i] for i in range(1, 4)):,} ({sum(length_counts[i] for i in range(1, 4))/total_words*100:.1f}%)")
    print(f"• Long words (12+ chars): {sum(count for length, count in length_counts.items() if length >= 12):,} ({sum(count for length, count in length_counts.items() if length >= 12)/total_words*100:.1f}%)")

    # Three-word system analysis
    print(f"\nThree-Word System Properties:")
    print(f"• Total entropy: 3 × 16 = 48 bits")
    print(f"• Possible combinations: {65536**3:,}")
    print(f"• Average total length: {avg_length * 3 + 2:.1f} characters (including dots)")
    print(f"• Security equivalent: 8-character complex password")

if __name__ == "__main__":
    main()