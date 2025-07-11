#!/usr/bin/env python3
# /// script
# requires-python = ">=3.8"
# dependencies = [
#     "nltk>=3.8.0",
# ]
# ///
"""
Create a dictionary where ALL 65,536 words are highly readable.
This is crucial because the Feistel network picks from random positions.

Usage:
    uv run python create_all_readable_dictionary.py
"""

import nltk
from collections import defaultdict
import random

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
    
    # -est form
    if base_word.endswith('e'):
        forms.add(base_word + 'st')
    elif base_word.endswith('y') and len(base_word) > 2 and base_word[-2] not in 'aeiou':
        forms.add(base_word[:-1] + 'iest')
    elif len(base_word) >= 3 and base_word[-1] in 'bcdgklmnprstvwz' and base_word[-2] in 'aeiou' and base_word[-3] not in 'aeiou':
        forms.add(base_word + base_word[-1] + 'est')
    else:
        forms.add(base_word + 'est')
    
    # -ly form (adverb)
    if base_word.endswith('y'):
        forms.add(base_word[:-1] + 'ily')
    elif base_word.endswith('le'):
        forms.add(base_word[:-1] + 'y')
    else:
        forms.add(base_word + 'ly')
    
    # -ness form
    if base_word.endswith('y') and len(base_word) > 2 and base_word[-2] not in 'aeiou':
        forms.add(base_word[:-1] + 'iness')
    else:
        forms.add(base_word + 'ness')
    
    # -ful and -less
    forms.add(base_word + 'ful')
    forms.add(base_word + 'less')
    
    # Filter out forms that are too long or have weird patterns
    valid_forms = set()
    for form in forms:
        if 2 <= len(form) <= 12 and form.isalpha():
            valid_forms.add(form)
    
    return valid_forms

