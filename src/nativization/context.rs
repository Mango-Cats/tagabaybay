use crate::consts::NativizationConfig;
use crate::g2p::phonemize;
use crate::nativization::error::TagabaybayErrors;
use crate::tokenization::graphemes::Grapheme;
use crate::tokenization::tokenize::tokenize;

/// Helper struct for accessing grapheme context during pattern matching
#[derive(Debug, Clone)]
pub struct Context {
    pub graphemes: Vec<Grapheme>,
    pub ipa: Vec<Grapheme>,
    pub index: usize,
}

impl Context {
    /// Create a new context with each parameter
    pub fn new(graphemes: &[Grapheme], index: usize, ipa: &[Grapheme]) -> Self {
        Self {
            graphemes: graphemes.to_vec(),
            index,
            ipa: ipa.to_vec(),
        }
    }

    /// Create a new context from a word
    pub fn from_word(
        word: &str,
        word_number: Option<usize>,
        dataset_name: Option<&str>,
        config: &NativizationConfig,
    ) -> Result<Self, TagabaybayErrors> {
        let graphemes = tokenize(word);

        let ipa_string = phonemize(word).map_err(|mut err| {
            err.word_number = word_number;
            err.dataset_name = dataset_name.map(str::to_string);

            err.print_error();

            if config.panic_at_error {
                panic!("Phonetization failed: {:?}", err);
            }

            TagabaybayErrors::Phonetization(err)
        })?;

        let ipa = tokenize(&ipa_string.to_ascii_lowercase());

        Ok(Self {
            graphemes,
            index: 0,
            ipa,
        })
    }

    /// Return the current grapheme, normalized to lowercase
    pub fn current(&self) -> Grapheme {
        self.graphemes[self.index].to_lowercase()
    }

    /// Return the previous grapheme, lowercase if it exists
    pub fn prev(&self) -> Option<Grapheme> {
        if self.index > 0 {
            Some(self.graphemes[self.index - 1].to_lowercase())
        } else {
            None
        }
    }

    /// Return the next grapheme, lowercase if it exists
    pub fn next(&self) -> Option<Grapheme> {
        self.graphemes.get(self.index + 1).map(|g| g.to_lowercase())
    }

    /// Look ahead n positions, lowercase if exists
    pub fn lookahead(&self, n: isize) -> Option<Grapheme> {
        let idx = self.index.checked_add_signed(n)?;

        self.graphemes.get(idx).map(|g| g.to_lowercase())
    }

    /// Flag if context is at start.
    pub fn at_start(&self) -> bool {
        self.index == 0
    }

    /// Flag if context is at end.
    pub fn at_end(&self) -> bool {
        self.index >= self.graphemes.len() - 1
    }

    /// Returns your current position at the context.
    pub fn position(&self) -> usize {
        self.index
    }

    /// Retuns the length of the graphemes
    pub fn len(&self) -> usize {
        self.graphemes.len()
    }
}
