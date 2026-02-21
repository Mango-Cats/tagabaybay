pub mod alignment;
pub mod aligned_string;

// Re-export alignment functions
pub use alignment::{
    phoneme_grapheme_alignment,
    phoneme_grapheme_alignment_with_morphology,
    rebuild_and_align_from_morphology,
};