import os
import threading

try:
    import spacy
    from spacy.cli import download
    SPACY_AVAILABLE = True
except ImportError:
    SPACY_AVAILABLE = False

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
    """Basic dictionary-based morpheme segmentation.
    
    This is a simple rule-based approach that doesn't require spaCy.
    Returns a list of morphemes based on common English affixes.
    """
    morphemes = []
    remaining = word.lower()
    
    # Common prefixes
    prefixes = ["un", "re", "in", "dis", "en", "non", "pre", "post", "anti", "de"]
    # Common suffixes  
    suffixes = ["ing", "ed", "er", "est", "ly", "ness", "ment", "tion", "sion", "ity", "ful", "less", "ous", "ive", "al"]
    
    # Extract prefix
    for prefix in prefixes:
        if remaining.startswith(prefix) and len(remaining) > len(prefix) + 2:
            morphemes.append(prefix)
            remaining = remaining[len(prefix):]
            break
    
    # Extract suffix with special handling for -tion/-sion
    for suffix in suffixes:
        if remaining.endswith(suffix) and len(remaining) > len(suffix) + 2:
            stem = remaining[:-len(suffix)]
            
            # Special case: -tion/-sion often comes from verbs ending in -ate/-ite/-ute/-ize
            # e.g., allocate + ion -> allocation (but we want "allocate" + "tion")
            if suffix in ["tion", "sion"]:
                # Check patterns to restore the proper verb form
                if len(stem) >= 3:
                    # Pattern: ...iza + tion -> ...ize + tion (organize, realize)
                    if stem.endswith("iza"):
                        stem = stem[:-1] + "e"  # iza -> ize
                    # Pattern: ...ca + tion -> ...cate + tion (allocate, communicate, dedicate)
                    # Pattern: ...ga + tion -> ...gate + tion (navigate, delegate)
                    # Pattern: ...ra + tion -> ...rate + tion (operate, generate)
                    # Pattern: ...ta + tion -> ...tate + tion (rotate, imitate)
                    # etc.
                    elif stem.endswith("a") and len(stem) >= 2:
                        # Check if it's likely from an -ate verb (not from -ct verbs)
                        # Most -ation words come from -ate verbs
                        stem = stem + "te"
                    # Pattern: ...i + tion -> ...ite + tion (ignite, excite)
                    elif stem.endswith("i") and not stem.endswith("it"):  # avoid "ition"
                        stem = stem + "te"
                    # Pattern: ...u + tion -> ...ute + tion (contribute, distribute)
                    elif stem.endswith("u"):
                        stem = stem + "te"
                    # Pattern: ...us + ion -> ...use + ion (confuse, diffuse)
                    elif stem.endswith("us"):
                        stem = stem + "e"
                    # Pattern: ...c + tion -> ...ct + tion (construct, destruct, abstract)
                    # The 't' in '-tion' is actually part of the verb stem
                    elif stem.endswith("c") and len(stem) >= 2:
                        stem = stem + "t"
            
            # Special case: -ness often follows adjectives where y->i (happy -> happiness)
            elif suffix == "ness":
                if stem.endswith("i") and len(stem) >= 2:
                    # Check if it's likely from y->i transformation
                    # happi -> happy, easi -> easy, etc.
                    consonants = "bcdfghjklmnpqrstvwxz"
                    if len(stem) >= 2 and stem[-2] in consonants:
                        stem = stem[:-1] + "y"
            
            morphemes.append(stem)
            morphemes.append(suffix)
            return morphemes
    
    # No suffix found, add remaining word
    if morphemes:
        morphemes.append(remaining)
    else:
        morphemes.append(word)
    
    return morphemes