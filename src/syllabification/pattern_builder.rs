use super::algorithm::matches_pattern;
use super::types::Pat;
use crate::grapheme::filipino::FilipinoGrapheme;
use crate::tokens;

pub struct PatternBuilder {
    pattern: Vec<Pat>,
}

impl PatternBuilder {
    pub fn new() -> Self {
        Self { pattern: tokens![] }
    }

    /// Append a consonant (K) to the pattern
    pub fn k(mut self) -> Self {
        self.pattern.push(Pat::K);
        self
    }

    /// Append a vowel (P) to the pattern
    pub fn p(mut self) -> Self {
        self.pattern.push(Pat::P);
        self
    }

    /// Build the final pattern
    pub fn build(self) -> Vec<Pat> {
        self.pattern
    }

    /// Check if graphemes match this pattern
    pub fn matches(&self, graphemes: &[FilipinoGrapheme]) -> bool {
        matches_pattern(graphemes, &self.pattern)
    }
}

impl Default for PatternBuilder {
    fn default() -> Self {
        Self::new()
    }
}
