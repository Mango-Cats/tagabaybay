from parser import load_model, segment, segment_spacy, load_spacy_model, SPACY_AVAILABLE

# Load the dictionary model
print("Loading dictionary model...")
model = load_model('model.bin')

# Test all words in the model
print("\n=== Testing Known Words ===")
print("-" * 60)
test_words = [
    'unhappiness', 'rebuilding', 'unfriendly', 'transportation',
    'indestructible', 'disagreement', 'previewed', 'reactivations',
    'autograph', 'biography', 'geothermal', 'misunderstanding',
    'predetermination', 'subconsciousness', 'transformation'
]

for word in test_words:
    morphemes = segment(word, model)
    print(f"{word:20} -> {' + '.join(morphemes)}")

# Test unknown words with recognizable patterns
print("\n=== Testing Unknown Words (auto-segmentation) ===")
print("-" * 60)
unknown_words = [
    'unfinished',      # un + finish + ed
    'restarting',      # re + start + ing
    'mistaken',        # mis + take + n (but 'taken' not 'take')
    'prehistoric',     # Should already be known
    'disagreeable',    # dis + agree + able
    'quickly',         # Should already be known
    'unhappily',       # un + happi + ly
    'rebuild',         # Should already be known
    'uncertain',       # Should already be known
]

for word in unknown_words:
    morphemes = segment(word, model)
    status = "✓" if len(morphemes) > 1 else "○"
    print(f"{status} {word:20} -> {' + '.join(morphemes)}")

# Test words with no recognizable patterns
print("\n=== Testing Truly Unknown Words ===")
print("-" * 60)
unknown_words = ['hello', 'world', 'python', 'morphology']
for word in unknown_words:
    morphemes = segment(word, model)
    print(f"{word:20} -> {' + '.join(morphemes)}")

# Statistics
print("\n=== Statistics ===")
print(f"Total words in model: {len(model)}")
print(f"Average morphemes per word: {sum(len(v) for v in model.values()) / len(model):.2f}")

# Test spaCy-based segmentation if available
if SPACY_AVAILABLE:
    print("\n\n" + "="*60)
    print("=== spaCy-based Morphological Analysis ===")
    print("="*60)
    try:
        spacy_model = load_spacy_model()
        print("spaCy model loaded successfully!\n")
        
        comparison_words = ['running', 'walked', 'unhappiness', 'children', 'better', 'quickly']
        
        print("Comparison: Dictionary vs spaCy")
        print("-" * 60)
        for word in comparison_words:
            dict_result = segment(word, model)
            spacy_result = segment_spacy(word, spacy_model)
            print(f"{word:15} Dict: {' + '.join(dict_result):25} spaCy: {' | '.join(spacy_result)}")
    except Exception as e:
        print(f"Could not load spaCy model: {e}")
        print("Install with: pip install spacy && python -m spacy download en_core_web_sm")
else:
    print("\n\nspaCy not available. Install with: pip install spacy")