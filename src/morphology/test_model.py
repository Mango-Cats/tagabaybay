from parser import load_model, segment

# Load the model
print("Loading model...")
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