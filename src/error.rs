use crate::grapheme::source::SourceGrapheme;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorTypes {
    Adaptation(AdaptationError),
    Phonetization(PhonetizationError),
    G2P(G2PError),
}

/// Specific error types for G2P (Grapheme-to-Phoneme) failures
#[derive(Debug, Clone)]
pub enum G2PErrorKind {
    /// uv package manager is not installed
    UVNotFound,
    /// eSpeak-NG library not found (especially the DLL on Windows)
    EspeakNotFound {
        /// Platform-specific hint for resolution
        hint: String,
    },
    /// G2P server failed to start or is unavailable
    ServerUnavailable {
        /// Reason the server is unavailable
        reason: String,
    },
    /// IPA transcription failed (unknown symbol, invalid input, etc.)
    TranscriptionFailed {
        /// The specific error message from the backend
        message: String,
    },
    /// G2P mutex was poisoned (concurrent access failure)
    MutexPoisoned,
}

/// Error type for G2P (IPA) conversion failures
#[derive(Debug, Clone)]
pub struct G2PError {
    /// The kind of G2P error
    pub kind: G2PErrorKind,
    /// The original input text (if available)
    pub input: Option<String>,
    /// Word number in the dataset (if processing a batch)
    pub word_number: Option<usize>,
    /// Name of the dataset (if processing a batch)
    pub dataset_name: Option<String>,
}

impl G2PError {
    /// Create a new G2P error
    pub fn new(kind: G2PErrorKind) -> Self {
        Self {
            kind,
            input: None,
            word_number: None,
            dataset_name: None,
        }
    }

    /// Create a G2P error with input context
    pub fn with_input(kind: G2PErrorKind, input: &str) -> Self {
        Self {
            kind,
            input: Some(input.to_string()),
            word_number: None,
            dataset_name: None,
        }
    }

    /// Create a G2P error with full context
    pub fn with_context(
        kind: G2PErrorKind,
        input: Option<&str>,
        word_number: Option<usize>,
        dataset_name: Option<&str>,
    ) -> Self {
        Self {
            kind,
            input: input.map(String::from),
            word_number,
            dataset_name: dataset_name.map(String::from),
        }
    }

    /// Print the error with context (for debugging)
    pub fn print_error(&self) {
        match &self.kind {
            G2PErrorKind::UVNotFound => {
                println!("error: uv package manager not found");
                println!("  help: install uv from https://docs.astral.sh/uv/");
                println!("  help: on Linux/macOS: curl -LsSf https://astral.sh/uv/install.sh | sh");
                println!(
                    "  help: on Windows: powershell -ExecutionPolicy ByPass -c \"irm https://astral.sh/uv/install.ps1 | iex\""
                );
            }
            G2PErrorKind::EspeakNotFound { hint } => {
                println!("error: eSpeak-NG library not found");
                println!("  help: {}", hint);
            }
            G2PErrorKind::ServerUnavailable { reason } => {
                println!("error: G2P server unavailable");
                println!("  reason: {}", reason);
            }
            G2PErrorKind::TranscriptionFailed { message } => {
                println!("error: IPA transcription failed");
                if let Some(input) = &self.input {
                    println!("  input: '{}'", input);
                }
                println!("  reason: {}", message);
            }
            G2PErrorKind::MutexPoisoned => {
                println!("error: G2P mutex poisoned (concurrent access failure)");
                println!("  help: this is likely a bug, please report it");
            }
        }

        // Print dataset context if available
        if let Some(dataset) = &self.dataset_name {
            if let Some(word_num) = self.word_number {
                println!("  --> @ {}::{}", dataset, word_num);
            } else {
                println!("  --> @ {}", dataset);
            }
        }
    }
}

impl fmt::Display for G2PError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            G2PErrorKind::UVNotFound => write!(f, "uv package manager not found"),
            G2PErrorKind::EspeakNotFound { hint } => {
                write!(f, "eSpeak-NG not found: {}", hint)
            }
            G2PErrorKind::ServerUnavailable { reason } => {
                write!(f, "G2P server unavailable: {}", reason)
            }
            G2PErrorKind::TranscriptionFailed { message } => {
                if let Some(input) = &self.input {
                    write!(f, "IPA transcription failed for '{}': {}", input, message)
                } else {
                    write!(f, "IPA transcription failed: {}", message)
                }
            }
            G2PErrorKind::MutexPoisoned => write!(f, "G2P mutex poisoned"),
        }
    }
}

impl std::error::Error for G2PError {}

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
        let input = graphemes
            .iter()
            .map(|f| f.to_string_rep())
            .collect::<String>();
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
