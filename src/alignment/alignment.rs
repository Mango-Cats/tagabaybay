use crate::{grapheme::source::SourceGrapheme, phoneme::tokens::ipa::IPASymbol};
use crate::adaptation::cursor::Cursor;
use crate::{phoneme::tokenizer::ipa::detokenize_ipa};

pub type AlignedString = Vec<(SourceGrapheme, Vec<Option<IPASymbol>>)>;

/// Phoneme-Grapheme Alignment
/// 
/// Handles the alignment of grapheme tokens with its respective phoneme (ipa) tokens
/// and handles this based on surrounding context
/// 
/// # Arguments
/// * `p` - Contains the tokenized version of a phoneme string
/// * `g` - Contains the tokenized version of a grapheme string
/// 
/// # Returns
/// An AlignedString which is a vector of a graphame, phoneme tuple
/// `Vec<(SourceGrapheme, Vec<Option<IPASymbol>>)>`
pub fn phoneme_grapheme_alignment(
    p: Vec<IPASymbol>, 
    g: Vec<SourceGrapheme>, 
) -> AlignedString {
    let mut result = Vec::new();
    let mut p_index = 0;
    
    for (index , grapheme) in g.iter().enumerate() {
        let ctx = Cursor::new("", "", &g, &p, index);

        let phoneme = 
        if is_duplicate_grapheme(&ctx) ||
        is_double_vowel(&ctx) ||
        is_case_ck(&ctx) ||
        is_case_gh(&ctx) || 
        is_case_ld(&ctx, &p, p_index)
        {
            vec![None]
        } else if p_index < p.len() {
            handle_phonemes(&ctx, &p, &mut p_index)
        } else {
            vec![None]
        };

        result.push((grapheme.clone(), phoneme));
    };

    handle_leftover_phonemes(&mut result, &p, p_index);

    // for testing 
    print_aligned_string(&result);

    result
}

/// Determines if graphemes are duplicated
/// any grapheme after the first instance is matched with a phoneme of vec![None]
/// 
/// i.e
/// hello -> hɛloʊ
/// h -> h
/// e -> ɛ
/// l -> l
/// l -> None
/// o -> oʊ
/// 
/// # Returns a boolean value
fn is_duplicate_grapheme(ctx: &Cursor) -> bool {
    if let Some(prev) = ctx.prev_grapheme(){
        // case for double Cs
        if ctx.current_grapheme() == SourceGrapheme::C && prev == SourceGrapheme::C {
            return false
        }
        ctx.current_grapheme() == prev
    } else {
        false
    }
}

/// Determines if graphemes are vowels next to each other
/// usually 2 vowel sounds can flatten creating only 1 phoneme token
/// 
/// i.e
/// treat -> tɹiːt
/// t -> t
/// r -> ɹ
/// e -> i
/// a -> None
/// t -> t
/// 
/// # Returns a boolean value
fn is_double_vowel(ctx: &Cursor) -> bool {
    let current = ctx.current_grapheme();
    let current_vowel = current.is_vowel() || 
    current == SourceGrapheme::W ||
    current == SourceGrapheme::Y;

    if !current_vowel {
        return false;
    }

    if current == SourceGrapheme::OO || current == SourceGrapheme::EE {
        return false;
    }

    if let Some(prev) = ctx.prev_grapheme() {
        if prev == SourceGrapheme::OO || prev == SourceGrapheme::EE || !prev.is_vowel() {
            return false;
        }

        // Special case for UA
        if prev == SourceGrapheme::U && current == SourceGrapheme::A {
            return false;
        }

       if let Some(before_prev) = ctx.lookat_grapheme(-2) {
            if !before_prev.is_consonant() {
                return false;
            }
        }

        return true;
    }

    false
}

/// Determines if graphemes C and K are next to each other
/// 
/// i.e
/// picky -> pɪki
/// p -> p
/// i -> ɪ
/// c -> k
/// k -> None
/// y -> i
/// 
/// # Returns a boolean value
fn is_case_ck(ctx: &Cursor) -> bool {
    if ctx.current_grapheme() != SourceGrapheme::K {
        return false;
    }

    if let Some(prev) = ctx.prev_grapheme() {
        return prev == SourceGrapheme::C
    }

    false
}

/// Determines if graphemes G and H are next to each other
/// usually 'gh' is "silent"/ it serves as a silent extension to 
/// thw vowel that preceded it
/// 
/// i.e
/// thought -> θɔːt
/// th -> θ
/// o -> ɔ
/// u -> None
/// g -> None
/// h -> None
/// t -> t
/// 
/// # Returns a boolean value
fn is_case_gh(ctx: &Cursor) -> bool {
    if ctx.current_grapheme() == SourceGrapheme::H {
        if let Some(prev) = ctx.prev_grapheme() {
            return prev == SourceGrapheme::G;
        }
    }

    if ctx.current_grapheme() == SourceGrapheme::G {
        if let Some(next) = ctx.next_grapheme() {
            return next == SourceGrapheme::H;
        }
    }

    false
}

