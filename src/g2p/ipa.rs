//! Python-based G2P module using espeak-ng backend.
//!
//! This module provides IPA phoneme transcription by spawning a persistent
//! Python subprocess that uses the `phonemizer` library with espeak-ng.
//!
//!  # Usage
//!
//! ```ignore
//! use tagabaybay::g2py::phonemize_to_ipa;
//! use tagabaybay::configs::AdapterConfig;
//!
//! let config = AdapterConfig::default();
//! let phonemes = phonemize_to_ipa("action", None, None, &config)?;
//! // Returns IPA like "ækʃən"
//! ```
//!
//! # Non-Rust Requirements
//!
//! - **[uv](https://docs.astral.sh/uv/)**: Python package manager (handles dependencies automatically)
//! - **espeak-ng**: Speech synthesizer (install via system package manager)
//!
//! # Architecture
//!
//! ```text
//!         [a]           stdin   
//! ------------------- ----------> -------------------
//! ( rust process    )             ( python process  )
//! ( phonemize()     )             ( phonemizer      )
//! ------------------- <---------- -------------------
//!                        stdout           [b]        
//! ```
//!
//! The Python subprocess is spawned lazily on first use and kept alive
//! for the lifetime of the program, avoiding repeated startup overhead.
//!
//! At the start the Python subprocess [b] is spawn and kept alive for
//! the lifetime of the program. That is, while TagaBaybay is running
//! a Python file (the subprocess) is also running. This subprocess is
//! the phonemization algorithm using Phonemizer from Bernard et al.,
//! (2021). Whenever the Rust function phonemize() is called, it
//! throws the input word via stdin which is consumed by the Python
//! process which then throws back its corresponding IPA transcription.
//!
//! ## PEP 723 and UV
//!
//! This architecture only works due to PEP 723 and UV, see the G2P Python
//! script below for the source code [`G2P_SCRIPT`].
//!
//! PEP 723 specifies a standard format for metadata embedding within
//! a Python script. This allows us to include external libraries and
//! Python version required by the Python script.
//!
//! Using base Python (that is, without UV) does not
//! automatically "install" the dependencies specified in the metadata.
//! However, by using UV (that is, running the script with `uv run <script>`)
//! automatically creates a Python environment with the external libraries.
//!
//! Hence, at the start up, two things happen: not only does a Python
//! subprocess cook up. But, also an independent Python environment with
//! the dependencies is created.
//!
//! IMPORTANT. Which is why an important dependency of this file is UV.
//! This is PANIC when UV is NOT AVAILABLE.
//!
//! All of which are removed when the program dies.
//!
//! # References
//!
//! - [PEP 723 - Inline script metadata](https://peps.python.org/pep-0723/)
//! - [uv script dependencies](https://docs.astral.sh/uv/guides/scripts/#running-a-script-with-dependencies)
//! - Bernard et al., (2021). Phonemizer: Text to Phones Transcription for
//!   Multiple Languages in Python. JOSS, 6(68), 3958.
//!   <https://doi.org/10.21105/joss.03958>
//!
//! # See Also
//!
//! - [`g2p`]: Rust-native ARPA phonemization (no Python dependency)
//! - [`g2p_common`]: Shared phrase processing utilities

use crate::configs::AdapterConfig;
use crate::error::{ErrorTypes, PhonetizationError};
use crate::g2p::common::phonemize_phrase;
use once_cell::sync::Lazy;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Mutex;

/// Embedded Python script for G2P conversion.
///
/// Uses PEP 723 inline script metadata for automatic dependency management.
/// When run via `uv run`, dependencies are installed automatically in an
/// isolated environment.
///
/// # Protocol
///
/// 1. Script prints "READY" when initialized
/// 2. Reads words from stdin (one per line)
/// 3. Writes IPA phonemes to stdout (one per line)
/// 4. Prefixes errors with "ERROR:"
/// 5. Exits on "CEXIT" (Clean EXIT) sentinel
const G2P_SCRIPT: &str = r#"# /// script
# requires-python = ">=3.11"
# dependencies = ["phonemizer"]
# ///
import sys
from phonemizer import phonemize

def g2p(word):
    return phonemize(word, language="en-us", backend="espeak", strip=True, with_stress=False).strip()

sys.stdout.reconfigure(line_buffering=True)
print("READY", flush=True)
for line in sys.stdin:
    word = line.strip()
    if not word:
        print("", flush=True)
    elif word == "CEXIT":
        break
    else:
        try:
            print(g2p(word), flush=True)
        except Exception as e:
            print(f"ERROR:{e}", flush=True)
"#;

/// Persistent Python G2P subprocess.
///
/// Manages a long-running Python process that performs phonemization.
/// The process is spawned lazily and reused for all subsequent calls,
/// avoiding the overhead of repeated process creation.
///
/// # Fields
///
/// * `_child` - The spawned child process (kept for lifetime management)
/// * `stdin` - Write channel for sending words to phonemize
/// * `stdout` - Read channel for receiving phonemized results
/// * `_temp_file` - Path to the temporary Python script file
struct G2PProcess {
    _child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    _temp_file: std::path::PathBuf,
}

