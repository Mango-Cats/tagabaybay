use crate::error::PhonetizationError;
use once_cell::sync::Lazy;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Mutex;

/// Embedded Python G2P server script with inline dependencies (PEP 723)
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
    elif word == "QUIT":
        break
    else:
        try:
            print(g2p(word), flush=True)
        except Exception as e:
            print(f"ERROR:{e}", flush=True)
"#;

/// Persistent Python G2P process
struct G2PProcess {
    _child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    _temp_file: std::path::PathBuf,
}

impl G2PProcess {
    fn new() -> Result<Self, std::io::Error> {
        let temp_dir = std::env::temp_dir();
        let script_path = temp_dir.join("tagabaybay_g2p.py");
        std::fs::write(&script_path, G2P_SCRIPT)?;

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
            panic!("Uv doesn't exist")
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

        // Wait for READY signal
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

    fn phonemize(&mut self, word: &str) -> Result<String, std::io::Error> {
        // Send word
        writeln!(self.stdin, "{}", word)?;
        self.stdin.flush()?;

        // Read response
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

/// Global G2P process
static G2P: Lazy<Mutex<Option<G2PProcess>>> = Lazy::new(|| match G2PProcess::new() {
    Ok(process) => Mutex::new(Some(process)),
    Err(e) => {
        eprintln!("Warning: Failed to start G2P server: {}", e);
        Mutex::new(None)
    }
});

/// Phonetic transducer using Python espeak backend
///
/// # Backend
///
/// See `G2P_SCRIPT` in `g2py.rs` (should be at the top of the file).
///
/// For a Rust-only implementation, see `g2p.rs` which uses a phonetisaurus.
///
/// # Arguments
///
/// * `word` - The word to transcribe phonetically
///
/// # Returns
///
/// Returns `Ok(String)` with the IPA phonetic transcription.
/// Returns `Err` if phonemization fails.
///
/// # Example
///
/// ```ignore
/// let phonemes = phonemize("action")?;
/// assert_eq!(phonemes, "ækʃən");
/// ```
pub fn phonemize(word: &str) -> Result<String, PhonetizationError> {
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
