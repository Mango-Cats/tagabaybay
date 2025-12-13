use crate::consts::PANIC_AT_ERROR;
use crate::tokenization::graphemes::Grapheme;

/// Print an error message for nativization
#[inline]
pub fn printe(
    grapheme_vec: &[Grapheme],
    err_loc: usize,
    word_number: Option<usize>,
    dataset_name: Option<&str>,
) {
    println!("error: the word nativization is invalid or impossible");
    let graphs = grapheme_vec.iter().map(|f| f.as_str()).collect::<String>();
    match dataset_name {
        Some(s) => match word_number {
            Some(n) => println!("  --> {graphs} @ {s}::{n}"),
            None => println!("  --> {graphs} @ {s}"),
        },
        None => println!("  --> graphs"),
    }

    println!("    |");
    println!(
        "    |\t{}",
        grapheme_vec.iter().map(|f| f.as_str()).collect::<String>()
    );
    println!(
        "    |\t{}^ error at token {err_loc}",
        " ".repeat(err_loc.saturating_sub(1))
    );
    println!("    |");

    if PANIC_AT_ERROR {
        panic!()
    }
}
