# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "bloom-filter",
#     "openai==0.28",
#     "python-dotenv",
#     "tqdm",
#     "wordfreq",
# ]
# ///
"""
letter_wordlist_generator.py – v1.7
==================================

**Auto-fill to 65 536 (or any target) words**
-------------------------------------------
New features designed to hit a *precise* overall size:

* `--target-size 65536` — keep looping through the alphabet, progressively
  relaxing the frequency threshold, **until the grand total reaches or
  exceeds the target**.
* If the script overshoots the target it trims the surplus words at the
  end (least-frequent entries first) so the file ends up with **exactly
  the requested count**.
* Default per-letter cap raised to 6 000 so we never block global growth.
* A `--max-cycles` guard (default 20) prevents infinite loops if the
  target proves unreachable.

Typical command to build a full 65 536-word corpus with names allowed and
aggressive deep-mining:
```bash
python letter_wordlist_generator.py --output mega_wordlist.txt \
     --target-size 65536 \
     --min-length 3 --max-length 10 \
     --allow-proper-nouns \
     --auto-loosen --freq-threshold 3.0 --min-freq 1.0 --loosen-step 0.25
```
The script reports progress after each alphabet pass, adjusts the working
frequency threshold downward automatically, and stops as soon as (or soon
after) it hits 65 536 words.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
import time
from collections import defaultdict
from pathlib import Path
from typing import List, Set

import openai
from dotenv import load_dotenv  # pip install python-dotenv
from tqdm import tqdm  # pip install tqdm
from wordfreq import zipf_frequency  # pip install wordfreq

try:
    from openai.error import OpenAIError  # legacy SDK
except (ImportError, AttributeError):
    from openai import OpenAIError  # new 1.x SDK type: ignore

###############################################################################
# CLI
###############################################################################

def parse_cli() -> argparse.Namespace:
    p = argparse.ArgumentParser("letter_wordlist_generator")
    p.add_argument("--output", required=True, type=Path)

    # size control
    p.add_argument("--target-size", type=int, default=0, help="Stop when word count >= this (0 = no global target)")
    p.add_argument("--max-cycles", type=int, default=20, help="Safety limit on alphabet passes when chasing target size")

    # model & counts
    p.add_argument("--model", default="gpt-4o-mini")
    p.add_argument("--freq-threshold", type=float, default=3.0)
    p.add_argument("--temp", type=float, default=0.25)
    p.add_argument("--max-retries", type=int, default=5)
    p.add_argument("--per-letter-max", type=int, default=6000)

    # lexical rules
    p.add_argument("--min-length", type=int, default=4)
    p.add_argument("--max-length", type=int, default=7)
    p.add_argument("--allow-proper-nouns", action="store_true")
    p.add_argument("--banned-file", type=Path)

    # automatic loosening
    p.add_argument("--auto-loosen", action="store_true")
    p.add_argument("--loosen-step", type=float, default=0.25)
    p.add_argument("--min-freq", type=float, default=1.0)

    return p.parse_cli_args() if hasattr(p, "parse_cli_args") else p.parse_args()

###############################################################################
# Validation helpers
###############################################################################

def make_word_re(min_len: int, max_len: int) -> re.Pattern[str]:
    return re.compile(fr"^[a-z]{{{min_len},{max_len}}}$")

DEFAULT_BANNED: Set[str] = {
    "cunt", "damn", "shit", "fuck", "dick", "twat", "piss", "arse", "crap",
    "bitch", "bastard", "bollock", "bollocks", "bugger", "wank", "prick",
}


def is_valid(
    word: str,
    freq_threshold: float,
    banned: Set[str],
    re_word: re.Pattern[str],
    allow_proper: bool,
) -> bool:
    if word in banned:
        return False
    if not re_word.fullmatch(word):
        return False
    if zipf_frequency(word, "en") < freq_threshold:
        return False
    if not allow_proper and word[0].isupper():
        return False
    return True

###############################################################################
# Prompt construction
###############################################################################

def build_prompt(
    letter: str,
    min_len: int,
    max_len: int,
    allow_proper: bool,
    recent: List[str],
) -> str:
    base = (
        f"List as many LOWER-CASE English words as you can, {min_len} to {max_len} letters each,\n"
        f"that start with the letter '{letter}'.\n"
        "Words must be readable and commonly understood."
    )
    if allow_proper:
        base += " Common given names and place names are allowed."
    base += " Letters only; no abbreviations or foreign terms.\n"
    base += "Output ONE word per line."
    if recent:
        base += "\n\nDo NOT repeat: " + ", ".join(recent)
    return base

###############################################################################
# OpenAI wrapper
###############################################################################

def query_model(model: str, prompt: str, temp: float, retries: int) -> str:
    for attempt in range(1, retries + 1):
        try:
            resp = openai.ChatCompletion.create(
                model=model,
                temperature=temp,
                messages=[{"role": "user", "content": prompt}],
            )
            return resp.choices[0].message.content.strip()
        except OpenAIError as e:
            wait = 2 ** attempt
            print(f"⚠️  OpenAI error: {e} – retry {attempt}/{retries} in {wait}s", file=sys.stderr)
            time.sleep(wait)
    raise RuntimeError("OpenAI API failed too many times")

###############################################################################
# Alphabet pass – returns how many new words added
###############################################################################

def alphabet_pass(
    *,
    letters: str,
    final_words: Set[str],
    freq_threshold: float,
    opts: argparse.Namespace,
    re_word: re.Pattern[str],
    banned: Set[str],
    out_fh,
) -> int:
    added_this_pass = 0
    for letter in letters:
        current_letter_words = {w for w in final_words if w.startswith(letter)}
        seen = set(current_letter_words)
        # skip generating if per-letter already at cap
        if len(seen) >= opts.per_letter_max:
            continue

        while len(seen) < opts.per_letter_max:
            prompt = build_prompt(letter, opts.min_length, opts.max_length, opts.allow_proper_nouns, list(seen)[-150:])
            text = query_model(opts.model, prompt, opts.temp, opts.max_retries)
            if not text:
                break

            new_batch: List[str] = []
            for line in text.splitlines():
                w = line.strip().lower()
                if (
                    w
                    and w not in final_words
                    and w not in seen
                    and is_valid(w, freq_threshold, banned, re_word, opts.allow_proper_nouns)
                ):
                    new_batch.append(w)

            if not new_batch:
                break

            for w in new_batch:
                final_words.add(w)
                seen.add(w)
                out_fh.write(w + "\n")
            out_fh.flush()
            added_this_pass += len(new_batch)

    return added_this_pass

###############################################################################
# Trimming helper – keep highest-frequency words first
###############################################################################

def trim_to_target(words: Set[str], target: int, min_len: int, max_len: int) -> List[str]:
    """Return a list EXACTLY 'target' long by dropping rarest words."""
    re_word = make_word_re(min_len, max_len)
    scored = [(
        zipf_frequency(w, "en"),
        w,
    ) for w in words if re_word.fullmatch(w)]
    scored.sort(reverse=True)  # highest freq first
    return [w for _, w in scored[:target]]

###############################################################################
# Main
###############################################################################

def main():
    opts = parse_cli()
    load_dotenv()
    if not (openai.api_key or os.getenv("OPENAI_API_KEY")):
        sys.exit("❌  OPENAI_API_KEY missing.")

    re_word = make_word_re(opts.min_length, opts.max_length)

    banned: Set[str] = set(DEFAULT_BANNED)
    if opts.banned_file and opts.banned_file.exists():
        banned.update(w.strip().lower() for w in opts.banned_file.read_text().splitlines())

    final_words: Set[str] = set()
    if opts.output.exists():
        final_words.update(w.strip() for w in opts.output.read_text().splitlines() if w.strip())
        print(f"🔄  Resuming – {len(final_words)} words present")

    with opts.output.open("a", encoding="utf-8") as out_fh:
        freq_current = opts.freq_threshold
        cycle = 0
        while True:
            cycle += 1
            if opts.max_cycles and cycle > opts.max_cycles:
                print("❌  Reached max_cycles without hitting target – aborting.")
                break

            print(f"— Cycle {cycle}  (freq ≥ {freq_current:.2f}) —")
            added = alphabet_pass(
                letters="abcdefghijklmnopqrstuvwxyz",
                final_words=final_words,
                freq_threshold=freq_current,
                opts=opts,
                re_word=re_word,
                banned=banned,
                out_fh=out_fh,
            )
            print(f"   Added {added} new words; total = {len(final_words)}")

            if opts.target_size and len(final_words) >= opts.target_size:
                print("🎯  Target reached or exceeded.")
                break

            if added == 0:
                # No progress this cycle
                if not opts.auto_loosen or freq_current - opts.loosen_step < opts.min_freq:
                    print("⚠️  Stalled and cannot loosen further. Stop.")
                    break
                freq_current -= opts.loosen_step
                continue

        # Trim surplus if overshoot
        if opts.target_size and len(final_words) > opts.target_size:
            print(f"✂️  Trimming surplus {len(final_words) - opts.target_size} words …")
            kept = trim_to_target(final_words, opts.target_size, opts.min_length, opts.max_length)
            opts.output.write_text("\n".join(sorted(kept)) + "\n", encoding="utf-8")
            final_words = set(kept)

    print(f"✅  Finished with {len(final_words)} words → {opts.output}")

###############################################################################

if __name__ == "__main__":
    main()