impl G2PProcess {
    /// Spawn a new Python G2P subprocess.
    ///
    /// Writes the embedded Python script to a temp file and spawns it
    /// via `uv run`, which handles dependency installation automatically.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `uv` is not installed
    /// - The Python process fails to start
    /// - The process doesn't send the "READY" signal
    ///
    /// # Panics
    ///
    /// Panics if `uv` is not available on the system.
    fn new() -> Result<Self, std::io::Error> {
        let temp_dir = std::env::temp_dir();
        let script_path = temp_dir.join("tagabaybay_g2p.py");
        std::fs::write(&script_path, G2P_SCRIPT)?;

        // Check for uv and prepare command
        let (program, args): (&str, Vec<String>) = if Command::new("uv")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            (
                "uv",
                vec!["run".into(), script_path.to_string_lossy().into()],
            )
        } else {
            panic!("uv is required but not found. Install from: https://docs.astral.sh/uv/")
        };

        let mut child = Command::new(program)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = BufReader::new(child.stdout.take().expect("Failed to open stdout"));

        let mut process = G2PProcess {
            _child: child,
            stdin,
            stdout,
            _temp_file: script_path,
        };

        // Wait for the Python process to signal readiness
        let mut ready_line = String::new();
        process.stdout.read_line(&mut ready_line)?;
        if !ready_line.trim().eq("READY") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("G2P server didn't send READY, got: {}", ready_line),
            ));
        }

        Ok(process)
    }

    /// Phonemize a single word via the Python subprocess.
    ///
    /// Sends the word to the Python process and reads the IPA result.
    ///
    /// # Arguments
    ///
    /// * `word` - A single word to phonemize (will be lowercased)
    ///
    /// # Errors
    ///
    /// Returns an error if the Python process reports an error or
    /// if communication with the subprocess fails.
    fn phonemize(&mut self, word: &str) -> Result<String, std::io::Error> {
        writeln!(self.stdin, "{}", word)?;
        self.stdin.flush()?;

        let mut response = String::new();
        self.stdout.read_line(&mut response)?;

        let response = response.trim();
        if response.starts_with("ERROR:") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                response[6..].to_string(),
            ));
        }

        Ok(response.to_string())
    }
}

impl Drop for G2PProcess {
    /// Sends the sentinel value to trigger a clean exit in the Python script.
    fn drop(&mut self) {
        let _ = writeln!(self.stdin, "CEXIT");
        let _ = self.stdin.flush();
    }
}

/// Global lazily-initialized G2P process.
///
/// The process is created on first use and protected by a mutex for
/// thread-safe access. If initialization fails, the mutex contains `None`
/// and all phonemization attempts will fail gracefully.
static G2P: Lazy<Mutex<Option<G2PProcess>>> = Lazy::new(|| match G2PProcess::new() {
    Ok(process) => Mutex::new(Some(process)),
    Err(e) => {
        eprintln!("Warning: Failed to start G2P server: {}", e);
        Mutex::new(None)
    }
});

/// Convert a single word to IPA phonemes using the Python backend.
///
/// Thread-safe wrapper around the global G2P process.
///
/// # Arguments
///
/// * `word` - A single word (no spaces or hyphens) to phonemize
///
/// # Returns
///
/// IPA phoneme string (e.g., "ækʃən" for "action").
///
/// # Errors
///
/// Returns a `PhonetizationError` if:
/// - The G2P mutex is poisoned
/// - The G2P server is not available
/// - The Python process returns an error
///
/// # Example
///
/// ```ignore
/// let phonemes = phonemize_internal("action")?;
/// assert_eq!(phonemes, "ækʃən");
/// ```
fn phonemize_internal(word: &str) -> Result<String, PhonetizationError> {
    let mut guard = G2P
        .lock()
        .map_err(|_| PhonetizationError::new(word.to_string(), None, Some("G2P mutex poisoned")))?;

    let process = guard.as_mut().ok_or_else(|| {
        PhonetizationError::new(word.to_string(), None, Some("G2P server not available"))
    })?;

    process
        .phonemize(&word.to_lowercase())
        .map_err(|e| PhonetizationError::new(word.to_string(), None, Some(&e.to_string())))
}

/// Phonemize a phrase to IPA phonemes.
///
/// Processes multi-word phrases and hyphenated compounds, returning
/// IPA phonemes with `$` separating word boundaries.
///
/// # Arguments
///
/// * `phrase` - Input text (may contain multiple words separated by spaces)
/// * `word_number` - Optional line number for error reporting
/// * `dataset_name` - Optional dataset identifier for error context
/// * `config` - Adapter configuration controlling error behavior
///
/// # Returns
///
/// * `Ok(String)` - IPA phonemes with `$` word separators
/// * `Err(ErrorTypes)` - If phonemization fails
///
/// # Example
///
/// ```ignore
/// let config = AdapterConfig::default();
///
/// // Single word
/// let result = phonemize_to_ipa("action", None, None, &config)?;
/// // Returns "ækʃən"
///
/// // Multi-word phrase  
/// let result = phonemize_to_ipa("hello world", None, None, &config)?;
/// // Returns "həloʊ$wɜːld"
///
/// // Hyphenated compound
/// let result = phonemize_to_ipa("self-care", None, None, &config)?;
/// // Returns "sɛlf$kɛɹ"
/// ```
///
/// # Error Handling
///
/// If `config.panic_at_error` is `true`, panics on phonemization failure.
/// Otherwise, returns an `ErrorTypes::Phonetization` error.
pub fn phonemize_to_ipa(
    phrase: &str,
    word_number: Option<usize>,
    dataset_name: Option<&str>,
    config: &AdapterConfig,
) -> Result<String, ErrorTypes> {
    phonemize_phrase(
        phrase,
        word_number,
        dataset_name,
        config,
        phonemize_internal,
    )
}
