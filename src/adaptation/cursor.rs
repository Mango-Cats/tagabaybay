use crate::{grapheme::source::SourceGrapheme, phoneme::tokens::ipa::IPASymbol};
use crate::{phoneme::tokenizer::ipa::detokenize_ipa};

type AlignedString = Vec<(SourceGrapheme, Vec<Option<IPASymbol>>)>;

// note to self: pls fix these conditionals bru 😭

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
        is_case_ld(&ctx, &p, p_index)||
        is_case_sc(&ctx)
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

    if current.is_consonant() {
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

/// Determines graphemes S and C are next to each other
/// in some cases SC is pronounced as 1 phoneme instead of 2
/// 
/// i.e
/// ascend -> ɐsɛnd
/// a -> ɐ
/// s -> s
/// c -> None
/// e -> ɛ
/// n -> n
/// d -> d
/// 
/// # Returns a boolean value
fn is_case_sc(ctx: &Cursor) -> bool {
    let next_gh = ctx.next_grapheme();
    if ctx.current_grapheme() == SourceGrapheme::C &&
    ctx.prev_grapheme() == Some(SourceGrapheme::S) {
        if next_gh == Some(SourceGrapheme::A) || 
        next_gh == Some(SourceGrapheme::E) ||
        next_gh == Some(SourceGrapheme::I) ||
        next_gh == Some(SourceGrapheme::O) ||
        next_gh == Some(SourceGrapheme::U) ||
        next_gh == Some(SourceGrapheme::OO) ||
        next_gh == Some(SourceGrapheme::EE)
        {
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
    let prev_grapheme = ctx.prev_grapheme();

    // If the previous phoneme was a diphthong and the prev and current graphemes are vowels, 'Y', or 'W' make phoneme None 
    // ie
    // 0: b -> b
    // 1: o -> oʊ
    // 2: w -> None
    // 3: l -> l
    // lowkey idk how necessary ALL the conditionals here are (or how to arrange all of this) since im sure i can clean this (somehow...hopefully) 
    if *p_index > 1 {
        let prev_ph = p[*p_index - 1].clone();

        if (prev_ph == IPASymbol::DiphthongAI ||
            prev_ph == IPASymbol::DiphthongAU ||
            prev_ph == IPASymbol::DiphthongEI ||
            prev_ph == IPASymbol::DiphthongOI ||
            prev_ph == IPASymbol::DiphthongOU) &&
            (prev_grapheme == Some(SourceGrapheme::A) || 
            prev_grapheme == Some(SourceGrapheme::E) ||
            prev_grapheme == Some(SourceGrapheme::I) ||
            prev_grapheme == Some(SourceGrapheme::O) ||
            prev_grapheme == Some(SourceGrapheme::U) ||
            prev_grapheme == Some(SourceGrapheme::OO) ||
            prev_grapheme == Some(SourceGrapheme::EE) ||
            prev_grapheme == Some(SourceGrapheme::W) ||
            prev_grapheme == Some(SourceGrapheme::Y)) && 
            (current_grapheme.is_vowel() ||
            current_grapheme == SourceGrapheme::W ||
            current_grapheme == SourceGrapheme::Y)
            {
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
            vec![Some(ph), Some(next_ph)]
        } 
        
        // If PalatalApproximant is encountered or /j/ or the 'y' sound, combine with the previous phoneme
        else if next_ph == IPASymbol::PalatalApproximant {
            *p_index += 1;
            vec![Some(ph), Some(next_ph)]
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

/// A cursor over a word, tracking both graphemes and phonetic transcription.
#[derive(Debug, Clone)]
pub struct Cursor {
    /// Only use this for printing
    pub input_word: String,
    pub input_pronunciation: String,
    /// Use this for processing/adaptation
    pub graphemes: Vec<SourceGrapheme>,
    pub phonemes: Vec<IPASymbol>,
    pub index: usize,
}

impl Cursor {
    /// Create a new cursor from graphemes and phonemes explicitly
    pub fn new(
        input_word: &str,
        input_pronunciation: &str,
        graphemes: &[SourceGrapheme],
        phonemes: &[IPASymbol],
        index: usize,
    ) -> Self {
        Self {
            input_word: input_word.to_string(),
            input_pronunciation: input_pronunciation.to_string(),
            graphemes: graphemes.to_vec(),
            phonemes: phonemes.to_vec(),
            index,
        }
    }

    /// Current grapheme (preserves case)
    pub fn current_grapheme(&self) -> SourceGrapheme {
        self.graphemes[self.index].clone()
    }

    /// Current grapheme (lowercased)
    pub fn current_grapheme_low(&self) -> SourceGrapheme {
        self.graphemes[self.index].to_lowercase()
    }

    /// Previous grapheme (preserves case)
    pub fn prev_grapheme(&self) -> Option<SourceGrapheme> {
        if self.index > 0 {
            Some(self.graphemes[self.index - 1].clone())
        } else {
            None
        }
    }

    /// Previous grapheme (lowercased)
    pub fn prev_grapheme_low(&self) -> Option<SourceGrapheme> {
        if self.index > 0 {
            Some(self.graphemes[self.index - 1].to_lowercase())
        } else {
            None
        }
    }

    /// Next grapheme (preserves case)
    pub fn next_grapheme(&self) -> Option<SourceGrapheme> {
        self.graphemes.get(self.index + 1).cloned()
    }

    /// Next grapheme (lowercased)
    pub fn next_grapheme_low(&self) -> Option<SourceGrapheme> {
        self.graphemes.get(self.index + 1).map(|g| g.to_lowercase())
    }

    /// Look ahead n graphemes (preserves case)
    pub fn lookat_grapheme(&self, n: isize) -> Option<SourceGrapheme> {
        let idx = self.index.checked_add_signed(n)?;
        self.graphemes.get(idx).cloned()
    }

    /// Look ahead n graphemes (lowercased)
    pub fn lookat_grapheme_low(&self, n: isize) -> Option<SourceGrapheme> {
        let idx = self.index.checked_add_signed(n)?;
        self.graphemes.get(idx).map(|g| g.to_lowercase())
    }

    /// Current phoneme
    pub fn current_phoneme(&self) -> Option<IPASymbol> {
        self.phonemes.get(self.index).cloned()
    }

    /// Previous phoneme
    pub fn prev_phoneme(&self) -> Option<IPASymbol> {
        if self.index > 0 {
            Some(self.phonemes[self.index - 1].clone())
        } else {
            None
        }
    }

    /// Next phoneme
    pub fn next_phoneme(&self) -> Option<IPASymbol> {
        self.phonemes.get(self.index + 1).cloned()
    }

    /// Look ahead n phonemes
    pub fn lookat_phoneme(&self, n: isize) -> Option<IPASymbol> {
        let idx = self.index.checked_add_signed(n)?;
        self.phonemes.get(idx).cloned()
    }

    /// Check if cursor is at start
    pub fn at_start(&self) -> bool {
        self.index == 0
    }

    /// Check if cursor is at end (graphemes)
    pub fn at_end(&self) -> bool {
        self.index >= self.graphemes.len() - 1
    }

    /// Current index
    pub fn position(&self) -> usize {
        self.index
    }

    /// Length of graphemes
    pub fn len(&self) -> usize {
        self.graphemes.len()
    }

    /// Advance cursor by one
    pub fn advance(&mut self) {
        if self.index < self.len() {
            self.index += 1;
        }
    }

    /// Retreat cursor by one
    pub fn retreat(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }
}
