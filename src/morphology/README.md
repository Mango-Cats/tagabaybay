# English Morphology
=============================

This module provides morpheme segmentation with **deadlock prevention** and **thread-safe** operation.

**Two approaches:**
- **Dictionary-based** - Fast, rule-based segmentation (no dependencies)
- **spaCy-based** - Advanced NLP with automatic lemmatization and morphological features 

## Deadlock Prevention Features

✅ **Mutex-protected process spawning** - Prevents concurrent Python process conflicts  
✅ **Thread-safe model loading** - Double-checked locking in Python  
✅ **Timeout handling** - Prevents hanging processes  
✅ **Process isolation** - Each call uses stdin=null to prevent blocking

## Setup

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
// Dictionary-based approach (no dependencies, always available)
let morphemes = segment_morphemes("unhappily")?;

// spaCy-based approach (requires spaCy installation)
let morphemes = segment_morphemes_spacy("running")?;
```

## Concurrency Safety

Both functions are safe to call from multiple threads. A global mutex serializes Python process calls to prevent deadlocks and resource contention.