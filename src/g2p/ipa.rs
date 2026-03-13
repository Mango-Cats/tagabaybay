//! Python-based G2P module using espeak-ng backend.
//!
//! This module provides IPA phoneme transcription by spawning a persistent
//! Python subprocess that uses the `phonemizer` library with espeak-ng.
//!
//! # Usage
//!
//! ```ignore
//! use tagabaybay::g2p::ipa::G2Py;
//!
//! let mut g2p = G2Py::new()?;
//! let phonemes = g2p.phonemize("action")?;
//! // Returns IPA like "ækʃən"
//! // When `g2p` is dropped, the temp script file is automatically deleted.
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
//! The Python subprocess is spawned on creation of `G2Py` and kept alive
//! until the `G2Py` is dropped, avoiding repeated startup overhead.
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
//! All of which are removed when the `G2Py` is dropped.
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
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

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
///
/// # Performance
///
/// Uses `EspeakBackend` directly instead of the `phonemize()` function.
/// The backend is initialized once at startup and reused for all calls,
/// avoiding repeated backend initialization overhead.
///
/// Before optimization: ~17.5 s
/// After optimization:  ~0.35 s
const G2P_SCRIPT: &str = r#"
# /// script
# requires-python = ">=3.11"
# dependencies = ["phonemizer"]
# ///
import sys
import os
import platform

sys.stdin.reconfigure(encoding='utf-8')
sys.stdout.reconfigure(encoding='utf-8', line_buffering=True)

from phonemizer.backend import EspeakBackend
from phonemizer.separator import Separator

if platform.system() == "Windows":
    from phonemizer.backend.espeak.wrapper import EspeakWrapper
    lib = os.environ.get("ESPEAK_LIB")
    if lib:
        EspeakWrapper.set_library(lib)

backend = EspeakBackend('en-us', with_stress=False)
separator = Separator(phone=' ', word='')

def g2p(word):
    result = backend.phonemize([word], separator=separator, strip=True)
    return result[0].replace(' ', '') if result else ''

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

/// IPA G2P (Grapheme-to-Phoneme) converter using a Python subprocess.
///
/// This struct owns a persistent Python subprocess that performs phonemization.
/// When dropped, it sends a clean exit signal to the subprocess and deletes
/// the temporary Python script file.
///
/// # Example
///
/// ```ignore
/// use tagabaybay::g2p::ipa::G2Py;
///
/// let mut g2p = G2Py::new()?;
/// let ipa = g2p.phonemize("hello")?;
/// println!("{}", ipa); // "həloʊ"
/// // Temp file is automatically deleted when `g2p` goes out of scope
/// ```
pub struct G2Py {
    _child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    temp_file: std::path::PathBuf,
}

impl G2Py {
    /// Create a new IPA G2P subprocess.
    ///
    /// Writes the embedded Python script to a temp file and spawns it
    /// via `uv run`, which handles dependency installation automatically.
    ///
    /// # Errors
    ///
    /// Returns a `G2PError` if:
    /// - `uv` is not installed (`G2PErrorKind::UVNotFound`)
    /// - eSpeak-NG is not found (`G2PErrorKind::EspeakNotFound`)
    /// - The Python process fails to start (`G2PErrorKind::ServerUnavailable`)
    /// - The process doesn't send the "READY" signal (`G2PErrorKind::ServerUnavailable`)
    pub fn new() -> Result<Self, G2PError> {
        let script_path =
            std::env::temp_dir().join(format!("tagabaybay_g2p_{}.py", std::process::id()));
        std::fs::write(&script_path, G2P_SCRIPT).map_err(|e| {
            G2PError::new(G2PErrorKind::ServerUnavailable {
                reason: format!("failed to write G2P script: {}", e),
            })
        })?;

        if !Command::new("uv")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            let _ = std::fs::remove_file(&script_path);
            return Err(G2PError::new(G2PErrorKind::UVNotFound));
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
                let _ = std::fs::remove_file(&script_path);
                G2PError::new(G2PErrorKind::ServerUnavailable {
                    reason: format!("failed to spawn G2P process: {}", e),
                })
            })?;

        let stdin = child.stdin.take().ok_or_else(|| {
            let _ = std::fs::remove_file(&script_path);
            G2PError::new(G2PErrorKind::ServerUnavailable {
                reason: "failed to open stdin pipe".to_string(),
            })
        })?;
        let stdout = BufReader::new(child.stdout.take().ok_or_else(|| {
            let _ = std::fs::remove_file(&script_path);
            G2PError::new(G2PErrorKind::ServerUnavailable {
                reason: "failed to open stdout pipe".to_string(),
            })
        })?);

        let mut process = G2Py {
            _child: child,
            stdin,
            stdout,
            temp_file: script_path,
        };

        let mut ready_line = String::new();
        process.stdout.read_line(&mut ready_line).map_err(|e| {
            G2PError::new(G2PErrorKind::ServerUnavailable {
                reason: format!("failed to read from G2P process: {}", e),
            })
        })?;

        let ready_line = ready_line.trim();

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

    /// Phonemize a single word to IPA.
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
    pub fn phonemize(&mut self, word: &str) -> Result<String, G2PError> {
        let word_lower = word.to_lowercase();
        writeln!(self.stdin, "{}", word_lower).map_err(|e| {
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

    /// Phonemize a phrase (may contain spaces or hyphens) to IPA.
    ///
    /// Words are separated by `$` in the output.
    ///
    /// # Arguments
    ///
    /// * `phrase` - Input text (may contain multiple words)
    /// * `word_number` - Optional line number for error reporting
    /// * `dataset_name` - Optional dataset identifier for error context
    /// * `config` - Adapter configuration controlling error behavior
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - IPA phonemes with `$` word separators
    /// * `Err(ErrorTypes)` - If phonemization fails
    pub fn phonemize_phrase(
        &mut self,
        phrase: &str,
        word_number: Option<usize>,
        dataset_name: Option<&str>,
        config: &AdapterConfig,
    ) -> Result<String, ErrorTypes> {
        phonemize_phrase(phrase, word_number, dataset_name, config, |word| {
            self.phonemize(word).map_err(|mut e| {
                e.word_number = word_number;
                e.dataset_name = dataset_name.map(String::from);
                e
            })
        })
    }

}

impl Drop for G2Py {
    /// Sends the sentinel value to trigger a clean exit in the Python script
    /// and deletes the temporary Python script file.
    fn drop(&mut self) {
        let _ = writeln!(self.stdin, "CEXIT");
        let _ = self.stdin.flush();
        let _ = std::fs::remove_file(&self.temp_file);
    }
}
