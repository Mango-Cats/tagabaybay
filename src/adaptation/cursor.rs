use crate::configs::AdaptationConfig;
use crate::error::ErrorTypes;
use crate::g2p::phonemize;
use crate::grapheme;
use crate::grapheme::source::SourceGrapheme;
use crate::phoneme;
use crate::phoneme::symbols::ArpabetSymbols;

/// A cursor over a word, tracking both graphemes and phonetic transcription.
#[derive(Debug, Clone)]
pub struct Cursor {
    pub graphemes: Vec<SourceGrapheme>,
    pub phonemes: Vec<ArpabetSymbols>,
    pub index: usize,
}

impl Cursor {
    /// Create a new cursor from graphemes and phonemes explicitly
    pub fn new(graphemes: &[SourceGrapheme], phonemes: &[ArpabetSymbols], index: usize) -> Self {
        Self {
            graphemes: graphemes.to_vec(),
            phonemes: phonemes.to_vec(),
            index,
        }
    }

    /// Construct cursor from a word using tokenization and phonemization
    pub fn from_word(
        word: &str,
        word_number: Option<usize>,
        dataset_name: Option<&str>,
        config: &AdaptationConfig,
    ) -> Result<Self, ErrorTypes> {
        let graphemes = grapheme::tokenize::tokenize(word);

        let phonetic_str = phonemize(word).map_err(|mut err| {
            err.word_number = word_number;
            err.dataset_name = dataset_name.map(str::to_string);

            err.print_error();

            if config.panic_at_error {
                panic!("Phonetization failed: {:?}", err);
            }

            ErrorTypes::Phonetization(err)
        })?;

        let phonemes = phoneme::tokenize::tokenize(&phonetic_str);

        Ok(Self {
            graphemes,
            phonemes,
            index: 0,
        })
    }

    /// Current grapheme
    pub fn current_grapheme(&self) -> SourceGrapheme {
        self.graphemes[self.index].to_lowercase()
    }

    /// Previous grapheme
    pub fn prev_grapheme(&self) -> Option<SourceGrapheme> {
        if self.index > 0 {
            Some(self.graphemes[self.index - 1].to_lowercase())
        } else {
            None
        }
    }

    /// Next grapheme
    pub fn next_grapheme(&self) -> Option<SourceGrapheme> {
        self.graphemes.get(self.index + 1).map(|g| g.to_lowercase())
    }

    /// Look ahead n graphemes
    pub fn lookahead_grapheme(&self, n: isize) -> Option<SourceGrapheme> {
        let idx = self.index.checked_add_signed(n)?;
        self.graphemes.get(idx).map(|g| g.to_lowercase())
    }

    /// Current phoneme
    pub fn current_phoneme(&self) -> Option<ArpabetSymbols> {
        self.phonemes.get(self.index).cloned()
    }

    /// Previous phoneme
    pub fn prev_phoneme(&self) -> Option<ArpabetSymbols> {
        if self.index > 0 {
            Some(self.phonemes[self.index - 1].clone())
        } else {
            None
        }
    }

    /// Next phoneme
    pub fn next_phoneme(&self) -> Option<ArpabetSymbols> {
        self.phonemes.get(self.index + 1).cloned()
    }

    /// Look ahead n phonemes
    pub fn lookahead_phoneme(&self, n: isize) -> Option<ArpabetSymbols> {
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
