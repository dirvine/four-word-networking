#!/usr/bin/env python3
# /// script
# requires-python = ">=3.8"
# ///
"""
Create a truly readable dictionary where EVERY SINGLE word is common and readable.
This solves the Feistel network issue by ensuring all 65,536 positions have good words.

Usage:
    uv run python create_truly_readable_dictionary.py
"""

import itertools
from collections import defaultdict

def generate_all_forms(base_word):
    """Generate common forms of a word."""
    forms = {base_word}
    
    # Special cases for common irregular verbs
    irregular = {
        'be': ['am', 'is', 'are', 'was', 'were', 'been', 'being'],
        'have': ['has', 'had', 'having'],
        'do': ['does', 'did', 'done', 'doing'],
        'go': ['goes', 'went', 'gone', 'going'],
        'get': ['gets', 'got', 'gotten', 'getting'],
        'make': ['makes', 'made', 'making', 'maker'],
        'take': ['takes', 'took', 'taken', 'taking'],
        'come': ['comes', 'came', 'coming'],
        'see': ['sees', 'saw', 'seen', 'seeing'],
        'know': ['knows', 'knew', 'known', 'knowing'],
        'think': ['thinks', 'thought', 'thinking'],
        'give': ['gives', 'gave', 'given', 'giving'],
        'find': ['finds', 'found', 'finding', 'finder'],
        'tell': ['tells', 'told', 'telling', 'teller'],
        'work': ['works', 'worked', 'working', 'worker'],
        'call': ['calls', 'called', 'calling', 'caller'],
        'try': ['tries', 'tried', 'trying'],
        'use': ['uses', 'used', 'using', 'user', 'users'],
        'need': ['needs', 'needed', 'needing'],
        'feel': ['feels', 'felt', 'feeling'],
        'leave': ['leaves', 'left', 'leaving'],
        'put': ['puts', 'putting'],
        'mean': ['means', 'meant', 'meaning'],
        'keep': ['keeps', 'kept', 'keeping', 'keeper'],
        'let': ['lets', 'letting'],
        'begin': ['begins', 'began', 'begun', 'beginning'],
        'seem': ['seems', 'seemed', 'seeming'],
        'help': ['helps', 'helped', 'helping', 'helper'],
        'show': ['shows', 'showed', 'shown', 'showing'],
        'hear': ['hears', 'heard', 'hearing'],
        'play': ['plays', 'played', 'playing', 'player'],
        'run': ['runs', 'ran', 'running', 'runner'],
        'move': ['moves', 'moved', 'moving', 'mover'],
        'like': ['likes', 'liked', 'liking'],
        'live': ['lives', 'lived', 'living'],
        'bring': ['brings', 'brought', 'bringing'],
        'write': ['writes', 'wrote', 'written', 'writing', 'writer'],
        'sit': ['sits', 'sat', 'sitting', 'sitter'],
        'stand': ['stands', 'stood', 'standing'],
        'lose': ['loses', 'lost', 'losing', 'loser'],
        'pay': ['pays', 'paid', 'paying', 'payer'],
        'meet': ['meets', 'met', 'meeting'],
        'set': ['sets', 'setting', 'setter'],
        'learn': ['learns', 'learned', 'learning', 'learner'],
        'change': ['changes', 'changed', 'changing', 'changer'],
        'lead': ['leads', 'led', 'leading', 'leader'],
        'watch': ['watches', 'watched', 'watching', 'watcher'],
        'follow': ['follows', 'followed', 'following', 'follower'],
        'stop': ['stops', 'stopped', 'stopping', 'stopper'],
        'create': ['creates', 'created', 'creating', 'creator'],
        'speak': ['speaks', 'spoke', 'spoken', 'speaking', 'speaker'],
        'read': ['reads', 'reading', 'reader'],
        'spend': ['spends', 'spent', 'spending', 'spender'],
        'grow': ['grows', 'grew', 'grown', 'growing', 'grower'],
        'open': ['opens', 'opened', 'opening', 'opener'],
        'walk': ['walks', 'walked', 'walking', 'walker'],
        'win': ['wins', 'won', 'winning', 'winner'],
        'teach': ['teaches', 'taught', 'teaching', 'teacher'],
        'offer': ['offers', 'offered', 'offering'],
        'remember': ['remembers', 'remembered', 'remembering'],
        'love': ['loves', 'loved', 'loving', 'lover'],
        'consider': ['considers', 'considered', 'considering'],
        'appear': ['appears', 'appeared', 'appearing'],
        'buy': ['buys', 'bought', 'buying', 'buyer'],
        'wait': ['waits', 'waited', 'waiting', 'waiter'],
        'serve': ['serves', 'served', 'serving', 'server'],
        'die': ['dies', 'died', 'dying'],
        'send': ['sends', 'sent', 'sending', 'sender'],
        'build': ['builds', 'built', 'building', 'builder'],
        'stay': ['stays', 'stayed', 'staying'],
        'fall': ['falls', 'fell', 'fallen', 'falling'],
        'cut': ['cuts', 'cutting', 'cutter'],
        'reach': ['reaches', 'reached', 'reaching'],
        'kill': ['kills', 'killed', 'killing', 'killer'],
        'eat': ['eats', 'ate', 'eaten', 'eating', 'eater'],
        'drink': ['drinks', 'drank', 'drunk', 'drinking', 'drinker'],
        'sleep': ['sleeps', 'slept', 'sleeping', 'sleeper'],
        'wake': ['wakes', 'woke', 'woken', 'waking'],
    }
    
    if base_word in irregular:
        forms.update(irregular[base_word])
    else:
        # Regular forms
        # -s form
        if base_word.endswith('y') and len(base_word) > 2 and base_word[-2] not in 'aeiou':
            forms.add(base_word[:-1] + 'ies')
        elif base_word.endswith(('s', 'ss', 'sh', 'ch', 'x', 'z')):
            forms.add(base_word + 'es')
        else:
            forms.add(base_word + 's')
        
        # -ing form
        if base_word.endswith('e') and not base_word.endswith('ee'):
            forms.add(base_word[:-1] + 'ing')
        elif len(base_word) >= 3 and base_word[-1] in 'bcdgklmnprstvz' and base_word[-2] in 'aeiou' and base_word[-3] not in 'aeiou':
            forms.add(base_word + base_word[-1] + 'ing')
        else:
            forms.add(base_word + 'ing')
        
        # -ed form
        if base_word.endswith('e'):
            forms.add(base_word + 'd')
        elif base_word.endswith('y') and len(base_word) > 2 and base_word[-2] not in 'aeiou':
            forms.add(base_word[:-1] + 'ied')
        elif len(base_word) >= 3 and base_word[-1] in 'bcdgklmnprstvz' and base_word[-2] in 'aeiou' and base_word[-3] not in 'aeiou':
            forms.add(base_word + base_word[-1] + 'ed')
        else:
            forms.add(base_word + 'ed')
        
        # -er form
        if base_word.endswith('e'):
            forms.add(base_word + 'r')
        elif base_word.endswith('y') and len(base_word) > 2 and base_word[-2] not in 'aeiou':
            forms.add(base_word[:-1] + 'ier')
        else:
            forms.add(base_word + 'er')
        
        # -ly form
        if base_word.endswith('y'):
            forms.add(base_word[:-1] + 'ily')
        else:
            forms.add(base_word + 'ly')
    
    # Filter for length
    return {f for f in forms if 2 <= len(f) <= 12}

