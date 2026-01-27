use crate::{grapheme::source::SourceGrapheme, phoneme::tokens::ipa::IPASymbol};

//for debugging
use crate::{phoneme::tokenizer::ipa::detokenize_ipa};

type AlignedString = Vec<(SourceGrapheme, Vec<Option<IPASymbol>>)>;

// note to self: pls fix these conditionals bru 😭
pub fn phoneme_grapheme_alignment(
    p: Vec<IPASymbol>, 
    g: Vec<SourceGrapheme>, 
) -> AlignedString {
    let mut result = Vec::new();
    let mut p_index = 0;
    
    for (index, grapheme) in g.iter().enumerate() {
        let ctx = Cursor::new("", "", &g, &p, index);

        let phoneme = if is_duplicate_grapheme(&ctx) {
            vec![None]
        } else if is_double_vowel(&ctx) {
            vec![None]
        } 
        // CK if ! end of string
        else if index < g.len() && *grapheme == SourceGrapheme::K && g[index - 1] == SourceGrapheme::C {
            vec![None]
        } else if p_index < p.len() {
            let ph = p[p_index].clone();
            let next_ph = ctx.next_phoneme();
            let prev_ph = ctx.prev_phoneme();
            
            // debug
            // dbg!(ph.clone());

            p_index += 1;

            // Case where X is encountered, combines k and s to make [Some(k), Some(s)] smt like that (can be expanded) and j case, for the yuuuu sound
            if *grapheme == SourceGrapheme::X {
                p_index += 1;
                vec![Some(ph), next_ph]
            } else {
                vec![Some(ph)]
            }
        } else {
            vec![None]
        };
        result.push((grapheme.clone(), phoneme));
    };

    handle_leftover_phonemes(&mut result, p, p_index);

    print_aligned_string(&result);

    result
}

/// handling grapheme cases ?
fn is_duplicate_grapheme(ctx: &Cursor) -> bool {
    if let Some(prev) = ctx.prev_grapheme(){
        ctx.prev_grapheme() == Some(prev)
    } else {
        false
    }
}

/// remeber to add special case for U A grapheme combo
fn is_double_vowel(ctx: &Cursor) -> bool {
    let current = ctx.current_grapheme();

    if !current.is_vowel() {
        return false;
    }

    if current == SourceGrapheme::OO || current == SourceGrapheme::EE {
        return false;
    }

    if let Some(prev) = ctx.prev_grapheme() {
        if prev == SourceGrapheme::OO || prev == SourceGrapheme::EE || !prev.is_vowel() {
            return false;
        }

        if let Some(before_prev) = ctx.lookat_grapheme(-2) {
            return before_prev.is_consonant();
        }
    }

    false
}

/// handling phoneme cases 
// fn handle_phonemes(ctx: &Cursor) -> Vec<Option<IPASymbol>> {

// }

fn handle_leftover_phonemes(result: &mut AlignedString, p: Vec<IPASymbol>, mut p_index: usize) {
    // Case where g.len() is shorter than p.len() -> append the remaining phonemes left behind to the corresponding index in p of the last grapheme
    // ok -> oʊkeɪ, so (O, oʊ), (K, keɪ)
    if p_index < p.len() {
        while p_index < p.len() {
            let remaining_phonemes = p[p_index].clone();
            result.last_mut().unwrap().1.push(Some(remaining_phonemes));
            p_index += 1;
        }
    }
}

/// temp printing func
/// 
fn print_aligned_string(result: &AlignedString) {
    //printing purposes
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
