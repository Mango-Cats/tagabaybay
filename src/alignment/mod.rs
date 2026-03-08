pub mod alignment;
pub mod aligned_string;
pub mod morphology_alignment;

// Re-export alignment functions
pub use alignment::{
    phoneme_grapheme_alignment,
};

pub use morphology_alignment::{
    phoneme_grapheme_alignment_with_morphology,
    rebuild_and_align_from_morphology,
};