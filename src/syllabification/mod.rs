pub mod algorithm;
pub mod pattern_builder;
pub mod types;
pub mod validation;

/// Convenience macro for defining patterns inline
///
/// # Example
///
/// ```ignore
/// let is_kkpk = pat!(K K P K);
/// assert!(matches_pattern(&graphemes, &is_kkpk));
/// ```
#[macro_export]
macro_rules! pat {
    ($($elem:ident)*) => {
        &[$(Pat::$elem),*]
    };
}
