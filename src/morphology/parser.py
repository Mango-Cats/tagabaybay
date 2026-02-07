import os

try:
    import spacy
    from spacy.cli import download
    SPACY_AVAILABLE = True
except ImportError:
    SPACY_AVAILABLE = False

# Global spaCy model
_nlp = None

def load_spacy_model(model_name="en_core_web_sm"):
    """Loads the spaCy model for morphological analysis."""
    global _nlp
    
    if not SPACY_AVAILABLE:
        raise ImportError("spaCy is not installed. Install with: pip install spacy")
    
    if _nlp is not None:
        return _nlp
    
    try:
        _nlp = spacy.load(model_name)
    except OSError:
        print(f"Model '{model_name}' not found. Downloading...")
        download(model_name)
        _nlp = spacy.load(model_name)
    
    return _nlp

def segment_spacy(word, model=None):
    """Returns morphological analysis of a word using spaCy.
    
    Returns a list containing:
    - lemma (base form)
    - morphological features as strings
    """
    if model is None:
        model = load_spacy_model()
    
    # Process the word
    doc = model(word)
    
    if len(doc) == 0:
        return [word]
    
    token = doc[0]
    
    # Build morpheme representation
    morphemes = []
    
    # Add lemma as the base form
    lemma = token.lemma_
    
    # Extract morphological features
    morph_features = []
    if token.morph:
        for feature in token.morph:
            morph_features.append(str(feature))
    
    # If word differs from lemma, show the transformation
    if word.lower() != lemma.lower():
        morphemes.append(lemma)
        if morph_features:
            morphemes.extend(morph_features)
    else:
        # Word is already in base form
        morphemes.append(word)
        if morph_features:
            morphemes.extend(morph_features)
    
    return morphemes if morphemes else [word]

def load_model(path):
    """Loads morpheme rules from a dictionary file."""
    if not os.path.exists(path):
        raise FileNotFoundError(f"Model file not found: {path}")
    
    morpheme_dict = {}
    with open(path, 'r') as f:
        for line in f:
            line = line.strip()
            if line and ' -> ' in line:
                word, morphemes = line.split(' -> ')
                morpheme_dict[word] = morphemes.split()
    
    return morpheme_dict

def extract_morpheme_patterns(model):
    """Extract common prefixes and suffixes from the model."""
    prefixes = set()
    suffixes = set()
    
    for word, morphemes in model.items():
        if len(morphemes) > 1:
            # First morpheme might be a prefix
            if len(morphemes[0]) <= 5:  # Prefixes are usually short
                prefixes.add(morphemes[0])
            
            # Last morpheme might be a suffix
            if len(morphemes[-1]) <= 5:  # Suffixes are usually short
                suffixes.add(morphemes[-1])
    
    return prefixes, suffixes

def segment_unknown(word, prefixes, suffixes):
    """Attempt to segment an unknown word using learned patterns."""
    morphemes = []
    remaining = word
    
    # Try to strip prefixes
    for prefix in sorted(prefixes, key=len, reverse=True):
        if remaining.startswith(prefix) and len(remaining) > len(prefix):
            morphemes.append(prefix)
            remaining = remaining[len(prefix):]
            break
    
    # Try to strip suffixes
    for suffix in sorted(suffixes, key=len, reverse=True):
        if remaining.endswith(suffix) and len(remaining) > len(suffix):
            # Check if there's a root left
            root = remaining[:-len(suffix)]
            if len(root) >= 2:  # Root should be at least 2 characters
                morphemes.append(root)
                morphemes.append(suffix)
                return morphemes
    
    # If we found a prefix but no suffix, add the remaining as root
    if morphemes:
        morphemes.append(remaining)
        return morphemes
    
    # No segmentation found, return word as-is
    return [word]

def segment(word, model):
    """Returns a list of morphemes for a given word."""
    # Check if word is in dictionary
    if word in model:
        return model[word]
    
    # Extract morpheme patterns from model
    prefixes, suffixes = extract_morpheme_patterns(model)
    
    # Try to segment the unknown word
    return segment_unknown(word, prefixes, suffixes)