//! Morphology module for handling word formation and morpheme segmentation.
//! 
//! This module provides functionality for segmenting English words into morphemes
//! using either dictionary-based patterns or spaCy-based linguistic analysis.

pub mod spacy;
// pub mod morfessor_morphology;

// Re-export main functions
pub use spacy::{segment_morphemes, segment_morphemes_spacy};
// pub use morfessor_morphology::{segment_morphemes as segment_morphemes_morfessor, segment_morphemes_spacy as segment_morphemes_spacy_morfessor};