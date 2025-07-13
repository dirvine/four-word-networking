# Four-Word Paradigm: Security and Linguistic Analysis

This document provides a comprehensive analysis of the four-word encoding system, examining its security properties, linguistic characteristics, and potential applications beyond networking.

## Table of Contents

1. [Dictionary Analysis](#dictionary-analysis)
2. [Security Comparison](#security-comparison)
3. [Linguistic Properties](#linguistic-properties)
4. [Advanced Applications](#advanced-applications)
5. [Analysis Scripts](#analysis-scripts)

## Dictionary Analysis

Our 4K dictionary contains exactly **4,096 words** (2^12), carefully curated for human usability.

### Word Length Distribution

| Length | Word Count | % of Dict | Entropy (bits) | Bits/Char | Voice Score |
|--------|------------|-----------|----------------|-----------|-------------|
|   1    |     26     |   0.04%   |      4.70      |   4.70    |    Poor     |
|   2    |    295     |   0.45%   |      8.20      |   4.10    |    Fair     |
|   3    |   2,445    |   3.73%   |     11.26      |   3.75    |    Fair     |
|   4    |   4,707    |   7.18%   |     12.20      |   3.05    |  Excellent  |
|   5    |   7,303    |   11.14%  |     12.83      |   2.57    |  Excellent  |
|   6    |   9,508    |   14.51%  |     13.21      |   2.20    |  Excellent  |
|   7    |   10,342   |   15.78%  |     13.34      |   1.91    |  Excellent  |
|   8    |   9,564    |   14.59%  |     13.22      |   1.65    |    Good     |
|   9    |   8,051    |   12.28%  |     12.97      |   1.44    |    Good     |
|   10   |   6,200    |   9.46%   |     12.60      |   1.26    |    Fair     |
|   11   |   4,198    |   6.41%   |     12.04      |   1.09    |    Fair     |
|   12   |   2,710    |   4.14%   |     11.40      |   0.95    |    Poor     |
|   13   |    132     |   0.20%   |      7.04      |   0.54    |    Poor     |
|   14   |     43     |   0.07%   |      5.43      |   0.39    |    Poor     |
|   15   |     9      |   0.01%   |      3.17      |   0.21    |    Poor     |
|   16   |     3      |   0.00%   |      1.58      |   0.10    |    Poor     |

### Visual Distribution

```
Word Length Distribution Histogram
============================================================
 1 chars:                                                    
 2 chars: █                                                 
 3 chars: ███████████                                       
 4 chars: ██████████████████████                            
 5 chars: ███████████████████████████████████               
 6 chars: █████████████████████████████████████████████     
 7 chars: ██████████████████████████████████████████████████
 8 chars: ██████████████████████████████████████████████    
 9 chars: ██████████████████████████████████████            
10 chars: █████████████████████████████                     
11 chars: ████████████████████                              
12 chars: █████████████                                     
13 chars:                                                   
14 chars:                                                   
15 chars:                                                   
16 chars:                                                   
```

### Key Metrics

- **Total dictionary size**: 4,096 words (2^12)
- **Total entropy**: 12.00 bits per word
- **Average word length**: 7.40 characters
- **Most common length**: 7 characters (15.8%)
- **Optimal length range (4-9 chars)**: 49,475 words (75.5%)

## Security Comparison

### Four-Word Address vs Traditional Passwords

Four-word addresses provide **48 bits of entropy** (4 × 12 bits), equivalent to:

| Password Type | Required Length | Example |
|--------------|-----------------|---------|
| Lowercase only (a-z) | 11 characters | xkqvbnmwert |
| Alphanumeric (a-z, 0-9) | 10 characters | p7k3m9nw2 |
| Mixed case + digits | 9 characters | Kj7mN2pQ |
| With symbols (!@#$%^&*) | 8 characters | Tr0ub&3x |
| Full ASCII printable | 8 characters | 9k#X$2p! |

### Time to Crack (50% probability)

| Attack Capability | Time to Crack |
|------------------|---------------|
| Regular computer (1M/sec) | 4.5 years |
| Gaming GPU (1B/sec) | 1.6 days |
| Professional rig (1T/sec) | 2.3 minutes |
| Nation-state (1Q/sec) | 0.1 seconds |

### Security Context

```
  Typical user password           30 bits
  'Strong' password (most sites)  40 bits
→ Four-word address               48 bits    ← We are here
  NIST 2030 minimum              112 bits
  AES-128 encryption             128 bits
  Bitcoin private key            256 bits
```

## Linguistic Properties

### Character Analysis

- **Total characters**: 484,906
- **Average per word**: 7.40
- **Character entropy**: 4.21 bits
- **Alphabet size**: 26 (lowercase only)

### Most Common Characters

1. 'e': 11.33%
2. 'a': 8.72%
3. 'i': 8.42%
4. 's': 7.54%
5. 'r': 7.30%

### Common Word Endings

- -ing: 5.84% of words
- -ion: 1.96%
- -ted: 1.74%
- -ers: 1.68%
- -ons: 1.10%

### Voice-Friendliness Analysis

- **Optimal words (4-7 chars)**: 48.6%
- **Good words (8-9 chars)**: 26.9%
- **Suboptimal (1-3 or 12+ chars)**: 8.6%
- **Vowel/Consonant ratio**: 1:1.62 (ideal for pronunciation)

## Advanced Applications

The four-word paradigm offers interesting possibilities beyond IP addresses:

### 1. Human-Readable Cryptographic Addresses

Instead of:
```
1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa  (Bitcoin address)
```

Use:
```
monkey rain bike forest  (48 bits of the address)
```

### 2. Memorable API Keys

Traditional API key:
```
api_key_ABC123XYZ789FakeExample001
```

Four-word version:
```
sunset river song mountain
```

### 3. Device Pairing Codes

Instead of entering "792514" on your TV, say:
```
"Pair with: happy green door valley"
```

### 4. Secure Temporary Passwords

Generate memorable temporary passwords:
```
ocean thunder falcon mountain (valid for 24 hours)
```

### 5. Location Sharing

Share approximate locations without revealing exact coordinates:
```
"Meet me near: castle bridge moon river"
```

### 6. Transaction References

Instead of:
```
Reference: TXN-2024-0C4F9B
```

Use:
```
Reference: paper violin stream ocean
```

## Analysis Scripts

All analysis scripts used to generate this report are available in the [`analysis/`](analysis/) directory. You can run these scripts yourself to reproduce the results or analyze different wordlists.

### Available Scripts

1. **[`word_length_distribution.py`](analysis/word_length_distribution.py)** - Word length distribution and entropy analysis
2. **[`visualization_histogram.py`](analysis/visualization_histogram.py)** - ASCII histogram visualization  
3. **[`character_analysis.py`](analysis/character_analysis.py)** - Character patterns and linguistic analysis
4. **[`security_comparison.py`](analysis/security_comparison.py)** - Security comparison with traditional passwords
5. **[`comprehensive_summary.py`](analysis/comprehensive_summary.py)** - Complete summary with all metrics

### Running the Analysis

```bash
# Clone the repository
git clone https://github.com/dirvine/three-word-networking.git
cd three-word-networking

# Run individual analyses
python3 analysis/word_length_distribution.py
python3 analysis/security_comparison.py

# Or run all analyses
cd analysis
for script in *.py; do
    echo "=== Running $script ==="
    python3 "$script"
    echo
done
```

### Requirements

- Python 3.6+
- No external dependencies (uses only standard library)
- GOLD_WORDLIST.txt in the project root

### Example Output

Running `word_length_distribution.py`:

```
Word Length Distribution Analysis
================================================================================

Length | Count  | Percentage | Entropy (bits) | Example Words
-------|--------|------------|----------------|---------------
   1   |   26   |    0.04   % |      4.70      | a, b, c, ...
   2   |   295  |    0.45   % |      8.20      | aa, ab, ac, ...
   3   |  2445  |    3.73   % |     11.26      | aaa, abc, abe, ...
   4   |  4707  |    7.18   % |     12.20      | abad, abba, abbe, ...
   5   |  7303  |   11.14   % |     12.83      | aaron, ababa, aback, ...
   ...
```

See the [`analysis/README.md`](analysis/README.md) for detailed documentation on customizing and extending the analysis scripts.

## Conclusion

The four-word paradigm offers a unique balance between security and usability. While 48 bits of entropy may not suffice for high-security cryptographic applications, it provides:

1. **Memorable security**: Equivalent to an 8-character complex password, but actually memorable
2. **Voice-friendly**: Can be communicated over phone or radio without confusion
3. **Cross-cultural**: Can be adapted to any language with a 4,096-word dictionary
4. **Error-resistant**: Spell-checkers and context make typos less critical
5. **Versatile**: Applicable to many scenarios beyond IP addresses

The system's true innovation lies not in its cryptographic strength, but in making moderate security accessible to humans without requiring password managers or complex mnemonics.