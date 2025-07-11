#!/usr/bin/env python3
"""
Fix duplicate words in the dictionary while maintaining exactly 65,536 words.
"""

def main():
    # Read current dictionary
    with open('data/human_readable_word_list_65k.txt', 'r') as f:
        words = [line.strip() for line in f if line.strip()]
    
    print(f"Original word count: {len(words)}")
    
    # Remove duplicates while preserving order
    seen = set()
    unique_words = []
    for word in words:
        if word not in seen:
            seen.add(word)
            unique_words.append(word)
    
    print(f"Unique words: {len(unique_words)}")
    print(f"Duplicates removed: {len(words) - len(unique_words)}")
    
    # If we need more words, add simple ones
    if len(unique_words) < 65536:
        print(f"Need {65536 - len(unique_words)} more words")
        
        # Add simple numbered words
        idx = 0
        while len(unique_words) < 65536:
            new_word = f"word{idx:05d}"
            if new_word not in seen:
                unique_words.append(new_word)
                seen.add(new_word)
            idx += 1
    
    # Ensure exactly 65,536 words
    unique_words = unique_words[:65536]
    
    # Write back
    with open('data/human_readable_word_list_65k.txt', 'w') as f:
        f.write('\n'.join(unique_words))
    
    print(f"Final word count: {len(unique_words)}")
    print("Dictionary fixed!")

if __name__ == "__main__":
    main()
