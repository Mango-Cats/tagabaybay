use crate::adaptation::cursor::Cursor;
use crate::adaptation::orthographic::free::free_replacement;
use crate::adaptation::orthographic::sensitive::sensitive_replacement;
use crate::adaptation::orthographic::spelling::letter_to_phonetic;
use crate::adaptation::phonetic::free::phonetic_replacements;
use crate::configs::AdapterConfig;
use crate::error::{AdaptationError, ErrorTypes};
use crate::g2p::G2Py;
use crate::grapheme::filipino::FilipinoGrapheme;
use crate::grapheme::source::SourceGrapheme;
use crate::grapheme::tokenize::source_tokenizer;
use crate::phoneme::tokenizer::ipa::tokenize_ipa;
use crate::phoneme::tokens::ipa::IPASymbol;

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
    g2p: Option<G2Py>,
}

impl Adapter {
    /// Create a new Adapter with default configuration
    pub fn new() -> Self {
        Self {
            config: AdapterConfig::default(),
            g2p: None,
        }
    }

    /// Create a Adapter with a custom configuration
    pub fn new_with_config(config: AdapterConfig) -> Self {
        Self { config, g2p: None }
    }

    fn adapter_internal(
        &mut self,
        word: &str,
        word_number: Option<usize>,
        dataset_name: Option<&str>,
    ) -> Result<Vec<FilipinoGrapheme>, ErrorTypes> {
        // Variable declarations
        // --------------------
        let mut result: Vec<FilipinoGrapheme> = Vec::new();

        let graphemes = source_tokenizer(word);

        // Only invoke G2P when the word contains unpredictable-variant graphemes
        // (vowels: a, e, i, o, u, y) and the config has g2p enabled.
        let needs_g2p = self.config.g2p_unpredictable_variants
            && graphemes
                .iter()
                .any(|g| g.to_lowercase().is_unpredictable_variant());

        let phon_str = if needs_g2p {
            if self.g2p.is_none() {
                self.g2p = Some(G2Py::new().map_err(ErrorTypes::G2P)?);
            }
            let config = &self.config;
            let g2p = self.g2p.as_mut().unwrap();
            g2p.phonemize_phrase(word, word_number, dataset_name, config)?
        } else {
            String::new()
        };

        let phonemes = tokenize_ipa(&phon_str);

        let mut ctx = Cursor::new(word, &phon_str, &graphemes, &phonemes, 0);

        // ========================
        // = Adaptation algorithm =
        // ========================
        while ctx.index < ctx.len() {
            #[cfg(feature = "debug-trace")]
            println!("~ curr: {}", ctx.current_grapheme());

            #[cfg(feature = "debug-trace")]
            print!("@ abbr: ");
            // ------------------------
            // - abbreviations        -
            // ------------------------
            if ctx.current_grapheme().is_uppercase() {
                if let Some((abbr_repl, consumed)) = detect_and_process_abbreviation(&ctx) {
                    #[cfg(feature = "debug-trace")]
                    println!("ACCEPT");
                    result.extend(abbr_repl);
                    ctx.index += consumed;
                    continue;
                }
            }
            #[cfg(feature = "debug-trace")]
            println!("REJECT");

            #[cfg(feature = "debug-trace")]
            print!("@ sens: ");
            // ------------------------
            // - ortho sensitive      -
            // ------------------------
            if let Some((sens_repl, consumed)) = sensitive_replacement(&ctx, &self.config) {
                #[cfg(feature = "debug-trace")]
                println!("ACCEPT");

                result.extend(sens_repl);
                ctx.index += consumed;
                continue;
            }
            #[cfg(feature = "debug-trace")]
            println!("REJECT");

            #[cfg(feature = "debug-trace")]
            print!("@ phon: ");
            // ------------------------
            // - phonetics            -
            // ------------------------
            // Handle unpredictable variants via phonetic replacements
            // only if `sensitive_replacements` is none
            // that is, even if it is unpredictable variants. Maybe there is a
            // subset of patters where predicting it is possible.
            if let Some((arpa_repl, consumed)) = phonetic_replacements(&ctx, &self.config) {
                #[cfg(feature = "debug-trace")]
                println!("ACCEPT with {:?} ", arpa_repl,);

                result.extend(arpa_repl);
                ctx.index += consumed;
                continue;
            }
            #[cfg(feature = "debug-trace")]
            println!("REJECT");

            #[cfg(feature = "debug-trace")]
            print!("@ free: ");
            // ------------------------
            // - ortho free           -
            // ------------------------
            if let Some((free_repl, consumed)) = free_replacement(&ctx, &self.config) {
                #[cfg(feature = "debug-trace")]
                println!("ACCEPT");

                result.extend(free_repl);
                ctx.index += consumed;
                continue;
            }
            #[cfg(feature = "debug-trace")]
            println!("REJECT");

            // ------------------------
            // - if none, error case  -
            // ------------------------
            let error =
                AdaptationError::new(ctx.graphemes.to_vec(), ctx.index, word_number, dataset_name);
            error.print_error();
            if self.config.panic_at_error {
                return Err(ErrorTypes::Adaptation(error));
            }

            ctx.index += 1;
        }

        // <COPILOT>
        // Apply output-side normalization after all adaptation rules have fired.
        // This catches duplicate letters emitted by different rule branches and
        // repairs schwa-driven consonant clusters such as "eybl" -> "eybol".
        // </COPILOT>
        Ok(finalize_adaptation(result, &phonemes))
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
    pub fn adaptation(&mut self, input: &str) -> Result<Vec<FilipinoGrapheme>, ErrorTypes> {
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
        &mut self,
        word_list: &[&str],
        dataset_name: &str,
    ) -> Vec<Result<Vec<FilipinoGrapheme>, ErrorTypes>> {
        let mut results = Vec::with_capacity(word_list.len());
        for (i, word) in word_list.iter().enumerate() {
            results.push(self.adapter_internal(word, Some(i), Some(dataset_name)));
        }
        results
    }
}

impl Default for Adapter {
    fn default() -> Self {
        Self::new()
    }
}

fn finalize_adaptation(
    adapted: Vec<FilipinoGrapheme>,
    phonemes: &[IPASymbol],
) -> Vec<FilipinoGrapheme> {
    let deduped = collapse_contiguous_duplicates(adapted);
    let deduped = collapse_redundant_affricate_prefixes(deduped);
    repair_final_schwa_liquid_cluster(deduped, phonemes)
}

fn collapse_contiguous_duplicates(adapted: Vec<FilipinoGrapheme>) -> Vec<FilipinoGrapheme> {
    let mut collapsed = Vec::with_capacity(adapted.len());

    for grapheme in adapted {
        if collapsed.last() != Some(&grapheme) {
            collapsed.push(grapheme);
        }
    }

    collapsed
}

fn collapse_redundant_affricate_prefixes(adapted: Vec<FilipinoGrapheme>) -> Vec<FilipinoGrapheme> {
    let mut result = Vec::with_capacity(adapted.len());

    for i in 0..adapted.len() {
        let curr = &adapted[i];
        let next = adapted.get(i + 1);

        if matches!(
            (curr, next),
            (FilipinoGrapheme::D, Some(FilipinoGrapheme::DY))
        ) {
            continue;
        }
        if matches!(
            (curr, next),
            (FilipinoGrapheme::T, Some(FilipinoGrapheme::TS))
        ) {
            continue;
        }

        result.push(curr.clone());
    }

    result
}

fn repair_final_schwa_liquid_cluster(
    mut adapted: Vec<FilipinoGrapheme>,
    phonemes: &[IPASymbol],
) -> Vec<FilipinoGrapheme> {
    if !contains_schwa(phonemes) {
        return adapted;
    }

    if adapted.len() >= 2 {
        let last_index = adapted.len() - 1;
        let last = &adapted[last_index];
        let prev = &adapted[last_index - 1];

        if prev.is_consonant()
            && matches!(last, FilipinoGrapheme::L | FilipinoGrapheme::R)
            && !matches!(prev, FilipinoGrapheme::Y | FilipinoGrapheme::W)
        {
            adapted.insert(last_index, FilipinoGrapheme::O);
        }
    }

    adapted
}

fn contains_schwa(phonemes: &[IPASymbol]) -> bool {
    phonemes.iter().any(|phoneme| {
        matches!(
            phoneme,
            IPASymbol::Schwa | IPASymbol::RColoredSchwa | IPASymbol::RColoredMid
        )
    })
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
