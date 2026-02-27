use crate::grapheme::filipino::FilipinoGrapheme;
use crate::{grapheme::source::SourceGrapheme, phoneme::tokens::ipa::IPASymbol};
use super::alignment::AlignedString;
use crate::phoneme::tokens::map::IPA_TO_FG;
use std::vec;

pub fn ipa_to_filipino_graphemes(aligned: &AlignedString) -> Vec<FilipinoGrapheme> {
    let mut result = Vec::new();
    
    for (idx, (grapheme, phonemes)) in aligned.iter().enumerate() {
        for phoneme_opt in phonemes {
            if let Some(symbol) = phoneme_opt {

                // "dʒ"
                if *symbol == IPASymbol::VoicedPostalveolarAffricate {
                    let fg = if idx == aligned.len() - 1 {
                        vec![FilipinoGrapheme::J]
                    } else {
                        vec![FilipinoGrapheme::D, FilipinoGrapheme::Y]
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
                        _ => vec![FilipinoGrapheme::I],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // "ə"
                if *symbol == IPASymbol::Schwa {
                    let fg = match grapheme {
                        SourceGrapheme::E => vec![FilipinoGrapheme::E],
                        SourceGrapheme::A => vec![FilipinoGrapheme::A],
                        SourceGrapheme::I => vec![FilipinoGrapheme::I],
                        SourceGrapheme::O => vec![FilipinoGrapheme::O],
                        SourceGrapheme::U => vec![FilipinoGrapheme::U],
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

                // "ɚ"
                if *symbol == IPASymbol::RColoredSchwa {
                    let fg = match grapheme {
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
                        SourceGrapheme::ED => vec![FilipinoGrapheme::D],
                        _ => vec![FilipinoGrapheme::T],
                    };
                    for g in fg {
                        result.push(g)
                    };
                    continue;
                }

                // "ŋ"
                if *symbol == IPASymbol::VelarNasal {
                    let next_is_g = aligned.get(idx + 1)
                        .map(|(next_g, _)| *next_g == SourceGrapheme::G).unwrap_or(false);
                    let fg = match grapheme {
                        SourceGrapheme::N if !next_is_g => {
                            vec![FilipinoGrapheme::N]
                        },
                        _ => vec![FilipinoGrapheme::Ng],
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