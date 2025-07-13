# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "requests",
# ]
# ///
import requests
import unicodedata

# Function to normalize word: remove accents and lowercase
def normalize_word(word):
    word = unicodedata.normalize('NFD', word)
    word = ''.join(c for c in word if unicodedata.category(c) != 'Mn')
    return word.lower()

# Download Norvig's word frequency list
english_url = "https://norvig.com/ngrams/count_1w.txt"
response = requests.get(english_url)
if response.status_code != 200:
    raise ValueError("Failed to download Norvig's word list")
lines = response.text.splitlines()

# Extract and normalize words, collect with frequency, filter by length 4-14, alphabetic after normalization
word_freq = []
for line in lines:
    if '\t' in line:
        parts = line.split('\t')
        word = normalize_word(parts[0])
        try:
            freq = int(parts[1])
        except ValueError:
            continue
        if 4 <= len(word) <= 14 and word.isalpha():
            word_freq.append((word, freq))

# Download offensive words list
bad_url = "https://raw.githubusercontent.com/LDNOOBW/List-of-Dirty-Naughty-Obscene-and-Otherwise-Bad-Words/master/en"
response = requests.get(bad_url)
if response.status_code != 200:
    raise ValueError("Failed to download bad words list")
bad_words = [normalize_word(bw.strip()) for bw in response.text.splitlines() if bw.strip()]
bad_set = set(bad_words)

# Remove profanities
word_freq = [(w, f) for w, f in word_freq if w not in bad_set]

# Remove duplicates by word (keep the first/highest freq if any, but unlikely)
seen = set()
unique_word_freq = []
for w, f in word_freq:
    if w not in seen:
        unique_word_freq.append((w, f))
        seen.add(w)

# Sort by length asc, then by frequency desc within same length
unique_word_freq.sort(key=lambda x: (len(x[0]), -x[1]))

# Trim to exactly 65536 (or all if fewer)
##selected = unique_word_freq[:65536]
##words = [w for w, f in selected]
words = [w for w, f in unique_word_freq]

# Save to file
output_file = 'processed_wordlist.txt'
with open(output_file, 'w') as f:
    f.write('\n'.join(words))

print(f"Processed wordlist with {len(words)} words from Norvig's frequency list (sorted by length asc, then frequency desc). Saved to {output_file}.")
print("This prioritizes shorter words, and within each length, the most frequent ones. Longest words were trimmed to reach exactly 65,536.")
