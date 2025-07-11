#!/usr/bin/env python3
"""
filter_human_readable_words.py
------------------------------

Filter a plain-text word list (one word per line) for “human readability”.

Criteria (all adjustable):
    • Zipf frequency ≥ 3.5  – keeps common words
    • ≤ 3 syllables         – easier to pronounce
    • No triple-consonant cluster at either end (crude tongue-twister guard)
    • ASCII letters only (no underscores, digits, accents)

Usage
-----
    python filter_human_readable_words.py aliases.txt \
           --outfile human_readable_aliases.txt \
           --min-zipf 3.5 --max-syllables 3

Dependencies
------------
    pip install pandas wordfreq pronouncing syllables

Syllable checking still works if *pronouncing* or *syllables* is missing,
but will skip the unavailable layer.

Author: ChatGPT (assistant for David Irvine)
Licence: MIT
"""
import argparse
import re
import sys
from typing import Optional

import pandas as pd

# ───────────────────────── optional imports ──────────────────────────────── #
try:
    from wordfreq import zipf_frequency        # type: ignore
except ImportError:
    sys.exit("The 'wordfreq' package is required.  Install with:\n"
             "    pip install wordfreq")

try:
    import pronouncing                         # type: ignore
except ImportError:
    pronouncing = None

try:
    import syllables                           # type: ignore
except ImportError:
    syllables = None
# ─────────────────────────────────────────────────────────────────────────── #

DEFAULT_MIN_ZIPF = 3.5       # frequency threshold
DEFAULT_MAX_SYLLABLES = 3    # pronunciation effort
_CONSONANT_CLUSTER = re.compile(r"(^[^aeiouy]{3,}|[^aeiouy]{3,}$)")


# ───────────────────────── helper functions ─────────────────────────────── #

def count_syllables(word: str) -> Optional[int]:
    """Return a syllable count, or None if we can’t measure."""
    if pronouncing is not None:
        phones = pronouncing.phones_for_word(word)
        if phones:
            return min(pronouncing.syllable_count(p) for p in phones)

    if syllables is not None:
        return syllables.estimate(word)

    return None


def easy_enough(word: str,
                *,
                min_zipf: float = DEFAULT_MIN_ZIPF,
                max_syllables: int = DEFAULT_MAX_SYLLABLES) -> bool:
    """True iff *word* passes all readability gates."""
    if not isinstance(word, str):
        return False
    word = word.strip().lower()

    # basic shape
    if not re.fullmatch(r"[a-z]+", word):
        return False

    # frequency
    if zipf_frequency(word, "en") < min_zipf:
        return False

    # syllables
    syl = count_syllables(word)
    if syl is not None and syl > max_syllables:
        return False

    # crude tongue-twister check
    if _CONSONANT_CLUSTER.search(word):
        return False

    return True


# ─────────────────────────────── main ───────────────────────────────────── #

def main() -> None:
    p = argparse.ArgumentParser(description="Filter words for human readability")
    p.add_argument("infile", help="Input text file (one word per line)")
    p.add_argument("--outfile", default="human_readable_aliases.txt",
                   help="Output filename (default: human_readable_aliases.txt)")
    p.add_argument("--min-zipf", type=float, default=DEFAULT_MIN_ZIPF,
                   help="Minimum Zipf frequency (default: 3.5)")
    p.add_argument("--max-syllables", type=int, default=DEFAULT_MAX_SYLLABLES,
                   help="Maximum syllables (default: 3)")
    args = p.parse_args()

    # load and pre-clean
    words = (pd.read_csv(args.infile, header=None, names=["word"])
               .dropna(subset=["word"])
               .assign(word=lambda df: df["word"].astype(str).str.strip()))
    words = words[words["word"].str.fullmatch(r"[A-Za-z]+", na=False)]

    # filter
    mask = words["word"].apply(easy_enough,
                               min_zipf=args.min_zipf,
                               max_syllables=args.max_syllables)
    easy_words = words.loc[mask, "word"]

    # save
    easy_words.to_csv(args.outfile, index=False, header=False)
    print(f"Kept {len(easy_words)} of {len(words)} words  →  {args.outfile}")


if __name__ == "__main__":
    main()
