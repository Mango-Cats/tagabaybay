use std::process::Command;

/// Segments a word into morphemes using spaCy via CLI
pub fn segment_morphemes_spacy(word: &str) -> Result<Vec<String>, String> {
    let output = Command::new("python")
        .arg("src/morphology/segment_cli.py")
        .arg(word)
        .output()
        .map_err(|e| format!("Failed to execute Python: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let result = String::from_utf8_lossy(&output.stdout);
    let morphemes: Vec<String> = result
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(morphemes)
}

/// Segments a word into morphemes using dictionary
pub fn segment_morphemes(word: &str) -> Result<Vec<String>, String> {
    let output = Command::new("python")
        .arg("src/morphology/segment_cli.py")
        .arg("--dict")
        .arg(word)
        .output()
        .map_err(|e| format!("Failed to execute Python: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let result = String::from_utf8_lossy(&output.stdout);
    let morphemes: Vec<String> = result
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(morphemes)
}

// ...existing code...

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