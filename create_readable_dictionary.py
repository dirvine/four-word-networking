#!/usr/bin/env python3
# /// script
# requires-python = ">=3.8"
# dependencies = [
#     "pandas>=2.0.0",
#     "textstat>=0.7.0",
#     "nltk>=3.8.0",
#     "requests>=2.28.0",
# ]
# ///
"""
Create a human-readable dictionary for three-word networking.
Uses multiple readability criteria to select the most readable 65,536 words.

Usage:
    uv run python create_readable_dictionary.py
"""

import os
import re
import pandas as pd
import textstat
import nltk
import requests
from collections import defaultdict
from typing import Dict, List, Tuple, Set
import json

# Download required NLTK data
try:
    nltk.data.find('tokenizers/punkt')
except LookupError:
    nltk.download('punkt')

try:
    nltk.data.find('corpora/cmudict')
except LookupError:
    nltk.download('cmudict')

class ReadabilityScorer:
    def __init__(self):
        self.cmu_dict = nltk.corpus.cmudict.dict()
        self.offensive_words = self.load_offensive_words()
        self.dale_chall_words = set()  # Will be populated
        self.oxford_words = {}  # word -> CEFR level
        
    def load_offensive_words(self) -> Set[str]:
        """Load list of offensive words to filter out."""
        offensive = {
            "fuck", "shit", "damn", "hell", "ass", "dick", "cunt", "bitch",
            "pussy", "cock", "bastard", "piss", "fag", "dyke", "nigger", "nigga",
            "retard", "rape", "nazi", "hitler", "whore", "slut"
        }
        return offensive
    
    def download_dale_chall_list(self):
        """Download Dale-Chall 3000 word list from GitHub."""
        print("Downloading Dale-Chall 3000 word list...")
        try:
            # Try the GitHub gist directly
            url = "https://gist.githubusercontent.com/e00edcc6f508640fe24f263f5836a7dc/raw/1b1427e45cf3d476c4b9a21731c51b59e7fb7bad/dale-chall-3000-words.txt"
            response = requests.get(url)
            if response.status_code == 200:
                words = response.text.strip().split('\n')
                self.dale_chall_words = set(word.strip().lower() for word in words if word.strip())
                print(f"Loaded {len(self.dale_chall_words)} Dale-Chall words")
            else:
                print(f"Failed to download Dale-Chall list: {response.status_code}")
        except Exception as e:
            print(f"Error downloading Dale-Chall list: {e}")
            # Use a subset of known Dale-Chall words as fallback
            self.dale_chall_words = {
                "a", "able", "about", "above", "across", "act", "add", "afraid", "after", "again",
                "against", "age", "ago", "agree", "air", "all", "allow", "almost", "alone", "along",
                "already", "also", "always", "am", "among", "an", "and", "angry", "animal", "another",
                "answer", "any", "anyone", "anything", "appear", "apple", "are", "area", "arm", "around",
                "arrive", "art", "as", "ask", "at", "ate", "attack", "aunt", "away", "baby", "back",
                "bad", "bag", "ball", "bank", "bar", "base", "basket", "bat", "be", "bean", "bear",
                "beat", "beautiful", "became", "because", "become", "bed", "been", "before", "began",
                "begin", "behind", "being", "believe", "bell", "belong", "below", "beside", "best",
                "better", "between", "big", "bill", "bird", "birthday", "bit", "bite", "black", "blew",
                "block", "blood", "blow", "blue", "board", "boat", "body", "bone", "book", "born",
                "both", "bottle", "bottom", "bought", "bow", "bowl", "box", "boy", "branch", "brave",
                "bread", "break", "breakfast", "breath", "bring", "broke", "brother", "brought", "brown",
                "build", "burn", "bus", "business", "busy", "but", "butter", "buy", "by", "cabin",
                "cake", "call", "came", "camp", "can", "cap", "captain", "car", "card", "care",
                "careful", "carry", "case", "cat", "catch", "cattle", "caught", "cause", "cent",
                "center", "century", "certain", "chair", "chance", "change", "chapter", "charge",
                "chase", "cheap", "check", "cheese", "chicken", "chief", "child", "children", "choice",
                "choose", "chose", "church", "circle", "circus", "city", "class", "clean", "clear",
                "climb", "clock", "close", "cloth", "clothes", "cloud", "club", "coast", "coat",
                "coffee", "cold", "collect", "college", "color", "come", "comfort", "coming", "company",
                "compare", "complete", "cook", "cool", "copy", "corn", "corner", "correct", "cost",
                "cotton", "could", "count", "country", "course", "cousin", "cover", "cow", "crack",
                "cream", "creek", "cried", "crop", "cross", "crowd", "crown", "cry", "cup", "cut",
                "dad", "dance", "danger", "dark", "date", "daughter", "day", "dead", "dear", "death",
                "decide", "deep", "deer", "desk", "did", "die", "difference", "different", "dig",
                "dinner", "direction", "discover", "distance", "do", "doctor", "does", "dog", "dollar",
                "done", "door", "dot", "double", "down", "dozen", "drag", "draw", "dream", "dress",
                "drew", "drink", "drive", "drop", "drove", "dry", "duck", "during", "dust", "each",
                "ear", "early", "earn", "earth", "east", "easy", "eat", "edge", "egg", "eight",
                "either", "elephant", "else", "empty", "end", "enemy", "enjoy", "enough", "enter",
                "equal", "escape", "even", "evening", "ever", "every", "everyone", "everything",
                "exact", "except", "exchange", "excite", "excuse", "expect", "explain", "eye", "face",
                "fact", "fair", "fall", "family", "famous", "far", "farm", "farmer", "fast", "fat",
                "father", "fear", "feather", "fed", "feed", "feel", "feet", "fell", "fellow", "felt",
                "fence", "few", "field", "fight", "figure", "fill", "film", "find", "fine", "finger",
                "finish", "fire", "first", "fish", "fit", "five", "fix", "flag", "flat", "flew",
                "floor", "flower", "fly", "follow", "food", "foot", "for", "force", "forest", "forget",
                "form", "fort", "forward", "fought", "found", "four", "fox", "frame", "free", "freedom",
                "fresh", "friend", "frighten", "from", "front", "fruit", "full", "fun", "funny", "fur",
                "further", "future", "game", "garden", "gas", "gate", "gather", "gave", "general",
                "gentle", "get", "gift", "girl", "give", "glad", "glass", "go", "goat", "god", "goes",
                "gold", "golden", "gone", "good", "got", "government", "grab", "grade", "grain", "grand",
                "grandfather", "grandmother", "grass", "grave", "gray", "great", "green", "grew", "ground",
                "group", "grow", "guard", "guess", "guest", "guide", "gun", "had", "hair", "half", "hall",
                "hand", "handle", "hang", "happen", "happy", "hard", "has", "hat", "hate", "have", "he",
                "head", "health", "hear", "heard", "heart", "heat", "heavy", "held", "hello", "help",
                "hen", "her", "here", "herself", "hid", "hide", "high", "hill", "him", "himself", "his",
                "hit", "hold", "hole", "holiday", "home", "honest", "hope", "horn", "horse", "hospital",
                "hot", "hour", "house", "how", "however", "huge", "human", "hundred", "hung", "hungry",
                "hunt", "hurry", "hurt", "husband", "I", "ice", "idea", "if", "ill", "imagine",
                "important", "in", "inch", "include", "indeed", "indian", "inside", "instead", "interest",
                "into", "iron", "is", "island", "it", "its", "itself", "jack", "jail", "jar", "jet",
                "job", "join", "joke", "journey", "joy", "judge", "jump", "just", "keep", "kept", "key",
                "kick", "kid", "kill", "kind", "king", "kiss", "kitchen", "knee", "knew", "knife",
                "knock", "know", "lack", "ladder", "lady", "laid", "lake", "lamp", "land", "language",
                "large", "last", "late", "laugh", "law", "lay", "lazy", "lead", "leader", "leaf", "lean",
                "learn", "least", "leather", "leave", "led", "left", "leg", "lend", "length", "less",
                "lesson", "let", "letter", "level", "liberty", "library", "lie", "life", "lift", "light",
                "like", "likely", "line", "lion", "lip", "list", "listen", "little", "live", "load",
                "loaf", "lock", "log", "lone", "long", "look", "loose", "lord", "lose", "loss", "lost",
                "lot", "loud", "love", "lovely", "low", "lower", "luck", "lunch", "machine", "mad",
                "made", "magic", "mail", "main", "major", "make", "man", "manner", "many", "map",
                "march", "mark", "market", "marry", "mass", "master", "mat", "match", "matter", "may",
                "maybe", "me", "meal", "mean", "measure", "meat", "medicine", "meet", "member", "men",
                "met", "metal", "method", "middle", "might", "mile", "milk", "mill", "mind", "mine",
                "minute", "miss", "mistake", "mix", "moment", "money", "monkey", "month", "moon", "more",
                "morning", "most", "mother", "motion", "mountain", "mouse", "mouth", "move", "movie",
                "Mr.", "Mrs.", "much", "mud", "music", "must", "my", "myself", "nail", "name", "narrow",
                "nation", "nature", "near", "nearly", "necessary", "neck", "need", "needle", "neighbor",
                "neither", "nerve", "nest", "net", "never", "new", "news", "newspaper", "next", "nice",
                "night", "nine", "no", "nobody", "noise", "none", "noon", "nor", "north", "nose", "not",
                "note", "nothing", "notice", "now", "number", "nut", "object", "ocean", "of", "off",
                "offer", "office", "officer", "often", "oh", "oil", "old", "on", "once", "one", "only",
                "onto", "open", "opposite", "or", "orange", "order", "other", "ought", "our", "out",
                "outside", "over", "own", "owner", "pack", "package", "page", "paid", "pain", "paint",
                "pair", "palace", "pan", "paper", "parent", "park", "part", "party", "pass", "passage",
                "past", "pasture", "path", "pattern", "pay", "peace", "pen", "pencil", "penny", "people",
                "perfect", "perhaps", "person", "pet", "phone", "piano", "pick", "picnic", "picture",
                "pie", "piece", "pig", "pile", "pilot", "pin", "pink", "pipe", "place", "plain", "plan",
                "plane", "plant", "plate", "play", "pleasant", "please", "pleasure", "plenty", "plow",
                "pocket", "point", "poison", "pole", "police", "policeman", "pond", "pony", "pool",
                "poor", "pop", "popular", "porch", "position", "possible", "post", "pot", "potato",
                "pound", "pour", "powder", "power", "practice", "prepare", "present", "president",
                "press", "pretty", "prevent", "price", "prince", "princess", "print", "prison",
                "private", "prize", "probably", "problem", "produce", "promise", "proper", "protect",
                "proud", "prove", "public", "pull", "pump", "pupil", "puppy", "purchase", "pure",
                "purple", "purpose", "push", "put", "puzzle", "quarter", "queen", "question", "quick",
                "quiet", "quit", "quite", "rabbit", "race", "radio", "rag", "rail", "railroad", "rain",
                "raise", "ran", "ranch", "rang", "rapid", "rate", "rather", "raw", "reach", "read",
                "ready", "real", "reason", "receive", "record", "red", "refuse", "remain", "remember",
                "remove", "rent", "repair", "repeat", "reply", "report", "represent", "require", "rest",
                "result", "return", "reveal", "reward", "ribbon", "rice", "rich", "rid", "ride", "rider",
                "ridge", "riding", "right", "ring", "rise", "river", "road", "roar", "rob", "rock",
                "rode", "roll", "roof", "room", "root", "rope", "rose", "rough", "round", "route", "row",
                "royal", "rub", "rubber", "rule", "run", "rush", "sad", "saddle", "safe", "safety",
                "said", "sail", "sailor", "sake", "salad", "sale", "salt", "same", "sand", "sang",
                "sat", "save", "saw", "say", "scale", "scare", "school", "science", "scissors", "score",
                "sea", "search", "season", "seat", "second", "secret", "section", "see", "seed", "seek",
                "seem", "seen", "seldom", "select", "self", "sell", "send", "sense", "sent", "sentence",
                "separate", "serve", "service", "set", "settle", "seven", "several", "sew", "shade",
                "shadow", "shake", "shall", "shame", "shape", "share", "sharp", "she", "sheep", "sheet",
                "shelf", "shell", "shelter", "shine", "ship", "shirt", "shock", "shoe", "shone", "shook",
                "shoot", "shop", "shore", "short", "shot", "should", "shoulder", "shout", "show", "shut",
                "sick", "side", "sight", "sign", "signal", "silence", "silent", "silk", "silver",
                "similar", "simple", "since", "sing", "single", "sink", "sir", "sister", "sit", "six",
                "size", "skate", "ski", "skill", "skin", "skirt", "sky", "slave", "sleep", "slept",
                "slide", "slip", "slow", "small", "smart", "smell", "smile", "smoke", "smooth", "snake",
                "snow", "so", "soap", "social", "society", "sock", "soft", "soil", "sold", "soldier",
                "solid", "some", "somebody", "somehow", "someone", "something", "sometime", "sometimes",
                "somewhere", "son", "song", "soon", "sore", "sorry", "sort", "sound", "soup", "south",
                "space", "speak", "special", "speech", "speed", "spell", "spend", "spent", "spider",
                "spin", "spirit", "spite", "split", "spoke", "spoon", "sport", "spot", "spread",
                "spring", "square", "stable", "stack", "stage", "stair", "stamp", "stand", "star",
                "stare", "start", "state", "station", "stay", "steady", "steak", "steal", "steam",
                "steel", "steep", "stem", "step", "stick", "stiff", "still", "stock", "stole", "stomach",
                "stone", "stood", "stool", "stop", "store", "storm", "story", "stove", "straight",
                "strange", "stranger", "stream", "street", "strength", "stretch", "strike", "string",
                "strip", "strong", "struggle", "stuck", "student", "study", "stuff", "subject", "such",
                "sudden", "suffer", "sugar", "suggest", "suit", "sum", "summer", "sun", "Sunday",
                "supper", "supply", "support", "suppose", "sure", "surface", "surprise", "swallow",
                "swam", "swear", "sweep", "sweet", "swept", "swift", "swim", "swing", "switch", "sword",
                "swore", "table", "tail", "take", "taken", "talk", "tall", "tank", "tap", "tape", "task",
                "taste", "taught", "tax", "tea", "teach", "teacher", "team", "tear", "telephone", "tell",
                "temperature", "ten", "tent", "term", "terrible", "test", "than", "thank", "that", "the",
                "theater", "thee", "their", "them", "themselves", "then", "there", "therefore", "these",
                "they", "thick", "thin", "thing", "think", "third", "this", "those", "though", "thought",
                "thousand", "thread", "three", "threw", "throat", "through", "throughout", "throw",
                "thrown", "thumb", "thunder", "thus", "thy", "ticket", "tide", "tie", "tiger", "tight",
                "till", "time", "tin", "tiny", "tip", "tire", "title", "to", "tobacco", "today", "toe",
                "together", "told", "tomorrow", "ton", "tone", "tongue", "tonight", "too", "took", "tool",
                "tooth", "top", "tore", "torn", "total", "touch", "toward", "tower", "town", "toy",
                "track", "trade", "traffic", "trail", "train", "trap", "travel", "tray", "treasure",
                "treat", "tree", "trick", "tried", "trim", "trip", "troop", "tropical", "trouble",
                "truck", "true", "trunk", "trust", "truth", "try", "tube", "tune", "turn", "twelve",
                "twenty", "twice", "two", "type", "ugly", "uncle", "under", "understand", "unhappy",
                "union", "unit", "united", "universe", "university", "unless", "until", "up", "upon",
                "upper", "upward", "us", "use", "useful", "usual", "valley", "valuable", "value", "variety",
                "various", "vast", "vegetable", "verb", "very", "vessel", "victory", "view", "village",
                "visit", "visitor", "voice", "vote", "vowel", "voyage", "wagon", "wait", "wake", "walk",
                "wall", "want", "war", "warm", "warn", "was", "wash", "waste", "watch", "water", "wave",
                "way", "we", "weak", "wealth", "weapon", "wear", "weather", "web", "wedding", "week",
                "weigh", "weight", "welcome", "well", "went", "were", "west", "wet", "whale", "what",
                "wheat", "wheel", "when", "whenever", "where", "whether", "which", "while", "whip",
                "whisper", "whistle", "white", "who", "whole", "whom", "whose", "why", "wicked", "wide",
                "wife", "wild", "will", "willing", "win", "wind", "window", "wine", "wing", "winter",
                "wipe", "wire", "wise", "wish", "with", "within", "without", "woke", "wolf", "woman",
                "women", "won", "wonder", "wonderful", "wood", "wooden", "wool", "word", "wore", "work",
                "worker", "world", "worm", "worn", "worry", "worse", "worst", "worth", "would", "wound",
                "wrap", "wreck", "wrist", "write", "written", "wrong", "wrote", "yard", "year", "yellow",
                "yes", "yesterday", "yet", "you", "young", "youngster", "your", "yourself", "youth", "zero"
            }
            print(f"Using fallback Dale-Chall words: {len(self.dale_chall_words)} words")
    
    def load_oxford_pdfs(self):
        """Note: In practice, you'd need to manually download and extract these."""
        # For now, we'll use a simulated subset
        print("Note: Oxford 3000/5000 PDFs need manual download from:")
        print("- https://www.oxfordlearnersdictionaries.com/external/pdf/wordlists/oxford-3000-5000/American_Oxford_3000.pdf")
        print("- https://www.oxfordlearnersdictionaries.com/external/pdf/wordlists/oxford-3000-5000/American_Oxford_5000.pdf")
        
        # Simulated Oxford words with CEFR levels
        self.oxford_words = {
            # A1 level (most basic)
            "the": "A1", "be": "A1", "have": "A1", "do": "A1", "say": "A1",
            "go": "A1", "can": "A1", "get": "A1", "make": "A1", "know": "A1",
            "think": "A1", "take": "A1", "see": "A1", "come": "A1", "want": "A1",
            "use": "A1", "find": "A1", "give": "A1", "tell": "A1", "work": "A1",
            "call": "A1", "try": "A1", "ask": "A1", "need": "A1", "feel": "A1",
            "become": "A1", "leave": "A1", "put": "A1", "mean": "A1", "keep": "A1",
            # A2 level
            "begin": "A2", "seem": "A2", "help": "A2", "show": "A2", "hear": "A2",
            "play": "A2", "run": "A2", "move": "A2", "live": "A2", "believe": "A2",
            # B1 level
            "include": "B1", "continue": "B1", "set": "B1", "learn": "B1", "change": "B1",
            # B2 level
            "develop": "B2", "consider": "B2", "appear": "B2", "involve": "B2", "require": "B2",
        }
    
    def count_syllables(self, word: str) -> int:
        """Count syllables using CMU pronouncing dictionary."""
        word_lower = word.lower()
        if word_lower in self.cmu_dict:
            # CMU dict uses numbers to indicate stress on vowels
            # Count the number of digits (each represents a vowel sound)
            pronunciation = self.cmu_dict[word_lower][0]  # Take first pronunciation
            return len([ph for ph in pronunciation if ph[-1].isdigit()])
        else:
            # Fallback to textstat
            return textstat.syllable_count(word)
    
    def is_phonetically_regular(self, word: str) -> bool:
        """Check if word follows regular phonetic patterns."""
        # Simple heuristic: words in CMU dict are generally more regular
        return word.lower() in self.cmu_dict
    
    def get_word_frequency_score(self, word: str, position_in_list: int) -> float:
        """Score based on position in frequency lists."""
        # Lower position = higher frequency = better score
        if position_in_list < 1000:
            return 1.0
        elif position_in_list < 5000:
            return 0.8
        elif position_in_list < 10000:
            return 0.6
        elif position_in_list < 20000:
            return 0.4
        else:
            return 0.2
    
    def get_cefr_score(self, word: str) -> float:
        """Score based on CEFR level."""
        cefr_scores = {
            "A1": 1.0,
            "A2": 0.9,
            "B1": 0.7,
            "B2": 0.5,
            "C1": 0.3,
            "C2": 0.1
        }
        return cefr_scores.get(self.oxford_words.get(word.lower(), "C2"), 0.1)
    
    def calculate_readability_score(self, word: str, position: int = 50000) -> Dict[str, float]:
        """Calculate comprehensive readability score for a word."""
        scores = {}
        
        # 1. Syllable score (prefer 1-2 syllables)
        syllables = self.count_syllables(word)
        if syllables == 1:
            scores['syllable'] = 1.0
        elif syllables == 2:
            scores['syllable'] = 0.9
        elif syllables == 3:
            scores['syllable'] = 0.6
        else:
            scores['syllable'] = 0.2
        
        # 2. Length score (prefer 3-7 characters)
        length = len(word)
        if 3 <= length <= 5:
            scores['length'] = 1.0
        elif 6 <= length <= 7:
            scores['length'] = 0.8
        elif length == 2 or length == 8:
            scores['length'] = 0.6
        elif length == 9:
            scores['length'] = 0.4
        else:
            scores['length'] = 0.2
        
        # 3. Dale-Chall familiarity
        scores['dale_chall'] = 1.0 if word.lower() in self.dale_chall_words else 0.3
        
        # 4. CEFR level
        scores['cefr'] = self.get_cefr_score(word)
        
        # 5. Frequency score
        scores['frequency'] = self.get_word_frequency_score(word, position)
        
        # 6. Phonetic regularity
        scores['phonetic'] = 1.0 if self.is_phonetically_regular(word) else 0.5
        
        # 7. No numbers or special characters
        scores['clean'] = 1.0 if word.isalpha() else 0.0
        
        # 8. Not offensive
        scores['appropriate'] = 0.0 if word.lower() in self.offensive_words else 1.0
        
        # Calculate weighted total
        weights = {
            'syllable': 0.20,
            'length': 0.10,
            'dale_chall': 0.20,
            'cefr': 0.15,
            'frequency': 0.20,
            'phonetic': 0.10,
            'clean': 0.03,
            'appropriate': 0.02
        }
        
        scores['total'] = sum(scores[k] * weights[k] for k in weights)
        return scores

