use crate::adaptation::context::Context;
use crate::adaptation::replacement::{
    free_replacement, handle_vowel, letter_to_phonetic, sensitive_replacement,
};
use crate::consts::NativizationConfig;
use crate::error::{ErrorTypes, NativizationError};
use crate::tokenization::phl_graphemes::FilipinoGrapheme;
use crate::tokenization::src_graphemes::SourceGrapheme;

/// Builder for nativization with customizable configuration
///
/// The `Nativizer` provides a flexible interface for converting loanwords
/// to Filipino representation. It supports customization through
/// configuration options.
///
/// # Examples
///
/// ```
/// use tagabaybay::nativization::nativize::Nativizer;
/// use tagabaybay::tokenization::phl_graphemes::phl_graphemes_to_string;
///
/// let nativizer = Nativizer::new();
/// let result = nativizer.nativize("chocolate").unwrap();
/// assert_eq!(phl_graphemes_to_string(&result), "tsokoleyt");
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
    /// Converts loanwords to their Filipino representation.
    ///
    /// # Arguments
    ///
    /// * `input` - The loanword to adapt
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<FilipinoGrapheme>)` on success, or `Err(NativizationError)` if
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
    pub fn nativize(
        &self,
        input: &str,
        config: &NativizationConfig,
    ) -> Result<Vec<FilipinoGrapheme>, ErrorTypes> {
        self.nativize_internal(input, None, None, config)
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
        config: &NativizationConfig,
    ) -> Vec<Result<Vec<FilipinoGrapheme>, ErrorTypes>> {
        word_list
            .iter()
            .enumerate()
            .map(|(i, word)| self.nativize_internal(word, Some(i), Some(dataset_name), config))
            .collect()
    }

    fn nativize_internal(
        &self,
        word: &str,
        word_number: Option<usize>,
        dataset_name: Option<&str>,
        config: &NativizationConfig,
    ) -> Result<Vec<FilipinoGrapheme>, ErrorTypes> {
        let mut result: Vec<FilipinoGrapheme> = Vec::new();
        let mut ctx = match Context::from_word(word, word_number, dataset_name, config) {
            Ok(ctx) => ctx,
            Err(e) => return Err(e),
        };
        dbg!(&ctx.phonetic_transcription);
        while ctx.index < ctx.len() {
            let curr = ctx.current();

            // Handle abbreviations and single letters (spelled out phonetically)
            if curr.is_uppercase() {
                if let Some((abbr_repl, consumed)) = detect_and_process_abbreviation(&ctx) {
                    result.extend(abbr_repl);
                    ctx.index += consumed;
                    continue;
                }
            }

            // Handle vowels (special case)
            if curr.is_vowel() {
                // ERR here
                if let Some((arpa, consumed)) = handle_vowel(&ctx, &ctx.phonetic_transcription) {
                    result.extend(arpa);
                    ctx.index += consumed;
                    continue;
                }
            }

            // Context-sensitive replacement
            if let Some((sens_repl, consumed)) = sensitive_replacement(&ctx, &self.config) {
                result.extend(sens_repl);
                ctx.index += consumed;
                continue;
            }

            // Context-free replacement (fallback)
            if let Some((free_repl, consumed)) = free_replacement(&ctx, &self.config) {
                result.push(free_repl);
                ctx.index += consumed;
                continue;
            }

            // Could not process current grapheme -> handle error
            let error = NativizationError::new(
                ctx.graphemes.to_vec(),
                ctx.index,
                word_number,
                dataset_name,
            );
            error.print_error();
            if self.config.panic_at_error {
                return Err(ErrorTypes::Nativization(error));
            }

            ctx.index += 1;
        }

        Ok(result)
    }
}

impl Default for Nativizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Detect and process abbreviations
/// Returns (FilipinoGrapheme, graphemes_consumed) or None if not an abbreviation
fn detect_and_process_abbreviation(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let prev = ctx.prev();

    let after_separator = match prev {
        None => true,
        Some(SourceGrapheme::Space) => true,
        Some(SourceGrapheme::Passthrough(s)) if s == "-" => true,
        _ => false,
    };

    if !after_separator {
        return None;
    }

    // Find the end of the uppercase sequence
    let mut end = ctx.index;
    while end < ctx.len() && ctx.graphemes[end].is_uppercase() {
        end += 1;
    }

    let next = ctx.graphemes.get(end);
    let before_separator = match next {
        None => true,
        Some(SourceGrapheme::Space) => true,
        Some(SourceGrapheme::Passthrough(s)) if s == "-" => true,
        _ => false,
    };

    // Check if this is an abbreviation (2+ letters or single letter before separator)
    if (end - ctx.index >= 2) || before_separator {
        let abbr_segment = ctx.graphemes[(ctx.index)..end].to_vec();
        let graphemes = nativize_abbreviation(abbr_segment);
        return Some((graphemes, end - ctx.index));
    }

    None
}

/// Convert an abbreviation to Filipino phonetic transcription
/// E.g., "XR" -> "eks ar", "IV" -> "ay bi"
fn nativize_abbreviation(abbr: Vec<SourceGrapheme>) -> Vec<FilipinoGrapheme> {
    let mut result: Vec<FilipinoGrapheme> = Vec::new();
    for (i, grapheme) in abbr.iter().enumerate() {
        if let Some(graphemes) = letter_to_phonetic(grapheme.clone()) {
            // Add space before each letter except the first
            if i > 0 {
                result.push(FilipinoGrapheme::Space);
            }
            result.extend(graphemes);
        }
    }
    result
}
