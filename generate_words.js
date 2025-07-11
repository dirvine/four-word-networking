// Word Generator - Creates 66,000 unique words (3-10 characters)
// Run this with Node.js: node generate-words.js

const fs = require('fs');
const path = require('path');

function generateWordList() {
  // Base words collection
  const baseWords = [
    // Common verbs
    'run', 'jump', 'walk', 'talk', 'book', 'look', 'work', 'play', 'read', 'write',
    'think', 'make', 'take', 'give', 'get', 'put', 'see', 'say', 'go', 'come',
    'want', 'need', 'like', 'love', 'hate', 'help', 'call', 'tell', 'ask', 'use',
    'find', 'know', 'feel', 'try', 'keep', 'let', 'begin', 'seem', 'move', 'live',
    'die', 'sit', 'stand', 'lie', 'pay', 'meet', 'include', 'continue', 'set', 'learn',
    'change', 'lead', 'understand', 'watch', 'follow', 'stop', 'create', 'speak', 'spend',
    'grow', 'open', 'close', 'win', 'offer', 'remember', 'consider', 'appear', 'buy',
    'wait', 'serve', 'send', 'expect', 'build', 'stay', 'fall', 'cut', 'reach', 'kill',
    'remain', 'suggest', 'raise', 'pass', 'sell', 'require', 'report', 'decide', 'pull',
    'break', 'hope', 'develop', 'carry', 'drive', 'eat', 'fill', 'fly', 'form', 'join',
    'care', 'cause', 'deal', 'bear', 'beat', 'blow', 'burn', 'cast', 'catch', 'choose',
    'climb', 'cook', 'count', 'cover', 'cross', 'dance', 'doubt', 'draw', 'dream', 'dress',
    'drink', 'drop', 'fail', 'feed', 'fight', 'forget', 'grab', 'grow', 'hang', 'hear',
    'hide', 'hit', 'hold', 'hunt', 'hurt', 'jump', 'kick', 'kiss', 'laugh', 'lay',
    'lean', 'leap', 'leave', 'lend', 'lift', 'light', 'listen', 'lose', 'marry', 'match',
    'miss', 'mix', 'nod', 'note', 'obtain', 'occur', 'offer', 'order', 'own', 'pack',
    'paint', 'park', 'pass', 'pick', 'plan', 'plant', 'point', 'pour', 'practice', 'pray',
    'prepare', 'press', 'pretend', 'prevent', 'print', 'produce', 'promise', 'protect', 'prove',
    'provide', 'publish', 'pull', 'push', 'quit', 'race', 'rain', 'raise', 'reach', 'read',
    'realize', 'receive', 'recognize', 'record', 'reduce', 'refer', 'reflect', 'refuse', 'regard',
    'relate', 'release', 'rely', 'remain', 'remember', 'remove', 'repeat', 'replace', 'reply',
    'report', 'represent', 'require', 'rest', 'return', 'reveal', 'ride', 'ring', 'rise',
    'risk', 'roll', 'rub', 'rule', 'rush', 'save', 'score', 'search', 'seat', 'seek',
    'select', 'sell', 'send', 'sense', 'separate', 'serve', 'settle', 'shake', 'shall',
    'shape', 'share', 'shine', 'shoot', 'shop', 'shout', 'show', 'shut', 'sign', 'sing',
    'sink', 'sit', 'sleep', 'slide', 'slip', 'smile', 'smoke', 'solve', 'sort', 'sound',
    'speak', 'spend', 'split', 'spread', 'spring', 'stand', 'start', 'state', 'stay', 'steal',
    'step', 'stick', 'stop', 'store', 'strike', 'study', 'suffer', 'suggest', 'suit', 'supply',
    'support', 'suppose', 'surprise', 'survive', 'swim', 'swing', 'take', 'talk', 'taste',
    'teach', 'tear', 'tell', 'tend', 'test', 'thank', 'think', 'throw', 'tie', 'touch',
    'train', 'travel', 'treat', 'trust', 'try', 'turn', 'understand', 'use', 'visit', 'vote',
    'wait', 'wake', 'walk', 'want', 'warn', 'wash', 'waste', 'watch', 'wave', 'wear',
    'weigh', 'welcome', 'win', 'wind', 'wish', 'wonder', 'worry', 'write', 'yell', 'yield',

    // Common nouns
    'cat', 'dog', 'act', 'add', 'age', 'ago', 'aid', 'aim', 'air', 'all', 'and',
    'any', 'are', 'arm', 'art', 'bad', 'bag', 'bar', 'base', 'bat', 'bay', 'bed',
    'bee', 'bet', 'bid', 'big', 'bit', 'blue', 'boat', 'body', 'bold', 'bone', 'born',
    'both', 'bowl', 'box', 'boy', 'bus', 'busy', 'but', 'cab', 'cake', 'calm', 'came',
    'camp', 'can', 'cap', 'car', 'card', 'case', 'cash', 'cell', 'cent', 'chin', 'cite',
    'city', 'clay', 'club', 'coal', 'coat', 'code', 'coin', 'cold', 'comb', 'cool', 'cope',
    'copy', 'core', 'corn', 'cost', 'crew', 'crop', 'crow', 'cure', 'dark', 'data', 'date',
    'dawn', 'day', 'dead', 'deaf', 'dear', 'debt', 'deck', 'deep', 'deny', 'desk', 'dial',
    'dice', 'diet', 'dine', 'dirt', 'dish', 'dive', 'dock', 'does', 'door', 'dose', 'down',
    'drag', 'drew', 'drug', 'drum', 'dual', 'duck', 'dull', 'dump', 'dust', 'duty', 'each',
    'earn', 'ease', 'east', 'easy', 'echo', 'edge', 'edit', 'else', 'emit', 'end', 'even',
    'ever', 'evil', 'exit', 'face', 'fact', 'fade', 'fair', 'fake', 'fame', 'farm', 'fast',
    'fate', 'fear', 'feet', 'fell', 'felt', 'file', 'film', 'fine', 'fire', 'firm', 'fish',
    'fist', 'five', 'flag', 'flat', 'flee', 'flew', 'flex', 'flip', 'flow', 'foam', 'fold',
    'folk', 'fond', 'food', 'fool', 'foot', 'ford', 'fork', 'fort', 'foul', 'four', 'free',
    'from', 'fuel', 'full', 'fund', 'fuse', 'gain', 'game', 'gang', 'gate', 'gave', 'gear',
    'gift', 'girl', 'glad', 'goal', 'goat', 'gold', 'golf', 'gone', 'good', 'gray', 'grew',
    'grid', 'grim', 'grin', 'grip', 'gulf', 'hair', 'half', 'hall', 'halt', 'hand', 'hard',
    'harm', 'have', 'hawk', 'head', 'heal', 'heap', 'heat', 'held', 'hell', 'hero', 'high',
    'hill', 'hint', 'hire', 'hole', 'holy', 'home', 'hook', 'horn', 'host', 'hour', 'huge',
    'idea', 'idle', 'inch', 'into', 'iron', 'isle', 'item', 'jack', 'jade', 'jail', 'jazz',
    'jest', 'joke', 'jolt', 'july', 'june', 'jury', 'just', 'keen', 'kind', 'king', 'knee',
    'knew', 'knit', 'knot', 'lack', 'lady', 'laid', 'lake', 'lamb', 'lamp', 'land', 'lane',
    'last', 'late', 'lawn', 'lazy', 'leaf', 'left', 'lens', 'less', 'liar', 'life', 'lime',
    'line', 'link', 'lion', 'list', 'load', 'loan', 'lock', 'loft', 'lone', 'long', 'loop',
    'lord', 'loss', 'lost', 'loud', 'luck', 'lump', 'lung', 'made', 'maid', 'mail', 'main',
    'male', 'mall', 'many', 'mark', 'mars', 'mask', 'mass', 'mate', 'math', 'meal', 'mean',
    'meat', 'melt', 'memo', 'menu', 'mere', 'mesh', 'mess', 'mice', 'mild', 'mile', 'milk',
    'mill', 'mind', 'mine', 'mint', 'mist', 'mode', 'mold', 'mood', 'moon', 'more', 'most',
    'moth', 'much', 'myth', 'nail', 'name', 'navy', 'near', 'neat', 'neck', 'next', 'nice',
    'nine', 'node', 'none', 'noon', 'norm', 'nose', 'noun', 'null', 'oath', 'obey', 'odds',
    'omit', 'once', 'only', 'onto', 'oral', 'oven', 'over', 'owed', 'pace', 'page', 'paid',
    'pain', 'pair', 'pale', 'palm', 'part', 'past', 'path', 'peak', 'pear', 'peel', 'peer',
    'pest', 'pile', 'pill', 'pine', 'pink', 'pipe', 'plot', 'plug', 'plus', 'poem', 'poet',
    'pole', 'poll', 'pond', 'pool', 'poor', 'port', 'pose', 'post', 'prep', 'prey', 'prop',
    'pump', 'pure', 'quiz', 'rack', 'rage', 'raid', 'rail', 'rain', 'rank', 'rare', 'rate',
    'real', 'rear', 'rent', 'rice', 'rich', 'ride', 'ring', 'road', 'roar', 'rock', 'rode',
    'role', 'roof', 'room', 'root', 'rope', 'rose', 'rude', 'ruin', 'rule', 'rust', 'sack',
    'safe', 'sage', 'said', 'sail', 'sake', 'sale', 'salt', 'same', 'sand', 'sane', 'sang',
    'sank', 'scan', 'scar', 'seal', 'seat', 'seed', 'self', 'shed', 'ship', 'shoe', 'shot',
    'sick', 'side', 'sigh', 'sign', 'silk', 'site', 'size', 'skip', 'slam', 'slap', 'sled',
    'slid', 'slim', 'slip', 'slow', 'snap', 'snow', 'soak', 'soap', 'soar', 'sock', 'soft',
    'soil', 'sold', 'sole', 'some', 'song', 'soon', 'sore', 'soul', 'soup', 'sour', 'span',
    'spin', 'spot', 'star', 'stem', 'step', 'stir', 'such', 'suit', 'sure', 'swap', 'tail',
    'tale', 'tall', 'tank', 'tape', 'task', 'team', 'tear', 'tech', 'term', 'test', 'text',
    'than', 'that', 'them', 'then', 'they', 'thin', 'this', 'thus', 'tide', 'tied', 'tier',
    'tile', 'till', 'time', 'tiny', 'tire', 'told', 'toll', 'tomb', 'tone', 'took', 'tool',
    'torn', 'tour', 'town', 'trap', 'tray', 'tree', 'trim', 'trip', 'true', 'tube', 'tune',
    'turn', 'twin', 'type', 'ugly', 'unit', 'upon', 'urge', 'used', 'user', 'vain', 'vary',
    'vast', 'veil', 'verb', 'very', 'vest', 'veto', 'vice', 'view', 'vine', 'void', 'vote',
    'wade', 'wage', 'wake', 'wall', 'wand', 'ward', 'warm', 'warn', 'wave', 'wavy', 'weak',
    'weed', 'week', 'well', 'went', 'were', 'west', 'what', 'when', 'whip', 'whom', 'wide',
    'wife', 'wild', 'will', 'wind', 'wine', 'wing', 'wink', 'wipe', 'wire', 'wise', 'with',
    'woke', 'wolf', 'womb', 'wood', 'wool', 'word', 'wore', 'worn', 'wrap', 'yard', 'yarn',
    'year', 'yoga', 'your', 'zero', 'zone', 'zoom'
  ];

  // Additional longer base words
  const additionalBases = [
    'accept', 'access', 'account', 'achieve', 'acquire', 'action', 'active', 'actual',
    'adapt', 'adjust', 'admin', 'admit', 'adopt', 'adult', 'advance', 'advice', 'affect',
    'afford', 'afraid', 'after', 'again', 'agent', 'agree', 'ahead', 'alarm', 'album',
    'alert', 'alive', 'allow', 'almost', 'alone', 'along', 'alpha', 'alter', 'always',
    'amaze', 'among', 'amount', 'angle', 'angry', 'animal', 'annual', 'answer', 'anyone',
    'appeal', 'apply', 'argue', 'arise', 'around', 'arrest', 'arrive', 'artist', 'aspect',
    'assert', 'assess', 'assign', 'assist', 'assume', 'assure', 'attach', 'attack', 'attend',
    'august', 'author', 'autumn', 'avenue', 'avoid', 'awake', 'aware', 'baby', 'back',
    'badge', 'badly', 'baker', 'balance', 'ball', 'banana', 'band', 'bank', 'banner',
    'barely', 'barrel', 'barrier', 'basic', 'basket', 'battle', 'beach', 'bean', 'bear',
    'beast', 'beat', 'beauty', 'became', 'become', 'before', 'began', 'behalf', 'behave',
    'behind', 'being', 'belief', 'belong', 'below', 'bench', 'beneath', 'benefit', 'beside',
    'best', 'better', 'between', 'beyond', 'bicycle', 'bigger', 'billion', 'binary', 'biology',
    'bird', 'birth', 'bishop', 'bitter', 'black', 'blade', 'blame', 'blank', 'blast',
    'bleed', 'blend', 'bless', 'blind', 'block', 'blood', 'bloom', 'board', 'boost',
    'border', 'boring', 'borrow', 'boss', 'bottle', 'bottom', 'bought', 'bounce', 'bound',
    'brain', 'branch', 'brand', 'brave', 'bread', 'breath', 'breed', 'breeze', 'brick',
    'bridge', 'brief', 'bright', 'bring', 'broad', 'broken', 'bronze', 'brother', 'brought',
    'brown', 'brush', 'bubble', 'bucket', 'budget', 'buffer', 'bullet', 'bundle', 'burden',
    'bureau', 'burial', 'burned', 'burst', 'button', 'buyer', 'cabin', 'cable', 'cache',
    'camera', 'campus', 'canal', 'cancel', 'cancer', 'candle', 'candy', 'cannon', 'canvas',
    'canyon', 'capable', 'capital', 'captain', 'capture', 'carbon', 'career', 'careful',
    'cargo', 'carpet', 'carrot', 'carry', 'carve', 'castle', 'casual', 'catch', 'cattle',
    'caught', 'ceiling', 'cement', 'census', 'center', 'central', 'century', 'cereal',
    'certain', 'chain', 'chair', 'chalk', 'chamber', 'chance', 'change', 'channel', 'chaos',
    'chapter', 'charge', 'charity', 'charm', 'chart', 'chase', 'cheap', 'check', 'cheese',
    'cherry', 'chest', 'chicken', 'chief', 'child', 'choice', 'choose', 'chosen', 'chrome',
    'chunk', 'church', 'circle', 'circuit', 'citizen', 'civil', 'claim', 'clamp', 'class',
    'classic', 'clause', 'clean', 'clear', 'clever', 'client', 'cliff', 'climate', 'climb',
    'clinic', 'clock', 'clone', 'close', 'closet', 'cloth', 'cloud', 'clown', 'cluster',
    'coach', 'coast', 'cobalt', 'cocktail', 'cocoa', 'coffee', 'cohort', 'collar', 'collect',
    'college', 'colony', 'color', 'column', 'combat', 'combine', 'comedy', 'comfort', 'comic',
    'coming', 'command', 'comment', 'commit', 'common', 'compact', 'company', 'compare',
    'compete', 'compile', 'complex', 'compose', 'compute', 'concept', 'concern', 'concert',
    'condemn', 'conduct', 'confirm', 'conflict', 'confuse', 'connect', 'consent', 'consist',
    'console', 'constant', 'construct', 'consult', 'consume', 'contact', 'contain', 'content',
    'contest', 'context', 'continue', 'contract', 'contrast', 'control', 'convert', 'convey',
    'convince', 'cookie', 'copper', 'corner', 'correct', 'cosmic', 'cotton', 'council',
    'count', 'counter', 'country', 'county', 'couple', 'courage', 'course', 'court', 'cousin',
    'cover', 'crack', 'craft', 'crash', 'crater', 'crazy', 'cream', 'create', 'credit',
    'cricket', 'crime', 'crisp', 'critic', 'cross', 'crowd', 'crown', 'crucial', 'crude',
    'cruise', 'crumb', 'crush', 'crystal', 'culture', 'curious', 'current', 'cursor',
    'curve', 'custom', 'cycle', 'daily', 'damage', 'dance', 'danger', 'daring', 'darker',
    'debate', 'debris', 'decade', 'decent', 'decide', 'declare', 'decline', 'decode',
    'defeat', 'defend', 'define', 'degree', 'delay', 'delete', 'deliver', 'demand', 'depart',
    'depend', 'depict', 'deploy', 'deposit', 'depth', 'deputy', 'derive', 'describe', 'desert',
    'design', 'desire', 'destroy', 'detail', 'detect', 'develop', 'device', 'devote', 'diagram',
    'diamond', 'diary', 'differ', 'digital', 'dinner', 'direct', 'disable', 'disagree',
    'discard', 'discuss', 'disease', 'dismiss', 'display', 'dispose', 'dispute', 'distance',
    'distant', 'distort', 'disturb', 'divide', 'divine', 'doctor', 'domain', 'donate',
    'double', 'doubt', 'dozen', 'draft', 'dragon', 'drama', 'drastic', 'drawer', 'dream',
    'dress', 'drift', 'drill', 'drink', 'driver', 'during', 'dynamic', 'eager', 'early',
    'earth', 'easily', 'eastern', 'economy', 'editor', 'educate', 'effect', 'effort',
    'either', 'elbow', 'elder', 'elect', 'element', 'eleven', 'elite', 'embark', 'embrace',
    'emerge', 'emotion', 'empire', 'employ', 'empty', 'enable', 'enact', 'encode', 'encounter',
    'endless', 'endorse', 'endure', 'enemy', 'energy', 'enforce', 'engage', 'engine',
    'enhance', 'enjoy', 'enlist', 'enough', 'enrich', 'enroll', 'ensure', 'enter', 'entire',
    'entry', 'episode', 'equal', 'equip', 'erase', 'error', 'escape', 'essay', 'essence',
    'estate', 'eternal', 'ethics', 'ethnic', 'evade', 'event', 'evolve', 'exact', 'examine',
    'example', 'exceed', 'excel', 'except', 'excess', 'excite', 'exclude', 'excuse', 'execute',
    'exempt', 'exercise', 'exhale', 'exhibit', 'exile', 'exist', 'exotic', 'expand', 'expect',
    'expense', 'expert', 'explain', 'explode', 'explore', 'export', 'expose', 'express',
    'extend', 'extent', 'extract', 'extreme', 'fabric', 'facial', 'factor', 'faculty',
    'failed', 'fairly', 'faith', 'falcon', 'fallen', 'false', 'family', 'famous', 'fancy',
    'fantasy', 'farmer', 'fashion', 'faster', 'father', 'fatigue', 'fault', 'favor', 'feature',
    'federal', 'female', 'fence', 'festival', 'fetch', 'fever', 'fiber', 'fiction', 'field',
    'fierce', 'fifteen', 'fifty', 'fight', 'figure', 'filter', 'final', 'finance', 'finger',
    'finish', 'fiscal', 'fitness', 'fixed', 'flame', 'flash', 'flavor', 'flight', 'float',
    'floor', 'floral', 'flour', 'flower', 'fluid', 'flush', 'focus', 'follow', 'forbid',
    'force', 'foreign', 'forest', 'forever', 'forget', 'forgive', 'formal', 'format',
    'former', 'formula', 'fortune', 'forward', 'fossil', 'foster', 'found', 'founder',
    'fourth', 'frame', 'freedom', 'freeze', 'french', 'frequent', 'fresh', 'friday', 'friend',
    'fringe', 'front', 'frozen', 'fruit', 'fulfill', 'function', 'funny', 'furnish', 'further',
    'future', 'gadget', 'galaxy', 'gallery', 'gamble', 'garage', 'garbage', 'garden',
    'garlic', 'garment', 'gather', 'gauge', 'gender', 'general', 'genetic', 'genius',
    'gentle', 'genuine', 'gesture', 'ghost', 'giant', 'glance', 'glass', 'global', 'glory',
    'glove', 'golden', 'gossip', 'govern', 'grace', 'grade', 'grain', 'grand', 'grant',
    'grape', 'graph', 'grasp', 'grass', 'grateful', 'gravity', 'great', 'green', 'greet',
    'grief', 'grocery', 'ground', 'group', 'growth', 'guard', 'guess', 'guest', 'guide',
    'guilty', 'guitar', 'habit', 'hammer', 'handle', 'happen', 'happy', 'harbor', 'hardly',
    'harvest', 'hasten', 'hazard', 'health', 'heart', 'heaven', 'heavy', 'height', 'helmet',
    'helpful', 'herald', 'herbs', 'hidden', 'higher', 'highly', 'history', 'hobby', 'hockey',
    'holder', 'hollow', 'honest', 'honey', 'honor', 'horizon', 'horror', 'horse', 'hospital',
    'hotel', 'house', 'human', 'humble', 'humor', 'hundred', 'hunger', 'hunter', 'hurdle',
    'hurry', 'husband', 'hybrid', 'iceberg', 'ideal', 'identify', 'ignore', 'illegal',
    'illness', 'image', 'imagine', 'immune', 'impact', 'impart', 'imply', 'import', 'impose',
    'improve', 'impulse', 'include', 'income', 'indeed', 'index', 'indoor', 'induce',
    'infant', 'infect', 'infer', 'inflict', 'inform', 'inhale', 'inject', 'injure', 'injury',
    'inmate', 'inner', 'input', 'inquire', 'insect', 'insert', 'inside', 'insight', 'insist',
    'inspect', 'inspire', 'install', 'instant', 'instead', 'instinct', 'insult', 'intact',
    'intake', 'intend', 'intent', 'invest', 'invite', 'involve', 'island', 'isolate', 'issue',
    'jacket', 'jaguar', 'january', 'jargon', 'jasmine', 'jealous', 'jeans', 'jelly',
    'jewel', 'jockey', 'joint', 'journal', 'journey', 'judge', 'juice', 'jumble', 'jungle',
    'junior', 'justice', 'kernel', 'kettle', 'kidney', 'killer', 'kindle', 'kingdom',
    'kitchen', 'kitten', 'knife', 'knight', 'ladder', 'lagoon', 'laptop', 'large', 'laser',
    'latest', 'latin', 'latter', 'laugh', 'launch', 'laundry', 'lavish', 'lawsuit', 'lawyer',
    'layer', 'layout', 'leader', 'league', 'leave', 'lecture', 'ledger', 'legacy', 'legal',
    'legend', 'legion', 'leisure', 'lemon', 'length', 'lesson', 'letter', 'level', 'lever',
    'liable', 'liberty', 'library', 'license', 'light', 'likely', 'limit', 'linger', 'liquid',
    'listen', 'litter', 'little', 'lively', 'living', 'lizard', 'local', 'locate', 'locker',
    'lodge', 'logic', 'lonely', 'loose', 'lotion', 'lottery', 'lounge', 'lovely', 'lower',
    'loyal', 'lucky', 'lumber', 'lunar', 'lunch', 'luxury', 'lyrics', 'machine', 'magic',
    'magnet', 'maiden', 'major', 'makeup', 'mammal', 'manage', 'mandate', 'mango', 'manner',
    'manual', 'maple', 'marble', 'march', 'margin', 'marine', 'market', 'marriage', 'martial',
    'martin', 'marvel', 'master', 'match', 'material', 'matrix', 'matter', 'mature', 'maximum',
    'mayor', 'meadow', 'measure', 'medal', 'media', 'medical', 'medium', 'member', 'memory',
    'mental', 'mentor', 'mercy', 'merge', 'merit', 'merry', 'message', 'metal', 'method',
    'metric', 'middle', 'migrate', 'military', 'mimic', 'mineral', 'minimal', 'minimum',
    'minor', 'minute', 'miracle', 'mirror', 'misery', 'missile', 'mission', 'mistake',
    'mixture', 'mobile', 'model', 'modern', 'modest', 'modify', 'module', 'moment', 'monday',
    'money', 'monitor', 'monkey', 'monster', 'month', 'moral', 'morning', 'mortal', 'mosaic',
    'mother', 'motion', 'motor', 'mountain', 'mouse', 'mouth', 'movie', 'moving', 'murder',
    'muscle', 'museum', 'music', 'mutual', 'myself', 'mystery', 'mystic', 'naive', 'naked',
    'namely', 'napkin', 'narrow', 'nasty', 'nation', 'native', 'nature', 'nausea', 'nearby',
    'nearly', 'nectar', 'needle', 'negative', 'neglect', 'neighbor', 'neither', 'nephew',
    'nerve', 'nested', 'network', 'neural', 'neutral', 'never', 'newest', 'newly', 'nibble',
    'nickel', 'night', 'nimble', 'noble', 'nobody', 'noise', 'nominal', 'normal', 'north',
    'nostril', 'notable', 'nothing', 'notice', 'notion', 'novel', 'nuclear', 'nudge',
    'number', 'nurse', 'nurture', 'object', 'oblige', 'obscure', 'observe', 'obtain',
    'obvious', 'occupy', 'occur', 'ocean', 'october', 'oddly', 'offer', 'office', 'offset',
    'often', 'olive', 'olympic', 'omega', 'onion', 'online', 'onset', 'opaque', 'opening',
    'operate', 'opinion', 'oppose', 'option', 'orange', 'orbit', 'orchard', 'order',
    'ordinary', 'organ', 'orient', 'origin', 'orphan', 'other', 'ought', 'ounce', 'outer',
    'outfit', 'output', 'outrage', 'outside', 'overall', 'overlap', 'owner', 'oxygen',
    'oyster', 'ozone', 'pacific', 'package', 'packet', 'paddle', 'palace', 'panel',
    'panic', 'paper', 'parade', 'parent', 'parish', 'parlor', 'parrot', 'party', 'passage',
    'passion', 'passive', 'pastel', 'pastor', 'patent', 'patient', 'patrol', 'patron',
    'pattern', 'pause', 'payment', 'peace', 'peach', 'peanut', 'peasant', 'pebble',
    'pedal', 'penalty', 'pencil', 'pending', 'penguin', 'pension', 'people', 'pepper',
    'percent', 'perfect', 'perform', 'perfume', 'perhaps', 'period', 'permit', 'person',
    'phase', 'phone', 'photo', 'phrase', 'physical', 'piano', 'picnic', 'picture', 'piece',
    'pigeon', 'pillar', 'pillow', 'pilot', 'pimple', 'pioneer', 'pirate', 'pistol',
    'pitch', 'pivot', 'pixel', 'pizza', 'place', 'plague', 'plain', 'planet', 'plank',
    'plant', 'plastic', 'plate', 'platform', 'player', 'please', 'pledge', 'plenty',
    'pliers', 'plunge', 'plush', 'pocket', 'poetry', 'point', 'poison', 'polar', 'police',
    'policy', 'polish', 'polite', 'pollen', 'ponder', 'popular', 'porch', 'portal',
    'portion', 'portrait', 'position', 'positive', 'possess', 'possible', 'postal',
    'poster', 'potato', 'pottery', 'poultry', 'pound', 'poverty', 'powder', 'power',
    'praise', 'prayer', 'preach', 'precise', 'predict', 'prefer', 'prefix', 'premier',
    'premium', 'prepare', 'present', 'preserve', 'preside', 'press', 'presume', 'pretend',
    'pretty', 'prevent', 'preview', 'price', 'pride', 'priest', 'primary', 'prince',
    'print', 'prior', 'prison', 'privacy', 'private', 'prize', 'problem', 'proceed',
    'process', 'produce', 'product', 'profile', 'profit', 'program', 'project', 'promise',
    'promote', 'prompt', 'proof', 'proper', 'property', 'prophet', 'propose', 'prosper',
    'protect', 'protest', 'proud', 'prove', 'provide', 'province', 'public', 'publish',
    'puddle', 'pumpkin', 'punish', 'pupil', 'puppet', 'purchase', 'purple', 'purpose',
    'pursue', 'puzzle', 'pyramid', 'quality', 'quantum', 'quarter', 'queen', 'query',
    'quest', 'question', 'quick', 'quiet', 'quilt', 'quirk', 'quota', 'quote', 'rabbit',
    'racial', 'racing', 'radar', 'radial', 'radiant', 'radical', 'radio', 'radius',
    'raffle', 'raging', 'railway', 'rainbow', 'rainy', 'rally', 'ramble', 'rampant',
    'ranch', 'random', 'range', 'rapid', 'rarely', 'rascal', 'rather', 'rating', 'ratio',
    'rattle', 'ravage', 'raven', 'razor', 'reach', 'react', 'reader', 'ready', 'realm',
    'reaper', 'reason', 'rebel', 'rebuild', 'recall', 'recede', 'receive', 'recent',
    'recipe', 'recite', 'record', 'recover', 'recruit', 'recycle', 'reduce', 'refer',
    'refine', 'reflect', 'reform', 'refuge', 'refuse', 'regard', 'regime', 'region',
    'regret', 'regular', 'reject', 'relate', 'relax', 'release', 'relevant', 'relief',
    'relish', 'remain', 'remark', 'remedy', 'remind', 'remote', 'remove', 'render',
    'renew', 'rental', 'repair', 'repeat', 'replace', 'reply', 'report', 'request',
    'rescue', 'research', 'resemble', 'reserve', 'reside', 'resist', 'resolve', 'resort',
    'respect', 'respond', 'restore', 'result', 'resume', 'retail', 'retain', 'retire',
    'retreat', 'return', 'reveal', 'revenue', 'review', 'revise', 'revolt', 'reward',
    'rhythm', 'ribbon', 'riddle', 'rifle', 'right', 'rigid', 'ring', 'rinse', 'ripple',
    'rising', 'ritual', 'rival', 'river', 'robust', 'rocket', 'rodent', 'roller', 'romance',
    'rookie', 'roster', 'rotate', 'rotten', 'rough', 'round', 'router', 'routine',
    'royal', 'rubber', 'rubble', 'rudder', 'rugby', 'rugged', 'rumble', 'rumor', 'runner',
    'runway', 'rural', 'rustic', 'sacred', 'saddle', 'sadly', 'safari', 'safety', 'sailor',
    'saint', 'salad', 'salary', 'salmon', 'salon', 'salute', 'sample', 'sandal', 'sanity',
    'sardine', 'satin', 'satisfy', 'sauce', 'savage', 'saving', 'savor', 'scale', 'scandal',
    'scarce', 'scared', 'scatter', 'scene', 'scenic', 'scheme', 'scholar', 'school',
    'science', 'scissors', 'scope', 'score', 'scotch', 'scout', 'scramble', 'scrap',
    'scratch', 'scream', 'screen', 'script', 'scroll', 'search', 'season', 'second',
    'secret', 'section', 'sector', 'secure', 'seduce', 'segment', 'select', 'seller',
    'senate', 'senior', 'sense', 'sentence', 'separate', 'sequel', 'series', 'serious',
    'sermon', 'servant', 'serve', 'service', 'session', 'settle', 'setup', 'seven',
    'several', 'severe', 'shadow', 'shaft', 'shake', 'shallow', 'shame', 'shape', 'share',
    'shark', 'sharp', 'shatter', 'shave', 'sheep', 'sheer', 'shelf', 'shell', 'shelter',
    'sheriff', 'shield', 'shift', 'shine', 'shiny', 'shirt', 'shiver', 'shock', 'shoot',
    'shore', 'short', 'should', 'shout', 'shove', 'shower', 'shred', 'shrewd', 'shrink',
    'shrug', 'shuffle', 'shutter', 'sibling', 'sight', 'signal', 'silent', 'silly',
    'silver', 'similar', 'simple', 'sincere', 'single', 'sister', 'situate', 'sixteen',
    'sixty', 'skate', 'sketch', 'skill', 'skinny', 'skull', 'slang', 'slash', 'slate',
    'slave', 'sleek', 'sleep', 'sleeve', 'slender', 'slice', 'slide', 'slight', 'slope',
    'sloth', 'slowly', 'slump', 'small', 'smart', 'smash', 'smell', 'smile', 'smoke',
    'smooth', 'smuggle', 'snack', 'snake', 'snatch', 'sneak', 'sneeze', 'sniff', 'snore',
    'snort', 'snowy', 'snuggle', 'soccer', 'social', 'socket', 'sodium', 'solar', 'soldier',
    'solid', 'solitary', 'solve', 'somber', 'someone', 'something', 'somewhat', 'somewhere',
    'sonic', 'soothe', 'soprano', 'sorrow', 'sorry', 'sound', 'source', 'south', 'space',
    'spare', 'spark', 'spatial', 'spawn', 'speak', 'spear', 'special', 'species', 'specific',
    'speech', 'speed', 'spell', 'spend', 'sphere', 'spice', 'spider', 'spike', 'spill',
    'spine', 'spiral', 'spirit', 'splash', 'split', 'spoil', 'sponge', 'sponsor', 'spoon',
    'sport', 'spray', 'spread', 'spring', 'sprint', 'sprout', 'square', 'squash', 'squeeze',
    'squint', 'squirrel', 'stable', 'stack', 'stadium', 'staff', 'stage', 'stain', 'stairs',
    'stake', 'stale', 'stamp', 'stance', 'stand', 'staple', 'stare', 'start', 'startle',
    'state', 'static', 'station', 'statue', 'status', 'steady', 'steak', 'steal', 'steam',
    'steel', 'steep', 'steer', 'stellar', 'stereo', 'stern', 'stick', 'sticky', 'stiff',
    'still', 'sting', 'stitch', 'stock', 'stomach', 'stone', 'stool', 'store', 'storm',
    'story', 'stove', 'straight', 'strain', 'strand', 'strange', 'strap', 'straw',
    'stream', 'street', 'stress', 'stretch', 'stride', 'strike', 'string', 'strip',
    'stripe', 'strive', 'stroke', 'strong', 'struck', 'structure', 'struggle', 'stubborn',
    'student', 'studio', 'study', 'stuff', 'stumble', 'stump', 'stupid', 'sturdy',
    'style', 'subject', 'submit', 'subset', 'substance', 'subtle', 'suburb', 'subway',
    'succeed', 'success', 'sudden', 'suffer', 'sugar', 'suggest', 'suicide', 'suite',
    'sulfur', 'sultan', 'summary', 'summer', 'summit', 'summon', 'sunday', 'sunny',
    'sunset', 'super', 'supper', 'supply', 'support', 'suppose', 'supreme', 'surface',
    'surge', 'surplus', 'surprise', 'surround', 'survey', 'survive', 'suspect', 'sustain',
    'swallow', 'swamp', 'swarm', 'swear', 'sweat', 'sweater', 'sweep', 'sweet', 'swell',
    'swift', 'swing', 'switch', 'sword', 'symbol', 'symptom', 'synapse', 'syntax',
    'syrup', 'system', 'table', 'tablet', 'tackle', 'tactic', 'tailor', 'talent', 'tangle',
    'target', 'tariff', 'taste', 'tattoo', 'tavern', 'teach', 'tease', 'tedious', 'teeth',
    'temple', 'tempt', 'tenant', 'tender', 'tennis', 'tense', 'tenure', 'terrain',
    'terror', 'thank', 'theater', 'theft', 'theme', 'theory', 'therapy', 'there', 'thermal',
    'thigh', 'thing', 'think', 'third', 'thirst', 'thirty', 'thorn', 'those', 'though',
    'thought', 'thread', 'threat', 'three', 'thrill', 'thrive', 'throat', 'throne',
    'through', 'throw', 'thrust', 'thumb', 'thunder', 'ticket', 'tidal', 'tiger', 'tight',
    'timber', 'timely', 'timing', 'tingle', 'tinker', 'tissue', 'titan', 'title', 'toast',
    'tobacco', 'today', 'toddle', 'toilet', 'token', 'tomato', 'tomorrow', 'tongue',
    'tonight', 'topaz', 'topic', 'torch', 'tornado', 'torque', 'torrent', 'torture',
    'total', 'touch', 'tough', 'tourist', 'toward', 'towel', 'tower', 'toxic', 'trace',
    'track', 'tractor', 'trade', 'traffic', 'tragic', 'trail', 'train', 'trait', 'tramp',
    'trance', 'transfer', 'transform', 'transit', 'transmit', 'transport', 'trash',
    'trauma', 'travel', 'trawl', 'treasure', 'treat', 'treaty', 'treble', 'tremble',
    'trench', 'trend', 'trial', 'tribal', 'tribe', 'tribute', 'trick', 'trickle', 'tricky',
    'trigger', 'trilogy', 'trinity', 'triple', 'tripod', 'triumph', 'troop', 'trophy',
    'tropic', 'trouble', 'troupe', 'truck', 'trudge', 'truly', 'trumpet', 'trunk',
    'trust', 'truth', 'trying', 'tubing', 'tumble', 'tumor', 'tundra', 'tunnel', 'turban',
    'turkey', 'turnip', 'turtle', 'tutor', 'twelve', 'twenty', 'twice', 'twine', 'twins',
    'twirl', 'twist', 'typical', 'tyrant', 'udder', 'ultra', 'umber', 'umpire', 'unable',
    'unaware', 'unbearable', 'uncertain', 'uncle', 'uncover', 'under', 'understand',
    'unfair', 'unfold', 'unhappy', 'uniform', 'unify', 'union', 'unique', 'unite',
    'universe', 'unjust', 'unknown', 'unless', 'unlike', 'unlock', 'unpaid', 'unrest',
    'unsafe', 'unseen', 'unsure', 'untidy', 'until', 'untrue', 'unused', 'unveil',
    'unwell', 'unwind', 'update', 'upgrade', 'uphill', 'uphold', 'uplift', 'upload',
    'upper', 'upright', 'uproar', 'upset', 'upside', 'upstairs', 'upturn', 'upward',
    'urban', 'urgent', 'usage', 'useful', 'useless', 'usual', 'utmost', 'utter', 'vacant',
    'vaccine', 'vacuum', 'vague', 'valid', 'valley', 'valor', 'value', 'valve', 'vampire',
    'vandal', 'vanilla', 'vanish', 'vanity', 'vapor', 'variable', 'variant', 'variety',
    'various', 'varnish', 'vassal', 'vault', 'vector', 'vehicle', 'velvet', 'vendor',
    'veneer', 'venom', 'venture', 'venue', 'verbal', 'verdict', 'verify', 'verse',
    'version', 'versus', 'vessel', 'veteran', 'viable', 'vibrant', 'vicious', 'victim',
    'victor', 'video', 'viewer', 'vigil', 'viking', 'village', 'villain', 'vintage',
    'violin', 'viral', 'virgin', 'virtue', 'virus', 'viscous', 'visible', 'vision',
    'visit', 'visual', 'vital', 'vivid', 'vocal', 'vodka', 'vogue', 'voice', 'volcano',
    'volume', 'vomit', 'vortex', 'voting', 'vouch', 'vowel', 'voyage', 'vulgar', 'waffle',
    'wager', 'wagon', 'waist', 'waiter', 'wakeful', 'walnut', 'walrus', 'wander', 'waning',
    'warfare', 'warmth', 'warning', 'warrant', 'warrior', 'watery', 'waving', 'wealth',
    'weapon', 'weary', 'weasel', 'weather', 'weaver', 'webbing', 'website', 'wedding',
    'wedge', 'weekly', 'weight', 'weird', 'welcome', 'welfare', 'western', 'wetland',
    'whale', 'wheat', 'wheel', 'whence', 'where', 'which', 'while', 'whine', 'whisper',
    'whistle', 'white', 'whole', 'wicked', 'widget', 'widow', 'width', 'wiggle', 'wildly',
    'willow', 'wimpy', 'windy', 'winner', 'winter', 'wisdom', 'wither', 'within',
    'without', 'witness', 'wizard', 'wobble', 'woman', 'wonder', 'wooden', 'worker',
    'world', 'worry', 'worsen', 'worship', 'worthy', 'would', 'wreath', 'wreck', 'wrestle',
    'wriggle', 'wrinkle', 'wrist', 'writer', 'writing', 'written', 'wrong', 'yacht',
    'yearly', 'yellow', 'yield', 'yogurt', 'yonder', 'young', 'youth', 'zealot', 'zebra',
    'zenith', 'zephyr', 'zigzag', 'zipper', 'zodiac', 'zombie', 'zoning'
  ];

  // Common suffixes
  const suffixes = [
    's', 'es', 'ed', 'ing', 'er', 'est', 'ly', 'ish', 'ness', 'ment', 'ful',
    'less', 'tion', 'sion', 'ity', 'ous', 'ive', 'able', 'ible', 'al', 'ial',
    'y', 'en', 'ify', 'ize', 'ate', 'fy', 'age', 'ance', 'ence', 'dom', 'ee',
    'eer', 'hood', 'ism', 'ist', 'let', 'ling', 'or', 'ry', 'ship', 'th', 
    'ward', 'wise', 'like', 'ware', 'room', 'side', 'way', 'some', 'time',
    'cent', 'fold', 'land', 'load', 'lock', 'most', 'proof', 'sick', 'wide',
    'work', 'yard'
  ];

  // Common prefixes
  const prefixes = [
    'un', 're', 'in', 'dis', 'en', 'non', 'pre', 'pro', 'anti', 'de', 'over',
    'under', 'semi', 'mid', 'mis', 'sub', 'super', 'trans', 'inter', 'fore',
    'ex', 'co', 'auto', 'bi', 'bio', 'geo', 'mono', 'micro', 'multi', 'neo',
    'out', 'post', 'tele', 'up', 'tri', 'ultra', 'uni', 'vice', 'meta', 'mega',
    'mini', 'macro', 'hyper', 'cyber', 'eco', 'electro', 'hydro', 'photo',
    'psycho', 'retro', 'techno', 'thermo'
  ];

  // Generate unique words
  const uniqueWords = new Set();

  // 1. Add all base words
  [...baseWords, ...additionalBases].forEach(word => {
    if (word.length >= 3 && word.length <= 10) {
      uniqueWords.add(word);
    }
  });

  // 2. Add words with suffixes
  [...baseWords, ...additionalBases].forEach(base => {
    suffixes.forEach(suffix => {
      const word = base + suffix;
      if (word.length >= 3 && word.length <= 10) {
        uniqueWords.add(word);
      }
    });
  });

  // 3. Add words with prefixes
  [...baseWords, ...additionalBases].forEach(base => {
    prefixes.forEach(prefix => {
      const word = prefix + base;
      if (word.length >= 3 && word.length <= 10) {
        uniqueWords.add(word);
      }
    });
  });

  // 4. Add words with both prefixes and suffixes
  baseWords.forEach(base => {
    prefixes.forEach(prefix => {
      suffixes.forEach(suffix => {
        const word = prefix + base + suffix;
        if (word.length >= 3 && word.length <= 10) {
          uniqueWords.add(word);
        }
      });
    });
  });

  // 5. Generate compound words
  const shortWords = Array.from(uniqueWords).filter(w => w.length <= 5);
  const targetCount = 66000;

  for (let i = 0; i < shortWords.length && uniqueWords.size < targetCount * 2; i++) {
    for (let j = 0; j < shortWords.length && uniqueWords.size < targetCount * 2; j++) {
      if (i !== j) {
        const compound = shortWords[i] + shortWords[j];
        if (compound.length >= 3 && compound.length <= 10) {
          uniqueWords.add(compound);
        }
      }
    }
  }

  // 6. Add number combinations
  const numbers = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine', 'ten'];
  const digits = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
  
  baseWords.forEach(base => {
    numbers.forEach(num => {
      if (num.length + base.length <= 10) {
        uniqueWords.add(num + base);
        uniqueWords.add(base + num);
      }
    });
    digits.forEach(digit => {
      if (base.length + 1 <= 10) {
        uniqueWords.add(base + digit);
        uniqueWords.add(digit + base);
      }
    });
  });

  // 7. Add double consonant variations
  const consonants = 'bcdfghjklmnpqrstvwxyz';
  const wordArray = Array.from(uniqueWords);
  
  wordArray.forEach(word => {
    if (uniqueWords.size >= targetCount * 2) return;
    for (let cons of consonants) {
      if (word.includes(cons) && !word.includes(cons + cons) && word.length < 10) {
        const doubled = word.replace(cons, cons + cons);
        if (doubled.length <= 10) {
          uniqueWords.add(doubled);
        }
      }
    }
  });

  // 8. Add 'y' ending variations
  wordArray.forEach(word => {
    if (uniqueWords.size >= targetCount * 2) return;
    if (!word.endsWith('y') && word.length < 10) {
      uniqueWords.add(word + 'y');
    }
  });

  // 9. Add past tense variations
  const pastEndings = ['d', 'ed', 't'];
  wordArray.forEach(word => {
    if (uniqueWords.size >= targetCount * 2) return;
    pastEndings.forEach(ending => {
      const past = word + ending;
      if (past.length <= 10 && !uniqueWords.has(past)) {
        uniqueWords.add(past);
      }
    });
  });

  // 10. Add letter combination variations
  const letterCombos = ['er', 'ar', 'or', 'en', 'an', 'on', 'in', 'im', 'em', 'el', 'le', 'al'];
  wordArray.forEach(word => {
    if (uniqueWords.size >= targetCount * 2) return;
    letterCombos.forEach(combo => {
      if (word.length + combo.length <= 10) {
        uniqueWords.add(word + combo);
      }
    });
  });

  // Convert to array, sort, and take exactly 66000
  const finalWords = Array.from(uniqueWords).sort();
  return finalWords.slice(0, targetCount);
}

// Generate the words
console.log('Generating 66,000 unique words...');
const words = generateWordList();

// Create file content
const fileContent = `# List of 66,000 unique words (3-10 characters)
# Generated on ${new Date().toISOString()}

${words.join('\n')}`;

// Write to file
const outputPath = path.join(process.cwd(), 'unique_words_66000.txt');
fs.writeFileSync(outputPath, fileContent, 'utf8');

console.log(`✓ Successfully generated ${words.length} unique words`);
console.log(`✓ File saved to: ${outputPath}`);
console.log(`✓ File size: ${(fileContent.length / 1024).toFixed(2)} KB`);
console.log(`\nFirst 10 words: ${words.slice(0, 10).join(', ')}`);
console.log(`Last 10 words: ${words.slice(-10).join(', ')}`);
