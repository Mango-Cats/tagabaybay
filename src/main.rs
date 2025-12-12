#![allow(dead_code)]

mod nativization;
mod tokenization;
mod consts;

use crate::nativization::error::printe;
use crate::nativization::replacement::nativize_word;
use crate::tokenization::tokenize::tokenize;

fn main() {
    let test_words = ["???", "cycle", "queen", "xray", "exam", "bangon", "vitamin"];
    for word in test_words {
        let x = nativize_word(word);
        let y = tokenize(word);
        printe(y.as_slice(), 3, Some(1), Some("Hello World"));
    }
}
