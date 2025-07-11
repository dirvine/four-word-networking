#!/usr/bin/env python3
"""
Filter word list using Claude's language understanding for readability assessment.

This script processes words in batches and uses Claude to assess:
- Readability and ease of pronunciation
- Common usage and familiarity
- Appropriateness for voice communication

The script supports multiple filtering stages with different retention rates.

Input: data/cleaned_frequency_list.txt
Output: data/claude_filtered_words.txt (and intermediate files)
"""

import json
import sys
from pathlib import Path
from typing import List, Dict, Tuple
import time

class WordFilter:
    def __init__(self, input_file: Path, output_dir: Path):
        self.input_file = input_file
        self.output_dir = output_dir
        self.output_dir.mkdir(exist_ok=True)
        
        # Load all words
        with open(input_file, 'r', encoding='utf-8') as f:
            self.all_words = [line.strip() for line in f if line.strip()]
        
        print(f"Loaded {len(self.all_words):,} words from {input_file}")
    
    def create_batches(self, words: List[str], batch_size: int = 10000) -> List[List[str]]:
        """Split words into batches for processing."""
        batches = []
        for i in range(0, len(words), batch_size):
            batches.append(words[i:i + batch_size])
        return batches
    
    def save_intermediate(self, words: List[str], stage: str, iteration: int):
        """Save intermediate results."""
        filename = self.output_dir / f"stage_{stage}_iter_{iteration}.txt"
        with open(filename, 'w', encoding='utf-8') as f:
            for word in words:
                f.write(f"{word}\n")
        print(f"  Saved {len(words):,} words to {filename}")
    
    def create_claude_prompt(self, words: List[str], filter_level: str) -> str:
        """Create appropriate prompt based on filter level."""
        prompts = {
            'light': """Please review these 10,000 words for use in a voice-based addressing system.

Mark each word as KEEP or REMOVE based on these criteria:
- REMOVE if very difficult to pronounce or spell
- REMOVE if clearly non-English
- REMOVE if potentially offensive
- KEEP all common 2-letter words (be, to, is, at, by, in, on, up, we, me, he, it)
- KEEP most other words (aim for ~90% retention)

Focus on removing only the most problematic words. When in doubt, KEEP the word.

Respond with a JSON object mapping each word to "KEEP" or "REMOVE".
Example: {"word1": "KEEP", "word2": "REMOVE", ...}

Words to review:
""",
            'medium': """Review these words for a voice-friendly addressing system.

Mark each word as KEEP or REMOVE based on:
- Clarity when spoken aloud
- Ease of understanding over phone/radio
- Common usage in English
- No homophones with other common words
- Appropriate for all audiences

KEEP all essential 2-letter words.
Aim for ~80% retention of good quality words.

Respond with a JSON object mapping each word to "KEEP" or "REMOVE".

Words to review:
""",
            'final': """Select words for a voice-based addressing system requiring exactly 65,536 words.

Requirements:
- Minimum 2 characters
- Clear pronunciation
- Common usage preferred
- No offensive terms
- No strong homophones
- Include ALL common 2-letter words: be, to, is, at, by, in, on, up, we, me, he, it, or, if, so, no, go, do

From the provided words, mark KEEP for the best candidates.

Respond with a JSON object mapping each word to "KEEP" or "REMOVE".

Words to review:
"""
        }
        
        prompt = prompts.get(filter_level, prompts['light'])
        # Add the words as a simple list
        word_list = '\n'.join(words)
        return prompt + word_list
    
    def process_batch_simulation(self, words: List[str], filter_level: str, batch_num: int) -> Dict[str, str]:
        """
        Simulate Claude's response for testing.
        In production, this would make an actual API call to Claude.
        """
        print(f"  Processing batch {batch_num} with {len(words)} words ({filter_level} filter)...")
        
        # Simulate different retention rates
        retention_rates = {
            'light': 0.90,
            'medium': 0.80,
            'final': 0.75
        }
        
        retention = retention_rates.get(filter_level, 0.85)
        results = {}
        
        # Always keep essential 2-letter words
        essential_2_letter = {'be', 'to', 'is', 'at', 'by', 'in', 'on', 'up', 'we', 'me', 'he', 'it', 'or', 'if', 'so', 'no', 'go', 'do'}
        
        for i, word in enumerate(words):
            if word in essential_2_letter:
                results[word] = "KEEP"
            elif i < len(words) * retention:
                results[word] = "KEEP"
            else:
                results[word] = "REMOVE"
        
        # Simulate processing time
        time.sleep(0.1)
        
        kept = sum(1 for v in results.values() if v == "KEEP")
        print(f"    Kept {kept:,} words ({kept/len(words)*100:.1f}%)")
        
        return results
    
    def run_filtering_stage(self, words: List[str], stage_name: str, filter_level: str) -> List[str]:
        """Run a complete filtering stage on a word list."""
        print(f"\nRunning {stage_name} stage ({filter_level} filter)...")
        print(f"  Input: {len(words):,} words")
        
        batches = self.create_batches(words)
        print(f"  Created {len(batches)} batches")
        
        kept_words = []
        
        for i, batch in enumerate(batches, 1):
            # In production, this would call Claude API
            results = self.process_batch_simulation(batch, filter_level, i)
            
            # Collect kept words
            batch_kept = [word for word in batch if results.get(word) == "KEEP"]
            kept_words.extend(batch_kept)
        
        print(f"  Output: {len(kept_words):,} words ({len(kept_words)/len(words)*100:.1f}% retained)")
        
        return kept_words
    
    def run_multi_stage_filtering(self, target_count: int = 65536):
        """Run multiple filtering stages to reach target word count."""
        current_words = self.all_words.copy()
        
        # Stage 1: Light filter
        if len(current_words) > target_count * 1.5:
            current_words = self.run_filtering_stage(current_words, "Stage 1", "light")
            self.save_intermediate(current_words, "light", 1)
        
        # Stage 2: Medium filter if needed
        if len(current_words) > target_count * 1.2:
            current_words = self.run_filtering_stage(current_words, "Stage 2", "medium")
            self.save_intermediate(current_words, "medium", 2)
        
        # Stage 3: Final selection to exact count
        if len(current_words) > target_count:
            print(f"\nFinal selection: reducing {len(current_words):,} to exactly {target_count:,} words...")
            # For final selection, we can simply take the top N words since they're frequency-ordered
            current_words = current_words[:target_count]
            self.save_intermediate(current_words, "final", 3)
        
        return current_words
    
    def save_final_output(self, words: List[str]):
        """Save the final filtered word list."""
        output_file = self.output_dir.parent / "claude_filtered_words.txt"
        with open(output_file, 'w', encoding='utf-8') as f:
            for word in words:
                f.write(f"{word}\n")
        
        print(f"\nFinal output saved to {output_file}")
        print(f"Total words: {len(words):,}")
        
        # Show word length distribution
        length_dist = {}
        for word in words:
            length = len(word)
            length_dist[length] = length_dist.get(length, 0) + 1
        
        print("\nWord length distribution:")
        for length in sorted(length_dist.keys()):
            count = length_dist[length]
            print(f"  {length:2d} chars: {count:6,} words")

def main():
    # Set up paths
    project_root = Path(__file__).parent.parent
    input_file = project_root / "data" / "cleaned_frequency_list.txt"
    output_dir = project_root / "data" / "claude_filtering"
    
    if not input_file.exists():
        print(f"Error: Input file not found: {input_file}")
        print("Please run clean_frequency_list.py first.")
        sys.exit(1)
    
    # Create filter instance
    filter = WordFilter(input_file, output_dir)
    
    # Run multi-stage filtering
    final_words = filter.run_multi_stage_filtering(target_count=65536)
    
    # Save final output
    filter.save_final_output(final_words)
    
    print("\nFiltering complete!")
    print("\nNote: This script currently simulates Claude's responses.")
    print("In production, you would need to:")
    print("1. Set up Claude API access")
    print("2. Replace process_batch_simulation() with actual API calls")
    print("3. Handle API rate limits and errors")
    print("4. Parse Claude's JSON responses")

if __name__ == "__main__":
    main()