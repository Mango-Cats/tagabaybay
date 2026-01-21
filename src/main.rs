use tagabaybay::adaptation::adapter::Adapter;
use tagabaybay::configs::AdaptationConfig;
use tagabaybay::grapheme::filipino::graphemes_to_string;
use tagabaybay::syllabification::algorithm::syllabify;
use tagabaybay::grapheme::filipino::hyphenate;

fn main() {
    let words = ["hello", "aspirin", "chocolate", "ibuprofen", "tetracycline"];
    let adapter = Adapter::new();
    let config = AdaptationConfig::new();

    for word in &words {
        match adapter.adaptation(word, &config) {
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
