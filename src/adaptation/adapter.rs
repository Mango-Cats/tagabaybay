use crate::adaptation::cursor::Cursor;
use crate::adaptation::orthographic::free::free_replacement;
use crate::adaptation::orthographic::sensitive::sensitive_replacement;
use crate::adaptation::orthographic::spelling::letter_to_phonetic;
use crate::adaptation::phonetic::free::phonetic_replacements;
use crate::configs::AdapterConfig;
use crate::error::{AdaptationError, ErrorTypes};
use crate::g2p::phonemize_to_arpa;
use crate::g2p::phonemize_to_ipa;
use crate::grapheme::filipino::FilipinoGrapheme;
use crate::grapheme::source::SourceGrapheme;
use crate::grapheme::tokenize::source_tokenizer;
use crate::phoneme::tokenize::{tokenize_arpa, tokenize_ipa};

/// Builder for adaptation with customizable configuration
///
/// The `Adapter` provides a flexible interface for converting loanwords
/// to Filipino representation. It supports customization through
/// configuration options.
///
/// # Examples
///
/// ```
/// use tagabaybay::adaptation::adaptation::Adapter;
/// use tagabaybay::tokenization::phl_graphemes::phl_graphemes_to_string;
///
/// let adapter = Adapter::new();
/// let result = adapter.adaptation("chocolate").unwrap();
/// assert_eq!(phl_graphemes_to_string(&result), "tsokoleyt");
/// ```
///
/// With custom configuration:
///
/// ```
/// use tagabaybay::adaptation::adaptation::Adapter;
///
/// let adapter = Adapter::new()
///
/// ```
pub struct Adapter {
    pub config: AdapterConfig,
}

impl Adapter {
    /// Create a new Adapter with default configuration
    pub fn new() -> Self {
        Self {
            config: AdapterConfig::default(),
        }
    }

    /// Create a Adapter with a custom configuration
    pub fn new_with_config(config: AdapterConfig) -> Self {
        Self { config }
    }

    /// Adapt an entire word or phrase
    ///
    /// Converts loanwords to their Filipino representation.
    ///
    /// # Arguments
    ///
    /// * `input` - The loanword to adapt
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<FilipinoGrapheme>)` on success, or `Err(AdaptationError)` if
    /// adaptation fails and `panic_at_error` is enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use tagabaybay::adaptation::adaptation::Adapter;
    ///
    /// let adapter = Adapter::new();
    /// let result = adapter.adaptation("hello").unwrap();
    /// ```
    pub fn adaptation(&self, input: &str) -> Result<Vec<FilipinoGrapheme>, ErrorTypes> {
        self.adapter_internal(input, None, None)
    }

    /// Adapt a list of words or phrases
    ///
    /// Processes multiple words in batch, providing detailed error information
    /// including the word number and dataset name.
    ///
    /// # Arguments
    ///
    /// * `word_list` - Slice of words to adaptation
    /// * `dataset_name` - Name of the dataset for error reporting
    ///
    /// # Returns
    ///
    /// Returns a vector of results, one for each input word.
    ///
    /// # Examples
    ///
    /// ```
    /// use tagabaybay::adaptation::adapter::Adapter;
    /// use tagabaybay::config::Config;
    ///
    /// let adapter = Adapter::new();
    /// let config = Config::new();
    /// let words = tokens!["hello", "world"];
    /// let results = adapter.adapt_batch(&words, "test_dataset", config);
    /// ```
    pub fn adapt_batch(
        &self,
        word_list: &[&str],
        dataset_name: &str,
    ) -> Vec<Result<Vec<FilipinoGrapheme>, ErrorTypes>> {
        word_list
            .iter()
            .enumerate()
            .map(|(i, word)| self.adapter_internal(word, Some(i), Some(dataset_name)))
            .collect()
    }

    fn adapter_internal(
        &self,
        word: &str,
        word_number: Option<usize>,
        dataset_name: Option<&str>,
    ) -> Result<Vec<FilipinoGrapheme>, ErrorTypes> {
        // preapre variables here
        let config = &self.config;
        let mut result: Vec<FilipinoGrapheme> = Vec::new();

        let graphemes = source_tokenizer(word);

        let phonemes = if config.use_ipa {
            let phon_str = phonemize_to_ipa(word, word_number, dataset_name, config)?;
            tokenize_ipa(&phon_str)
        } else {
            let phon_str = phonemize_to_arpa(word, word_number, dataset_name, config)?;
            tokenize_arpa(&phon_str)
        };

        let mut ctx = Cursor::new(&graphemes, &phonemes, 0);

        dbg!(&ctx.graphemes);
        dbg!(&ctx.phonemes);

        while ctx.index < ctx.len() {
            // Handle abbreviations and single letters (spelled out phonetically)
            if ctx.current_grapheme().is_uppercase() {
                if let Some((abbr_repl, consumed)) = detect_and_process_abbreviation(&ctx) {
                    result.extend(abbr_repl);
                    ctx.index += consumed;
                    continue;
                }
            }

            // Context-sensitive orthographic cases
            if let Some((sens_repl, consumed)) = sensitive_replacement(&ctx, &self.config) {
                result.extend(sens_repl);
                ctx.index += consumed;
                continue;
            }

            // Handle unpredictable variants via phonetic replacements
            // only if `sensitive_replacements` is none
            // that is, even if it is unpredictable variants. Maybe there is a
            // subset of patters where predicting it is possible.
            if let Some((arpa_repl, consumed)) = phonetic_replacements(&ctx, &self.config) {
                result.extend(arpa_repl);
                ctx.index += consumed;
                continue;
            }

            // Context-free orthographic cases (fallback)
            if let Some((free_repl, consumed)) = free_replacement(&ctx, &self.config) {
                result.extend(free_repl);
                ctx.index += consumed;
                continue;
            }

            // Could not process current grapheme -> handle error
            let error =
                AdaptationError::new(ctx.graphemes.to_vec(), ctx.index, word_number, dataset_name);
            error.print_error();
            if self.config.panic_at_error {
                return Err(ErrorTypes::Adaptation(error));
            }

            ctx.index += 1;
        }

        Ok(result)
    }
}

impl Default for Adapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Detect and process abbreviations
/// Returns (FilipinoGrapheme, graphemes_consumed) or None if not an abbreviation
fn detect_and_process_abbreviation(ctx: &Cursor) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let prev = ctx.prev_grapheme_low();

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
        let graphemes = adapt_abbreviation(abbr_segment);
        return Some((graphemes, end - ctx.index));
    }

    None
}

/// Convert an abbreviation to Filipino phonetic transcription
/// E.g., "XR" -> "eks ar", "IV" -> "ay bi"
fn adapt_abbreviation(abbr: Vec<SourceGrapheme>) -> Vec<FilipinoGrapheme> {
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