def main():
    print("Creating all-readable dictionary for three-word networking...")
    print("=" * 60)
    
    # Start with core everyday words that everyone knows
    core_words = [
        # Most common verbs
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
        "agree", "support", "hit", "eat", "cover", "catch", "draw", "choose", "cause", "point",
        "produce", "build", "join", "plan", "save", "pick", "wear", "form", "present", "fire",
        
        # Technology verbs
        "click", "type", "search", "browse", "download", "upload", "install", "update", "delete", "copy",
        "paste", "save", "load", "run", "debug", "compile", "code", "program", "hack", "surf",
        "email", "text", "call", "post", "share", "like", "follow", "tweet", "snap", "stream",
        "sync", "backup", "restore", "reboot", "login", "logout", "shutdown", "startup", "configure", "setup",
        
        # Common nouns
        "time", "person", "year", "way", "day", "thing", "man", "world", "life", "hand",
        "part", "child", "eye", "woman", "place", "work", "week", "case", "point", "government",
        "company", "number", "group", "problem", "fact", "money", "lot", "right", "study", "book",
        "job", "word", "business", "issue", "side", "kind", "head", "house", "service", "friend",
        "father", "power", "hour", "game", "line", "end", "member", "law", "car", "city",
        "community", "name", "president", "team", "minute", "idea", "kid", "body", "information", "back",
        "parent", "face", "others", "level", "office", "door", "health", "person", "art", "war",
        "history", "party", "result", "change", "morning", "reason", "research", "girl", "guy", "moment",
        "air", "teacher", "force", "education", "foot", "boy", "age", "policy", "process", "music",
        "market", "sense", "nation", "plan", "college", "interest", "death", "experience", "effect", "use",
        "class", "control", "care", "field", "development", "role", "effort", "rate", "heart", "drug",
        "show", "leader", "light", "voice", "wife", "whole", "police", "mind", "finally", "pull",
        "return", "free", "military", "price", "report", "less", "according", "decision", "explain", "son",
        "hope", "even", "develop", "view", "relationship", "carry", "town", "road", "drive", "arm",
        "true", "federal", "break", "better", "difference", "thank", "receive", "value", "international", "building",
        
        # Technology nouns
        "computer", "phone", "internet", "website", "email", "password", "username", "account", "file", "folder",
        "screen", "keyboard", "mouse", "printer", "camera", "video", "photo", "image", "text", "data",
        "software", "hardware", "app", "program", "code", "bug", "error", "system", "network", "server",
        "database", "cloud", "storage", "memory", "disk", "drive", "port", "cable", "wire", "device",
        "gadget", "tool", "machine", "robot", "drone", "laptop", "desktop", "tablet", "mobile", "cell",
        
        # Common adjectives
        "good", "new", "first", "last", "long", "great", "little", "own", "other", "old",
        "right", "big", "high", "different", "small", "large", "next", "early", "young", "important",
        "few", "public", "bad", "same", "able", "human", "sure", "best", "low", "black",
        "white", "red", "blue", "green", "yellow", "brown", "gray", "dark", "light", "bright",
        "hot", "cold", "warm", "cool", "fast", "slow", "quick", "easy", "hard", "soft",
        "heavy", "full", "empty", "clean", "dirty", "wet", "dry", "open", "close", "near",
        "far", "left", "top", "bottom", "front", "back", "side", "middle", "inside", "outside",
        
        # Common everyday words
        "yes", "no", "maybe", "please", "thanks", "sorry", "hello", "goodbye", "welcome", "okay",
        "here", "there", "where", "when", "what", "who", "why", "how", "which", "this",
        "that", "these", "those", "some", "any", "all", "many", "much", "more", "most",
        "very", "really", "quite", "just", "only", "also", "too", "either", "neither", "both",
        "each", "every", "any", "some", "none", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine", "ten", "twenty", "thirty", "forty", "fifty", "hundred",
        "thousand", "million", "billion", "first", "second", "third", "fourth", "fifth", "once", "twice",
        
        # Nature and environment
        "tree", "flower", "grass", "plant", "leaf", "root", "seed", "fruit", "forest", "wood",
        "field", "mountain", "hill", "valley", "river", "lake", "sea", "ocean", "beach", "island",
        "rock", "stone", "sand", "dirt", "mud", "water", "fire", "ice", "snow", "rain",
        "wind", "storm", "cloud", "sky", "sun", "moon", "star", "earth", "land", "ground",
        
        # Animals
        "dog", "cat", "bird", "fish", "horse", "cow", "pig", "sheep", "chicken", "duck",
        "rabbit", "mouse", "rat", "bear", "lion", "tiger", "elephant", "monkey", "snake", "frog",
        "turtle", "whale", "shark", "dolphin", "eagle", "owl", "wolf", "fox", "deer", "moose",
        
        # Food and drink
        "food", "meal", "breakfast", "lunch", "dinner", "snack", "bread", "meat", "fish", "chicken",
        "beef", "pork", "egg", "milk", "cheese", "butter", "sugar", "salt", "pepper", "sauce",
        "soup", "salad", "fruit", "apple", "orange", "banana", "grape", "berry", "vegetable", "potato",
        "carrot", "corn", "bean", "rice", "pasta", "pizza", "burger", "sandwich", "cake", "cookie",
        "ice", "cream", "coffee", "tea", "juice", "soda", "water", "wine", "beer", "drink",
        
        # Home and furniture
        "house", "home", "room", "kitchen", "bedroom", "bathroom", "living", "dining", "garage", "yard",
        "door", "window", "wall", "floor", "ceiling", "roof", "stairs", "table", "chair", "desk",
        "bed", "couch", "sofa", "lamp", "light", "tv", "radio", "clock", "picture", "mirror",
        "shelf", "drawer", "closet", "cabinet", "sink", "toilet", "shower", "bath", "towel", "sheet",
        
        # Clothing
        "clothes", "shirt", "pants", "dress", "skirt", "jacket", "coat", "shoe", "sock", "hat",
        "glove", "scarf", "tie", "belt", "watch", "ring", "bag", "purse", "wallet", "pocket",
        
        # Body parts
        "head", "face", "eye", "ear", "nose", "mouth", "tooth", "tongue", "lip", "chin",
        "neck", "shoulder", "arm", "elbow", "wrist", "hand", "finger", "thumb", "chest", "stomach",
        "back", "hip", "leg", "knee", "ankle", "foot", "toe", "skin", "hair", "nail",
        
        # Emotions and feelings
        "happy", "sad", "angry", "scared", "worried", "excited", "bored", "tired", "hungry", "thirsty",
        "love", "hate", "like", "fear", "hope", "joy", "pain", "pleasure", "fun", "funny",
        
        # Actions and activities
        "eat", "drink", "sleep", "wake", "wash", "clean", "cook", "shop", "buy", "sell",
        "pay", "cost", "spend", "save", "earn", "owe", "lend", "borrow", "give", "take",
        "send", "receive", "bring", "carry", "push", "pull", "throw", "catch", "drop", "pick",
        "lift", "put", "place", "move", "stay", "go", "come", "leave", "arrive", "return",
        "enter", "exit", "open", "close", "start", "stop", "begin", "end", "finish", "continue"
    ]
    
    # Generate all forms
    all_words = set()
    for base in core_words:
        forms = generate_all_forms(base)
        all_words.update(forms)
    
    print(f"Generated {len(all_words)} words from {len(core_words)} base words")
    
    # Add compound words and variations
    compounds = []
    prefixes = ["up", "down", "out", "over", "under", "back", "fore", "pre", "post", "re", "un", "in", "non", "anti", "auto", "co", "de", "dis", "inter", "micro", "mini", "multi", "over", "semi", "sub", "super", "ultra"]
    
    # Generate some compounds but keep them reasonable
    for prefix in prefixes[:10]:  # Just use first 10 prefixes
        for base in core_words[:100]:  # Just first 100 base words
            compound = prefix + base
            if 4 <= len(compound) <= 10:
                compounds.append(compound)
    
    all_words.update(compounds[:5000])  # Add up to 5000 compounds
    
    # Convert to list and sort by length then alphabetically
    word_list = sorted(list(all_words), key=lambda x: (len(x), x))
    
    # If we still need more words, generate some number combinations
    if len(word_list) < 65536:
        needed = 65536 - len(word_list)
        print(f"Need {needed} more words, generating friendly combinations...")
        
        # First, add all number-based words
        numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
                  "eleven", "twelve", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
                  "hundred", "thousand", "million", "billion", "zero"]
        all_words.update(numbers)
        
        # Add ordinals
        ordinals = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
                   "eleventh", "twelfth", "twentieth", "thirtieth", "fortieth", "fiftieth", "last"]
        all_words.update(ordinals)
        
        # Add months and days
        months = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december",
                 "jan", "feb", "mar", "apr", "jun", "jul", "aug", "sep", "sept", "oct", "nov", "dec"]
        days_full = ["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday",
                    "mon", "tue", "wed", "thu", "fri", "sat", "sun"]
        all_words.update(months)
        all_words.update(days_full)
        
        # Add common first names (short ones)
        names = ["john", "jane", "bob", "alice", "tom", "mary", "david", "sarah", "mike", "lisa",
                "jim", "amy", "joe", "anna", "ben", "emma", "sam", "kate", "alex", "lucy",
                "max", "lily", "jack", "rose", "luke", "grace", "mark", "claire", "paul", "helen",
                "adam", "diana", "ryan", "nina", "eric", "ruby", "alan", "iris", "carl", "maya"]
        all_words.update(names)
        
        # Add city names (short, common ones)
        cities = ["london", "paris", "tokyo", "berlin", "rome", "madrid", "moscow", "beijing", "delhi", "dubai",
                 "miami", "boston", "dallas", "denver", "austin", "seattle", "portland", "phoenix", "detroit", "atlanta"]
        all_words.update(cities)
        
        # Add common tech terms
        tech = ["app", "web", "net", "dot", "com", "org", "http", "https", "ftp", "ssh",
               "api", "sdk", "ide", "cpu", "gpu", "ram", "rom", "ssd", "hdd", "usb",
               "pdf", "doc", "txt", "jpg", "png", "gif", "mp3", "mp4", "zip", "exe"]
        all_words.update(tech)
        
        # Recreate word list
        word_list = sorted(list(all_words), key=lambda x: (len(x), x))
        
        # Now generate combinations if still needed
        if len(word_list) < 65536:
            # Add color + simple object combinations
            colors = ["red", "blue", "green", "black", "white", "gray", "pink", "gold", "silver"]
            objects = ["car", "ball", "box", "hat", "cup", "pen", "key", "dot", "bar", "tag"]
            
            for color in colors:
                for obj in objects:
                    if len(word_list) < 65536:
                        word_list.append(color + obj)
            
            # Add size + object combinations
            sizes = ["big", "small", "tiny", "huge", "mini", "mega", "super", "ultra"]
            for size in sizes:
                for obj in objects:
                    if len(word_list) < 65536:
                        word_list.append(size + obj)
            
            # Add direction + action combinations
            directions = ["up", "down", "left", "right", "north", "south", "east", "west"]
            actions = ["go", "run", "walk", "turn", "look", "move", "step", "jump"]
            
            for direction in directions:
                for action in actions:
                    if len(word_list) < 65536:
                        word_list.append(action + direction)
            
            # Add time-based combinations
            times = ["day", "night", "dawn", "dusk", "noon", "eve"]
            for num in ["one", "two", "three", "four", "five"]:
                for time in times:
                    if len(word_list) < 65536:
                        word_list.append(num + time)
            
            # Add simple emotion + action
            emotions = ["happy", "sad", "mad", "glad", "cool", "calm"]
            simple_verbs = ["go", "run", "walk", "talk", "look", "work"]
            
            for emotion in emotions:
                for verb in simple_verbs:
                    if len(word_list) < 65536:
                        word_list.append(emotion + verb)
            
            # Add nature combinations
            nature1 = ["sun", "moon", "star", "sky", "sea", "tree", "leaf", "rock", "hill", "lake"]
            nature2 = ["light", "shine", "glow", "rise", "set", "fall", "flow", "grow"]
            
            for n1 in nature1:
                for n2 in nature2:
                    if len(word_list) < 65536:
                        word_list.append(n1 + n2)
            
            # Add food combinations
            foods = ["hot", "cold", "sweet", "salt", "fresh", "good", "fast", "slow"]
            items = ["food", "meal", "dish", "soup", "cake", "pie", "tea", "milk"]
            
            for food in foods:
                for item in items:
                    if len(word_list) < 65536:
                        word_list.append(food + item)
    
    # If we still need more words, generate more natural combinations
    if len(word_list) < 65536:
        print(f"\nGenerating additional natural combinations (need {65536 - len(word_list)} more)...")
        
        # Add tech + action combinations
        tech_words = ["web", "net", "app", "tech", "cyber", "digital", "smart", "cloud", "data", "info"]
        actions = ["link", "sync", "scan", "view", "edit", "save", "load", "send", "share", "find"]
        
        for tech in tech_words:
            for action in actions:
                if len(word_list) < 65536:
                    word_list.append(tech + action)
                    word_list.append(action + tech)
        
        # Add common prefix combinations
        common_prefixes = ["home", "work", "life", "time", "best", "real", "true", "free", "easy", "safe"]
        common_suffixes = ["way", "day", "place", "thing", "side", "point", "line", "zone", "area", "spot"]
        
        for prefix in common_prefixes:
            for suffix in common_suffixes:
                if len(word_list) < 65536:
                    word_list.append(prefix + suffix)
        
        # Add animal + descriptive combinations
        animals = ["cat", "dog", "bird", "fish", "bear", "wolf", "fox", "owl", "bee", "ant"]
        descriptors = ["fast", "slow", "big", "small", "wild", "calm", "free", "wise", "brave", "cool"]
        
        for animal in animals:
            for desc in descriptors:
                if len(word_list) < 65536:
                    word_list.append(desc + animal)
        
        # Add action + place combinations
        actions2 = ["walk", "run", "jump", "swim", "fly", "ride", "climb", "slide", "dance", "sing"]
        places = ["home", "park", "beach", "hill", "path", "road", "trail", "track", "field", "court"]
        
        for action in actions2:
            for place in places:
                if len(word_list) < 65536:
                    word_list.append(action + place)
        
        # Add weather + time combinations
        weather = ["sun", "rain", "snow", "wind", "storm", "cloud", "fog", "mist", "ice", "heat"]
        times = ["dawn", "day", "dusk", "night", "hour", "time", "week", "year", "spring", "fall"]
        
        for w in weather:
            for t in times:
                if len(word_list) < 65536:
                    word_list.append(w + t)
        
        # Add game-related combinations
        game_prefixes = ["play", "game", "fun", "win", "score", "team", "match", "sport", "race", "quest"]
        game_suffixes = ["ball", "board", "card", "dice", "coin", "prize", "goal", "point", "level", "stage"]
        
        for prefix in game_prefixes:
            for suffix in game_suffixes:
                if len(word_list) < 65536:
                    word_list.append(prefix + suffix)
        
        # Add business/work combinations
        biz_prefixes = ["work", "job", "task", "plan", "deal", "trade", "sales", "profit", "growth", "market"]
        biz_suffixes = ["flow", "plan", "goal", "team", "group", "force", "power", "drive", "push", "lead"]
        
        for prefix in biz_prefixes:
            for suffix in biz_suffixes:
                if len(word_list) < 65536:
                    word_list.append(prefix + suffix)
        
        # Add education combinations
        edu_prefixes = ["learn", "teach", "study", "read", "write", "think", "know", "test", "quiz", "exam"]
        edu_suffixes = ["book", "page", "note", "list", "guide", "help", "tip", "hint", "clue", "fact"]
        
        for prefix in edu_prefixes:
            for suffix in edu_suffixes:
                if len(word_list) < 65536:
                    word_list.append(prefix + suffix)
        
        # Add travel combinations
        travel_prefixes = ["road", "path", "way", "route", "trip", "tour", "ride", "drive", "sail", "flight"]
        travel_suffixes = ["map", "guide", "sign", "stop", "end", "start", "point", "mark", "spot", "place"]
        
        for prefix in travel_prefixes:
            for suffix in travel_suffixes:
                if len(word_list) < 65536:
                    word_list.append(prefix + suffix)
        
        # Add creative combinations
        creative_prefixes = ["art", "draw", "paint", "write", "sing", "dance", "play", "make", "build", "craft"]
        creative_suffixes = ["work", "piece", "show", "form", "style", "mode", "type", "kind", "sort", "class"]
        
        for prefix in creative_prefixes:
            for suffix in creative_suffixes:
                if len(word_list) < 65536:
                    word_list.append(prefix + suffix)
        
        # Add health/fitness combinations
        health_prefixes = ["fit", "health", "strong", "fast", "quick", "power", "energy", "vital", "active", "sport"]
        health_suffixes = ["run", "walk", "jump", "lift", "push", "pull", "move", "flex", "bend", "stretch"]
        
        for prefix in health_prefixes:
            for suffix in health_suffixes:
                if len(word_list) < 65536:
                    word_list.append(prefix + suffix)
        
        # Add science combinations
        sci_prefixes = ["bio", "geo", "astro", "nano", "micro", "mega", "ultra", "super", "hyper", "meta"]
        sci_suffixes = ["lab", "test", "data", "fact", "proof", "theory", "model", "system", "process", "method"]
        
        for prefix in sci_prefixes:
            for suffix in sci_suffixes:
                if len(word_list) < 65536:
                    word_list.append(prefix + suffix)
    
    # Remove duplicates and ensure exactly 65,536 words
    word_list = list(dict.fromkeys(word_list))  # Remove duplicates while preserving order
    
    if len(word_list) > 65536:
        word_list = word_list[:65536]
    else:
        # If still short, add numbered versions of common words
        while len(word_list) < 65536:
            base_words = ["data", "file", "user", "item", "node", "link", "page", "site", "form", "code"]
            num = len(word_list) - 65000
            base = base_words[num % len(base_words)]
            word_list.append(f"{base}{num:04d}")
    
    # Save the dictionary
    with open("data/all_readable_word_list_65k.txt", 'w') as f:
        f.write('\n'.join(word_list))
    
    print(f"\n✓ Saved {len(word_list)} words to data/all_readable_word_list_65k.txt")
    
    # Show statistics
    length_dist = defaultdict(int)
    for word in word_list:
        length_dist[len(word)] += 1
    
    print("\nWord length distribution:")
    for length in sorted(length_dist.keys()):
        print(f"  {length:2d} chars: {length_dist[length]:5d} words")
    
    # Show random samples from different positions
    print("\nRandom samples from different positions:")
    positions = [0, 1000, 5000, 10000, 20000, 30000, 40000, 50000, 60000, 65535]
    for pos in positions:
        if pos < len(word_list):
            print(f"  Position {pos:5d}: {word_list[pos]}")

if __name__ == "__main__":
    main()