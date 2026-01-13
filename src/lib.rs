//! # Tagabaybay
//! Tagabaybay is for converting loanwords to the corresponding Filipino representation.
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
//! use tagabaybay::tokenization::phl_graphemes::phl_graphemes_to_string;
//!
//! let nativizer = Nativizer::new();
//! let result = nativizer.nativize("chocolate").unwrap();
//! println!("{}", phl_graphemes_to_string(&result)); // "tsokoleyt"
//! ```
//!
//! ## Modules
//!
//! - `nativization`: Core nativization logic and configuration
//! - `tokenization`: SourceGrapheme and graphemes handling
//! - `consts`: Configuration constants and types

pub mod consts;
pub mod g2p;
pub mod nativization;
pub mod syllabification;
pub mod tokenization;
