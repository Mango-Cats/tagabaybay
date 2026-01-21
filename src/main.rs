use std::io;
use std::io::Write;

use tagabaybay::adaptation::adapter::Adapter;
use tagabaybay::configs::AdapterConfig;
use tagabaybay::g2py::phonemize;
use tagabaybay::grapheme::filipino::graphemes_to_string;
use tagabaybay::grapheme::filipino::hyphenate;
use tagabaybay::syllabification::algorithm::syllabify;

fn main() {
    let adapter =
        Adapter::new_with_config(AdapterConfig::new().set_g2p_unpredictable_variants(false));

    loop {
        print!("=? ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error!");
        let input = input.trim();
        if input == "??" {
            break;
        }
        if let Ok(phonemes) = phonemize(&input) {
            println!("* {phonemes}")
        }

        match adapter.adaptation(&input) {
            Ok(result) => {
                println!("* {} -> {}", input, graphemes_to_string(&result));
                if let Some((syll, is_valid)) = syllabify(&result) {
                    let hyph = hyphenate(&syll);
                    println!("* {} || {}\n", hyph, is_valid)
                }
            }
            Err(_) => (),
        }
        println!("===============")
    }
}
