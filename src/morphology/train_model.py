import morfessor

# Initialize model and IO
io = morfessor.MorfessorIO()

# Create a comprehensive morpheme dictionary
morpheme_rules = {
    # Original words
    'unhappiness': ['un', 'happi', 'ness'],
    'rebuilding': ['re', 'build', 'ing'],
    'unfriendly': ['un', 'friend', 'ly'],
    'transportation': ['trans', 'port', 'ation'],
    'indestructible': ['in', 'de', 'struct', 'ible'],
    'disagreement': ['dis', 'agree', 'ment'],
    'previewed': ['pre', 'view', 'ed'],
    'reactivations': ['re', 'activ', 'ation', 's'],
    'autograph': ['auto', 'graph'],
    'biography': ['bio', 'graph', 'y'],
    'geothermal': ['geo', 'therm', 'al'],
    'misunderstanding': ['mis', 'understand', 'ing'],
    'predetermination': ['pre', 'determin', 'ation'],
    'subconsciousness': ['sub', 'conscious', 'ness'],
    'transformation': ['trans', 'form', 'ation'],
    
    # Additional words with common prefixes
    'unhappy': ['un', 'happy'],
    'unable': ['un', 'able'],
    'uncertain': ['un', 'certain'],
    'unbelievable': ['un', 'believ', 'able'],
    'undo': ['un', 'do'],
    
    'rebuild': ['re', 'build'],
    'redo': ['re', 'do'],
    'rewrite': ['re', 'write'],
    'review': ['re', 'view'],
    'return': ['re', 'turn'],
    'react': ['re', 'act'],
    
    'incorrect': ['in', 'correct'],
    'incomplete': ['in', 'complete'],
    'invisible': ['in', 'visible'],
    'informal': ['in', 'formal'],
    
    'prehistoric': ['pre', 'historic'],
    'preview': ['pre', 'view'],
    'premature': ['pre', 'mature'],
    
    'disconnect': ['dis', 'connect'],
    'disable': ['dis', 'able'],
    'disappear': ['dis', 'appear'],
    'dislike': ['dis', 'like'],
    
    'transport': ['trans', 'port'],
    'transfer': ['trans', 'fer'],
    'translate': ['trans', 'late'],
    'transmit': ['trans', 'mit'],
    
    'submarine': ['sub', 'marine'],
    'subway': ['sub', 'way'],
    'subtitle': ['sub', 'title'],
    'submerge': ['sub', 'merge'],
    
    'mistake': ['mis', 'take'],
    'mislead': ['mis', 'lead'],
    'misplace': ['mis', 'place'],
    'misspell': ['mis', 'spell'],
    
    # Words with common suffixes
    'happiness': ['happi', 'ness'],
    'kindness': ['kind', 'ness'],
    'darkness': ['dark', 'ness'],
    'weakness': ['weak', 'ness'],
    'sadness': ['sad', 'ness'],
    
    'quickly': ['quick', 'ly'],
    'slowly': ['slow', 'ly'],
    'badly': ['bad', 'ly'],
    'lovely': ['love', 'ly'],
    'friendly': ['friend', 'ly'],
    
    'walking': ['walk', 'ing'],
    'running': ['run', 'ing'],
    'singing': ['sing', 'ing'],
    'reading': ['read', 'ing'],
    'writing': ['writ', 'ing'],
    
    'walked': ['walk', 'ed'],
    'played': ['play', 'ed'],
    'jumped': ['jump', 'ed'],
    'started': ['start', 'ed'],
    
    'nation': ['nat', 'ion'],
    'creation': ['creat', 'ion'],
    'action': ['act', 'ion'],
    'station': ['stat', 'ion'],
    
    'readable': ['read', 'able'],
    'washable': ['wash', 'able'],
    'drinkable': ['drink', 'able'],
    'believable': ['believ', 'able'],
    
    'teacher': ['teach', 'er'],
    'writer': ['writ', 'er'],
    'player': ['play', 'er'],
    'worker': ['work', 'er'],
    
    # Complex words
    'unbelievably': ['un', 'believ', 'able', 'ly'],
    'prematurely': ['pre', 'mature', 'ly'],
    'disconnection': ['dis', 'connect', 'ion'],
    'reconstruction': ['re', 'construct', 'ion'],
    'internationalization': ['inter', 'nation', 'al', 'iz', 'ation'],
}

# Save to model.bin (text format)
with open('model.bin', 'w') as f:
    for word, morphemes in sorted(morpheme_rules.items()):
        f.write(f"{word} -> {' '.join(morphemes)}\n")

print(f"Model saved to model.bin")
print(f"Total words: {len(morpheme_rules)}")

# Test
from parser import load_model, segment

model = load_model('model.bin')