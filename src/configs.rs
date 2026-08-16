/// Configuration for the loanword adaptation process
///
/// Controls various aspects of how Source text is converted to Filipino phonetics.
/// Use the builder methods to customize behavior.
///
/// # Examples
///
/// ```
/// use tagabaybay::consts::AdapterConfig;
///
/// let config = AdapterConfig::new()
///     .with_sh_sound(true)
///     .with_z_sound(false);
/// ```
#[derive(Debug, Clone)]
pub struct AdapterConfig {
    /// Whether to panic when an error occurs during loanword adaptation
    pub panic_at_error: bool,
    /// Whether the output of the phonemization step is in Arpabet or IPA
    pub use_ipa: bool,
    /// Whether to allow the 'sh' sound (instead of 's')
    pub allow_sh_letter: bool,
    /// Whether to allow the 'z' sound (instead of 's')
    pub allow_z_letter: bool,
    /// Whether to allow the 'j' sound (instead of 'dy')
    pub allow_j_letter: bool,
    /// Whether to allow the 'v' sound (instead of 'b')
    pub allow_v_letter: bool,
    /// Whether to use G2P for unpredictable variant graphemes.
    pub g2p_unpredictable_variants: bool,
    /// Whether to compute stress/prominence assignment (the stress rule)
    /// alongside adaptation. Off by default: it requires an extra English
    /// primary-stress lookup (via [`ProminenceBackend`]) on top of the
    /// normal adaptation pipeline. See [`crate::stress`] and
    /// [`crate::adaptation::adapter::Adapter::prominence`].
    pub assign_prominence: bool,
    /// Backend used for the English primary-stress lookup when
    /// `assign_prominence` is enabled.
    pub prominence_backend: ProminenceBackend,
}

/// Backend used to look up a source word's English primary stress for the
/// stress prominence rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProminenceBackend {
    /// Query eSpeak-NG's English frontend, reusing the same G2P subprocess
    /// already used for phonemization.
    Espeak,
    /// Look up a local CMUdict-format dictionary file.
    Cmudict {
        /// Path to a CMUdict-format `.dict` file.
        dict_path: std::path::PathBuf,
    },
}

impl Default for AdapterConfig {
    /// Create default configuration with conservative settings
    ///
    /// Default values:
    /// - `panic_at_error`: false (prints errors but continues)
    /// - `allow_sh_letter`: true (converts sh → s)
    /// - `allow_z_letter`: true (converts z → s)
    /// - `allow_j_letter`: true (converts j -> j not dy)
    /// - `g2p_unpredictable_variants`: true (uses phonetic rules for unpredictable variant graphemes)
    /// - `assign_prominence`: true (looks up English stress and emits stress-marked forms)
    fn default() -> Self {
        Self {
            panic_at_error: false,
            use_ipa: true,
            allow_sh_letter: true,
            allow_z_letter: true,
            allow_j_letter: true,
            allow_v_letter: true,
            g2p_unpredictable_variants: true,
            assign_prominence: true,
            prominence_backend: ProminenceBackend::Espeak,
        }
    }
}

impl AdapterConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set panic on error
    ///
    /// When enabled, the adapter will panic when encountering errors instead
    /// of printing them and continuing. Useful for strict validation.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to panic on errors, `false` to continue
    pub fn set_panic_at_error(mut self, value: bool) -> Self {
        self.panic_at_error = value;
        self
    }

    /// Set use ipa
    ///
    /// When enabled (TRUE), the adapter will use IPA transcription for the
    /// phonimization step. If disabled (FALSE), it will use Arpabet.
    ///
    /// For Phonetisaurus (original phonemization) set to FALSE.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to use ipa, `false` to use arpabet
    pub fn set_use_ipa(mut self, value: bool) -> Self {
        self.use_ipa = value;
        self
    }

    /// Set 'sh' sound preservation
    ///
    /// When enabled, "sh" digraphs are kept as "sh" sound. Otherwise,
    /// they're converted to "s" (e.g., "ship" → "sip" vs "ship").
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to keep 'sh' sound, `false` to convert to 's'
    pub fn set_sh_letter(mut self, value: bool) -> Self {
        self.allow_sh_letter = value;
        self
    }

    /// Set 'z' sound preservation
    ///
    /// When enabled, 'z' letters are kept as "z" sound. Otherwise,
    /// they're converted to "s" (e.g., "zoo" → "su" vs "zu").
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to keep 'z' sound, `false` to convert to 's'
    pub fn set_z_letter(mut self, value: bool) -> Self {
        self.allow_z_letter = value;
        self
    }

    /// Set 'j' sound preservation
    ///
    /// When enabled, 'j' letters are kept as the "j" sound. Otherwise,
    /// they're converted to 'dy' (e.g., "budyet" -> "bajet" vs "badyet")
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to keep 'j' should, false to convert to 'dy'
    pub fn set_j_letter(mut self, value: bool) -> Self {
        self.allow_j_letter = value;
        self
    }

    /// Set 'v' sound preservation
    ///
    /// When enabled, 'v' letters are kept as the "v" sound. Otherwise,
    /// they're converted to 'b' (e.g., "value" -> "valyu" vs "balyu")
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to keep 'v' should, false to convert to 'b'
    pub fn set_v_letter(mut self, value: bool) -> Self {
        self.allow_v_letter = value;
        self
    }

    pub fn set_g2p_unpredictable_variants(mut self, value: bool) -> Self {
        self.g2p_unpredictable_variants = value;
        self
    }

    /// Toggle stress/prominence assignment (the stress rule)
    ///
    /// When enabled, [`crate::adaptation::adapter::Adapter::prominence`]
    /// looks up the source word's English primary stress and applies the
    /// stress rule to determine which syllable of the adapted Filipino word
    /// is prominent and whether it is long. Disabled by default since it
    /// requires an extra lookup beyond normal adaptation.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to enable stress/prominence assignment
    pub fn set_assign_prominence(mut self, value: bool) -> Self {
        self.assign_prominence = value;
        self
    }

    /// Set the backend used for the English primary-stress lookup
    ///
    /// Only consulted when `assign_prominence` is enabled.
    ///
    /// # Arguments
    ///
    /// * `value` - `ProminenceBackend::Espeak` or `ProminenceBackend::Cmudict { dict_path }`
    pub fn set_prominence_backend(mut self, value: ProminenceBackend) -> Self {
        self.prominence_backend = value;
        self
    }
}
