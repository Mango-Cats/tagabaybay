//! Orthographic adaptation rules
//!
//! Rules based on **spelling patterns** (graphemes). These analyze the written
//! form of words to determine appropriate Filipino adaptations.
//!
//! ## Submodules
//!
//! - `free`: Context-free replacements (direct grapheme mappings)
//! - `context`: Context-sensitive consonant and digraph rules
//! - `vowel`: Vowel pattern transformations
//! - `spelling`: Letter-to-phonetic spelling for abbreviations

pub mod free;
pub mod sensitive;
pub mod spelling;
