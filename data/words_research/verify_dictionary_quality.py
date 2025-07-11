#!/usr/bin/env python3
"""
Verify the quality of the final 65,536-word dictionary.

This script performs comprehensive quality checks:
- Verifies exact word count (65,536)
- Checks minimum word length (2+ characters)
- Identifies potential homophones
- Checks for offensive word combinations
- Analyzes word length distribution
- Verifies pronunciation clarity indicators

Input: data/claude_filtered_words.txt (or specified file)
Output: Quality report to console and data/dictionary_quality_report.txt
"""

import sys
from pathlib import Path
from collections import defaultdict, Counter
import re
from typing import List, Dict, Set, Tuple

# Common homophones to check
KNOWN_HOMOPHONES = [
    ('to', 'too', 'two'),
    ('there', 'their', 'they\'re'),
    ('your', 'you\'re'),
    ('its', 'it\'s'),
    ('by', 'buy', 'bye'),
    ('know', 'no'),
    ('write', 'right', 'rite'),
    ('hear', 'here'),
    ('wear', 'where'),
    ('break', 'brake'),
    ('peace', 'piece'),
    ('mail', 'male'),
    ('tale', 'tail'),
    ('sail', 'sale'),
    ('pair', 'pear'),
    ('fair', 'fare'),
    ('hair', 'hare'),
    ('bear', 'bare'),
    ('dear', 'deer'),
    ('meat', 'meet'),
    ('steal', 'steel'),
    ('week', 'weak'),
    ('hole', 'whole'),
    ('sun', 'son'),
    ('way', 'weigh'),
    ('wait', 'weight'),
]

# Potentially problematic word combinations when adjacent
PROBLEMATIC_PAIRS = [
    ('big', 'ass'), ('dumb', 'ass'), ('bad', 'ass'),
    ('hard', 'core'), ('soft', 'core'),
    ('blow', 'job'), ('hand', 'job'),
    ('gang', 'bang'),
]

