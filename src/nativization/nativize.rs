use crate::consts::NativizationConfig;
use crate::nativization::error::NativizationError;
use crate::nativization::replacement::{
    free_replacement, letter_to_phonetic, sensitive_replacement,
};
use crate::tokenization::graphemes::Grapheme;
use crate::tokenization::phoneme::Phoneme;
use crate::tokenization::tokenize::tokenize;

/// Builder for nativization with customizable configuration
///
/// The `Nativizer` provides a flexible interface for converting English text
/// to Filipino phonetic representation. It supports customization through
/// configuration options.
///
/// # Examples
///
/// ```
/// use tagabaybay::nativization::nativize::Nativizer;
/// use tagabaybay::tokenization::phoneme::phonemes_to_string;
///
/// let nativizer = Nativizer::new();
/// let result = nativizer.nativize("chocolate").unwrap();
/// assert_eq!(phonemes_to_string(&result), "tsokoleyt");
/// ```
///
/// With custom configuration:
///
/// ```
/// use tagabaybay::nativization::nativize::Nativizer;
///
/// let nativizer = Nativizer::new()
///     .allow_sh_sound(true)
///     .allow_z_sound(true);
/// ```
pub struct Nativizer {
    config: NativizationConfig,
}

impl Nativizer {
    /// Create a new Nativizer with default configuration
    pub fn new() -> Self {
        Self {
            config: NativizationConfig::default(),
        }
    }

    /// Create a Nativizer with a custom configuration
    pub fn with_config(config: NativizationConfig) -> Self {
        Self { config }
    }

    /// Enable panic on error
    pub fn panic_at_error(mut self, value: bool) -> Self {
        self.config.panic_at_error = value;
        self
    }

    /// Enable 'sh' sound
    pub fn allow_sh_sound(mut self, value: bool) -> Self {
        self.config.allow_sh_sound = value;
        self
    }

    /// Enable 'z' sound
    pub fn allow_z_sound(mut self, value: bool) -> Self {
        self.config.allow_z_sound = value;
        self
    }

    /// Nativize an entire word or phrase
    ///
    /// Converts English text to Filipino phonetic representation.
    ///
    /// # Arguments
    ///
    /// * `input` - The English text to nativize
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<Phoneme>)` on success, or `Err(NativizationError)` if
    /// nativization fails and `panic_at_error` is enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use tagabaybay::nativization::nativize::Nativizer;
    ///
    /// let nativizer = Nativizer::new();
    /// let result = nativizer.nativize("hello").unwrap();
    /// ```
    pub fn nativize(&self, input: &str) -> Result<Vec<Phoneme>, NativizationError> {
        self.nativize_internal(input, None, None)
    }

    /// Nativize a list of words or phrases
    ///
    /// Processes multiple words in batch, providing detailed error information
    /// including the word number and dataset name.
    ///
    /// # Arguments
    ///
    /// * `word_list` - Slice of words to nativize
    /// * `dataset_name` - Name of the dataset for error reporting
    ///
    /// # Returns
    ///
    /// Returns a vector of results, one for each input word.
    ///
    /// # Examples
    ///
    /// ```
    /// use tagabaybay::nativization::nativize::Nativizer;
    ///
    /// let nativizer = Nativizer::new();
    /// let words = vec!["hello", "world"];
    /// let results = nativizer.nativize_batch(&words, "test_dataset");
    /// ```
    pub fn nativize_batch(
        &self,
        word_list: &[&str],
        dataset_name: &str,
    ) -> Vec<Result<Vec<Phoneme>, NativizationError>> {
        word_list
            .iter()
            .enumerate()
            .map(|(i, word)| self.nativize_internal(word, Some(i), Some(dataset_name)))
            .collect()
    }

    /// Internal nativization implementation
    fn nativize_internal(
        &self,
        word: &str,
        word_number: Option<usize>,
        dataset_name: Option<&str>,
    ) -> Result<Vec<Phoneme>, NativizationError> {
        let mut res: Vec<Phoneme> = Vec::new();
        let graphemes = tokenize(word);

        let mut i = 0;
        while i < graphemes.len() {
            let curr = &graphemes[i];

            // Handle abbreviations and single letters (spelled out phonetically)
            if curr.is_uppercase() {
                if let Some((abbr_phonemes, consumed)) =
                    detect_and_process_abbreviation(&graphemes, i)
                {
                    res.extend(abbr_phonemes);
                    i += consumed;
                    continue;
                }
            }

            // FIXME: issue probably here for fix: parsing, look-ahead/behind, and iterators
            // Try context-sensitive replacement first
            if let Some((sens_res, consumed)) = sensitive_replacement(&graphemes, i, &self.config) {
                res.extend(sens_res);
                i += consumed;
            } else {
                // Fall back to context-free replacement
                if let Some((free_res, consumed)) = free_replacement(&graphemes, i, &self.config) {
                    res.push(free_res);
                    i += consumed;
                } else {
                    let error =
                        NativizationError::new(graphemes.clone(), i, word_number, dataset_name);
                    error.print_error(self.config.panic_at_error);
                    if self.config.panic_at_error {
                        return Err(error);
                    }
                    i += 1;
                }
            }
        }

        Ok(res)
    }
}

impl Default for Nativizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Detect and process abbreviations
/// Returns (phonemes, graphemes_consumed) or None if not an abbreviation
fn detect_and_process_abbreviation(
    graphemes: &[Grapheme],
    start_idx: usize,
) -> Option<(Vec<Phoneme>, usize)> {
    let prev = if start_idx > 0 {
        Some(&graphemes[start_idx - 1])
    } else {
        None
    };

    let after_separator = match prev {
        None => true,
        Some(Grapheme::Space) => true,
        Some(Grapheme::Passthrough(s)) if s == "-" => true,
        _ => false,
    };

    if !after_separator {
        return None;
    }

    // Find the end of the uppercase sequence
    let mut end = start_idx;
    while end < graphemes.len() && graphemes[end].is_uppercase() {
        end += 1;
    }

    let next = graphemes.get(end);
    let before_separator = match next {
        None => true,
        Some(Grapheme::Space) => true,
        Some(Grapheme::Passthrough(s)) if s == "-" => true,
        _ => false,
    };

    // Check if this is an abbreviation (2+ letters or single letter before separator)
    if (end - start_idx >= 2) || before_separator {
        let abbr_segment = &graphemes[start_idx..end];
        let phonemes = nativize_abbreviation(abbr_segment);
        return Some((phonemes, end - start_idx));
    }

    None
}

/// Convert an abbreviation to Filipino phonetic transcription
/// E.g., "XR" -> "eks ar", "IV" -> "ay bi"
fn nativize_abbreviation(abbr: &[Grapheme]) -> Vec<Phoneme> {
    let mut result: Vec<Phoneme> = Vec::new();
    for (i, grapheme) in abbr.iter().enumerate() {
        if let Some(phonemes) = letter_to_phonetic(grapheme.clone()) {
            // Add space before each letter except the first
            if i > 0 {
                result.push(Phoneme::Space);
            }
            result.extend(phonemes);
        }
    }
    result
}