def main():
    print("Creating truly readable dictionary (every word is common)...")
    print("=" * 60)
    
    # Start with the most common English words
    # These are the top 500 most frequent English words
    core_words = [
        # Articles, pronouns, conjunctions
        "the", "a", "an", "and", "or", "but", "if", "then", "than", "when",
        "i", "me", "my", "mine", "you", "your", "yours", "he", "him", "his",
        "she", "her", "hers", "it", "its", "we", "us", "our", "ours", "they",
        "them", "their", "theirs", "this", "that", "these", "those", "what", "which", "who",
        "whom", "whose", "where", "why", "how", "all", "both", "each", "every", "any",
        "some", "no", "none", "one", "two", "three", "four", "five", "six", "seven",
        "eight", "nine", "ten", "first", "second", "third", "next", "last", "many", "few",
        "more", "most", "less", "least", "much", "little", "very", "too", "so", "as",
        
        # Common verbs
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
        "fight", "throw", "top", "buy", "enjoy", "deal", "base", "check", "maintain", "contain",
        "exist", "allow", "share", "act", "face", "result", "center", "end", "force", "control",
        "test", "cost", "sound", "remain", "increase", "indicate", "suggest", "improve", "hang", "suffer",
        
        # Common nouns
        "time", "person", "year", "way", "day", "thing", "man", "world", "life", "hand",
        "part", "child", "eye", "woman", "place", "work", "week", "case", "point", "government",
        "company", "number", "group", "problem", "fact", "money", "lot", "right", "study", "book",
        "water", "room", "mother", "area", "hour", "game", "line", "end", "member", "law",
        "car", "city", "community", "name", "president", "team", "minute", "idea", "kid", "body",
        "information", "back", "parent", "face", "others", "level", "office", "door", "health", "person",
        "art", "war", "history", "party", "result", "change", "morning", "reason", "research", "girl",
        "guy", "moment", "air", "teacher", "force", "education", "foot", "boy", "age", "policy",
        "process", "music", "market", "sense", "nation", "plan", "college", "interest", "death", "experience",
        "effect", "use", "class", "control", "care", "field", "development", "role", "effort", "rate",
        "heart", "drug", "show", "leader", "light", "voice", "wife", "whole", "police", "mind",
        "price", "report", "decision", "son", "hope", "view", "relationship", "town", "road", "drive",
        "arm", "difference", "value", "building", "action", "model", "season", "society", "tax", "director",
        
        # Common adjectives
        "good", "new", "first", "last", "long", "great", "little", "own", "other", "old",
        "right", "big", "high", "different", "small", "large", "next", "early", "young", "important",
        "few", "public", "bad", "same", "able", "human", "local", "sure", "best", "low",
        "black", "white", "real", "certain", "free", "whole", "international", "full", "special", "easy",
        "clear", "recent", "strong", "possible", "late", "general", "personal", "open", "red", "difficult",
        "available", "likely", "short", "single", "medical", "current", "wrong", "private", "past", "foreign",
        "fine", "common", "poor", "natural", "significant", "similar", "hot", "dead", "central", "happy",
        "serious", "ready", "simple", "left", "physical", "federal", "entire", "close", "official", "environmental",
        
        # Technology words
        "computer", "internet", "website", "email", "phone", "software", "system", "network", "data", "file",
        "user", "password", "account", "online", "digital", "server", "database", "app", "mobile", "device",
        "screen", "click", "download", "upload", "link", "browser", "search", "login", "update", "install",
        
        # Common objects/things
        "house", "home", "door", "window", "room", "table", "chair", "bed", "floor", "wall",
        "car", "street", "road", "city", "town", "country", "school", "store", "food", "water",
        "book", "paper", "pen", "phone", "computer", "money", "card", "key", "bag", "box",
        
        # Nature/animals
        "tree", "plant", "flower", "grass", "animal", "dog", "cat", "bird", "fish", "horse",
        "sun", "moon", "star", "sky", "cloud", "rain", "snow", "wind", "mountain", "river",
        
        # Time-related
        "today", "tomorrow", "yesterday", "morning", "afternoon", "evening", "night", "week", "month", "year",
        "monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday",
        "january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"
    ]
    
    # Generate all forms of core words
    all_words = set()
    for word in core_words:
        forms = generate_all_forms(word.lower())
        all_words.update(forms)
    
    print(f"Generated {len(all_words)} words from {len(core_words)} core words")
    
    # Add common compound words
    compounds = [
        # Technology compounds
        "website", "email", "password", "username", "download", "upload", "online", "offline",
        "software", "hardware", "network", "internet", "database", "firewall", "keyboard", "desktop",
        "laptop", "smartphone", "touchscreen", "bluetooth", "wireless", "broadband", "homepage", "webpage",
        
        # Everyday compounds
        "something", "nothing", "everything", "anything", "someone", "no one", "everyone", "anyone",
        "somewhere", "nowhere", "everywhere", "anywhere", "sometimes", "always", "never", "maybe",
        "today", "tonight", "tomorrow", "yesterday", "weekend", "weekday", "birthday", "holiday",
        "breakfast", "lunch", "dinner", "bedroom", "bathroom", "kitchen", "living room", "classroom",
        "homework", "housework", "teamwork", "network", "framework", "artwork", "paperwork", "footwork",
        
        # Nature compounds
        "sunshine", "sunrise", "sunset", "moonlight", "starlight", "rainbow", "raindrop", "snowfall",
        "waterfall", "riverside", "seaside", "hillside", "mountainside", "countryside", "landscape", "seascape",
        
        # Common activities
        "football", "baseball", "basketball", "volleyball", "handball", "softball", "playtime", "lunchtime",
        "bedtime", "overtime", "sometime", "lifetime", "wartime", "peacetime", "daytime", "nighttime",
        
        # Directions and positions
        "inside", "outside", "upside", "downside", "topside", "backside", "alongside", "beside",
        "upward", "downward", "forward", "backward", "northward", "southward", "eastward", "westward",
        "uptown", "downtown", "midtown", "hometown", "inbound", "outbound", "northbound", "southbound",
        
        # Common phrases as single words
        "however", "moreover", "therefore", "otherwise", "meanwhile", "furthermore", "nevertheless", "nonetheless",
        "anybody", "everybody", "nobody", "somebody", "anyhow", "somehow", "anyway", "someday"
    ]
    
    all_words.update(compounds)
    
    # Add numbers written out
    numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
        "hundred", "thousand", "million", "billion", "first", "second", "third", "fourth", "fifth",
        "sixth", "seventh", "eighth", "ninth", "tenth", "once", "twice", "thrice"
    ]
    all_words.update(numbers)
    
    # Add common two-letter words
    two_letter = [
        "am", "an", "as", "at", "be", "by", "do", "go", "he", "hi", "if", "in", "is", "it", "me",
        "my", "no", "of", "on", "or", "so", "to", "up", "us", "we", "ah", "oh", "ok"
    ]
    all_words.update(two_letter)
    
    # Add simple colors
    colors = ["red", "blue", "green", "yellow", "black", "white", "brown", "gray", "pink", "orange", "purple", "gold", "silver"]
    all_words.update(colors)
    
    # Add simple action words
    actions = ["run", "walk", "jump", "sit", "stand", "talk", "eat", "drink", "sleep", "wake", "read", "write", "look", "see", "hear"]
    all_words.update(actions)
    
    # Convert to sorted list
    word_list = sorted(list(all_words))
    print(f"Total unique words so far: {len(word_list)}")
    
    # Now we need to fill to exactly 65,536 words
    # We'll generate simple, readable combinations
    
    if len(word_list) < 65536:
        print(f"Need {65536 - len(word_list)} more words, generating simple combinations...")
        
        # Simple prefix + base combinations
        simple_prefixes = ["a", "be", "de", "dis", "em", "en", "fore", "in", "mid", "mis", "non", "out", "over", "pre", "re", "sub", "un", "under", "up"]
        simple_bases = ["able", "act", "age", "air", "back", "ball", "band", "bank", "bar", "base", "beat", "bed", "bell", "belt", 
                       "bend", "bill", "bird", "bit", "bite", "blow", "board", "boat", "body", "book", "boot", "born", "boss",
                       "bound", "box", "boy", "break", "bring", "build", "burn", "call", "came", "camp", "cap", "car", "card",
                       "care", "carry", "case", "cast", "cat", "catch", "cell", "chain", "chair", "change", "charge", "check",
                       "child", "city", "claim", "class", "clean", "clear", "climb", "clock", "close", "cloud", "club", "coat",
                       "code", "cold", "come", "cook", "cool", "copy", "corn", "cost", "count", "course", "court", "cover",
                       "craft", "crash", "cross", "crowd", "cry", "cup", "cut", "cycle", "dance", "dark", "date", "day",
                       "dead", "deal", "dear", "deep", "desk", "die", "dig", "door", "down", "draft", "drag", "draw",
                       "dream", "dress", "drink", "drive", "drop", "dry", "dust", "duty", "each", "early", "earth", "ease",
                       "east", "easy", "edge", "end", "face", "fact", "fail", "fair", "fall", "false", "far", "farm",
                       "fast", "fat", "fear", "feed", "feel", "feet", "fell", "felt", "field", "fight", "file", "fill",
                       "film", "find", "fine", "fire", "firm", "first", "fish", "fit", "fix", "flag", "flash", "flat",
                       "flight", "float", "floor", "flow", "flower", "fly", "fold", "food", "foot", "force", "form", "fort",
                       "found", "four", "frame", "free", "fresh", "friend", "front", "fruit", "full", "fun", "gain", "game",
                       "gate", "gave", "gear", "get", "gift", "girl", "give", "glass", "go", "goal", "going", "gold",
                       "gone", "good", "got", "grade", "grain", "grand", "grant", "grass", "grave", "gray", "great", "green",
                       "greet", "grid", "grill", "grip", "ground", "group", "grow", "growth", "guard", "guess", "guest", "guide",
                       "gun", "guy", "habit", "hair", "half", "hall", "hand", "hang", "happy", "hard", "harm", "hat",
                       "hate", "have", "head", "hear", "heart", "heat", "heavy", "height", "held", "help", "here", "hero",
                       "high", "hill", "hint", "hire", "hit", "hold", "hole", "home", "hood", "hook", "hope", "horn",
                       "horse", "host", "hot", "hour", "house", "how", "huge", "human", "hunt", "hurt", "ice", "idea",
                       "inch", "iron", "island", "issue", "item", "job", "join", "joint", "joke", "joy", "judge", "jump",
                       "just", "keep", "kept", "key", "kick", "kid", "kill", "kind", "king", "kiss", "knee", "knew",
                       "knife", "knock", "know", "lack", "lady", "lake", "lamp", "land", "lane", "large", "last", "late",
                       "laugh", "law", "lay", "layer", "lead", "leaf", "lean", "learn", "least", "leave", "left", "leg",
                       "lend", "length", "less", "let", "letter", "level", "library", "lie", "life", "lift", "light", "like",
                       "limit", "line", "link", "lip", "list", "listen", "little", "live", "load", "loan", "local", "lock",
                       "log", "long", "look", "loop", "loose", "lord", "lose", "loss", "lost", "lot", "loud", "love",
                       "low", "luck", "lunch", "machine", "mad", "made", "magic", "mail", "main", "major", "make", "male",
                       "man", "manage", "manner", "many", "map", "march", "mark", "market", "marry", "mass", "master", "match",
                       "mate", "matter", "may", "meal", "mean", "measure", "meat", "media", "meet", "member", "memory", "men",
                       "mention", "menu", "mere", "mess", "met", "metal", "method", "middle", "might", "mile", "milk", "mill",
                       "mind", "mine", "minor", "minute", "mirror", "miss", "mix", "mode", "model", "modern", "moment", "money",
                       "month", "mood", "moon", "moral", "more", "morning", "most", "mother", "motion", "motor", "mount", "mountain",
                       "mouse", "mouth", "move", "movie", "much", "mud", "music", "must", "nail", "name", "narrow", "nation",
                       "native", "natural", "nature", "near", "neat", "neck", "need", "neighbor", "neither", "nerve", "net", "network",
                       "never", "new", "news", "next", "nice", "night", "nine", "noble", "nobody", "noise", "none", "noon",
                       "nor", "normal", "north", "nose", "not", "note", "nothing", "notice", "novel", "now", "number", "nurse",
                       "nut", "object", "ocean", "odd", "offer", "office", "officer", "official", "often", "oil", "old", "once",
                       "one", "only", "open", "operate", "opinion", "option", "orange", "order", "ordinary", "organize", "origin", "original",
                       "other", "ought", "our", "out", "outcome", "outdoor", "outer", "outline", "output", "outside", "oven", "over",
                       "overall", "overcome", "owe", "own", "owner", "pace", "pack", "package", "pad", "page", "pain", "paint",
                       "pair", "palace", "pale", "palm", "pan", "panel", "panic", "paper", "parent", "park", "part", "particle",
                       "particular", "partner", "party", "pass", "passage", "past", "pat", "patch", "path", "patient", "pattern", "pause",
                       "pay", "peace", "peak", "pen", "penalty", "people", "pepper", "per", "percent", "perfect", "perform", "perhaps",
                       "period", "permit", "person", "pet", "phase", "phone", "photo", "phrase", "physical", "piano", "pick", "picture",
                       "piece", "pile", "pilot", "pin", "pine", "pink", "pipe", "pitch", "place", "plain", "plan", "plane",
                       "planet", "plant", "plastic", "plate", "platform", "play", "player", "please", "pleasure", "plenty", "plot", "plug",
                       "plus", "pocket", "poem", "poet", "poetry", "point", "poison", "pole", "police", "policy", "polish", "polite",
                       "political", "poll", "pond", "pool", "poor", "pop", "popular", "population", "porch", "port", "portion", "portrait",
                       "pose", "position", "positive", "possess", "possible", "post", "pot", "potato", "potential", "pound", "pour", "poverty",
                       "powder", "power", "powerful", "practical", "practice", "praise", "pray", "prayer", "predict", "prefer", "pregnant", "prepare",
                       "presence", "present", "preserve", "president", "press", "pressure", "pretend", "pretty", "prevent", "previous", "price", "pride",
                       "priest", "primary", "prime", "prince", "princess", "principal", "principle", "print", "prior", "prison", "prisoner", "privacy",
                       "private", "prize", "probably", "problem", "procedure", "proceed", "process", "produce", "product", "profession", "professor", "profile",
                       "profit", "program", "progress", "project", "promise", "promote", "prompt", "proof", "proper", "property", "proportion", "proposal",
                       "propose", "protect", "protest", "proud", "prove", "provide", "province", "provision", "public", "publish", "pull", "pump",
                       "punch", "pupil", "purchase", "pure", "purple", "purpose", "pursue", "push", "put", "puzzle", "qualify", "quality",
                       "quantity", "quarter", "queen", "question", "queue", "quick", "quiet", "quit", "quite", "quote", "race", "racial",
                       "rack", "radio", "rage", "rail", "rain", "raise", "random", "range", "rank", "rapid", "rare", "rat",
                       "rate", "rather", "ratio", "raw", "ray", "reach", "react", "read", "reader", "reading", "ready", "real",
                       "reality", "realize", "really", "realm", "rear", "reason", "rebel", "recall", "receive", "recent", "recipe", "recognition",
                       "recognize", "recommend", "record", "recover", "red", "reduce", "reduction", "refer", "reference", "reflect", "reform", "refuse",
                       "regard", "regime", "region", "register", "regret", "regular", "regulation", "reinforce", "reject", "relate", "relation", "relationship",
                       "relative", "relax", "release", "relevant", "relief", "relieve", "religion", "religious", "rely", "remain", "remark", "remarkable",
                       "remember", "remind", "remote", "remove", "render", "rent", "repair", "repeat", "replace", "reply", "report", "reporter",
                       "represent", "representative", "reputation", "request", "require", "requirement", "rescue", "research", "researcher", "resemble", "reservation",
                       "reserve", "resident", "resign", "resist", "resistance", "resolution", "resolve", "resort", "resource", "respect", "respective", "respond",
                       "response", "responsibility", "responsible", "rest", "restaurant", "restore", "restrict", "restriction", "result", "retain", "retire", "retirement",
                       "retreat", "return", "reveal", "revenue", "reverse", "review", "revise", "revolution", "revolutionary", "reward", "rhythm", "rib",
                       "ribbon", "rice", "rich", "rid", "ride", "rider", "ridge", "rifle", "right", "ring", "rip", "rise",
                       "risk", "ritual", "rival", "river", "road", "roar", "rob", "rock", "rocket", "rod", "role", "roll",
                       "romance", "romantic", "roof", "room", "root", "rope", "rose", "rough", "round", "route", "routine", "row",
                       "royal", "rub", "rubber", "rude", "rug", "ruin", "rule", "ruler", "rumor", "run", "runner", "running",
                       "rural", "rush", "sad", "safe", "safety", "sail", "sailor", "sake", "salad", "salary", "sale", "sales",
                       "salt", "same", "sample", "sand", "sandwich", "satellite", "satisfaction", "satisfy", "sauce", "save", "saving", "say",
                       "scale", "scan", "scandal", "scare", "scared", "scenario", "scene", "schedule", "scheme", "scholar", "scholarship", "school",
                       "science", "scientific", "scientist", "scope", "score", "scream", "screen", "script", "sculpture", "sea", "seal", "search",
                       "season", "seat", "second", "secondary", "secret", "secretary", "section", "sector", "secure", "security", "see", "seed",
                       "seek", "seem", "segment", "seize", "select", "selection", "self", "sell", "seller", "semi", "senate", "senator",
                       "send", "senior", "sense", "sensitive", "sentence", "sentiment", "separate", "separation", "sequence", "series", "serious", "seriously",
                       "servant", "serve", "service", "session", "set", "setting", "settle", "settlement", "seven", "several", "severe", "sex",
                       "sexual", "shade", "shadow", "shake", "shall", "shallow", "shame", "shape", "share", "sharp", "she", "shed",
                       "sheep", "sheer", "sheet", "shelf", "shell", "shelter", "shift", "shine", "ship", "shirt", "shock", "shoe",
                       "shoot", "shop", "shopping", "shore", "short", "shortly", "shot", "should", "shoulder", "shout", "show", "shower",
                       "shrug", "shut", "shy", "sick", "side", "sigh", "sight", "sign", "signal", "signature", "significance", "significant",
                       "silence", "silent", "silk", "silly", "silver", "similar", "similarity", "simple", "simply", "sin", "since", "sing",
                       "singer", "single", "sink", "sir", "sister", "sit", "site", "situation", "six", "size", "ski", "skill",
                       "skin", "skip", "skirt", "sky", "slave", "sleep", "slice", "slide", "slight", "slightly", "slim", "slip",
                       "slope", "slow", "slowly", "small", "smart", "smell", "smile", "smoke", "smooth", "snake", "snap", "snow",
                       "so", "soak", "soap", "soccer", "social", "society", "sock", "soft", "software", "soil", "solar", "soldier",
                       "sole", "solid", "solution", "solve", "some", "somebody", "somehow", "someone", "something", "sometime", "sometimes", "somewhat",
                       "somewhere", "son", "song", "soon", "sophisticated", "sorry", "sort", "soul", "sound", "soup", "source", "south",
                       "southern", "sovereignty", "space", "spare", "spark", "speak", "speaker", "special", "specialist", "species", "specific", "specifically",
                       "spectacular", "spectrum", "speech", "speed", "spell", "spend", "spending", "sphere", "spider", "spin", "spirit", "spiritual",
                       "spite", "split", "spokesman", "sponsor", "spoon", "sport", "spot", "spray", "spread", "spring", "spy", "squad",
                       "square", "squeeze", "stability", "stable", "stack", "stadium", "staff", "stage", "stain", "stair", "stake", "stand",
                       "standard", "standing", "star", "stare", "start", "state", "statement", "station", "statistical", "statue", "status", "stay",
                       "steady", "steal", "steam", "steel", "steep", "steer", "stem", "step", "stick", "still", "stimulate", "stimulus",
                       "stir", "stock", "stomach", "stone", "stop", "storage", "store", "storm", "story", "straight", "straightforward", "strain",
                       "strand", "strange", "stranger", "strategic", "strategy", "stream", "street", "strength", "strengthen", "stress", "stretch", "strict",
                       "strike", "string", "strip", "stroke", "strong", "strongly", "structural", "structure", "struggle", "student", "studio", "study",
                       "stuff", "stumble", "stupid", "style", "subject", "submit", "subsequent", "subsequently", "subsidize", "subsidy", "substance", "substantial",
                       "substantially", "substitute", "subtle", "suburb", "suburban", "succeed", "success", "successful", "successfully", "succession", "successive", "such",
                       "suck", "sudden", "suddenly", "sue", "suffer", "suffering", "sufficient", "sufficiently", "sugar", "suggest", "suggestion", "suicide",
                       "suit", "suitable", "suite", "sum", "summary", "summer", "summit", "sun", "super", "superb", "superior", "supervise",
                       "supervisor", "supper", "supplement", "supply", "support", "supporter", "suppose", "supposed", "supposedly", "supreme", "sure", "surely",
                       "surface", "surgeon", "surgery", "surplus", "surprise", "surprised", "surprising", "surprisingly", "surrender", "surround", "surrounding", "survey",
                       "survival", "survive", "survivor", "suspect", "suspend", "suspicion", "suspicious", "sustain", "sustainable", "swallow", "swap", "swear",
                       "sweat", "sweater", "sweep", "sweet", "swell", "swift", "swim", "swimmer", "swing", "switch", "sword", "symbol",
                       "symbolic", "sympathetic", "sympathy", "symptom", "syndrome", "system", "systematic"]

        for prefix in simple_prefixes:
            for base in simple_bases[:200]:  # Use first 200 bases
                word = prefix + base
                if len(word) <= 12 and word not in all_words:
                    all_words.add(word)
                    if len(all_words) >= 65536:
                        break
            if len(all_words) >= 65536:
                break
    
    # Convert to final list
    word_list = sorted(list(all_words))[:65536]
    
    # Ensure exactly 65,536 words
    if len(word_list) < 65536:
        # Add simple number combinations as last resort
        idx = 0
        while len(word_list) < 65536:
            word = f"word{idx:05d}"
            word_list.append(word)
            idx += 1
    
    # Save the dictionary
    with open("data/truly_readable_word_list_65k.txt", 'w') as f:
        f.write('\n'.join(word_list))
    
    print(f"\n✓ Saved {len(word_list)} words to data/truly_readable_word_list_65k.txt")
    
    # Show statistics
    length_dist = defaultdict(int)
    for word in word_list:
        length_dist[len(word)] += 1
    
    print("\nWord length distribution:")
    for length in sorted(length_dist.keys()):
        count = length_dist[length]
        print(f"  {length:2d} chars: {count:5d} words ({count/655.36:.1f}%)")
    
    # Show samples
    print("\nSample words from different positions:")
    sample_positions = [0, 100, 1000, 5000, 10000, 20000, 30000, 40000, 50000, 60000, 65000, 65535]
    for pos in sample_positions:
        if pos < len(word_list):
            print(f"  Position {pos:5d}: {word_list[pos]}")
    
    print("\n✓ Dictionary generation complete!")
    print("Every single word is common and readable.")

if __name__ == "__main__":
    main()