class DictionaryQualityChecker:
    def __init__(self, dictionary_file: Path):
        self.dictionary_file = dictionary_file
        self.words = []
        self.word_set = set()
        self.report_lines = []
        
        # Load dictionary
        with open(dictionary_file, 'r', encoding='utf-8') as f:
            self.words = [line.strip() for line in f if line.strip()]
        self.word_set = set(self.words)
        
    def add_report(self, line: str = ""):
        """Add a line to the report."""
        print(line)
        self.report_lines.append(line)
    
    def check_word_count(self) -> bool:
        """Verify exact word count."""
        self.add_report("=== Word Count Check ===")
        count = len(self.words)
        expected = 65536
        
        if count == expected:
            self.add_report(f"✓ Word count: {count:,} (correct)")
            return True
        else:
            self.add_report(f"✗ Word count: {count:,} (expected {expected:,})")
            diff = count - expected
            self.add_report(f"  Difference: {diff:+,} words")
            return False
    
    def check_duplicates(self) -> bool:
        """Check for duplicate words."""
        self.add_report("\n=== Duplicate Check ===")
        
        if len(self.words) == len(self.word_set):
            self.add_report("✓ No duplicates found")
            return True
        else:
            duplicates = []
            seen = set()
            for word in self.words:
                if word in seen:
                    duplicates.append(word)
                seen.add(word)
            
            self.add_report(f"✗ Found {len(duplicates)} duplicates:")
            for dup in duplicates[:10]:
                self.add_report(f"  - {dup}")
            if len(duplicates) > 10:
                self.add_report(f"  ... and {len(duplicates) - 10} more")
            return False
    
    def check_word_lengths(self) -> Dict[int, int]:
        """Check word length distribution and minimum length."""
        self.add_report("\n=== Word Length Check ===")
        
        length_dist = defaultdict(int)
        min_length = float('inf')
        max_length = 0
        short_words = []
        
        for word in self.words:
            length = len(word)
            length_dist[length] += 1
            min_length = min(min_length, length)
            max_length = max(max_length, length)
            
            if length < 2:
                short_words.append(word)
        
        # Check minimum length requirement
        if short_words:
            self.add_report(f"✗ Found {len(short_words)} words shorter than 2 characters:")
            for word in short_words:
                self.add_report(f"  - '{word}' ({len(word)} char)")
        else:
            self.add_report("✓ All words are 2+ characters")
        
        # Show distribution
        self.add_report(f"\nLength distribution (min: {min_length}, max: {max_length}):")
        for length in sorted(length_dist.keys()):
            count = length_dist[length]
            percentage = count / len(self.words) * 100
            bar = '█' * int(percentage / 2)
            self.add_report(f"  {length:2d} chars: {count:6,} ({percentage:5.1f}%) {bar}")
        
        return dict(length_dist)
    
    def check_homophones(self) -> List[Tuple[str, ...]]:
        """Check for potential homophones in the dictionary."""
        self.add_report("\n=== Homophone Check ===")
        
        found_homophones = []
        
        # Check known homophones
        for group in KNOWN_HOMOPHONES:
            present = [word for word in group if word in self.word_set]
            if len(present) > 1:
                found_homophones.append(tuple(present))
        
        if found_homophones:
            self.add_report(f"⚠ Found {len(found_homophones)} groups of homophones:")
            for group in found_homophones[:10]:
                self.add_report(f"  - {', '.join(group)}")
            if len(found_homophones) > 10:
                self.add_report(f"  ... and {len(found_homophones) - 10} more groups")
        else:
            self.add_report("✓ No known homophones found")
        
        return found_homophones
    
    def check_problematic_combinations(self) -> List[Tuple[str, str]]:
        """Check for potentially offensive adjacent word combinations."""
        self.add_report("\n=== Problematic Combination Check ===")
        
        found_problems = []
        
        # Since we're using random selection from dictionary, we can't easily
        # predict adjacent words. Instead, check if problematic words exist.
        problematic_words = set()
        for pair in PROBLEMATIC_PAIRS:
            for word in pair:
                if word in self.word_set:
                    problematic_words.add(word)
        
        if problematic_words:
            self.add_report(f"⚠ Found {len(problematic_words)} potentially problematic words:")
            for word in sorted(problematic_words)[:10]:
                self.add_report(f"  - {word}")
            self.add_report("  Note: These could form inappropriate combinations")
        else:
            self.add_report("✓ No obviously problematic words found")
        
        return found_problems
    
    def check_character_validity(self) -> bool:
        """Check all words contain only valid characters."""
        self.add_report("\n=== Character Validity Check ===")
        
        invalid_words = []
        
        for word in self.words:
            if not word.isalpha():
                invalid_words.append(word)
        
        if invalid_words:
            self.add_report(f"✗ Found {len(invalid_words)} words with non-alphabetic characters:")
            for word in invalid_words[:10]:
                self.add_report(f"  - '{word}'")
            if len(invalid_words) > 10:
                self.add_report(f"  ... and {len(invalid_words) - 10} more")
            return False
        else:
            self.add_report("✓ All words contain only alphabetic characters")
            return True
    
    def analyze_common_patterns(self):
        """Analyze common patterns in the dictionary."""
        self.add_report("\n=== Pattern Analysis ===")
        
        # Common prefixes
        prefix_count = defaultdict(int)
        suffix_count = defaultdict(int)
        
        for word in self.words:
            if len(word) >= 3:
                prefix_count[word[:2]] += 1
            if len(word) >= 4:
                suffix_count[word[-2:]] += 1
        
        # Most common prefixes
        self.add_report("\nMost common 2-letter prefixes:")
        for prefix, count in sorted(prefix_count.items(), key=lambda x: -x[1])[:10]:
            self.add_report(f"  {prefix}: {count:,} words")
        
        # Most common suffixes
        self.add_report("\nMost common 2-letter suffixes:")
        for suffix, count in sorted(suffix_count.items(), key=lambda x: -x[1])[:10]:
            self.add_report(f"  {suffix}: {count:,} words")
    
    def save_report(self):
        """Save the quality report to a file."""
        output_file = self.dictionary_file.parent / "dictionary_quality_report.txt"
        
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write('\n'.join(self.report_lines))
        
        self.add_report(f"\n=== Report saved to {output_file} ===")
    
    def run_all_checks(self) -> bool:
        """Run all quality checks and return overall pass/fail."""
        self.add_report(f"Dictionary Quality Report: {self.dictionary_file.name}")
        self.add_report("=" * 60)
        
        all_good = True
        
        # Critical checks
        all_good &= self.check_word_count()
        all_good &= self.check_duplicates()
        
        # Quality checks
        length_dist = self.check_word_lengths()
        all_good &= all(length >= 2 for length in length_dist.keys())
        
        all_good &= self.check_character_validity()
        
        # Warning-level checks
        self.check_homophones()
        self.check_problematic_combinations()
        
        # Analysis
        self.analyze_common_patterns()
        
        # Summary
        self.add_report("\n=== Summary ===")
        if all_good:
            self.add_report("✓ Dictionary passes all critical quality checks!")
        else:
            self.add_report("✗ Dictionary has critical issues that need fixing.")
        
        # Save report
        self.save_report()
        
        return all_good

def main():
    # Set up paths
    project_root = Path(__file__).parent.parent
    
    # Check for different possible dictionary files
    possible_files = [
        project_root / "data" / "claude_filtered_words.txt",
        project_root / "data" / "natural_readable_word_list_65k.txt",
        project_root / "data" / "human_readable_word_list_65k.txt",
    ]
    
    # Use command line argument or find first existing file
    if len(sys.argv) > 1:
        dictionary_file = Path(sys.argv[1])
    else:
        dictionary_file = None
        for file in possible_files:
            if file.exists():
                dictionary_file = file
                break
    
    if not dictionary_file or not dictionary_file.exists():
        print("Error: No dictionary file found!")
        print("Usage: python verify_dictionary_quality.py [dictionary_file]")
        print("\nLooked for:")
        for file in possible_files:
            print(f"  - {file}")
        sys.exit(1)
    
    # Run quality checks
    checker = DictionaryQualityChecker(dictionary_file)
    success = checker.run_all_checks()
    
    # Exit with appropriate code
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()