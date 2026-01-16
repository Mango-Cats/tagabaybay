//! Phonetic adaptation rules
//!
//! Rules based on **sound patterns** (phonemes). These analyze the pronunciation
//! of words (via G2P/ARPAbet) to determine appropriate Filipino adaptations.
//!
//! Unlike orthographic rules which look at spelling, phonetic rules consider
//! how the word actually sounds, which can differ significantly from spelling
//! (e.g., "knight" → /naɪt/, "phone" → /foʊn/).
//!
//! All phonetic rules are context-free replacements
//!
//! ## Submodules
//!
//! - `vowel`: Phoneme-based vowel handling

pub mod free;
pub mod p2g;
