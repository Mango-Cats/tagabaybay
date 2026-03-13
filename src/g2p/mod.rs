//! G2P (Grapheme-to-Phoneme) conversion modules.
//!

pub mod common;
pub mod ipa;

// Re-export public APIs for convenience
pub use common::phonemize_phrase;
pub use ipa::G2Py;
