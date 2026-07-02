//! # TagaBaybay stream worker
//!
//! A long-lived CLI worker meant to be spawned by a parent process and driven
//! over a persistent stream (stdin/stdout IPC). Unlike the interactive shell in
//! `main.rs`, this binary speaks a machine-readable, newline-delimited JSON
//! (JSONL) protocol: the parent writes **one JSON request per line** to the
//! child's stdin, and the child writes **exactly one JSON response per line**
//! to its stdout, flushing after each so the parent never blocks.
//!
//! The G2P subprocess and the [`Adapter`] are initialized once at startup and
//! reused across every request, so per-word latency stays low.
//!
//! ## Protocol
//!
//! On startup the worker emits a readiness event:
//!
//! ```json
//! {"type":"ready","ipa_g2p":true}
//! ```
//!
//! ### Requests (one JSON object per line)
//!
//! - `{"id":1,"cmd":"adapt","word":"chocolate"}` — adapt a loanword.
//! - `{"cmd":"config","allow_v_letter":false,"allow_z_letter":false}` — update
//!   the adapter configuration (any omitted flag keeps its current value). The
//!   adapter is rebuilt with the new config for subsequent `adapt` calls.
//! - `{"cmd":"ping"}` — liveness check; replies with a `pong`.
//! - `{"cmd":"shutdown"}` — clean exit. Closing stdin (EOF) also exits.
//!
//! The optional `id` field is echoed back on the matching response so the parent
//! can correlate requests and replies when pipelining.
//!
//! ### Responses
//!
//! - Result: `{"id":1,"type":"result","word":"chocolate","adapted":"tsokoleyt",
//!   "syllables":"tso-ko-leyt","valid":true,"ipa":"tʃɑkɫət",
//!   "ipa_mapped":"tsokolet"}`
//! - Ok:    `{"id":null,"type":"ok"}`
//! - Pong:  `{"id":null,"type":"pong"}`
//! - Error: `{"id":1,"type":"error","message":"..."}`
//!
//! When the IPA G2P backend is unavailable (eSpeak-NG / UV missing) the `ipa`
//! and `ipa_mapped` fields are omitted from results.

use std::io::{self, BufRead, Write};

use serde::{Deserialize, Serialize};

use tagabaybay::adaptation::adapter::Adapter;
use tagabaybay::alignment::aligned_string::ipa_to_filipino_graphemes;
use tagabaybay::alignment::alignment::phoneme_grapheme_alignment;
use tagabaybay::configs::AdapterConfig;
use tagabaybay::g2p::G2Py;
use tagabaybay::grapheme::filipino::{graphemes_to_string, hyphenate};
use tagabaybay::grapheme::tokenize::source_tokenizer;
use tagabaybay::phoneme::tokenizer::ipa::tokenize_ipa;
use tagabaybay::syllabification::algorithm::syllabify;

/// A single request line from the parent process.
#[derive(Debug, Deserialize)]
struct Request {
    /// Optional correlation id, echoed back on the matching response.
    #[serde(default)]
    id: Option<serde_json::Value>,
    /// The command to run: `adapt`, `config`, `ping`, or `shutdown`.
    cmd: String,
    /// Word to adapt (required for `cmd == "adapt"`).
    #[serde(default)]
    word: Option<String>,

    // Optional config overrides (used by `cmd == "config"`).
    #[serde(default)]
    use_ipa: Option<bool>,
    #[serde(default)]
    allow_sh_letter: Option<bool>,
    #[serde(default)]
    allow_z_letter: Option<bool>,
    #[serde(default)]
    allow_j_letter: Option<bool>,
    #[serde(default)]
    allow_v_letter: Option<bool>,
    #[serde(default)]
    g2p_unpredictable_variants: Option<bool>,
}

