use crate::alignment::{
    aligned_string::ipa_to_filipino_graphemes, alignment::phoneme_grapheme_alignment,
};
use crate::configs::AdapterConfig;
use crate::g2p::G2Py;
use crate::grapheme::filipino::graphemes_to_string;
use crate::grapheme::tokenize::source_tokenizer;
use crate::phoneme::tokenizer::ipa::tokenize_ipa;

pub fn adapt_aligned(input: &str, g2p: &mut G2Py, config: &AdapterConfig) -> String {
    let mut result = String::new();

    let chunks: Vec<(String, String)> = split_preserving_special(input);

    for (letters, special) in chunks {
        if !letters.is_empty() {
            if let Ok(phonemes) = g2p.phonemize_phrase(&letters, None, None, &config) {
                let aligned_string =
                    phoneme_grapheme_alignment(tokenize_ipa(&phonemes), source_tokenizer(&letters));
                let ipa_to_fg = ipa_to_filipino_graphemes(&aligned_string);
                let mapped_string = graphemes_to_string(&ipa_to_fg);

                result.push_str(&mapped_string);
            }
        }

        result.push_str(&special);
    }

    result
}

fn split_preserving_special(input: &str) -> Vec<(String, String)> {
    let mut chunks = Vec::new();
    let mut letters = String::new();
    let mut special = String::new();

    for ch in input.chars() {
        if ch.is_alphabetic() {
            if !special.is_empty() {
                chunks.push((letters.clone(), special.clone()));
                letters.clear();
                special.clear();
            }
            letters.push(ch);
        } else {
            special.push(ch);
        }
    }

    if !letters.is_empty() || !special.is_empty() {
        chunks.push((letters, special));
    }

    chunks
}
