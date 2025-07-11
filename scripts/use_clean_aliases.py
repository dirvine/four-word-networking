#!/usr/bin/env python3
"""
Use clean_aliases.txt as the source for our 65,536 word dictionary.
"""

def main():
    # Read clean_aliases.txt
    with open('clean_aliases.txt', 'r') as f:
        words = [line.strip() for line in f if line.strip()]
    
    print(f"Total words in clean_aliases.txt: {len(words)}")
    
    # Take the first 65,536 words
    dictionary_words = words[:65536]
    
    # Verify all words are clean (alphabetic only)
    clean_words = []
    for word in dictionary_words:
        if word.isalpha() and 2 <= len(word) <= 20:  # Allow 2-20 character words
            clean_words.append(word.lower())  # Ensure lowercase
    
    print(f"Clean words: {len(clean_words)}")
    
    # If we don't have enough, add more from the remaining words
    if len(clean_words) < 65536:
        for word in words[65536:]:
            if word.isalpha() and 2 <= len(word) <= 20:
                clean_words.append(word.lower())
                if len(clean_words) >= 65536:
                    break
    
    # Ensure exactly 65,536 words
    final_words = clean_words[:65536]
    
    # Write to our dictionary file
    with open('data/human_readable_word_list_65k.txt', 'w') as f:
        f.write('\n'.join(final_words))
    
    print(f"Final dictionary size: {len(final_words)}")
    
    # Show some statistics
    length_counts = {}
    for word in final_words:
        length = len(word)
        length_counts[length] = length_counts.get(length, 0) + 1
    
    print("\nWord length distribution:")
    for length in sorted(length_counts.keys()):
        print(f"  {length:2d} chars: {length_counts[length]:5d} words")
    
    # Show sample words
    print("\nSample words from different positions:")
    positions = [0, 100, 1000, 5000, 10000, 20000, 30000, 40000, 50000, 60000, 65535]
    for pos in positions:
        if pos < len(final_words):
            print(f"  Position {pos:5d}: {final_words[pos]}")
    
    print("\n✓ Dictionary created successfully from clean_aliases.txt!")

if __name__ == "__main__":
    main()
