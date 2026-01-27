use std::io;
use std::io::Write;
use tagabaybay::adaptation::cursor::phoneme_grapheme_alignment;
use tagabaybay::adaptation::adapter::Adapter;
use tagabaybay::configs::AdapterConfig;
use tagabaybay::g2p::G2Py;
use tagabaybay::grapheme::filipino::graphemes_to_string;
use tagabaybay::grapheme::filipino::hyphenate;
use tagabaybay::syllabification::algorithm::syllabify;
use tagabaybay::phoneme::tokenizer::ipa::tokenize_ipa;
use tagabaybay::grapheme::tokenize::source_tokenizer;

fn main() {
    let config = AdapterConfig::new();
    let mut adapter = Adapter::new_with_config(config.clone());
    let mut ipa_g2p = G2Py::new().ok();

    loop {
        print!("? ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error!");
        let input = input.trim();
        if input == "qq" {
            break;
        }
        if let Some(ref mut g2p) = ipa_g2p {
            if let Ok(phonemes) = g2p.phonemize_phrase(&input, None, None, &config) {
                println!("* {phonemes}");
                phoneme_grapheme_alignment(tokenize_ipa(&phonemes), source_tokenizer(input));
            }
        }

        match adapter.adaptation(&input) {
            Ok(result) => {
                println!("* {}\t-> {}", input, graphemes_to_string(&result));
                if let Some((syll, is_valid)) = syllabify(&result) {
                    let hyph = hyphenate(&syll);
                    println!("* {}\t|| {}\n", hyph, is_valid)
                }
            }
            Err(_) => (),
        }
    }
    // When main exits, `ipa_g2p` and `adapter` are dropped,
    // which cleans up the Python subprocess and deletes the temp script file.
}
