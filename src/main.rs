use tagabaybay::arpabet::tokenize::tokenize;
use tagabaybay::g2p::phonemize;

fn main() {
    let word = "rhythym".to_string();

    let p = dbg!(phonemize(&word)).unwrap();
    let _ = dbg!(tokenize(&p));
}
