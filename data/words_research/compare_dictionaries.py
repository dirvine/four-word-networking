#!/usr/bin/env python3
"""
Compare the current dictionary with the new frequency-based dictionary.

Shows words that are:
- Only in the current dictionary
- Only in the new dictionary
- Common to both
- Statistics about word length and frequency
"""

import sys
from pathlib import Path
from collections import defaultdict

def load_words(file_path):
    """Load words from a file into a set."""
    with open(file_path, 'r', encoding='utf-8') as f:
        return set(line.strip() for line in f if line.strip())

def analyze_word_lengths(words):
    """Analyze word length distribution."""
    length_dist = defaultdict(int)
    for word in words:
        length_dist[len(word)] += 1
    return dict(length_dist)

def main():
    # Set up paths
    project_root = Path(__file__).parent.parent
    current_dict = project_root / "data" / "human_readable_word_list_65k.txt"
    new_dict = project_root / "data" / "claude_filtered_words.txt"
    
    if not current_dict.exists():
        print(f"Error: Current dictionary not found: {current_dict}")
        sys.exit(1)
    
    if not new_dict.exists():
        print(f"Error: New dictionary not found: {new_dict}")
        sys.exit(1)
    
    # Load dictionaries
    print("Loading dictionaries...")
    current_words = load_words(current_dict)
    new_words = load_words(new_dict)
    
    # Calculate differences
    only_current = current_words - new_words
    only_new = new_words - current_words
    common = current_words & new_words
    
    # Print summary
    print("\n=== Dictionary Comparison ===")
    print(f"Current dictionary: {len(current_words):,} words")
    print(f"New dictionary: {len(new_words):,} words")
    print(f"\nCommon words: {len(common):,} ({len(common)/len(current_words)*100:.1f}%)")
    print(f"Only in current: {len(only_current):,} words")
    print(f"Only in new: {len(only_new):,} words")
    
    # Analyze word lengths
    print("\n=== Word Length Analysis ===")
    current_lengths = analyze_word_lengths(current_words)
    new_lengths = analyze_word_lengths(new_words)
    
    print("\nCurrent dictionary length distribution:")
    for length in sorted(current_lengths.keys())[:10]:
        count = current_lengths[length]
        print(f"  {length:2d} chars: {count:6,} words")
    
    print("\nNew dictionary length distribution:")
    for length in sorted(new_lengths.keys())[:10]:
        count = new_lengths[length]
        print(f"  {length:2d} chars: {count:6,} words")
    
    # Show sample differences
    print("\n=== Sample Words Only in Current Dictionary ===")
    sample_current = sorted(only_current)[:30]
    for i in range(0, len(sample_current), 6):
        print("  " + " | ".join(sample_current[i:i+6]))
    
    print("\n=== Sample Words Only in New Dictionary ===")
    sample_new = sorted(only_new)[:30]
    for i in range(0, len(sample_new), 6):
        print("  " + " | ".join(sample_new[i:i+6]))
    
    # Check for essential 2-letter words
    essential_2_letter = {'be', 'to', 'is', 'at', 'by', 'in', 'on', 'up', 'we', 'me', 'he', 'it', 'or', 'if', 'so', 'no', 'go', 'do'}
    
    print("\n=== Essential 2-Letter Words Check ===")
    current_essential = essential_2_letter & current_words
    new_essential = essential_2_letter & new_words
    
    print(f"Current dictionary has {len(current_essential)}/{len(essential_2_letter)} essential words")
    print(f"New dictionary has {len(new_essential)}/{len(essential_2_letter)} essential words")
    
    if new_essential != essential_2_letter:
        missing = essential_2_letter - new_essential
        print(f"Missing essential words in new: {missing}")
    else:
        print("✓ All essential 2-letter words present in new dictionary")
    
    # Write detailed comparison to file
    output_file = project_root / "data" / "dictionary_comparison.txt"
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write("Dictionary Comparison Report\n")
        f.write("=" * 50 + "\n\n")
        
        f.write(f"Current: {len(current_words):,} words\n")
        f.write(f"New: {len(new_words):,} words\n")
        f.write(f"Common: {len(common):,} words\n\n")
        
        f.write("Words only in current dictionary:\n")
        for word in sorted(only_current):
            f.write(f"  {word}\n")
        
        f.write("\nWords only in new dictionary:\n")
        for word in sorted(only_new):
            f.write(f"  {word}\n")
    
    print(f"\n=== Detailed comparison saved to {output_file} ===")

if __name__ == "__main__":
    main()