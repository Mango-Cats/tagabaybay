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
//! use tagabaybay::adaptation::adaptation::Adapter;
//! use tagabaybay::tokenization::phl_graphemes::phl_graphemes_to_string;
//!
//! let adapter = Adapter::new();
//! let result = adapter.adaptation("chocolate").unwrap();
//! println!("{}", phl_graphemes_to_string(&result)); // "tsokoleyt"
//! ```
//!
//! ## Modules
//!
//! - `adaptation`: Core adaptation logic and configuration
//! - `tokenization`: SourceGrapheme and graphemes handling
//! - `consts`: Configuration constants and types

pub mod adaptation;
pub mod consts;
pub mod error;
pub mod g2p;
pub mod syllabification;
pub mod tokenization;
pub mod arpabet;
