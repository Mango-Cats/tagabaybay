//! Context-free grapheme replacements
//!
//! Handles straightforward grapheme-to-grapheme conversions that don't require
//! analysis of surrounding context.

use crate::adaptation::cursor::Cursor;
use crate::configs::AdaptationConfig;
use crate::grapheme::filipino::FilipinoGrapheme;
use crate::grapheme::source::SourceGrapheme;

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
    config: &AdaptationConfig,
) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    // this has no uppercase character
    let g = ctx.current_grapheme_low();

    match g {
        // Digraph replacements (digraph count as 1 grapheme)
        SourceGrapheme::PH => Some((vec![FilipinoGrapheme::F], 1)),
        SourceGrapheme::PS => Some((vec![FilipinoGrapheme::S], 1)),
        SourceGrapheme::TH => Some((vec![FilipinoGrapheme::T], 1)),
        SourceGrapheme::EE => Some((vec![FilipinoGrapheme::I], 1)),
        SourceGrapheme::OO => Some((vec![FilipinoGrapheme::U], 1)),

        // Consonants
        SourceGrapheme::B => Some((vec![FilipinoGrapheme::B], 1)),
        SourceGrapheme::D => Some((vec![FilipinoGrapheme::D], 1)),
        SourceGrapheme::F => Some((vec![FilipinoGrapheme::F], 1)),
        SourceGrapheme::G => Some((vec![FilipinoGrapheme::G], 1)),
        SourceGrapheme::H => Some((vec![FilipinoGrapheme::H], 1)),
        SourceGrapheme::K => Some((vec![FilipinoGrapheme::K], 1)),
        SourceGrapheme::L => Some((vec![FilipinoGrapheme::L], 1)),
        SourceGrapheme::M => Some((vec![FilipinoGrapheme::M], 1)),
        SourceGrapheme::N => Some((vec![FilipinoGrapheme::N], 1)),
        SourceGrapheme::P => Some((vec![FilipinoGrapheme::P], 1)),
        SourceGrapheme::R => Some((vec![FilipinoGrapheme::R], 1)),
        SourceGrapheme::S => Some((vec![FilipinoGrapheme::S], 1)),
        SourceGrapheme::T => Some((vec![FilipinoGrapheme::T], 1)),
        SourceGrapheme::W => Some((vec![FilipinoGrapheme::W], 1)),
        SourceGrapheme::Y => Some((vec![FilipinoGrapheme::Y], 1)),

        // Handling borrowed letters
        SourceGrapheme::V => {
            if config.allow_v_letter {
                Some((vec![FilipinoGrapheme::V], 1))
            } else {
                Some((vec![FilipinoGrapheme::B], 1))
            }
        }

        SourceGrapheme::Z => {
            if config.allow_z_letter {
                Some((vec![FilipinoGrapheme::Z], 1))
            } else {
                Some((vec![FilipinoGrapheme::S], 1))
            }
        }

        // Spanish
        SourceGrapheme::Enye => Some((vec![FilipinoGrapheme::N], 1)),

        // Whitespace
        SourceGrapheme::Space => Some((vec![FilipinoGrapheme::Space], 1)),

        // ASCII passthrough (digits, punctuation, etc.)
        SourceGrapheme::Passthrough(c) => {
            Some((vec![FilipinoGrapheme::Passthrough(c.to_string())], 1))
        }

        // Context-sensitive letters (handled in sensitive_replacement)
        SourceGrapheme::C
        | SourceGrapheme::J
        | SourceGrapheme::Q
        | SourceGrapheme::X
        | SourceGrapheme::CH
        | SourceGrapheme::SH => None,

        // Vowels (unpredictable variants)
        SourceGrapheme::A
        | SourceGrapheme::E
        | SourceGrapheme::I
        | SourceGrapheme::O
        | SourceGrapheme::U => None,

        // Other characters (pass through as-is)
        SourceGrapheme::Other => Some((vec![FilipinoGrapheme::Other], 1)),

        _ => None,
    }
}
