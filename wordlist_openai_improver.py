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
Word-List Refinement Pipeline – v1.2
===================================

*Fixes placeholder problem & purges banned words*

v1.1 sometimes emitted `_missing_*` placeholders when the model rejected a
word but did not supply a valid alternative.  v1.2 fills those gaps with
*real* substitutes chosen from a high-frequency word corpus (`wordfreq`).

Key additions
-------------
1. **Post-pass filler** – after the main loop we scan for any placeholders
   and replace them automatically with the most frequent valid English
   words not already in the list.
2. **Banned word filter** – an explicit list of profanities/slurs that are
   rejected *before* even hitting the model, so they cannot appear in the
   first place.
3. **Cleaner logging** – reasons for placeholder substitution recorded.

No change to CLI usage.
"""

from __future__ import annotations

import argparse
import csv
import json
import os
import re
import sys
import time
from pathlib import Path
from typing import Iterable, List, Sequence, Set

import openai
from bloom_filter import BloomFilter  # pip install bloom-filter
from dotenv import load_dotenv  # pip install python-dotenv
from tqdm import tqdm  # pip install tqdm
from wordfreq import zipf_frequency, top_n_list  # pip install wordfreq

###############################################################################
# Parameter handling
###############################################################################

def parse_cli(argv: Sequence[str] | None = None) -> argparse.Namespace:
    p = argparse.ArgumentParser(prog="wordlist_pipeline")
    p.add_argument("--input", required=True, type=Path)
    p.add_argument("--output", required=True, type=Path)
    p.add_argument("--log", default=Path("changes.csv"), type=Path)
    p.add_argument("--model", default="gpt-4o-mini")
    p.add_argument("--batch-size", type=int, default=500)
    p.add_argument("--freq-threshold", type=float, default=3.5)
    p.add_argument("--temp", type=float, default=0.0)
    p.add_argument("--max-retries", type=int, default=5)
    return p.parse_args(argv)

###############################################################################
# Helpers
###############################################################################

RE_NON_ALPHA = re.compile(r"[^a-zA-Z]")
PLACEHOLDER_RE = re.compile(r"^_missing_[a-z]+_$")

# Minimal profanity list – extend as needed
BANNED: Set[str] = {
    "cunt", "dammit", "damn", "damned", "dick", "dicks",
    "shit", "shits", "fucker", "fucking", "fuck", "fucks",
    "asshole", "twat", "bastard", "bollocks", "bugger",
}


def obvious_bad(word: str, freq_threshold: float) -> bool:
    return (
        word in BANNED
        or bool(RE_NON_ALPHA.search(word))
        or word.isupper()
        or zipf_frequency(word, "en") < freq_threshold
    )


def chunked(seq: Sequence[str], n: int) -> Iterable[List[str]]:
    for i in range(0, len(seq), n):
        yield list(seq[i : i + n])

###############################################################################
# OpenAI schema
###############################################################################

FUNCTION_SCHEMA = {
    "name": "assess_words",
    "description": "Assess each word against seven validation rules.",
    "parameters": {
        "type": "object",
        "properties": {
            "results": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "word": {"type": "string"},
                        "keep": {"type": "boolean"},
                        "reason": {"type": "string"},
                        "replacements": {
                            "type": "array",
                            "items": {"type": "string"},
                            "maxItems": 3,
                        },
                    },
                    "required": ["word", "keep"],
                },
            }
        },
        "required": ["results"],
    },
}

SYSTEM_MSG = (
    "You are validating an English word list for a broad UK audience."
    " Strictly apply the seven rules (real, readable, common, appropriate,"
    "  no proper nouns, no abbreviations, no foreign words lacking adoption)."
    " Return JSON matching the function schema."
)

###############################################################################
# Main routine
###############################################################################

def refine_wordlist(opts: argparse.Namespace) -> None:
    load_dotenv()
    if not (openai.api_key or os.getenv("OPENAI_API_KEY")):
        sys.exit("❌  Set OPENAI_API_KEY in environment or .env file.")

    words = [w.strip().lower() for w in opts.input.read_text().splitlines() if w.strip()]
    if len(words) != 65_536:
        sys.exit(f"❌  Input list needs 65 536 words, found {len(words)}.")

    bloom = BloomFilter(max_elements=131_072, error_rate=1e-4)
    for w in words:
        bloom.add(w)
    wordset = set(words)

    # Remove any banned words immediately, placeholder until we refill
    placeholders: List[str] = []
    for bw in list(wordset & BANNED):
        words.remove(bw)
        wordset.remove(bw)
        ph = f"_missing_{bw}_"
        words.append(ph)
        wordset.add(ph)
        bloom.add(ph)
        placeholders.append(ph)

    candidates = [w for w in words if not obvious_bad(w, opts.freq_threshold) and not PLACEHOLDER_RE.match(w)]

    opts.log.parent.mkdir(parents=True, exist_ok=True)
    with opts.log.open("w", newline="", encoding="utf-8") as log_fh:
        writer = csv.writer(log_fh)
        writer.writerow(["original", "replacement", "reason"])

        pbar = tqdm(total=len(candidates), desc="Validating", unit="words")

        for batch in chunked(candidates, opts.batch_size):
            # Retry loop for OpenAI call
            for attempt in range(1, opts.max_retries + 1):
                try:
                    resp = openai.ChatCompletion.create(
                        model=opts.model,
                        temperature=opts.temp,
                        messages=[
                            {"role": "system", "content": SYSTEM_MSG},
                            {"role": "user", "content": ", ".join(batch)},
                        ],
                        functions=[FUNCTION_SCHEMA],
                        function_call={"name": "assess_words"},
                    )
                    break
                except openai.error.OpenAIError as e:
                    wait = 2 ** attempt
                    print(f"⚠️  API error: {e} – retry {attempt}/{opts.max_retries} in {wait}s", file=sys.stderr)
                    time.sleep(wait)
            else:
                sys.exit("❌  Too many consecutive OpenAI errors; aborting.")

            try:
                payload = json.loads(resp.choices[0].message.function_call.arguments)
            except (AttributeError, json.JSONDecodeError):
                print("⚠️  Unexpected response format; skipping batch.", file=sys.stderr)
                pbar.update(len(batch))
                continue

            for rec in payload.get("results", []):
                word = rec.get("word", "").lower()
                if word not in wordset:
                    continue
                if rec.get("keep", True):
                    pbar.update(1)
                    continue

                # Remove invalid word
                words.remove(word)
                wordset.remove(word)

                # Attempt to use suggested replacements
                reason = rec.get("reason", "")
                replacement_done = False
                for alt in map(str.lower, rec.get("replacements", [])):
                    if alt and alt.isalpha() and alt not in bloom and not obvious_bad(alt, opts.freq_threshold):
                        words.append(alt)
                        wordset.add(alt)
                        bloom.add(alt)
                        writer.writerow([word, alt, reason])
                        replacement_done = True
                        break

                if not replacement_done:
                    ph = f"_missing_{word}_"
                    words.append(ph)
                    wordset.add(ph)
                    bloom.add(ph)
                    placeholders.append(ph)
                    writer.writerow([word, ph, reason + " (placeholder)"])

                pbar.update(1)

        pbar.close()

    # ---------------
    # Fill placeholders with high-frequency safe words
    # ---------------
    if placeholders:
        common_pool = top_n_list("en", 50000)  # ordered by frequency
        pool_iter = (w for w in common_pool if w.isalpha())
        replacements_made = 0
        for idx, w in enumerate(words):
            if PLACEHOLDER_RE.match(w):
                # Find next suitable candidate
                for candidate in pool_iter:
                    if candidate not in bloom and not obvious_bad(candidate, opts.freq_threshold):
                        words[idx] = candidate
                        bloom.add(candidate)
                        replacements_made += 1
                        break
        print(f"🔄  Filled {replacements_made} placeholders with common words.")

    if len(words) != 65_536:
        sys.exit("❌  Length drifted – investigate.")

    opts.output.write_text("\n".join(sorted(words)) + "\n", encoding="utf-8")
    print(f"✅  Completed. Output written to {opts.output} (65 536 words)")

###############################################################################

if __name__ == "__main__":
    refine_wordlist(parse_cli())

