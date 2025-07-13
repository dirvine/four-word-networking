# Dictionary Analysis Scripts

This directory contains Python scripts for analyzing the GOLD_WORDLIST.txt dictionary and understanding the security properties of the three-word encoding system.

## Scripts Overview

### 1. Word Length Distribution Analysis
**File:** [`word_length_distribution.py`](word_length_distribution.py)

Analyzes the distribution of words by character length and calculates entropy for each length group.

```bash
python3 word_length_distribution.py
```

**Output:** Detailed table showing word count, percentage, and entropy for each word length.

### 2. Visual Distribution Histogram  
**File:** [`visualization_histogram.py`](visualization_histogram.py)

Creates a visual ASCII histogram of the word length distribution.

```bash
python3 visualization_histogram.py
```

**Output:** ASCII bar chart showing the distribution curve and efficiency metrics.

### 3. Character Pattern Analysis
**File:** [`character_analysis.py`](character_analysis.py)

Analyzes character frequencies, linguistic patterns, and voice-friendliness properties.

```bash
python3 character_analysis.py
```

**Output:** Character frequency analysis, common patterns, and linguistic quality metrics.

### 4. Security Comparison
**File:** [`security_comparison.py`](security_comparison.py)

Compares three-word address entropy against traditional password schemes.

```bash
python3 security_comparison.py
```

**Output:** Security comparison tables and time-to-crack estimates.

### 5. Comprehensive Summary
**File:** [`comprehensive_summary.py`](comprehensive_summary.py)

Generates a complete summary table with all key metrics.

```bash
python3 comprehensive_summary.py
```

**Output:** Master table with length distribution, entropy, and voice-friendliness scores.

## Running All Analyses

You can run all analyses sequentially with:

```bash
# Run from the analysis directory
cd analysis

echo "=== Word Length Distribution ===" && python3 word_length_distribution.py
echo -e "\n=== Visual Histogram ===" && python3 visualization_histogram.py  
echo -e "\n=== Character Analysis ===" && python3 character_analysis.py
echo -e "\n=== Security Comparison ===" && python3 security_comparison.py
echo -e "\n=== Comprehensive Summary ===" && python3 comprehensive_summary.py
```

Or create a simple runner script:

```bash
#!/bin/bash
# run_all_analyses.sh

scripts=(
    "word_length_distribution.py"
    "visualization_histogram.py" 
    "character_analysis.py"
    "security_comparison.py"
    "comprehensive_summary.py"
)

for script in "${scripts[@]}"; do
    echo "===================="
    echo "Running $script"
    echo "===================="
    python3 "$script"
    echo -e "\n\n"
done
```

## Requirements

- **Python 3.6+**
- **GOLD_WORDLIST.txt** in the parent directory
- No external dependencies (uses only Python standard library)

## Understanding the Output

### Key Metrics Explained

- **Entropy (bits)**: log₂(number of possibilities) - higher is more secure
- **Voice Score**: Subjective rating of how easy words are to communicate verbally
- **Bits/Char**: Entropy density - how much security per character typed
- **Character-level entropy**: Diversity of the alphabet used

### Security Context

The analysis shows that three words provide **48 bits of entropy**, which is:
- Equivalent to an 8-character password with uppercase, lowercase, digits, and symbols
- Stronger than most user-chosen passwords (typically 30-40 bits)
- Sufficient for moderate security applications
- Far more memorable than equivalent traditional passwords

### Voice-Friendliness

Words are categorized by their suitability for voice communication:
- **Excellent (4-7 chars)**: Easy to pronounce and remember
- **Good (8-9 chars)**: Acceptable for voice use
- **Fair (2-3, 10-11 chars)**: Usable but not ideal
- **Poor (1, 12+ chars)**: Difficult for voice communication

## Customizing the Analysis

### Analyzing Different Dictionaries

To analyze a different wordlist, modify the file path in each script:

```python
# Change this line in each script:
with open('../GOLD_WORDLIST.txt', 'r') as f:
# To:
with open('../your_wordlist.txt', 'r') as f:
```

### Adding New Metrics

To add custom analysis, you can extend any script. For example, to analyze words starting with specific letters:

```python
# Add to character_analysis.py
print("\nWords by starting letter:")
starting_letters = Counter(word[0] for word in words if word)
for letter, count in starting_letters.most_common(5):
    print(f"  '{letter}': {count} words")
```

### Security Model Customization

To test different security scenarios, modify the crack speeds in `security_comparison.py`:

```python
crack_speeds = [
    ("Your custom scenario", 1e8),  # 100 million/sec
    ("Different GPU model", 5e9),   # 5 billion/sec
    # Add more scenarios
]
```

## Contributing

Found an interesting pattern or want to add a new analysis? Consider:

1. Adding new linguistic metrics (syllable count, phoneme analysis)
2. Comparing against other wordlists (BIP39, EFF, etc.)
3. Adding visualization exports (CSV, JSON)
4. Creating plots with matplotlib or other libraries
5. Analyzing non-English dictionaries

See the main project's contributing guidelines for how to submit improvements.