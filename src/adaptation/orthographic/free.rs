//! Context-free grapheme replacements
//!
//! Handles straightforward grapheme-to-grapheme conversions that don't require
//! analysis of surrounding context.

use crate::adaptation::cursor::Cursor;
use crate::configs::AdapterConfig;
use crate::grapheme::filipino::FilipinoGrapheme;
use crate::grapheme::source::SourceGrapheme;
use crate::tokens;

/// Convert an input grapheme to output grapheme(s) - context-free replacements
///
/// Handles straightforward grapheme-to-grapheme conversions that don't require
/// context analysis.
///
/// # Arguments
///
/// * `ctx` - Cursor containing the grapheme sequence and current position
/// * `config` - Adaptation configuration (affects sh/z sounds)
///
/// # Returns
///
/// Returns `Some((FilipinoGrapheme, consumed))` if a context-free rule matches, where
/// `consumed` is typically 1. Returns `None` for context-sensitive letters.
pub fn free_replacement(
    ctx: &Cursor,
    config: &AdapterConfig,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // this has no uppercase character
    let g = ctx.current_grapheme_low();

    match g {
        // Digraph replacements (digraph count as 1 grapheme)
        SourceGrapheme::PH => Some((tokens![FilipinoGrapheme::F], 1)),
        SourceGrapheme::PS => Some((tokens![FilipinoGrapheme::S], 1)),
        SourceGrapheme::TH => Some((tokens![FilipinoGrapheme::T], 1)),
        SourceGrapheme::EE => Some((tokens![FilipinoGrapheme::I], 1)),
        SourceGrapheme::OO => Some((tokens![FilipinoGrapheme::U], 1)),

        // Trigraph replacements (trigraph count as 1 grapheme)
        SourceGrapheme::ORE => Some((tokens![FilipinoGrapheme::O, FilipinoGrapheme::R], 1)),
        SourceGrapheme::IGH => Some((tokens![FilipinoGrapheme::A, FilipinoGrapheme::Y], 1)),

        // Tetragraph replacements (tetragraph count as 1 grapheme)
        SourceGrapheme::AUGH => Some((tokens![FilipinoGrapheme::O], 1)),
        SourceGrapheme::EIGH => Some((tokens![FilipinoGrapheme::E, FilipinoGrapheme::Y], 1)),

        // Consonants
        SourceGrapheme::B => Some((tokens![FilipinoGrapheme::B], 1)),
        SourceGrapheme::D => Some((tokens![FilipinoGrapheme::D], 1)),
        SourceGrapheme::F => Some((tokens![FilipinoGrapheme::F], 1)),
        SourceGrapheme::G => Some((tokens![FilipinoGrapheme::G], 1)),
        SourceGrapheme::H => Some((tokens![FilipinoGrapheme::H], 1)),
        SourceGrapheme::K => Some((tokens![FilipinoGrapheme::K], 1)),
        SourceGrapheme::L => Some((tokens![FilipinoGrapheme::L], 1)),
        SourceGrapheme::M => Some((tokens![FilipinoGrapheme::M], 1)),
        SourceGrapheme::N => Some((tokens![FilipinoGrapheme::N], 1)),
        SourceGrapheme::P => Some((tokens![FilipinoGrapheme::P], 1)),
        SourceGrapheme::R => Some((tokens![FilipinoGrapheme::R], 1)),
        SourceGrapheme::S => Some((tokens![FilipinoGrapheme::S], 1)),
        SourceGrapheme::T => Some((tokens![FilipinoGrapheme::T], 1)),
        SourceGrapheme::W => Some((tokens![FilipinoGrapheme::W], 1)),
        SourceGrapheme::Y => Some((tokens![FilipinoGrapheme::Y], 1)),

        // Handling borrowed letters
        SourceGrapheme::V => {
            if config.allow_v_letter {
                Some((tokens![FilipinoGrapheme::V], 1))
            } else {
                Some((tokens![FilipinoGrapheme::B], 1))
            }
        }

        SourceGrapheme::Z => {
            if config.allow_z_letter {
                Some((tokens![FilipinoGrapheme::Z], 1))
            } else {
                Some((tokens![FilipinoGrapheme::S], 1))
            }
        }

        // Spanish
        SourceGrapheme::Enye => Some((tokens![FilipinoGrapheme::N], 1)),

        // Whitespace
        SourceGrapheme::Space => Some((tokens![FilipinoGrapheme::Space], 1)),

        // ASCII passthrough (digits, punctuation, etc.)
        SourceGrapheme::Passthrough(c) => {
            Some((tokens![FilipinoGrapheme::Passthrough(c.to_string())], 1))
        }

        // Context-sensitive letters (handled in sensitive_replacement)
        // at the worse case, do a direct mapping
        // this is also used by `handle_duplicates()` in `sensitive.rs`
        // these should only be used there
        SourceGrapheme::C => Some((tokens![FilipinoGrapheme::K], 1)),
        SourceGrapheme::J => {
            if config.allow_j_letter {
                Some((tokens![FilipinoGrapheme::J], 1))
            } else {
                Some((tokens![FilipinoGrapheme::DY], 1))
            }
        }
        SourceGrapheme::Q => Some((tokens![FilipinoGrapheme::K], 1)),
        SourceGrapheme::X => Some((tokens![FilipinoGrapheme::K, FilipinoGrapheme::S], 1)),
        SourceGrapheme::CH => Some((tokens![FilipinoGrapheme::TS], 1)),
        SourceGrapheme::SH => Some((tokens![FilipinoGrapheme::SH], 1)),

        // Vowels (unpredictable variants)
        // same comment with context-sensitive letters
        SourceGrapheme::A => Some((tokens![FilipinoGrapheme::A], 1)),
        SourceGrapheme::E => Some((tokens![FilipinoGrapheme::E], 1)),
        SourceGrapheme::I => Some((tokens![FilipinoGrapheme::I], 1)),
        SourceGrapheme::O => Some((tokens![FilipinoGrapheme::O], 1)),
        SourceGrapheme::U => Some((tokens![FilipinoGrapheme::U], 1)),

        // Other characters (pass through as-is)
        SourceGrapheme::Other => Some((tokens![FilipinoGrapheme::Other], 1)),

        _ => None,
    }
}