/// A single response line back to the parent process.
#[derive(Debug, Serialize)]
struct Response {
    id: Option<serde_json::Value>,
    #[serde(rename = "type")]
    kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    word: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    adapted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    syllables: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipa_mapped: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl Response {
    fn empty(id: Option<serde_json::Value>, kind: &'static str) -> Self {
        Response {
            id,
            kind,
            word: None,
            adapted: None,
            syllables: None,
            valid: None,
            ipa: None,
            ipa_mapped: None,
            message: None,
        }
    }

    fn error(id: Option<serde_json::Value>, message: impl Into<String>) -> Self {
        let mut r = Response::empty(id, "error");
        r.message = Some(message.into());
        r
    }
}

/// Serialize `response` as one JSON line and flush immediately.
///
/// The explicit flush is essential for IPC: when stdout is a pipe it is
/// block-buffered, so without flushing the parent would block waiting for a
/// reply that is still sitting in this process's buffer.
fn emit<W: Write>(out: &mut W, response: &Response) {
    // Serialization of our own flat struct cannot fail; fall back defensively.
    let line = serde_json::to_string(response).unwrap_or_else(|_| {
        r#"{"id":null,"type":"error","message":"serialization failed"}"#.into()
    });
    let _ = writeln!(out, "{line}");
    let _ = out.flush();
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut config = AdapterConfig::new();
    let mut adapter = Adapter::new_with_config(config.clone());

    // Initialize the IPA G2P backend once and reuse it for every request.
    let mut ipa_g2p = G2Py::new().ok();

    // Announce readiness so the parent knows the worker is up and whether the
    // IPA pipeline is available.
    let _ = writeln!(
        stdout,
        r#"{{"type":"ready","ipa_g2p":{}}}"#,
        ipa_g2p.is_some()
    );
    let _ = stdout.flush();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break, // stdin broken; nothing more to read.
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue; // ignore blank keep-alive lines
        }

        let request: Request = match serde_json::from_str(trimmed) {
            Ok(r) => r,
            Err(e) => {
                emit(
                    &mut stdout,
                    &Response::error(None, format!("invalid request: {e}")),
                );
                continue;
            }
        };

        match request.cmd.as_str() {
            "shutdown" => {
                emit(&mut stdout, &Response::empty(request.id, "ok"));
                break;
            }
            "ping" => {
                emit(&mut stdout, &Response::empty(request.id, "pong"));
            }
            "config" => {
                if let Some(v) = request.use_ipa {
                    config = config.set_use_ipa(v);
                }
                if let Some(v) = request.allow_sh_letter {
                    config = config.set_sh_letter(v);
                }
                if let Some(v) = request.allow_z_letter {
                    config = config.set_z_letter(v);
                }
                if let Some(v) = request.allow_j_letter {
                    config = config.set_j_letter(v);
                }
                if let Some(v) = request.allow_v_letter {
                    config = config.set_v_letter(v);
                }
                if let Some(v) = request.g2p_unpredictable_variants {
                    config = config.set_g2p_unpredictable_variants(v);
                }
                // Rebuild the adapter so later `adapt` calls use the new config.
                adapter = Adapter::new_with_config(config.clone());
                emit(&mut stdout, &Response::empty(request.id, "ok"));
            }
            "adapt" => {
                let Some(word) = request.word.as_deref().map(str::trim) else {
                    emit(
                        &mut stdout,
                        &Response::error(request.id, "missing `word` field for adapt"),
                    );
                    continue;
                };
                if word.is_empty() {
                    emit(&mut stdout, &Response::error(request.id, "`word` is empty"));
                    continue;
                }

                let mut response = Response::empty(request.id.clone(), "result");
                response.word = Some(word.to_string());

                // Optional IPA pipeline (mirrors the interactive shell).
                if let Some(g2p) = ipa_g2p.as_mut() {
                    if let Ok(phonemes) = g2p.phonemize_phrase(word, None, None, &config) {
                        let aligned = phoneme_grapheme_alignment(
                            tokenize_ipa(&phonemes),
                            source_tokenizer(word),
                        );
                        let mapped = graphemes_to_string(&ipa_to_filipino_graphemes(&aligned));
                        response.ipa = Some(phonemes);
                        response.ipa_mapped = Some(mapped);
                    }
                }

                match adapter.adaptation(word) {
                    Ok(result) => {
                        response.adapted = Some(graphemes_to_string(&result));
                        if let Some((syll, is_valid)) = syllabify(&result) {
                            response.syllables = Some(hyphenate(&syll));
                            response.valid = Some(is_valid);
                        }
                        emit(&mut stdout, &response);
                    }
                    Err(e) => {
                        emit(
                            &mut stdout,
                            &Response::error(request.id, format!("adaptation failed: {e:?}")),
                        );
                    }
                }
            }
            other => {
                emit(
                    &mut stdout,
                    &Response::error(request.id, format!("unknown command: {other}")),
                );
            }
        }
    }

    // Dropping `ipa_g2p` and `adapter` here tears down the Python subprocess and
    // removes its temp script, exactly as the interactive shell does on exit.
}
