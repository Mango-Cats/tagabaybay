/// Configuration for the nativization process
///
/// Controls various aspects of how English text is converted to Filipino phonetics.
/// Use the builder methods to customize behavior.
///
/// # Examples
///
/// ```
/// use tagabaybay::consts::NativizationConfig;
///
/// let config = NativizationConfig::new()
///     .with_sh_sound(true)
///     .with_z_sound(false);
/// ```
#[derive(Debug, Clone)]
pub struct NativizationConfig {
    /// Whether to panic when an error occurs during nativization
    pub panic_at_error: bool,
    /// Whether to allow the 'sh' sound (instead of just 's')
    pub allow_sh_sound: bool,
    /// Whether to allow the 'z' sound (instead of 's')
    pub allow_z_sound: bool,
}

impl Default for NativizationConfig {
    /// Create default configuration with conservative settings
    ///
    /// Default values:
    /// - `panic_at_error`: false (prints errors but continues)
    /// - `allow_sh_sound`: false (converts sh → s)
    /// - `allow_z_sound`: false (converts z → s)
    fn default() -> Self {
        Self {
            panic_at_error: false,
            allow_sh_sound: false,
            allow_z_sound: false,
        }
    }
}

impl NativizationConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable panic on error (builder pattern)
    ///
    /// When enabled, the nativizer will panic when encountering errors instead
    /// of printing them and continuing. Useful for strict validation.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to panic on errors, `false` to continue
    pub fn with_panic_at_error(mut self, value: bool) -> Self {
        self.panic_at_error = value;
        self
    }

    /// Enable 'sh' sound preservation (builder pattern)
    ///
    /// When enabled, "sh" digraphs are kept as "sh" sound. Otherwise,
    /// they're converted to "s" (e.g., "ship" → "sip" vs "ship").
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to keep 'sh' sound, `false` to convert to 's'
    pub fn with_sh_sound(mut self, value: bool) -> Self {
        self.allow_sh_sound = value;
        self
    }

    /// Enable 'z' sound preservation (builder pattern)
    ///
    /// When enabled, 'z' letters are kept as "z" sound. Otherwise,
    /// they're converted to "s" (e.g., "zoo" → "su" vs "zu").
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to keep 'z' sound, `false` to convert to 's'
    pub fn with_z_sound(mut self, value: bool) -> Self {
        self.allow_z_sound = value;
        self
    }
}
