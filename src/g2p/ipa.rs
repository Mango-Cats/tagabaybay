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
use crate::error::{ErrorTypes, G2PError, G2PErrorKind};
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
/// # Notes for Windows
///
/// Windows seems to have a problem with eSpeak where you need to set the library
/// with the `dll` of eSpeak-NG. This is set in the `ESPEAK_LIB` environment
/// variable.
///
/// Before running TagaBaybay ensure:
/// ```cmd
/// set ESPEAK_LIB=<PATH_TO_DLL>
/// ```
///
/// # Protocol
///
/// 1. Script prints "READY" when initialized
/// 2. Reads words from stdin (one per line)
/// 3. Writes IPA phonemes to stdout (one per line)
/// 4. Prefixes errors with "ERROR:"
/// 5. Exits on "CEXIT" (Clean EXIT) sentinel
const G2P_SCRIPT: &str = r#"
    # /// script
    # requires-python = ">=3.11"
    # dependencies = ["phonemizer"]
    # ///
    import sys
    import os
    import platform

    from phonemizer import phonemize

    if platform.system() == "Windows":
        from phonemizer.backend.espeak.wrapper import EspeakWrapper
        lib = os.environ.get("ESPEAK_LIB")
        if lib:
            EspeakWrapper.set_library(lib)
        
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
    /// Returns a `G2PError` if:
    /// - `uv` is not installed (`G2PErrorKind::UvNotFound`)
    /// - eSpeak-NG is not found (`G2PErrorKind::EspeakNotFound`)
    /// - The Python process fails to start (`G2PErrorKind::ServerUnavailable`)
    /// - The process doesn't send the "READY" signal (`G2PErrorKind::ServerUnavailable`)
    fn new() -> Result<Self, G2PError> {
        let temp_dir = std::env::temp_dir();
        let script_path = temp_dir.join("tagabaybay_g2p.py");
        std::fs::write(&script_path, G2P_SCRIPT).map_err(|e| {
            G2PError::new(G2PErrorKind::ServerUnavailable {
                reason: format!("failed to write G2P script: {}", e),
            })
        })?;

        // Check for uv availability
        if !Command::new("uv")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Err(G2PError::new(G2PErrorKind::UvNotFound));
        }

        let program = "uv";
        let args: Vec<String> = vec!["run".into(), script_path.to_string_lossy().into()];

        let mut child = Command::new(program)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                G2PError::new(G2PErrorKind::ServerUnavailable {
                    reason: format!("failed to spawn G2P process: {}", e),
                })
            })?;

        let stdin = child.stdin.take().ok_or_else(|| {
            G2PError::new(G2PErrorKind::ServerUnavailable {
                reason: "failed to open stdin pipe".to_string(),
            })
        })?;
        let stdout = BufReader::new(child.stdout.take().ok_or_else(|| {
            G2PError::new(G2PErrorKind::ServerUnavailable {
                reason: "failed to open stdout pipe".to_string(),
            })
        })?);

        let mut process = G2PProcess {
            _child: child,
            stdin,
            stdout,
            _temp_file: script_path,
        };

        // Wait for the Python process to signal readiness
        let mut ready_line = String::new();
        process.stdout.read_line(&mut ready_line).map_err(|e| {
            G2PError::new(G2PErrorKind::ServerUnavailable {
                reason: format!("failed to read from G2P process: {}", e),
            })
        })?;

        let ready_line = ready_line.trim();

        // Check for specific error conditions
        if ready_line.contains("espeak") && ready_line.contains("not") {
            let hint = if cfg!(target_os = "windows") {
                "set ESPEAK_LIB=C:\\Program Files\\eSpeak NG\\libespeak-ng.dll".to_string()
            } else if cfg!(target_os = "macos") {
                "brew install espeak-ng".to_string()
            } else {
                "sudo apt-get install espeak-ng (Debian/Ubuntu) or sudo pacman -S espeak-ng (Arch)"
                    .to_string()
            };
            return Err(G2PError::new(G2PErrorKind::EspeakNotFound { hint }));
        }

        if !ready_line.eq("READY") {
            return Err(G2PError::new(G2PErrorKind::ServerUnavailable {
                reason: format!("G2P server didn't send READY, got: '{}'", ready_line),
            }));
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
    /// Returns a `G2PError` if:
    /// - Communication with the subprocess fails (`G2PErrorKind::ServerUnavailable`)
    /// - The Python process reports an error (`G2PErrorKind::TranscriptionFailed`)
    fn phonemize(&mut self, word: &str) -> Result<String, G2PError> {
        writeln!(self.stdin, "{}", word).map_err(|e| {
            G2PError::with_input(
                G2PErrorKind::ServerUnavailable {
                    reason: format!("failed to write to G2P process: {}", e),
                },
                word,
            )
        })?;
        self.stdin.flush().map_err(|e| {
            G2PError::with_input(
                G2PErrorKind::ServerUnavailable {
                    reason: format!("failed to flush G2P stdin: {}", e),
                },
                word,
            )
        })?;

        let mut response = String::new();
        self.stdout.read_line(&mut response).map_err(|e| {
            G2PError::with_input(
                G2PErrorKind::ServerUnavailable {
                    reason: format!("failed to read G2P response: {}", e),
                },
                word,
            )
        })?;

        let response = response.trim();
        if response.starts_with("ERROR:") {
            let error_msg = response[6..].trim();
            return Err(G2PError::with_input(
                G2PErrorKind::TranscriptionFailed {
                    message: error_msg.to_string(),
                },
                word,
            ));
        }

        if response.is_empty() {
            return Err(G2PError::with_input(
                G2PErrorKind::TranscriptionFailed {
                    message: "empty IPA transcription returned".to_string(),
                },
                word,
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

/// Result of G2P process initialization.
///
/// Stores either the successfully initialized process or the error
/// that occurred during initialization, allowing for better error reporting.
enum G2PInitResult {
    Ok(G2PProcess),
    Err(G2PError),
}

/// Global lazily-initialized G2P process.
///
/// The process is created on first use and protected by a mutex for
/// thread-safe access. Stores the initialization result to provide
/// detailed error information on subsequent access attempts.
static G2P: Lazy<Mutex<G2PInitResult>> = Lazy::new(|| match G2PProcess::new() {
    Ok(process) => Mutex::new(G2PInitResult::Ok(process)),
    Err(e) => {
        e.print_error();
        Mutex::new(G2PInitResult::Err(e))
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
/// Returns a `G2PError` if:
/// - The G2P mutex is poisoned (`G2PErrorKind::MutexPoisoned`)
/// - The G2P server failed to initialize (returns the original init error)
/// - The Python process returns an error (`G2PErrorKind::TranscriptionFailed`)
///
/// # Example
///
/// ```ignore
/// let phonemes = phonemize_internal("action")?;
/// assert_eq!(phonemes, "ækʃən");
/// ```
fn phonemize_internal(word: &str) -> Result<String, G2PError> {
    let mut guard = G2P
        .lock()
        .map_err(|_| G2PError::with_input(G2PErrorKind::MutexPoisoned, word))?;

    match &mut *guard {
        G2PInitResult::Ok(process) => process.phonemize(&word.to_lowercase()),
        G2PInitResult::Err(init_error) => {
            // Return a clone of the initialization error with the current input context
            Err(G2PError::with_input(init_error.kind.clone(), word))
        }
    }
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
    phonemize_phrase(phrase, word_number, dataset_name, config, |word| {
        phonemize_internal(word).map_err(|mut e| {
            // Add context to the error
            e.word_number = word_number;
            e.dataset_name = dataset_name.map(String::from);
            e
        })
    })
}
