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
    }

    let ph = p[*p_index].clone();
    *p_index += 1;

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