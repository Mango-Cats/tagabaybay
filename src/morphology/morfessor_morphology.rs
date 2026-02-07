// use pyo3::prelude::*;
// use pyo3::types::PyList;

// /// Segments a word into morphemes using spaCy's linguistic models
// pub fn segment_morphemes_spacy(word: &str) -> PyResult<Vec<String>> {
//     Python::with_gil(|py| {
//         // Load the Python parser module
//         let py_code = include_str!("../parser.py");
//         let module = PyModule::from_code_bound(py, py_code, "parser.py", "parser")?;

//         // Load the spaCy model
//         let model = module.getattr("load_spacy_model")?.call0()?;

//         // Segment the word using spaCy
//         let result: Bound<PyList> = module
//             .getattr("segment_spacy")?
//             .call1((word, &model))?
//             .downcast_into()?;

//         // Convert to Rust Vec
//         let morphemes: Vec<String> = result.extract()?;
//         Ok(morphemes)
//     })
// }

// /// Segments a word into morphemes using the trained Morfessor model
// pub fn segment_morphemes(word: &str) -> PyResult<Vec<String>> {
//     Python::with_gil(|py| {
//         // Load the Python parser module
//         let py_code = include_str!("../parser.py");
//         let module = PyModule::from_code_bound(py, py_code, "parser.py", "parser")?;

//         // Load the model
//         let model_path = "src/morphology/model.bin";
//         let model = module.getattr("load_model")?.call1((model_path,))?;

//         // Segment the word
//         let result: Bound<PyList> = module
//             .getattr("segment")?
//             .call1((word, &model))?
//             .downcast_into()?;

//         // Convert to Rust Vec
//         let morphemes: Vec<String> = result.extract()?;
//         Ok(morphemes)
//     })
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_segmentation() {
//         let word = "unfriendly";
//         let morphemes = segment_morphemes(word).unwrap();
//         println!("{} -> {:?}", word, morphemes);
//         assert!(!morphemes.is_empty());
//     }

//     #[test]
//     fn test_spacy_segmentation() {
//         let word = "running";
//         match segment_morphemes_spacy(word) {
//             Ok(morphemes) => {
//                 println!("{} -> {:?}", word, morphemes);
//                 assert!(!morphemes.is_empty());
//             }
//             Err(e) => {
//                 println!("spaCy not available or model not installed: {}", e);
//                 // Don't fail the test if spaCy isn't installed
//             }
//         }
//     }
// }