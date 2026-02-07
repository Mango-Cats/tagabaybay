import sys
import os
from parser import load_model, segment, segment_spacy, load_spacy_model, SPACY_AVAILABLE

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
        if use_dict:
            # Get the directory where this script is located
            script_dir = os.path.dirname(os.path.abspath(__file__))
            model_path = os.path.join(script_dir, 'model.bin')
            model = load_model(model_path)
            morphemes = segment(word, model)
        else:
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