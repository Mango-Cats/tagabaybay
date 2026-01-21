use tagabaybay::adaptation::adapter::Adapter;
use tagabaybay::configs::AdapterConfig;
use tagabaybay::grapheme::filipino::graphemes_to_string;
use tagabaybay::grapheme::filipino::hyphenate;
use tagabaybay::syllabification::algorithm::syllabify;

fn main() {
    let words = ["hello", "aspirin", "chocolate", "ibuprofen", "tetracycline"];
    let adapter =
        Adapter::new_with_config(AdapterConfig::new().set_g2p_unpredictable_variants(false));

    for word in &words {
        match adapter.adaptation(word) {
            Ok(result) => {
                println!("{} -> {}", word, graphemes_to_string(&result));
                if let Some((syll, validity)) = syllabify(&result) {
                    let hyph = hyphenate(&syll);
                    println!("syllabification: {hyph} || {validity}")
                }
            }
            Err(e) => println!("{}: ERROR {:?}", word, e),
        }
    }
}