def load_existing_word_lists() -> List[Tuple[str, int]]:
    """Load existing word lists with frequency information."""
    words_with_position = []
    
    # 1. Load Google 10k list (highest priority)
    if os.path.exists("data/google-10000-english.txt"):
        print("Loading Google 10k list...")
        with open("data/google-10000-english.txt", 'r') as f:
            for position, line in enumerate(f):
                word = line.strip()
                if word and 2 <= len(word) <= 12:
                    words_with_position.append((word, position))
    
    # 2. Load words_alpha.txt (lower priority)
    if os.path.exists("data/words_alpha.txt"):
        print("Loading words_alpha.txt...")
        with open("data/words_alpha.txt", 'r') as f:
            for position, line in enumerate(f):
                word = line.strip()
                if word and 2 <= len(word) <= 10 and word.isalpha():
                    # Add 10000 to position to indicate lower priority
                    words_with_position.append((word, position + 10000))
    
    return words_with_position

def generate_word_forms(base_words: Set[str]) -> Set[str]:
    """Generate common word forms (plurals, tenses, etc.)."""
    word_forms = set(base_words)
    
    # Common suffixes
    for word in list(base_words):
        # Plurals
        if not word.endswith('s'):
            word_forms.add(word + 's')
            if word.endswith('y') and len(word) > 2 and word[-2] not in 'aeiou':
                word_forms.add(word[:-1] + 'ies')
        
        # -ing forms
        if word.endswith('e') and len(word) > 2:
            word_forms.add(word[:-1] + 'ing')
        else:
            word_forms.add(word + 'ing')
        
        # -ed forms
        if word.endswith('e'):
            word_forms.add(word + 'd')
        elif word.endswith('y') and len(word) > 2 and word[-2] not in 'aeiou':
            word_forms.add(word[:-1] + 'ied')
        else:
            word_forms.add(word + 'ed')
        
        # -er forms
        if word.endswith('e'):
            word_forms.add(word + 'r')
        else:
            word_forms.add(word + 'er')
        
        # -est forms
        if word.endswith('e'):
            word_forms.add(word + 'st')
        else:
            word_forms.add(word + 'est')
    
    return word_forms

