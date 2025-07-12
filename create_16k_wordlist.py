#!/usr/bin/env python3
"""
Create a 16,384-word subset of the main GOLD_WORDLIST.txt.
"""

def main():
    try:
        # Read the main dictionary
        with open('GOLD_WORDLIST.txt', 'r') as f:
            words = [line.strip() for line in f if line.strip()]
        
        print(f"Read {len(words)} words from GOLD_WORDLIST.txt")

        # Take the first 16,384 words
        subset = words[:16384]
        
        if len(subset) < 16384:
            print(f"Warning: GOLD_WORDLIST.txt has fewer than 16,384 words. The subset will have {len(subset)} words.")

        # Write the subset to a new file
        with open('wordlist_16k.txt', 'w') as f:
            f.write('\n'.join(subset))
            f.write('\n') # Add trailing newline

        print(f"Successfully created wordlist_16k.txt with {len(subset)} words.")

    except FileNotFoundError:
        print("Error: GOLD_WORDLIST.txt not found. Please ensure the main dictionary exists.")

if __name__ == "__main__":
    main()
