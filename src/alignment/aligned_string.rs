use crate::grapheme::filipino::FilipinoGrapheme;
use crate::{grapheme::source::SourceGrapheme, phoneme::tokens::ipa::IPASymbol};
use super::alignment::AlignedString;
use crate::phoneme::tokens::map::IPA_TO_FG;
use std::vec;

pub fn ipa_to_filipino_graphemes(aligned: &AlignedString) -> String {
    let mut result = String::new();
    
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
                    result.push_str(&fg.iter().map(|g| g.to_string()).collect::<String>());
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
                        _ => vec![FilipinoGrapheme::I],
                    };
                    result.push_str(&fg.iter().map(|g| g.to_string()).collect::<String>());
                    continue;
                }
                
                // Default mapping
                if let Some(graphemes) = IPA_TO_FG.get(symbol) {
                    result.push_str(&graphemes.iter().map(|g| g.to_string()).collect::<String>());
                }
            }
        }
    }
    
    result
}