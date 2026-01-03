//! # Tagabaybay: English to Filipino Phonetic Nativization
//!
//! Tagabaybay is a  for converting English text to Filipino phonetic representation.
//! It handles various English orthographic patterns and converts them to their Filipino
//! phonetic equivalents according to Filipino phonology.
//!
//! ## Features
//!
//! - Context-sensitive phonetic conversion
//! - Abbreviation handling (spells out uppercase letters)
//! - Configurable behavior (allow/disallow non-native sounds)
//! - Comprehensive error handling
//!
//! ## Quick Start
//!
//! ```
//! use tagabaybay::nativization::nativize::Nativizer;
//! use tagabaybay::tokenization::phoneme::phonemes_to_string;
//!
//! let nativizer = Nativizer::new();
//! let result = nativizer.nativize("chocolate").unwrap();
//! println!("{}", phonemes_to_string(&result)); // "tsokoleyt"
//! ```
//!
//! ## Modules
//!
//! - `nativization`: Core nativization logic and configuration
//! - `tokenization`: Grapheme and phoneme handling
//! - `consts`: Configuration constants and types

pub mod consts;
pub mod nativization;
pub mod syllabification;
pub mod tokenization;
