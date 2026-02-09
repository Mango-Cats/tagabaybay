from parser import segment_spacy, load_spacy_model, SPACY_AVAILABLE

# Test spaCy-based segmentation if available
if SPACY_AVAILABLE:
    print("\n\n" + "="*60)
    print("=== spaCy-based Morphological Analysis ===")
    print("="*60)
    try:
        spacy_model = load_spacy_model()
        print("spaCy model loaded successfully!\n")
        
        comparison_words = ['running', 'walked', 'unhappiness', 'children', 'better', 'quickly']
        
        print("spaCy-based Morphological Analysis")
        print("-" * 60)
        for word in comparison_words:
            spacy_result = segment_spacy(word, spacy_model)
            print(f"{word:15} spaCy: {' | '.join(spacy_result)}")
    except Exception as e:
        print(f"Could not load spaCy model: {e}")
        print("Install with: pip install spacy && python -m spacy download en_core_web_sm")
else:
    print("\n\nspaCy not available. Install with: pip install spacy")