import sys
import os
from parser import segment_spacy, segment_dict, load_spacy_model, SPACY_AVAILABLE

def main():
    if len(sys.argv) < 2:
        print("Usage: python segment_cli.py [--dict] <word>", file=sys.stderr)
        sys.exit(1)
    
    use_dict = False
    word = sys.argv[1]
    
    if word == "--dict":
        use_dict = True
        if len(sys.argv) < 3:
            print("Usage: python segment_cli.py --dict <word>", file=sys.stderr)
            sys.exit(1)
        word = sys.argv[2]
    
    try:
        # Use dictionary-based segmentation if requested
        if use_dict:
            morphemes = segment_dict(word)
        else:
            # Use spaCy-based segmentation
            if not SPACY_AVAILABLE:
                print("spaCy not installed", file=sys.stderr)
                sys.exit(1)
            model = load_spacy_model()
            morphemes = segment_spacy(word, model)
        
        # Output comma-separated morphemes
        print(','.join(morphemes))
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()