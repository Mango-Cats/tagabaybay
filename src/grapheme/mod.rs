pub mod filipino;
pub mod source;
pub mod tokenize;
pub mod types;

/// Helper macro to create grapheme vectors more easily
///
/// # Example
///
/// ```ignore
/// use tagabaybay::grapheme::tokens;
/// use tagabaybay::grapheme::filipino::FilipinoGrapheme::*;
///
/// let word = tokens![B, A, S, A];  // "basa"
/// ```
#[macro_export]
macro_rules! tokens {
    ($($g:expr),* $(,)?) => {
        vec![$($g),*]
    };
}
