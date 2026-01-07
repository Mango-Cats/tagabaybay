use crate::tokenization::graphemes::Grapheme;

/// Helper struct for accessing grapheme context during pattern matching
#[derive(Debug, Clone, Copy)]
pub struct Context<'a> {
    pub graphemes: &'a [Grapheme],
    pub index: usize,
    pub ipa: &'a str,
}

impl<'a> Context<'a> {
    /// Create a new context at a given index
    pub fn new(graphemes: &'a [Grapheme], index: usize, ipa: &'a str) -> Self {
        Self {
            graphemes,
            index,
            ipa,
        }
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
