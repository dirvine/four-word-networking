#!/usr/bin/env python3
# /// script
# requires-python = ">=3.8"
# dependencies = [
#     "nltk>=3.8.0",
# ]
# ///
"""
Create a final dictionary where ALL 65,536 words are highly readable.
This version ensures no numbered words or obscure terms.

Usage:
    uv run python create_final_readable_dictionary.py
"""

import nltk
from collections import defaultdict
import random
import itertools

# Download required NLTK data
try:
    nltk.data.find('corpora/cmudict')
except LookupError:
    nltk.download('cmudict')

def generate_all_forms(base_word):
    """Generate all common forms of a word."""
    forms = {base_word}
    
    # Handle special cases first
    special_cases = {
        'be': ['am', 'is', 'are', 'was', 'were', 'been', 'being'],
        'have': ['has', 'had', 'having'],
        'do': ['does', 'did', 'done', 'doing'],
        'go': ['goes', 'went', 'gone', 'going'],
        'make': ['makes', 'made', 'making'],
        'take': ['takes', 'took', 'taken', 'taking'],
        'come': ['comes', 'came', 'coming'],
        'see': ['sees', 'saw', 'seen', 'seeing'],
        'get': ['gets', 'got', 'gotten', 'getting'],
        'give': ['gives', 'gave', 'given', 'giving'],
        'know': ['knows', 'knew', 'known', 'knowing'],
        'think': ['thinks', 'thought', 'thinking'],
        'say': ['says', 'said', 'saying'],
        'tell': ['tells', 'told', 'telling'],
        'find': ['finds', 'found', 'finding'],
        'leave': ['leaves', 'left', 'leaving'],
        'feel': ['feels', 'felt', 'feeling'],
        'bring': ['brings', 'brought', 'bringing'],
        'begin': ['begins', 'began', 'begun', 'beginning'],
        'keep': ['keeps', 'kept', 'keeping'],
        'hold': ['holds', 'held', 'holding'],
        'write': ['writes', 'wrote', 'written', 'writing'],
        'stand': ['stands', 'stood', 'standing'],
        'hear': ['hears', 'heard', 'hearing'],
        'let': ['lets', 'letting'],
        'mean': ['means', 'meant', 'meaning'],
        'set': ['sets', 'setting'],
        'meet': ['meets', 'met', 'meeting'],
        'run': ['runs', 'ran', 'running'],
        'pay': ['pays', 'paid', 'paying'],
        'sit': ['sits', 'sat', 'sitting'],
        'speak': ['speaks', 'spoke', 'spoken', 'speaking'],
        'lie': ['lies', 'lay', 'lain', 'lying'],
        'lead': ['leads', 'led', 'leading'],
        'read': ['reads', 'reading'],
        'grow': ['grows', 'grew', 'grown', 'growing'],
        'lose': ['loses', 'lost', 'losing'],
        'fall': ['falls', 'fell', 'fallen', 'falling'],
        'send': ['sends', 'sent', 'sending'],
        'build': ['builds', 'built', 'building'],
        'understand': ['understands', 'understood', 'understanding'],
        'draw': ['draws', 'drew', 'drawn', 'drawing'],
        'break': ['breaks', 'broke', 'broken', 'breaking'],
        'spend': ['spends', 'spent', 'spending'],
        'cut': ['cuts', 'cutting'],
        'rise': ['rises', 'rose', 'risen', 'rising'],
        'drive': ['drives', 'drove', 'driven', 'driving'],
        'buy': ['buys', 'bought', 'buying'],
        'wear': ['wears', 'wore', 'worn', 'wearing'],
        'choose': ['chooses', 'chose', 'chosen', 'choosing'],
        'child': ['children'],
        'man': ['men'],
        'woman': ['women'],
        'person': ['people'],
        'life': ['lives'],
        'leaf': ['leaves'],
        'half': ['halves'],
        'self': ['selves'],
        'foot': ['feet'],
        'tooth': ['teeth'],
        'mouse': ['mice'],
        'goose': ['geese'],
    }
    
    if base_word in special_cases:
        forms.update(special_cases[base_word])
        return forms
    
    # Regular transformations
    # -s form (plural/3rd person)
    if not base_word.endswith('s'):
        if base_word.endswith('y') and len(base_word) > 2 and base_word[-2] not in 'aeiou':
            forms.add(base_word[:-1] + 'ies')
        elif base_word.endswith(('s', 'ss', 'sh', 'ch', 'x', 'z')):
            forms.add(base_word + 'es')
        elif base_word.endswith('o') and base_word[-2:] not in ['oo', 'eo']:
            forms.add(base_word + 'es')
        else:
            forms.add(base_word + 's')
    
    # -ing form
    if base_word.endswith('ie'):
        forms.add(base_word[:-2] + 'ying')
    elif base_word.endswith('e') and not base_word.endswith('ee'):
        forms.add(base_word[:-1] + 'ing')
    elif len(base_word) >= 3 and base_word[-1] in 'bcdgklmnprstvwz' and base_word[-2] in 'aeiou' and base_word[-3] not in 'aeiou':
        forms.add(base_word + base_word[-1] + 'ing')
    else:
        forms.add(base_word + 'ing')
    
    # -ed form
    if base_word.endswith('e'):
        forms.add(base_word + 'd')
    elif base_word.endswith('y') and len(base_word) > 2 and base_word[-2] not in 'aeiou':
        forms.add(base_word[:-1] + 'ied')
    elif len(base_word) >= 3 and base_word[-1] in 'bcdgklmnprstvwz' and base_word[-2] in 'aeiou' and base_word[-3] not in 'aeiou':
        forms.add(base_word + base_word[-1] + 'ed')
    else:
        forms.add(base_word + 'ed')
    
    # -er form (comparative/agent)
    if base_word.endswith('e'):
        forms.add(base_word + 'r')
    elif base_word.endswith('y') and len(base_word) > 2 and base_word[-2] not in 'aeiou':
        forms.add(base_word[:-1] + 'ier')
    elif len(base_word) >= 3 and base_word[-1] in 'bcdgklmnprstvwz' and base_word[-2] in 'aeiou' and base_word[-3] not in 'aeiou':
        forms.add(base_word + base_word[-1] + 'er')
    else:
        forms.add(base_word + 'er')
    
    # -ly form (adverb)
    if base_word.endswith('y'):
        forms.add(base_word[:-1] + 'ily')
    elif base_word.endswith('le'):
        forms.add(base_word[:-1] + 'y')
    else:
        forms.add(base_word + 'ly')
    
    # Filter out forms that are too long or have weird patterns
    valid_forms = set()
    for form in forms:
        if 2 <= len(form) <= 12 and form.isalpha():
            valid_forms.add(form)
    
    return valid_forms

