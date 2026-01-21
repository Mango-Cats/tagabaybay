use crate::configs::AdapterConfig;
use crate::error::ErrorTypes;
use crate::g2py::phonemize;
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
    /// Handles multi-word inputs by phonemizing each word separately
    pub fn from_word(
        word: &str,
        word_number: Option<usize>,
        dataset_name: Option<&str>,
        config: &AdapterConfig,
    ) -> Result<Self, ErrorTypes> {
        let graphemes = grapheme::tokenize::source_tokenizer(word);

        // Handle multi-word inputs by phonemizing each word separately
        let phonetic_str = phonemize_phrase(word, word_number, dataset_name, config)?;

        let phonemes = phoneme::tokenize::tokenizer(&phonetic_str);

        Ok(Self {
            graphemes,
            phonemes,
            index: 0,
        })
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
    pub fn lookat_phoneme(&self, n: isize) -> Option<ArpabetSymbols> {
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

/// Phonemize a phrase (potentially multiple words separated by spaces)
/// Each word is phonemized separately, with '$' as separator between words
/// Numbers and special tokens are passed through as-is
fn phonemize_phrase(
    phrase: &str,
    word_number: Option<usize>,
    dataset_name: Option<&str>,
    config: &AdapterConfig,
) -> Result<String, ErrorTypes> {
    let words: Vec<&str> = phrase.split_whitespace().collect();

    if words.is_empty() {
        return Ok(String::new());
    }

    let mut phonetic_parts: Vec<String> = Vec::new();

    for word_part in words {
        // Handle hyphenated words by phonemizing each part
        let subparts: Vec<&str> = word_part.split('-').collect();
        let mut subpart_phonetics: Vec<String> = Vec::new();

        for subpart in &subparts {
            // Skip numbers and special tokens - they'll be handled by grapheme pass-through
            if subpart
                .chars()
                .all(|c| c.is_ascii_digit() || c == '.' || c == '/' || c == '+')
            {
                subpart_phonetics.push(String::new()); // Empty phonemes for numbers
                continue;
            }

            // Skip empty parts (e.g., from leading hyphen)
            if subpart.is_empty() {
                subpart_phonetics.push(String::new());
                continue;
            }

            // Strip trailing special characters (like "Shield+" -> "Shield")
            let clean_subpart: String = subpart
                .chars()
                .take_while(|c| c.is_ascii_alphabetic())
                .collect();

            if clean_subpart.is_empty() {
                subpart_phonetics.push(String::new());
                continue;
            }

            let phonetic_str = phonemize(&clean_subpart).map_err(|mut err| {
                err.word_number = word_number;
                err.dataset_name = dataset_name.map(str::to_string);

                err.print_error();

                if config.panic_at_error {
                    panic!("Phonetization failed: {:?}", err);
                }

                ErrorTypes::Phonetization(err)
            })?;

            subpart_phonetics.push(phonetic_str);
        }

        // Rejoin hyphenated parts with '$' (boundary marker)
        phonetic_parts.push(subpart_phonetics.join("$"));
    }

    // Join with '$' which is used as syllable/word boundary
    Ok(phonetic_parts.join("$"))
}