use crate::{grapheme::source::SourceGrapheme, phoneme::tokens::ipa::IPASymbol};

//for debugging
use crate::{phoneme::tokenizer::ipa::detokenize_ipa};

type AlignedString = Vec<(SourceGrapheme, Option<IPASymbol>)>;

pub fn phoneme_grapheme_alignment(p: Vec<IPASymbol>, g: Vec<SourceGrapheme>) -> AlignedString {
    let mut res = Vec::new();
    let mut p_index = 0;
    
    for (index, grapheme) in g.iter().enumerate() {
        let phoneme = if index > 0 && *grapheme == g[index - 1] {
            None
         // Cases when vowels are next to each other, make the 2nd vowel None / silent? unless case of OO or EE. oh my god this logic is so cheeks
        } else if index > 0 && 
        ((*grapheme).is_vowel() && (*grapheme != SourceGrapheme::OO || *grapheme != SourceGrapheme::EE)) && 
        (g[index - 1].is_vowel() && (g[index - 1] != SourceGrapheme::OO || g[index - 1] != SourceGrapheme::EE)) &&
        //fix this logic ie. queue
        g[index - 2].is_consonant() {
            None
        } else if p_index < p.len() {
            let ph = p[p_index].clone();
            
            // debug
            // dbg!(ph.clone());

            p_index += 1;

            // Case where X is encountered, since X is ks 
            
            Some(ph)
        } else {
            None
        };

        res.push((grapheme.clone(), phoneme));
    };

    //printing purposes
    for (index, (grapheme, phoneme_opt)) in res.iter().enumerate() {
        let grapheme_str = grapheme.clone();
        let phoneme_str = match phoneme_opt {
            Some(ipa) => detokenize_ipa(&[ipa.clone()]),
            None => String::from("None"),
        };

        println!("{}: {} -> {}", index, grapheme_str, phoneme_str);

    };

    res
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