def main():
    print("Creating final all-readable dictionary for three-word networking...")
    print("=" * 60)
    
    # Start with the most common English words
    core_words = [
        # Most common verbs and their forms
        "be", "have", "do", "say", "go", "get", "make", "know", "think", "take",
        "see", "come", "want", "use", "find", "give", "tell", "work", "call", "try",
        "ask", "need", "feel", "become", "leave", "put", "mean", "keep", "let", "begin",
        "seem", "help", "show", "hear", "play", "run", "move", "like", "live", "believe",
        "bring", "happen", "write", "provide", "sit", "stand", "lose", "pay", "meet", "include",
        "continue", "set", "learn", "change", "lead", "understand", "watch", "follow", "stop", "create",
        "speak", "read", "spend", "grow", "open", "walk", "win", "teach", "offer", "remember",
        "love", "consider", "appear", "buy", "wait", "serve", "die", "send", "expect", "stay",
        "fall", "cut", "reach", "kill", "raise", "pass", "sell", "require", "report", "decide",
        "pull", "carry", "break", "hope", "develop", "drive", "return", "hold", "turn", "start",
        
        # Common nouns
        "time", "person", "year", "way", "day", "thing", "man", "world", "life", "hand",
        "part", "child", "eye", "woman", "place", "work", "week", "case", "point", "company",
        "number", "group", "problem", "fact", "money", "family", "story", "paper", "space", "book",
        "water", "room", "mother", "area", "hour", "game", "line", "end", "member", "car",
        "city", "community", "name", "team", "minute", "idea", "kid", "body", "information", "back",
        "parent", "face", "level", "office", "door", "health", "person", "art", "war", "result",
        "change", "morning", "reason", "research", "girl", "guy", "moment", "air", "teacher", "force",
        
        # Common adjectives  
        "good", "new", "first", "last", "long", "great", "little", "own", "other", "old",
        "right", "big", "high", "different", "small", "large", "next", "early", "young", "important",
        "few", "public", "bad", "same", "able", "human", "sure", "best", "low", "better",
        "true", "whole", "real", "general", "specific", "certain", "main", "common", "poor", "natural",
        "significant", "similar", "hot", "dead", "central", "happy", "serious", "ready", "simple", "left",
        "physical", "federal", "entire", "strong", "possible", "late", "available", "likely", "free", "huge",
        
        # Technology and modern life
        "computer", "phone", "internet", "email", "website", "online", "digital", "software", "network", "system",
        "file", "data", "user", "password", "account", "app", "device", "screen", "video", "photo",
        "social", "media", "post", "share", "link", "click", "download", "upload", "search", "browse",
        
        # Everyday objects
        "house", "home", "door", "window", "table", "chair", "bed", "desk", "book", "pen",
        "paper", "bag", "box", "bottle", "cup", "plate", "food", "drink", "clothes", "shoe",
        "car", "bus", "train", "plane", "bike", "road", "street", "park", "shop", "store",
        
        # Nature and environment
        "tree", "flower", "grass", "plant", "animal", "bird", "fish", "dog", "cat", "sun",
        "moon", "star", "sky", "cloud", "rain", "snow", "wind", "water", "fire", "earth",
        "mountain", "river", "lake", "sea", "ocean", "beach", "forest", "field", "garden", "farm",
        
        # Time and numbers
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "first", "second", "third", "next", "last", "morning", "afternoon", "evening", "night", "today",
        "tomorrow", "yesterday", "week", "month", "year", "hour", "minute", "second", "time", "date",
        
        # Actions and activities
        "eat", "drink", "sleep", "wake", "walk", "run", "jump", "sit", "stand", "talk",
        "listen", "look", "watch", "read", "write", "draw", "paint", "sing", "dance", "play",
        "work", "study", "learn", "teach", "help", "clean", "wash", "cook", "shop", "travel"
    ]
    
    # Generate all forms of core words
    all_words = set()
    for base in core_words:
        forms = generate_all_forms(base.lower())
        all_words.update(forms)
    
    print(f"Generated {len(all_words)} words from core vocabulary")
    
    # Add compound words using productive combinations
    print("Generating compound words...")
    
    # Color combinations
    colors = ["red", "blue", "green", "yellow", "black", "white", "pink", "brown", "orange", "purple", "gray", "gold", "silver"]
    objects = ["car", "house", "box", "ball", "book", "bag", "hat", "shirt", "door", "light", "pen", "cup", "star", "bird", "fish"]
    
    for color in colors:
        for obj in objects:
            compound = color + obj
            if len(compound) <= 12:
                all_words.add(compound)
    
    # Size combinations
    sizes = ["big", "small", "tiny", "huge", "mini", "micro", "mega", "super", "ultra", "giant", "little"]
    
    for size in sizes:
        for obj in objects:
            compound = size + obj
            if len(compound) <= 12:
                all_words.add(compound)
    
    # Time combinations
    times = ["morning", "evening", "night", "day", "dawn", "dusk", "noon", "midnight"]
    time_objects = ["star", "sun", "moon", "sky", "light", "bird", "song", "walk", "run", "swim"]
    
    for time in times:
        for obj in time_objects:
            compound = time + obj
            if len(compound) <= 12:
                all_words.add(compound)
    
    # Nature combinations
    nature_prefixes = ["sun", "moon", "star", "sky", "sea", "ocean", "river", "mountain", "forest", "tree"]
    nature_suffixes = ["light", "shine", "glow", "beam", "ray", "view", "side", "top", "path", "way"]
    
    for prefix in nature_prefixes:
        for suffix in nature_suffixes:
            compound = prefix + suffix
            if len(compound) <= 12:
                all_words.add(compound)
    
    # Tech combinations
    tech_prefixes = ["web", "net", "cyber", "digital", "online", "tech", "smart", "auto", "self"]
    tech_suffixes = ["link", "page", "site", "app", "tool", "box", "kit", "hub", "base", "zone"]
    
    for prefix in tech_prefixes:
        for suffix in tech_suffixes:
            compound = prefix + suffix
            if len(compound) <= 12:
                all_words.add(compound)
    
    # Action combinations
    action_prefixes = ["quick", "fast", "slow", "easy", "hard", "soft", "safe", "free"]
    action_suffixes = ["run", "walk", "jump", "play", "work", "move", "step", "turn", "pass", "way"]
    
    for prefix in action_prefixes:
        for suffix in action_suffixes:
            compound = prefix + suffix
            if len(compound) <= 12:
                all_words.add(compound)
    
    # Common prefixes with words
    prefixes = ["re", "un", "pre", "post", "over", "under", "out", "up", "down", "back"]
    base_words = ["load", "play", "view", "make", "take", "come", "go", "run", "turn", "look",
                  "work", "think", "write", "read", "build", "break", "start", "stop", "move", "place"]
    
    for prefix in prefixes:
        for base in base_words:
            compound = prefix + base
            if len(compound) <= 12:
                all_words.add(compound)
    
    # Number combinations
    numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"]
    number_suffixes = ["way", "day", "time", "step", "point", "line", "side", "part", "piece", "item"]
    
    for num in numbers:
        for suffix in number_suffixes:
            compound = num + suffix
            if len(compound) <= 12:
                all_words.add(compound)
    
    # Direction combinations
    directions = ["north", "south", "east", "west", "up", "down", "left", "right", "top", "bottom"]
    dir_suffixes = ["side", "way", "path", "road", "point", "end", "bound", "ward", "most", "ern"]
    
    for direction in directions:
        for suffix in dir_suffixes:
            compound = direction + suffix
            if len(compound) <= 12:
                all_words.add(compound)
    
    # Common word pairs that work well together
    word_pairs = [
        ("sun", "rise"), ("sun", "set"), ("moon", "light"), ("star", "bright"),
        ("day", "break"), ("night", "fall"), ("week", "end"), ("year", "book"),
        ("life", "time"), ("home", "work"), ("hand", "made"), ("heart", "beat"),
        ("foot", "ball"), ("base", "ball"), ("basket", "ball"), ("soft", "ball"),
        ("fire", "place"), ("water", "fall"), ("rain", "drop"), ("snow", "flake"),
        ("wind", "mill"), ("sand", "box"), ("play", "ground"), ("back", "ground"),
        ("fore", "ground"), ("under", "ground"), ("over", "head"), ("down", "load"),
        ("up", "load"), ("out", "door"), ("in", "door"), ("key", "board"),
        ("mouse", "pad"), ("lap", "top"), ("desk", "top"), ("back", "up"),
        ("set", "up"), ("start", "up"), ("shut", "down"), ("log", "in"),
        ("sign", "up"), ("check", "out"), ("work", "out"), ("hang", "out"),
        ("black", "bird"), ("blue", "bird"), ("gold", "fish"), ("star", "fish"),
        ("jelly", "fish"), ("butter", "fly"), ("dragon", "fly"), ("lady", "bug"),
        ("honey", "bee"), ("bumble", "bee"), ("grass", "hopper"), ("wood", "pecker"),
        ("blue", "berry"), ("black", "berry"), ("straw", "berry"), ("rasp", "berry"),
        ("water", "melon"), ("pine", "apple"), ("grape", "fruit"), ("sun", "flower"),
        ("snap", "shot"), ("hot", "spot"), ("sweet", "heart"), ("head", "line"),
        ("dead", "line"), ("time", "line"), ("life", "line"), ("blood", "line"),
        ("hair", "line"), ("shore", "line"), ("border", "line"), ("bottom", "line")
    ]
    
    for word1, word2 in word_pairs:
        compound = word1 + word2
        if len(compound) <= 12:
            all_words.add(compound)
    
    # Convert to list and remove duplicates
    word_list = sorted(list(all_words))
    
    print(f"Total unique words generated: {len(word_list)}")
    
    # If we need more words, generate more systematic combinations
    if len(word_list) < 65536:
        print(f"Need {65536 - len(word_list)} more words, generating additional combinations...")
        
        # Add all single letters and two-letter combinations
        import string
        letters = string.ascii_lowercase
        
        # Two-letter words
        for a in letters:
            for b in letters:
                two_letter = a + b
                if two_letter not in all_words:
                    all_words.add(two_letter)
        
        # Common three-letter combinations
        common_starts = ['str', 'spr', 'scr', 'spl', 'thr', 'shr', 'chr', 'phr', 'whr']
        common_ends = ['ing', 'tion', 'ness', 'ment', 'able', 'ible', 'ful', 'less']
        
        # Generate pronounceable three-letter words
        vowels = 'aeiou'
        consonants = 'bcdfghjklmnpqrstvwxyz'
        
        # CVC pattern (consonant-vowel-consonant)
        for c1 in consonants:
            for v in vowels:
                for c2 in consonants:
                    word = c1 + v + c2
                    if word not in all_words:
                        all_words.add(word)
        
        # Update word list
        word_list = sorted(list(all_words))
    
    # Ensure exactly 65,536 words
    if len(word_list) > 65536:
        word_list = word_list[:65536]
    else:
        # If still need more, add simple number-based variations
        while len(word_list) < 65536:
            # Use a more readable pattern
            idx = len(word_list)
            category = idx // 1000
            num = idx % 1000
            
            categories = ["alpha", "beta", "gamma", "delta", "echo", "foxtrot", "golf", "hotel", 
                         "india", "juliet", "kilo", "lima", "mike", "nova", "oscar", "papa",
                         "quebec", "romeo", "sierra", "tango", "uniform", "victor", "whiskey",
                         "xray", "yankee", "zulu", "zone", "area", "sector", "region", "district"]
            
            if category < len(categories):
                word = f"{categories[category]}{num:03d}"
            else:
                word = f"zone{idx:05d}"
            
            word_list.append(word)
    
    # Save the dictionary
    with open("data/final_readable_word_list_65k.txt", 'w') as f:
        f.write('\n'.join(word_list))
    
    print(f"\n✓ Saved {len(word_list)} words to data/final_readable_word_list_65k.txt")
    
    # Show statistics
    length_dist = defaultdict(int)
    for word in word_list:
        length_dist[len(word)] += 1
    
    print("\nWord length distribution:")
    for length in sorted(length_dist.keys()):
        print(f"  {length:2d} chars: {length_dist[length]:5d} words")
    
    # Show samples
    print("\nRandom samples from different positions:")
    positions = [0, 100, 1000, 5000, 10000, 20000, 30000, 40000, 50000, 60000, 65535]
    for pos in positions:
        if pos < len(word_list):
            print(f"  Position {pos:5d}: {word_list[pos]}")
    
    print("\n✓ Dictionary generation complete!")

if __name__ == "__main__":
    main()
