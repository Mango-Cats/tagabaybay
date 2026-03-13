import os
import threading

try:
    import spacy
    from spacy.cli import download
    SPACY_AVAILABLE = True
except ImportError:
    SPACY_AVAILABLE = False

try:
    from nltk.stem import WordNetLemmatizer
    import nltk
    NLTK_AVAILABLE = True
except ImportError:
    NLTK_AVAILABLE = False

# Global spaCy model with thread-safe loading
_nlp = None
_nlp_lock = threading.Lock()

def load_spacy_model(model_name="en_core_web_sm"):
    """Loads the spaCy model for morphological analysis (thread-safe)."""
    global _nlp
    
    if not SPACY_AVAILABLE:
        raise ImportError("spaCy is not installed. Install with: pip install spacy")
    
    # Double-checked locking pattern for thread safety
    if _nlp is not None:
        return _nlp
    
    with _nlp_lock:
        # Check again inside the lock
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

def segment_dict(word):
    """NLTK-based morpheme segmentation.
    
    Uses NLTK for morphological analysis. While NLTK's WordNetLemmatizer
    handles rea, we use rule-based patterns for
    derivational morphology (-tion, -ation, etc.).
    
    Args:
        word: The word to segment into morphemes
        
    Returns:
        List of morpheme strings
        
    Raises:
        ImportError: If NLTK is not installed
    """
    if not NLTK_AVAILABLE:
        raise ImportError("NLTK is not installed. Install with: pip install nltk")
    
    morphemes = []
    remaining = word.lower()
    
    # Common prefixes
    prefixes = ["un", "re", "in", "dis", "en", "non", "pre", "post", "anti", "de"]
    
    # Extract prefix
    for prefix in prefixes:
        if remaining.startswith(prefix) and len(remaining) > len(prefix) + 2:
            morphemes.append(prefix)
            remaining = remaining[len(prefix):]
            break
    
    # Suffix patterns for derivational morphology
    # Format: (suffix, stem_transformation_func)
    suffixes = [
        ("tion", lambda s: s[:-1] + "e" if s.endswith("iza") else (s + "te" if s.endswith("a") else (s + "t" if s.endswith("c") else s))),
        ("sion", lambda s: s + "e" if s.endswith("us") else s),
        ("ment", lambda s: s),
        ("ness", lambda s: s[:-1] + "y" if s.endswith("i") and len(s) >= 2 and s[-2] in "bcdfghjklmnpqrstvwxz" else s),
        ("ity", lambda s: s),
        ("ing", lambda s: s),
        ("ed", lambda s: s),
        ("ly", lambda s: s),
        ("er", lambda s: s),
        ("est", lambda s: s),
        ("ful", lambda s: s),
        ("less", lambda s: s),
        ("ous", lambda s: s),
        ("ive", lambda s: s),
        ("al", lambda s: s),
    ]
    
    # Extract suffix
    for suffix, transform in suffixes:
        if remaining.endswith(suffix) and len(remaining) > len(suffix) + 2:
            stem = remaining[:-len(suffix)]
            stem = transform(stem)
            
            morphemes.append(stem)
            morphemes.append(suffix)
            return morphemes
    
    # No suffix found, add remaining word
    if morphemes:
        morphemes.append(remaining)
    else:
        morphemes.append(word)
    
    return morphemes