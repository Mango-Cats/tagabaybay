use crate::tokenization::graphemes::Grapheme;
use std::fmt;

/// Error type for nativization failures
#[derive(Debug, Clone)]
pub struct NativizationError {
    /// The original input text
    pub input: String,
    /// Position where the error occurred
    pub position: usize,
    /// Word number in the dataset (if processing a batch)
    pub word_number: Option<usize>,
    /// Name of the dataset (if processing a batch)
    pub dataset_name: Option<String>,
    /// The grapheme vector for context
    pub graphemes: Vec<Grapheme>,
}

impl NativizationError {
    /// Create a new nativization error
    pub fn new(
        graphemes: Vec<Grapheme>,
        position: usize,
        word_number: Option<usize>,
        dataset_name: Option<&str>,
    ) -> Self {
        let input = graphemes.iter().map(|f| f.as_str()).collect::<String>();
        Self {
            input,
            position,
            word_number,
            dataset_name: dataset_name.map(String::from),
            graphemes,
        }
    }

    /// Print the error with context (for debugging)
    pub fn print_error(&self, go_panic: bool) {
        println!("error: the word nativization is invalid or impossible");
        match &self.dataset_name {
            Some(s) => match self.word_number {
                Some(n) => println!("  --> {} @ {}::{}", self.input, s, n),
                None => println!("  --> {} @ {}", self.input, s),
            },
            None => println!("  --> {}", self.input),
        }

        println!("    |");
        println!("    |\t{}", self.input);
        println!(
            "    |\t{}^ error at token {}",
            " ".repeat(self.position.saturating_sub(1)),
            self.position
        );
        println!("    |");

        if go_panic {
            panic!("Nativization error")
        }
    }
}

impl fmt::Display for NativizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Nativization error at position {} in '{}'",
            self.position, self.input
        )
    }
}

impl std::error::Error for NativizationError {}
