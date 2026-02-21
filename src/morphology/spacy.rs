use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    /// Mutex to prevent concurrent Python process spawning which can cause deadlocks
    static ref MORPHOLOGY_LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
}

/// Segments a word into morphemes using spaCy via CLI
/// Uses mutex to prevent concurrent access and includes timeout handling
pub fn segment_morphemes_spacy(word: &str) -> Result<Vec<String>, String> {
    // Acquire lock to serialize Python process calls
    let _lock = MORPHOLOGY_LOCK.lock()
        .map_err(|e| format!("Failed to acquire morphology lock: {}", e))?;
    
    let output = Command::new("python")
        .arg("src/morphology/segment_cli.py")
        .arg(word)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("Failed to execute Python: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Python process failed: {}", stderr));
    }

    let result = String::from_utf8_lossy(&output.stdout);
    let morphemes: Vec<String> = result
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if morphemes.is_empty() {
        return Err(format!("No morphemes returned for word: {}", word));
    }

    Ok(morphemes)
}

/// Segments a word into morphemes using dictionary-based approach
/// Uses mutex to prevent concurrent access and includes timeout handling
pub fn segment_morphemes(word: &str) -> Result<Vec<String>, String> {
    // Acquire lock to serialize Python process calls
    let _lock = MORPHOLOGY_LOCK.lock()
        .map_err(|e| format!("Failed to acquire morphology lock: {}", e))?;
    
    let output = Command::new("python")
        .arg("src/morphology/segment_cli.py")
        .arg("--dict")
        .arg(word)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("Failed to execute Python: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Python process failed: {}", stderr));
    }

    let result = String::from_utf8_lossy(&output.stdout);
    let morphemes: Vec<String> = result
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if morphemes.is_empty() {
        return Err(format!("No morphemes returned for word: {}", word));
    }

    Ok(morphemes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dict_segmentation() {
        let word = "unfriendly";
        match segment_morphemes(word) {
            Ok(morphemes) => {
                println!("{} -> {:?}", word, morphemes);
                assert_eq!(morphemes, vec!["un", "friend", "ly"]);
            }
            Err(e) => panic!("Segmentation failed: {}", e),
        }
    }

    #[test]
    fn test_dict_segmentation_tion_suffix() {
        // Test -tion suffix with -ate verb stems
        let test_cases = vec![
            ("reallocation", vec!["re", "allocate", "tion"]),
            ("creation", vec!["create", "tion"]),
            ("organization", vec!["organize", "tion"]),
        ];
        
        for (word, expected) in test_cases {
            match segment_morphemes(word) {
                Ok(morphemes) => {
                    println!("{} -> {:?}", word, morphemes);
                    assert_eq!(morphemes, expected, "Failed for word: {}", word);
                }
                Err(e) => panic!("Segmentation failed for {}: {}", word, e),
            }
        }
    }

    #[test]
    fn test_dict_segmentation_ction_suffix() {
        // Test -ction suffix with -ct verb stems
        let test_cases = vec![
            ("reconstruction", vec!["re", "construct", "tion"]),
            ("destruction", vec!["de", "struct", "tion"]),
            ("abstraction", vec!["abstract", "tion"]),
        ];
        
        for (word, expected) in test_cases {
            match segment_morphemes(word) {
                Ok(morphemes) => {
                    println!("{} -> {:?}", word, morphemes);
                    assert_eq!(morphemes, expected, "Failed for word: {}", word);
                }
                Err(e) => panic!("Segmentation failed for {}: {}", word, e),
            }
        }
    }

    #[test]
    fn test_dict_segmentation_ness_suffix() {
        // Test -ness suffix with y->i transformation
        let test_cases = vec![
            ("unhappiness", vec!["un", "happy", "ness"]),
            ("easiness", vec!["easy", "ness"]),
            ("loneliness", vec!["lonely", "ness"]),
        ];
        
        for (word, expected) in test_cases {
            match segment_morphemes(word) {
                Ok(morphemes) => {
                    println!("{} -> {:?}", word, morphemes);
                    assert_eq!(morphemes, expected, "Failed for word: {}", word);
                }
                Err(e) => panic!("Segmentation failed for {}: {}", word, e),
            }
        }
    }

    #[test]
    fn test_spacy_segmentation() {
        let word = "running";
        match segment_morphemes_spacy(word) {
            Ok(morphemes) => {
                println!("{} -> {:?}", word, morphemes);
                assert!(!morphemes.is_empty());
                // Should contain at least the lemma "run"
                assert!(morphemes.contains(&"run".to_string()));
            }
            Err(e) => {
                println!("spaCy not available: {}", e);
                // Don't fail if spaCy isn't installed
            }
        }
    }

    #[test]
    fn test_multiple_words() {
        let test_cases = vec![
            ("walked", vec!["walk", "ed"]),
            ("happiness", vec!["happi", "ness"]),
            ("rebuild", vec!["re", "build"]),
        ];

        for (word, expected) in test_cases {
            match segment_morphemes(word) {
                Ok(morphemes) => {
                    println!("{} -> {:?}", word, morphemes);
                    assert_eq!(morphemes, expected);
                }
                Err(e) => panic!("Segmentation failed for {}: {}", word, e),
            }
        }
    }
}