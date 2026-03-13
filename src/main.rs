use std::io;
use std::io::Write;
use tagabaybay::adaptation::adapter::Adapter;
use tagabaybay::configs::AdapterConfig;
use tagabaybay::g2p::G2Py;
use tagabaybay::grapheme::filipino::graphemes_to_string;
use tagabaybay::grapheme::filipino::hyphenate;
use tagabaybay::syllabification::algorithm::syllabify;
use tagabaybay::phoneme::tokenizer::ipa::tokenize_ipa;
use tagabaybay::grapheme::tokenize::source_tokenizer;
use tagabaybay::alignment::alignment::phoneme_grapheme_alignment;
use tagabaybay::alignment::aligned_string::ipa_to_filipino_graphemes;
use tagabaybay::alignment::{rebuild_and_align_from_morphology};

fn main() {
    let config = AdapterConfig::new();
    let mut adapter = Adapter::new_with_config(config.clone());
    
    println!("TagaBaybay Interactive Shell");
    println!("Initializing G2P...");
    let mut ipa_g2p = G2Py::new().ok();
    
    if ipa_g2p.is_some() {
        println!("✓ IPA G2P initialized successfully");
    } else {
        println!("✗ IPA G2P not available (eSpeak-NG or UV not found)");
        println!("  Morphology mode will not work without G2P");
    }
    
    println!("\nCommands:");
    println!("  qq - quit");
    println!("  m:<word> - use morphology-aware G2P");
    println!("  <word> - normal processing\n");

    loop {
        print!("? ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error!");
        let input = input.trim();
        if input == "qq" {
            break;
        }

        // Check if morphology mode is requested
        if input.starts_with("m:") {
            let word = input.trim_start_matches("m:");
            if let Some(ref mut g2p) = ipa_g2p {
                println!("\n=== Morphology-Aware G2P Pipeline ===");
                match g2p.phonemize_with_morphology(word) {
                    Ok((phonemes, morphemes)) => {
                        println!("Input: {}", word);
                        println!("Morphemes: {:?}", morphemes);
                        println!("Phonemes (with # boundaries): {}", phonemes);
                        
                        // Rebuild and align
                        let aligned = rebuild_and_align_from_morphology(word, &phonemes, &morphemes);
                        let ipa_fg = ipa_to_filipino_graphemes(&aligned);
                        let mapped = graphemes_to_string(&ipa_fg);
                        
                        println!("Rebuild phonemes: {}", phonemes.replace('#', ""));
                        println!("Filipino output: {}\n", mapped);
                    }
                    Err(e) => {
                        println!("Error: {:?}\n", e);
                    }
                }
            } else {
                println!("\n⚠️  IPA G2P not available!");
                println!("Morphology mode requires IPA G2P.");
                println!("\nTo fix this:");
                println!("  1. Install eSpeak-NG:");
                println!("     - Windows: Download from https://github.com/espeak-ng/espeak-ng/releases");
                println!("     - Set ESPEAK_LIB environment variable to the DLL path");
                println!("  2. Install UV: https://docs.astral.sh/uv/");
                println!();
            }
            continue;
        }

        if let Some(ref mut g2p) = ipa_g2p {
            if let Ok(phonemes) = g2p.phonemize_phrase(&input, None, None, &config) {
                println!("* {phonemes}");
                let aligned_string = phoneme_grapheme_alignment(tokenize_ipa(&phonemes), source_tokenizer(input));

                println!("\nFull IPA mapping:");
                let ipa_tpo_fg = ipa_to_filipino_graphemes(&aligned_string);
                let mapped_string = graphemes_to_string(&ipa_tpo_fg);
                println!("-> {mapped_string}\n");
            }
        }

        match adapter.adaptation(&input) {
            Ok(result) => {
                println!("* {}\t-> {}", input, graphemes_to_string(&result));
                if let Some((syll, is_valid)) = syllabify(&result) {
                    let hyph = hyphenate(&syll);
                    println!("* {}\t|| {}\n", hyph, is_valid)
                }
            }
            Err(_) => (),
        }
    }
    // When main exits, `ipa_g2p` and `adapter` are dropped,
    // which cleans up the Python subprocess and deletes the temp script file.
}
