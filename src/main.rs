use tagabaybay::adaptation::adapter::Adapter;
use tagabaybay::configs::AdapterConfig;
use tagabaybay::g2py::phonemize;
use tagabaybay::grapheme::filipino::graphemes_to_string;
use tagabaybay::grapheme::filipino::hyphenate;
use tagabaybay::syllabification::algorithm::syllabify;

fn main() {
    let words = ["account"];
    let adapter =
        Adapter::new_with_config(AdapterConfig::new().set_g2p_unpredictable_variants(false));

    for word in &words {
        if let Ok(p) = phonemize(word) {
            print!("{}\n", p);
        }
        match adapter.adaptation(word) {
            Ok(result) => {
                println!("{} -> {}", word, graphemes_to_string(&result));
                if let Some((syll, validity)) = syllabify(&result) {
                    let hyph = hyphenate(&syll);
                    println!("syllabification: {hyph} || {validity}\n")
                }
            }
            Err(e) => println!("{}: ERROR {:?}", word, e),
        }
    }
}
