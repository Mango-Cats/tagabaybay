# English Morphology
=============================

Two approaches for morpheme segmentation:

1. **Dictionary-based** - Fast, pattern matching for unknown words (default)
2. **spaCy-based** - Advanced NLP with automatic lemmatization and morphological features (optional)

## Setup

### Dictionary-based (default)
```bash
python train_model.py
```

This creates `model.bin` with 85+ morpheme rules.

### spaCy-based (optional)
```bash
pip install spacy
python -m spacy download en_core_web_sm
```

## Test

```rust
cargo test morphology
```

This will test both approaches if spaCy is available.

## Usage in Rust

```rust
// Dictionary-based approach
let morphemes = segment_morphemes("unfriendly")?; // currently not working so always be "running"

// spaCy-based approach (requires spaCy installation)
let morphemes = segment_morphemes_spacy("running")?;
```