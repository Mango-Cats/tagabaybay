use tagabaybay::g2p::phonemize;
use tagabaybay::phoneme::tokenize::tokenize;

fn main() {
    let word = "rhythym".to_string();

    let p = dbg!(phonemize(&word)).unwrap();
    let _ = dbg!(tokenize(&p));
}
