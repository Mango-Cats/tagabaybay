use super::alignment::AlignedString;
use crate::grapheme::tokenize::source_tokenizer;
use crate::phoneme::tokenizer::ipa::tokenize_ipa;
use crate::alignment::alignment::phoneme_grapheme_alignment;

/// Morpheme-aware Phoneme-Grapheme Alignment
/// 
/// This function handles alignment when the input word has been segmented into morphemes.
/// It processes each morpheme separately and then combines the results.
/// 
/// # Pipeline
/// Input Word → Morphology → G2P (each morpheme) → Alignment (each morpheme) → Combine
/// 
/// # Example
/// For the word "reallocation":
/// - Morphemes: ["re", "allocate", "tion"]
/// - G2P: ["riː", "æləkeɪt", "ʃən"]
/// - Alignment happens per morpheme, then combined
/// - Expected output: riːæləkeɪʃən
/// 
/// # Arguments
/// * `morphemes` - List of morpheme strings
/// * `morpheme_phonemes` - List of IPA phoneme strings for each morpheme
/// * `original_word` - The original unsegmented word
/// 
/// # Returns
/// An AlignedString for the complete word
pub fn phoneme_grapheme_alignment_with_morphology(
    morphemes: &[String],
    morpheme_phonemes: &[String],
    original_word: &str,
) -> AlignedString {

    if morphemes.len() != morpheme_phonemes.len() {
        eprintln!("Warning: morpheme count mismatch");
        // Fallback to regular alignment
        let g = source_tokenizer(original_word);
        let combined_phonemes = morpheme_phonemes.join("");
        let p = tokenize_ipa(&combined_phonemes);
        return phoneme_grapheme_alignment(p, g);
    }

    // Align each morpheme separately
    let mut morpheme_alignments: Vec<AlignedString> = Vec::new();
    
    for (morpheme, phonemes) in morphemes.iter().zip(morpheme_phonemes.iter()) {
        let g = source_tokenizer(morpheme);
        let p = tokenize_ipa(phonemes);
        let aligned = phoneme_grapheme_alignment(p, g);
        morpheme_alignments.push(aligned);
    }

    // Combine all alignments
    let mut result: AlignedString = Vec::new();
    for alignment in morpheme_alignments {
        result.extend(alignment);
    }

    result
}

/// Rebuild and align from morphology-aware G2P
/// 
/// This is a convenience function that takes the output from 
/// `G2Py::phonemize_with_morphology()` and performs alignment.
/// 
/// # Arguments
/// 
/// * `original_word` - The original input word
/// * `phoneme_string` - IPA phonemes for the whole word (no boundaries)
/// * `morphemes` - List of morpheme strings (for informational purposes)
/// 
/// # Returns
/// 
/// An AlignedString for the complete word
/// 
/// # Implementation Note
/// 
/// Since we now phonemize the whole word directly (not morphemes separately),
/// this function simply performs regular alignment. The morphemes parameter is
/// kept for backward compatibility and informational purposes.
pub fn rebuild_and_align_from_morphology(
    original_word: &str,
    phoneme_string: &str,
    _morphemes: &[String],
) -> AlignedString {
    use crate::grapheme::tokenize::source_tokenizer;
    use crate::phoneme::tokenizer::ipa::tokenize_ipa;

    // Phonemize the whole word using direct alignment
    // No need to split by morpheme boundaries since phoneme_string 
    // is already the IPA for the whole word
    let g = source_tokenizer(original_word);
    let p = tokenize_ipa(phoneme_string);
    phoneme_grapheme_alignment(p, g)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morphology_aware_alignment() {
        // Test case: "reallocation"
        // Morphemes: ["re", "allocate", "tion"]
        // Expected phonemes (approximation): ["riː", "æləkeɪt", "ʃən"]
        
        let morphemes = vec![
            "re".to_string(),
            "allocate".to_string(),
            "tion".to_string(),
        ];
        
        let morpheme_phonemes = vec![
            "riː".to_string(),
            "æləkeɪt".to_string(),
            "ʃən".to_string(),
        ];
        
        let original_word = "reallocation";
        
        let result = phoneme_grapheme_alignment_with_morphology(
            &morphemes,
            &morpheme_phonemes,
            original_word
        );
        
        // The result should have alignments for each grapheme
        assert!(!result.is_empty(), "Alignment result should not be empty");
        println!("\n=== Test: Morphology-Aware Alignment ===");
        println!("Word: {}", original_word);
        println!("Morphemes: {:?}", morphemes);
        println!("Phonemes: {:?}", morpheme_phonemes);
        println!("Alignment count: {}", result.len());
    }

    #[test]
    fn test_rebuild_and_align() {
        let original_word = "unfriendly";
        let phoneme_string = "ʌn#frɛndli#"; // with morpheme boundaries
        let morphemes = vec![
            "un".to_string(),
            "friend".to_string(),
            "ly".to_string(),
        ];
        
        let result = rebuild_and_align_from_morphology(
            original_word,
            phoneme_string,
            &morphemes
        );
        
        assert!(!result.is_empty(), "Rebuild and align result should not be empty");
        println!("\n=== Test: Rebuild and Align ===");
        println!("Word: {}", original_word);
        println!("Phoneme string: {}", phoneme_string);
        println!("Morphemes: {:?}", morphemes);
        println!("Result count: {}", result.len());
    }
}