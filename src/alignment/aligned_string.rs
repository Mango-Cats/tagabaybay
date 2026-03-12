use crate::grapheme::filipino::FilipinoGrapheme;
use crate::{grapheme::source::SourceGrapheme, phoneme::tokens::ipa::IPASymbol};
use super::alignment::AlignedString;
use crate::phoneme::tokens::map::IPA_TO_FG;
use std::vec;

/// AlignedString to Filipino Graphemes
/// 
/// Handles case mapping 
pub fn ipa_to_filipino_graphemes(aligned: &AlignedString) -> Vec<FilipinoGrapheme> {
    let mut result = Vec::new();
    
    for (idx, (grapheme, phonemes)) in aligned.iter().enumerate() {
        for phoneme_opt in phonemes {
            if let Some(symbol) = phoneme_opt {

                // "dʒ"
                if *symbol == IPASymbol::VoicedPostalveolarAffricate {
                    let next_grapheme = aligned.get(idx + 1)
                        .map(|(next_g, _)| next_g);
                    let fg = 
                    if idx == aligned.len() - 1 || 
                    (idx == aligned.len() - 2 && next_grapheme == Some(&SourceGrapheme::GE)){
                        vec![FilipinoGrapheme::J]
                    } else {
                        vec![FilipinoGrapheme::D, FilipinoGrapheme::Y]
                            
                    };
                    
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // "d"
                if *symbol == IPASymbol::VoicedAlveolarStop{
                    let fg = match grapheme {
                        SourceGrapheme::D | SourceGrapheme::ED => {
                             let next_has_d_sound = if idx < aligned.len() - 1 {
                                aligned[idx + 1].1.iter().any(|p| {
                                    matches!(p, Some(IPASymbol::VoicedPostalveolarAffricate))
                                })
                            } else {
                                false
                            };

                            if next_has_d_sound {
                                vec![]
                            } else {
                                vec![FilipinoGrapheme::D]
                            }
                        },
                        _ => vec![FilipinoGrapheme::D],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }
                
                // "ɪ"
                if *symbol == IPASymbol::NearCloseFront {
                    let next_is_consonant = aligned.get(idx + 1)
                        .map(|(next_g, _)| next_g.is_consonant())
                        .unwrap_or(false);
                    
                    let fg = match grapheme {
                        SourceGrapheme::A if next_is_consonant => {
                            vec![FilipinoGrapheme::E, FilipinoGrapheme::Y]
                        },
                        SourceGrapheme::E => vec![FilipinoGrapheme::E],
                        SourceGrapheme::ED => vec![FilipinoGrapheme::E],
                        SourceGrapheme::GE => vec![FilipinoGrapheme::E],
                        SourceGrapheme::DGE => vec![FilipinoGrapheme::E],
                        SourceGrapheme::ORE => {
                            if !next_is_consonant {
                                vec![FilipinoGrapheme::Y]
                            } else {
                                vec![FilipinoGrapheme::E]
                            }
                        },
                        _ => vec![FilipinoGrapheme::I],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // "ə"
                if *symbol == IPASymbol::Schwa {
                    let prev_grapheme = aligned.get(idx.saturating_sub(1))
                    .map(|(prev_g, _)| prev_g);
                    let fg = match grapheme {
                        SourceGrapheme::E => vec![FilipinoGrapheme::E],
                        SourceGrapheme::A => vec![FilipinoGrapheme::A],
                        SourceGrapheme::I => vec![FilipinoGrapheme::I],
                        SourceGrapheme::O => vec![FilipinoGrapheme::O],
                        SourceGrapheme::U => vec![FilipinoGrapheme::U],
                        SourceGrapheme::Y => vec![FilipinoGrapheme::Y],
                        SourceGrapheme::GE => vec![FilipinoGrapheme::E],

                        SourceGrapheme::L => {
                            if prev_grapheme == Some(&SourceGrapheme::K) ||
                                prev_grapheme == Some(&SourceGrapheme::C) || 
                                prev_grapheme == Some(&SourceGrapheme::T) {
                                vec![FilipinoGrapheme::E]
                            } else {
                                vec![FilipinoGrapheme::O]
                            }
                            
                        },
                        _ => vec![FilipinoGrapheme::O],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }
                
                // "ʌ"
                if *symbol == IPASymbol::OpenMidBack {
                    let fg = match grapheme {
                        SourceGrapheme::E => vec![FilipinoGrapheme::E],
                        SourceGrapheme::A => vec![FilipinoGrapheme::A],
                        SourceGrapheme::O => vec![FilipinoGrapheme::O],
                        SourceGrapheme::U => vec![FilipinoGrapheme::A],
                        _ => vec![FilipinoGrapheme::A],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // "ɑ"
                if *symbol == IPASymbol::OpenBackUnrounded {
                    let fg = match grapheme {
                        SourceGrapheme::O => vec![FilipinoGrapheme::O],
                        _ => vec![FilipinoGrapheme::A],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // "ɛ"
                if *symbol == IPASymbol::OpenMidFront {
                    let fg = match grapheme {
                        SourceGrapheme::A => vec![FilipinoGrapheme::A],
                        _ => vec![FilipinoGrapheme::E],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // "ɜ"
                if *symbol == IPASymbol::OpenMidCentral {
                    let fg = match grapheme {
                        SourceGrapheme::I => vec![FilipinoGrapheme::I, FilipinoGrapheme::R],
                        SourceGrapheme::E => vec![FilipinoGrapheme::E, FilipinoGrapheme::R],
                        SourceGrapheme::O => vec![FilipinoGrapheme::O, FilipinoGrapheme::R],
                        _ => vec![FilipinoGrapheme::U, FilipinoGrapheme::R],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // "ɔ"
                if *symbol == IPASymbol::OpenMidBackRounded {
                    let next_grapheme = aligned.get(idx + 1)
                        .map(|(next_g, _)| next_g);
                    let fg = match grapheme {
                        SourceGrapheme::A => {
                            if next_grapheme == Some(&SourceGrapheme::U) {
                                vec![FilipinoGrapheme::O]
                            } else {
                                vec![FilipinoGrapheme::A]
                            }
                        },
                        _ => vec![FilipinoGrapheme::O],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // "ɚ"
                if *symbol == IPASymbol::RColoredSchwa {
                    let fg = match grapheme {
                        SourceGrapheme::A => vec![FilipinoGrapheme::A, FilipinoGrapheme::R],
                        SourceGrapheme::O => vec![FilipinoGrapheme::O, FilipinoGrapheme::R],
                        SourceGrapheme::U => vec![FilipinoGrapheme::E, FilipinoGrapheme::R],
                        SourceGrapheme::ORE => vec![FilipinoGrapheme::O, FilipinoGrapheme::R],
                        _ => vec![FilipinoGrapheme::E, FilipinoGrapheme::R],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // "t"
                if *symbol == IPASymbol::VoicelessAlveolarStop {
                    let fg = match grapheme {
                        SourceGrapheme::D => vec![FilipinoGrapheme::D],
                        SourceGrapheme::ED => vec![FilipinoGrapheme::D],
                        _ => vec![FilipinoGrapheme::T],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // "ʃ"
                if *symbol == IPASymbol::VoicelessPostalveolarFricative {
                    let next_grapheme_vowel = aligned.get(idx + 1)
                        .map(|(next_g, _)| next_g.is_vowel()).unwrap_or(false);
                    let fg = match grapheme {
                        SourceGrapheme::SH => {
                            if !next_grapheme_vowel {
                                vec![FilipinoGrapheme::S]
                            } else {
                                vec![FilipinoGrapheme::S, FilipinoGrapheme::Y]
                            }
                        },

                        _ => vec![FilipinoGrapheme::S, FilipinoGrapheme::Y],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // "ŋ"
                if *symbol == IPASymbol::VelarNasal {
                    let next_has_velar = if idx < aligned.len() - 1 {
                                    aligned[idx + 1].1.iter().any(|p| {
                                        matches!(p, 
                                            Some(IPASymbol::VoicedVelarStop) |
                                            Some(IPASymbol::VoicelessVelarStop)
                                        )
                                    })
                                } else {
                                    false
                                };

                    let fg = if next_has_velar {
                            vec![FilipinoGrapheme::N]
                        } else {
                            vec![FilipinoGrapheme::Ng]
                        };

                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // "ɹ"
                if *symbol == IPASymbol::AlveolarApproximant {
                    let fg = match grapheme {
                        SourceGrapheme::R => {
                             let prev_has_r_sound = if idx > 0 {
                                aligned[idx - 1].1.iter().any(|p| {
                                    matches!(p, Some(IPASymbol::RColoredSchwa) | Some(IPASymbol::RColoredMid))
                                })
                            } else {
                                false
                            };

                            if prev_has_r_sound {
                                vec![]
                            } else {
                                vec![FilipinoGrapheme::R]
                            }
                        },
                        _ => vec![FilipinoGrapheme::R],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // Default mapping
                if let Some(graphemes) = IPA_TO_FG.get(symbol) {
                    for g in graphemes {
                        result.push(g.clone());
                    };
                }
            }
        }
    }
    
    result
}