/// Determines graphemes L and D are next to each other
/// in some cases the L is silent when in combination with D
/// 
/// i.e
/// would -> wʊd
/// w -> w
/// o -> ʊ
/// u -> None
/// l -> None
/// d -> d
/// 
/// # Returns a boolean value
fn is_case_ld(ctx: &Cursor, p: &Vec<IPASymbol>, p_index: usize) -> bool {
    if ctx.current_grapheme() == SourceGrapheme::L && 
    ctx.next_grapheme() == Some(SourceGrapheme::D) 
    {
        if p_index < p.len() && p[p_index] == IPASymbol::VoicedAlveolarStop {
            return true
        }
    }

    false
}

/// Handling different phoneme cases 
/// 
/// # Arguments
/// 
/// # Returns 
/// vec![Some(phoneme)]
fn handle_phonemes(ctx: &Cursor, p: &Vec<IPASymbol>, p_index: &mut usize) -> Vec<Option<IPASymbol>> {
    let current_grapheme = ctx.current_grapheme();
    // let next_grapheme = ctx.next_grapheme();
    let prev_grapheme = ctx.prev_grapheme();


    // non-consuming
    if *p_index >= 1 {
        let prev_ph = p[*p_index - 1].clone();
        
        if prev_ph == IPASymbol::RColoredSchwa && 
           current_grapheme == SourceGrapheme::R {
            return vec![None];
        }

        // handles double Cs
        if current_grapheme == SourceGrapheme::C &&
           prev_grapheme == Some(SourceGrapheme::C) &&
           p[*p_index] != IPASymbol::VoicelessAlveolarFricative {
            return vec![None];
        }

        //handles D, J
        if current_grapheme == SourceGrapheme::J &&
           prev_grapheme == Some(SourceGrapheme::D) &&
           p[*p_index] != IPASymbol::VoicedPostalveolarAffricate {
            return vec![None];
        }
    }

    let ph = p[*p_index].clone();
    *p_index += 1;

    // consuming
    if *p_index < p.len() {
        let next_ph = p[*p_index].clone();

        // If grapheme is an X, append the /ks/ phonemes together
        if current_grapheme == SourceGrapheme::X {
            *p_index += 1;
            return vec![Some(ph), Some(next_ph)]
        } 

        else if current_grapheme == SourceGrapheme::SC {
            if next_ph == IPASymbol::VoicelessVelarStop {
                *p_index += 1;
                return vec![Some(ph), Some(next_ph)]
            } else {
                return vec![Some(ph)];
            }
        }

        else if current_grapheme == SourceGrapheme::SE {
            if next_ph == IPASymbol::OpenMidFront {
                *p_index += 1;
                return vec![Some(ph), Some(next_ph)]
            } else {
                return vec![Some(ph)];
            }
        }

        else if current_grapheme == SourceGrapheme::TI {
            if next_ph == IPASymbol::NearCloseFront {
                *p_index += 1;
                return vec![Some(ph), Some(next_ph)]
            } else {
                return vec![Some(ph)];
            }
        }

        else if current_grapheme == SourceGrapheme::ED {
            *p_index += 1;
            return vec![Some(ph), Some(next_ph)]
        }

        else if current_grapheme == SourceGrapheme::GE {
            if next_ph == IPASymbol::OpenMidFront {
                *p_index += 1;
                return vec![Some(ph), Some(next_ph)]
            } else {
                return vec![Some(ph)];
            }
        }
        
        // If PalatalApproximant is encountered or /j/ or the 'y' sound, combine with the previous phoneme
        else if next_ph == IPASymbol::PalatalApproximant {
            *p_index += 1;
            return vec![Some(ph), Some(next_ph)]
        } 

        else {
            vec![Some(ph)]
        }

    } else {
        vec![Some(ph)]
    }
}

/// Handles cases where the phonemes have a longer length than the graphemes
/// appends the remaining phonemes left behind to the corresponding index in p of the last grapheme
/// 
/// i.e.
/// ok -> oʊkeɪ
/// (O, oʊ)
/// (K, keɪ)
fn handle_leftover_phonemes(result: &mut AlignedString, p: &Vec<IPASymbol>, mut p_index: usize) {
    if p_index < p.len() {
        while p_index < p.len() {
            let remaining_phonemes = p[p_index].clone();
            result.last_mut().unwrap().1.push(Some(remaining_phonemes));
            p_index += 1;
        }
    }
}

/// Printing of the aligned string 
fn print_aligned_string(result: &AlignedString) {
    for (index, (grapheme, phoneme_vec)) in result.iter().enumerate() {
        let grapheme_str = grapheme.clone();
        let phoneme_strs: Vec<String> = phoneme_vec.iter()
        .map(|p_opt| match p_opt {
            Some(ipa) => detokenize_ipa(&[ipa.clone()]),
            None => String::from("None"),
        })
        .collect();
    println!("{}: {} -> {}", index, grapheme_str, phoneme_strs.join(""));

    };
}

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
    use crate::grapheme::tokenize::source_tokenizer;
    use crate::phoneme::tokenizer::ipa::tokenize_ipa;

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