def main():
    print("Creating human-readable dictionary for three-word networking...")
    print("=" * 60)
    
    # Initialize scorer
    scorer = ReadabilityScorer()
    scorer.download_dale_chall_list()
    scorer.load_oxford_pdfs()
    
    # Load existing word lists
    print("\nLoading word sources...")
    words_with_position = load_existing_word_lists()
    print(f"Loaded {len(words_with_position)} candidate words")
    
    # Score all words
    print("\nScoring words for readability...")
    scored_words = []
    
    for i, (word, position) in enumerate(words_with_position):
        if i % 10000 == 0:
            print(f"Processed {i}/{len(words_with_position)} words...")
        
        scores = scorer.calculate_readability_score(word, position)
        if scores['appropriate'] > 0 and scores['clean'] > 0:  # Basic filters
            scored_words.append({
                'word': word,
                'position': position,
                'total_score': scores['total'],
                **scores
            })
    
    # Convert to DataFrame for easier manipulation
    df = pd.DataFrame(scored_words)
    
    # Sort by total score (descending)
    df = df.sort_values('total_score', ascending=False)
    
    # Take top 65,536 words
    top_words = df.head(65536)
    
    # Generate some statistics
    print("\n" + "=" * 60)
    print("DICTIONARY STATISTICS:")
    print(f"Total words selected: {len(top_words)}")
    print(f"Average readability score: {top_words['total_score'].mean():.3f}")
    print(f"Average syllables: {top_words['syllable'].mean():.2f}")
    print(f"Words in Dale-Chall list: {(top_words['dale_chall'] == 1.0).sum()}")
    
    # Show sample of top words
    print("\nTop 50 words by readability:")
    for i, row in top_words.head(50).iterrows():
        print(f"{row['word']:15} (score: {row['total_score']:.3f})")
    
    # Save the dictionary
    output_words = top_words['word'].tolist()
    
    # Ensure exactly 65,536 words
    while len(output_words) < 65536:
        output_words.append(f"word{len(output_words):05d}")
    
    with open("data/human_readable_word_list_65k.txt", 'w') as f:
        f.write('\n'.join(output_words))
    
    print(f"\n✓ Saved {len(output_words)} words to data/human_readable_word_list_65k.txt")
    
    # Save detailed scores for analysis
    top_words.to_csv("data/word_readability_scores.csv", index=False)
    print("✓ Saved detailed scores to data/word_readability_scores.csv")

if __name__ == "__main__":
    main()