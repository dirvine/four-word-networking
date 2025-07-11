#!/usr/bin/env python3
"""Validate dictionary meets all requirements."""

from pathlib import Path

data_dir = Path(__file__).parent.parent / "data"
dict_file = data_dir / "human_readable_word_list_65k.txt"

with open(dict_file, 'r', encoding='utf-8') as f:
    words = [line.strip() for line in f]

print(f'Total words: {len(words)}')

# Check for non-ASCII
non_ascii = [w for w in words if any(ord(c) > 127 for c in w)]
print(f'Non-ASCII words: {len(non_ascii)}')

# Check word lengths
short_words = [w for w in words if len(w) < 2]
print(f'Words shorter than 2 chars: {len(short_words)}')

# Check for non-alphabetic
non_alpha = [w for w in words if not w.isalpha()]
print(f'Non-alphabetic words: {len(non_alpha)}')

# Check for uppercase
uppercase = [w for w in words if w != w.lower()]
print(f'Words with uppercase: {len(uppercase)}')

print('\nDictionary validation: ' + ('PASSED' if len(non_ascii) == 0 and len(short_words) == 0 and len(non_alpha) == 0 and len(uppercase) == 0 else 'FAILED'))