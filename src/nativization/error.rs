use crate::tokenization::graphemes::Grapheme;

/// Print an error message for nativization
pub fn printe(
    grapheme_vec: &[Grapheme],
    err_loc: usize,
    word_number: Option<usize>,
    dataset_name: Option<&str>,
) {
    println!("error: the word nativization is invalid or impossible");

    match dataset_name {
        Some(s) => match word_number {
            Some(n) => println!("  --> {s}::{}", n),
            None => println!("  --> {s}"),
        },
        None => {}
    }

    println!("\n|");
    println!("\n|\t\t{:?}", grapheme_vec);
    println!("\n|\t\t^ error at token {err_loc}");
    println!("\n|");

    panic!()
}
