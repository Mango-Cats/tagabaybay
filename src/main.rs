#![allow(dead_code)]

mod tokenization;
mod nativization;

use crate::nativization::replacement::nativize_word;
use crate::tokenization::tokenize::tokenize;

fn main() {
    // Test tokenization
    let word = "bangon";
    let graphemes = tokenize(word);
    println!("Tokenized '{}': {:?}", word, graphemes);

    // Test nativization
    let test_words = ["phone", "cycle", "queen", "xray", "exam", "bangon", "vitamin"];
    for word in test_words {
        let result = nativize_word(word);
        println!("{} -> {}", word, result);
    }
}
