import re

def load_word_list(filepath):
    """Loads a simple list of words from a file into a set."""
    with open(filepath, 'r', encoding='utf-8') as f:
        return {line.strip().lower() for line in f}

def load_cmudict_words(filepath):
    """Loads just the words from the CMU Pronouncing Dictionary into a set."""
    words = set()
    with open(filepath, 'r', encoding='latin-1') as f:
        for line in f:
            if not line.startswith(';;;'):
                word = line.split('  ')[0]
                word = re.sub(r'\(\d+\)$', '', word)
                words.add(word.lower())
    return words

def analyze_readability():
    """
    Analyzes the readability of the generated word list by checking against CMUdict.
    """
    generated_list_path = 'data/my_word_list_with_suffixes.txt'
    cmudict_path = 'data/cmudict-0.7b.txt'

    try:
        generated_words = load_word_list(generated_list_path)
        cmudict_words = load_cmudict_words(cmudict_path)
        print(f"Loaded {len(generated_words)} words from our generated list.")
        print(f"Loaded {len(cmudict_words)} unique words from the CMU Pronouncing Dictionary.")
    except FileNotFoundError as e:
        print(f"Error: {e}. Please make sure the word list files are in the correct path.")
        return

    # Find the intersection of the two sets
    common_words = generated_words.intersection(cmudict_words)
    
    # Calculate the percentage
    percentage_in_cmudict = (len(common_words) / len(generated_words)) * 100

    print(f"\nReadability Analysis:")
    print(f"---------------------")
    print(f"{len(common_words)} out of {len(generated_words)} words from our list were found in the CMU Pronouncing Dictionary.")
    print(f"Readability Score: {percentage_in_cmudict:.2f}%")
    
    if percentage_in_cmudict < 80:
        print("\nNote: A lower score doesn't necessarily mean the words are unreadable,")
        print("but it indicates that a significant portion are not standard dictionary words")
        print("or are very obscure.")
    else:
        print("\nThis is a high score, suggesting the majority of words are standard and pronounceable.")


if __name__ == '__main__':
    analyze_readability()
