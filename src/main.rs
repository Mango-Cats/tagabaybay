use tagabaybay::adaptation::cursor::Cursor;
use tagabaybay::configs::AdaptationConfig;
use tagabaybay::g2p::phonemize;
use tagabaybay::phoneme::tokenize::tokenize;
use tagabaybay::adaptation::adapter::Adapter;

fn main() {
    let word = "textbook";
    let config: AdaptationConfig = AdaptationConfig::new();
    let ctx = Cursor::from_word(word, None, None, &config).unwrap();
    dbg!(&ctx.graphemes);
    dbg!(&ctx.phonemes);
    dbg!(&ctx.graphemes.len());
    dbg!(&ctx.phonemes.len());
    let tbb = Adapter::new().allow_sh_sound(true);
    if let Ok(result) = tbb.adaptation(word, &config) {
        dbg!(&result);
    }
}
