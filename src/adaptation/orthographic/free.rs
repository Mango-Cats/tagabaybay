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
) -> Option<(FilipinoGrapheme, usize)> {
    let g = ctx.current_grapheme().to_lowercase();

    match g {
        // Digraph replacements (digraph count as 1 grapheme)
        SourceGrapheme::PH => Some((FilipinoGrapheme::F, 1)),
        SourceGrapheme::PS => Some((FilipinoGrapheme::S, 1)),
        SourceGrapheme::TH => Some((FilipinoGrapheme::T, 1)),
        SourceGrapheme::SH => {
            if config.allow_sh_sound {
                Some((FilipinoGrapheme::SH, 1))
            } else {
                Some((FilipinoGrapheme::S, 1))
            }
        }
        SourceGrapheme::EE => Some((FilipinoGrapheme::I, 1)),
        SourceGrapheme::OO => Some((FilipinoGrapheme::U, 1)),

        // Consonants
        SourceGrapheme::B => Some((FilipinoGrapheme::B, 1)),
        SourceGrapheme::D => Some((FilipinoGrapheme::D, 1)),
        SourceGrapheme::F => Some((FilipinoGrapheme::F, 1)),
        SourceGrapheme::G => Some((FilipinoGrapheme::G, 1)),
        SourceGrapheme::H => Some((FilipinoGrapheme::H, 1)),
        SourceGrapheme::K => Some((FilipinoGrapheme::K, 1)),
        SourceGrapheme::L => Some((FilipinoGrapheme::L, 1)),
        SourceGrapheme::M => Some((FilipinoGrapheme::M, 1)),
        SourceGrapheme::N => Some((FilipinoGrapheme::N, 1)),
        SourceGrapheme::P => Some((FilipinoGrapheme::P, 1)),
        SourceGrapheme::R => Some((FilipinoGrapheme::R, 1)),
        SourceGrapheme::S => Some((FilipinoGrapheme::S, 1)),
        SourceGrapheme::T => Some((FilipinoGrapheme::T, 1)),
        SourceGrapheme::V => Some((FilipinoGrapheme::B, 1)),
        SourceGrapheme::W => Some((FilipinoGrapheme::W, 1)),
        SourceGrapheme::Y => Some((FilipinoGrapheme::Y, 1)),
        SourceGrapheme::Z => {
            if config.allow_z_sound {
                Some((FilipinoGrapheme::Z, 1))
            } else {
                Some((FilipinoGrapheme::S, 1))
            }
        }

        // Spanish
        SourceGrapheme::Enye => Some((FilipinoGrapheme::N, 1)),

        // Whitespace
        SourceGrapheme::Space => Some((FilipinoGrapheme::Space, 1)),

        // ASCII passthrough (digits, punctuation, etc.)
        SourceGrapheme::Passthrough(c) => Some((FilipinoGrapheme::Passthrough(c.to_string()), 1)),

        // Context-sensitive letters (handled in sensitive_replacement)
        SourceGrapheme::C
        | SourceGrapheme::J
        | SourceGrapheme::Q
        | SourceGrapheme::X
        | SourceGrapheme::CH => None,

        // Other characters (pass through as-is)
        SourceGrapheme::Other => Some((FilipinoGrapheme::Other, 1)),

        // Uppercase variants should not reach here (normalized by to_lowercase)
        _ => None,
    }
}
