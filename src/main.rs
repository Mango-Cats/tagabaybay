use tagabaybay::adaptation::adapter::Adapter;
use tagabaybay::configs::AdaptationConfig;
use tagabaybay::grapheme::filipino::phl_graphemes_to_string;

fn main() {
    let words = ["hello", "aspirin", "chocolate", "ibuprofen", "tetracycline"];
    let adapter = Adapter::new();
    let config = AdaptationConfig::new();

    for word in &words {
        match adapter.adaptation(word, &config) {
            Ok(result) => {
                println!("{} -> {}", word, phl_graphemes_to_string(&result));
            }
            Err(e) => println!("{}: ERROR {:?}", word, e),
        }
    }
}
