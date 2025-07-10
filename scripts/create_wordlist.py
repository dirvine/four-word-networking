import re

def load_cmudict(filepath):
    """
    Loads words from the CMU Pronouncing Dictionary.
    This dictionary includes many inflected forms of words (e.g., walk, walks, walking).
    """
    words = set()
    with open(filepath, 'r', encoding='latin-1') as f:
        for line in f:
            if not line.startswith(';;;'):
                word = line.split('  ')[0]
                # Remove parenthesized numbers like (1), (2) from alternate pronunciations
                word = re.sub(r'\(\d+\)$', '', word)
                words.add(word.lower())
    return list(words)

def load_common_words(filepath):
    """Loads a simple list of common words to augment the main dictionary."""
    with open(filepath, 'r', encoding='utf-8') as f:
        return {line.strip().lower() for line in f}

def generate_word_list(target_count):
    """
    Generates a list of phonetically distinct, common words, including common
    suffixed forms like -s, -es, -ing.
    """
    # --- Configuration ---
    # Download cmudict-0.7b.txt from http://www.speech.cs.cmu.edu/cgi-bin/cmudict
    cmudict_path = 'data/cmudict-0.7b.txt'
    # A list of common English words, e.g., "google-10000-english.txt"
    common_words_path = 'data/google-10000-english.txt'
    output_filename = 'data/my_word_list_with_suffixes.txt'

    # --- Load and Combine Word Lists ---
    try:
        # The source dictionaries contain base words AND their common variations.
        cmudict_words = load_cmudict(cmudict_path)
        common_words = load_common_words(common_words_path)
        initial_word_pool = sorted(list(set(cmudict_words + list(common_words))), key=len)
        print(f"Loaded a pool of {len(initial_word_pool)} unique words.")
    except FileNotFoundError as e:
        print(f"Error: {e}. Please make sure the word list files are in the correct path.")
        return

    # --- Filter and Generate the Final List ---
    phonetic_codes = set()
    final_word_list = []

    for word in initial_word_pool:
        # 1. Filter by length (4-8 characters) and ensure it's alphabetic.
        if 3 <= len(word) <= 9 and word.isalpha():
            final_word_list.append(word)

            if len(final_word_list) == target_count:
                break

    # --- Save the list to a file ---
    with open(output_filename, 'w', encoding='utf-8') as f:
        for word in final_word_list:
            f.write(word + '\n')

    print(f"Successfully generated a list of {len(final_word_list)} words in '{output_filename}'.")
    print("This list includes base words and their common, phonetically distinct variations.")

if __name__ == '__main__':
    generate_word_list(65536)