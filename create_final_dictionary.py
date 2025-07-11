#!/usr/bin/env python3
"""
Create a final dictionary by deduplicating and filling with readable words.
"""

def main():
    # Read natural readable word list
    with open('data/natural_readable_word_list_65k.txt', 'r') as f:
        words = [line.strip() for line in f if line.strip()]
    
    print(f"Original word count: {len(words)}")
    
    # Remove duplicates while preserving order
    seen = set()
    unique_words = []
    for word in words:
        if word not in seen and word.isalpha():  # Only alphabetic words
            seen.add(word)
            unique_words.append(word)
    
    print(f"Unique words: {len(unique_words)}")
    
    # If we need more words, add variations
    if len(unique_words) < 65536:
        needed = 65536 - len(unique_words)
        print(f"Need {needed} more words")
        
        # Try to generate variations of existing words
        base_words = list(unique_words[:10000])  # Use first 10k as base
        
        # Common suffixes to try
        suffixes = ['ly', 'ful', 'less', 'ness', 'ment', 'able', 'ible', 'ish', 'ize', 'ify']
        
        for base in base_words:
            if len(unique_words) >= 65536:
                break
            for suffix in suffixes:
                candidate = base + suffix
                if candidate not in seen and len(candidate) <= 12:
                    unique_words.append(candidate)
                    seen.add(candidate)
                    if len(unique_words) >= 65536:
                        break
        
        # If still need more, add prefixes
        if len(unique_words) < 65536:
            prefixes = ['re', 'un', 'pre', 'post', 'over', 'under', 'out', 'up', 'down', 'anti']
            
            for base in base_words:
                if len(unique_words) >= 65536:
                    break
                for prefix in prefixes:
                    candidate = prefix + base
                    if candidate not in seen and len(candidate) <= 12:
                        unique_words.append(candidate)
                        seen.add(candidate)
                        if len(unique_words) >= 65536:
                            break
        
        # If still need more, use simple combinations
        if len(unique_words) < 65536:
            # Use colors + objects
            colors = ['red', 'blue', 'green', 'black', 'white', 'yellow', 'pink', 'brown', 'gray', 'orange']
            objects = ['box', 'ball', 'hat', 'bag', 'cup', 'pen', 'book', 'door', 'key', 'star']
            
            for color in colors:
                for obj in objects:
                    if len(unique_words) >= 65536:
                        break
                    candidate = color + obj
                    if candidate not in seen:
                        unique_words.append(candidate)
                        seen.add(candidate)
    
    # Ensure exactly 65,536 words
    unique_words = unique_words[:65536]
    
    # Write the final dictionary
    with open('data/human_readable_word_list_65k.txt', 'w') as f:
        f.write('\n'.join(unique_words))
    
    print(f"Final word count: {len(unique_words)}")
    print("Dictionary created successfully!")
    
    # Verify no duplicates
    final_check = len(set(unique_words))
    print(f"Unique words in final dictionary: {final_check}")
    
    # Check for non-alphabetic words
    non_alpha = [w for w in unique_words if not w.isalpha()]
    if non_alpha:
        print(f"Warning: {len(non_alpha)} non-alphabetic words found")
        print(f"First few: {non_alpha[:5]}")

if __name__ == "__main__":
    main()