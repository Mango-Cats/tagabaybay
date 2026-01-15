use crate::graphemes::src_graphemes::SourceGrapheme;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorTypes {
    Adaptation(AdaptationError),
    Phonetization(PhonetizationError),
}

/// Error type for phonetization failures
#[derive(Debug, Clone)]
pub struct PhonetizationError {
    /// The original input text
    pub input: String,
    /// Word number in the dataset (if processing a batch)
    pub word_number: Option<usize>,
    /// Name of the dataset (if processing a batch)
    pub dataset_name: Option<String>,
}

impl PhonetizationError {
    /// Create a new adaptation error
    pub fn new(input: String, word_number: Option<usize>, dataset_name: Option<&str>) -> Self {
        Self {
            input,
            word_number,
            dataset_name: dataset_name.map(String::from),
        }
    }

    /// Print the error with context (for debugging)
    pub fn print_error(&self) {
        println!("error: the word phonetization is impossible; ensure it is lowercased");
    }
}
/// Error type for adaptation failures
#[derive(Debug, Clone)]
pub struct AdaptationError {
    /// The original input text
    pub input: String,
    /// Position where the error occurred
    pub position: usize,
    /// Word number in the dataset (if processing a batch)
    pub word_number: Option<usize>,
    /// Name of the dataset (if processing a batch)
    pub dataset_name: Option<String>,
    /// The grapheme vector for context
    pub graphemes: Vec<SourceGrapheme>,
}

impl AdaptationError {
    /// Create a new adaptation error
    pub fn new(
        graphemes: Vec<SourceGrapheme>,
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
    pub fn print_error(&self) {
        println!("error: the word adaptation is invalid or impossible");
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
            " ".repeat(self.position),
            self.position
        );
        println!("    |");
    }
}

impl fmt::Display for AdaptationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Adaptation error at position {} in '{}'",
            self.position, self.input
        )
    }
}
