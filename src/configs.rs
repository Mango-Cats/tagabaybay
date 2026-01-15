/// Configuration for the loanword adaptation process
///
/// Controls various aspects of how Source text is converted to Filipino phonetics.
/// Use the builder methods to customize behavior.
///
/// # Examples
///
/// ```
/// use tagabaybay::consts::AdaptationConfig;
///
/// let config = AdaptationConfig::new()
///     .with_sh_sound(true)
///     .with_z_sound(false);
/// ```
#[derive(Debug, Clone)]
pub struct AdaptationConfig {
    /// Whether to panic when an error occurs during loanword adaptation
    pub panic_at_error: bool,
    /// Whether to allow the 'sh' sound (instead of just 's')
    pub allow_sh_sound: bool,
    /// Whether to allow the 'z' sound (instead of 's')
    pub allow_z_sound: bool,
    /// Whether to use G2P for unpredictable variant graphemes.
    pub g2p_unpredictable_variants: bool,
}

impl Default for AdaptationConfig {
    /// Create default configuration with conservative settings
    ///
    /// Default values:
    /// - `panic_at_error`: false (prints errors but continues)
    /// - `allow_sh_sound`: false (converts sh → s)
    /// - `allow_z_sound`: false (converts z → s)
    /// - `g2p_unpredictable_variants`: true (uses g2p for vowels)
    fn default() -> Self {
        Self {
            panic_at_error: false,
            allow_sh_sound: false,
            allow_z_sound: false,
            g2p_unpredictable_variants: true,
        }
    }
}

impl AdaptationConfig {
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

    /// Set 'sh' sound preservation
    ///
    /// When enabled, "sh" digraphs are kept as "sh" sound. Otherwise,
    /// they're converted to "s" (e.g., "ship" → "sip" vs "ship").
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to keep 'sh' sound, `false` to convert to 's'
    pub fn set_sh_sound(mut self, value: bool) -> Self {
        self.allow_sh_sound = value;
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
    pub fn set_z_sound(mut self, value: bool) -> Self {
        self.allow_z_sound = value;
        self
    }

    pub fn set_g2p_unpredictable_variants(mut self, value: bool) -> Self {
        self.g2p_unpredictable_variants = value;
        self
    }
}
