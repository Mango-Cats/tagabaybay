//! G2P (Grapheme-to-Phoneme) conversion modules.
//!
//! This module provides two phonemization backends:
//! - `fst`: Rust-native FST-based ARPA phoneme conversion (deprecated)
//! - `server`: Python espeak-ng based IPA phoneme conversion (recommended)
//! - `util`: Shared utilities for phrase processing

pub mod arpa;
pub mod common;
pub mod ipa;

// Re-export public APIs for convenience
pub use arpa::phonemize_to_arpa;
pub use common::phonemize_phrase;
pub use ipa::G2Py;
