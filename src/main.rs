use std::io;
use std::io::Write;
use tagabaybay::adaptation::adapter::Adapter;
use tagabaybay::alignment::aligned_string::ipa_to_filipino_graphemes;
use tagabaybay::alignment::alignment::phoneme_grapheme_alignment;
use tagabaybay::configs::AdapterConfig;
use tagabaybay::g2p::G2Py;
use tagabaybay::grapheme::filipino::graphemes_to_string;
use tagabaybay::grapheme::filipino::hyphenate;
use tagabaybay::grapheme::tokenize::source_tokenizer;
use tagabaybay::phoneme::tokenizer::ipa::tokenize_ipa;
use tagabaybay::syllabification::algorithm::syllabify;

fn main() {
    let mut config = AdapterConfig::new();
    let mut adapter = Adapter::new_with_config(config.clone());

    println!("TagaBaybay Interactive Shell");
    println!("Initializing G2P...");
    let mut ipa_g2p = G2Py::new().ok();

    if ipa_g2p.is_some() {
        println!("IPA G2P initialized successfully");
    } else {
        println!("IPA G2P not available (eSpeak-NG or UV not found)");
    }

    println!("\nCommands:");
    println!("  qq - quit");
    println!("  !stress - toggle stress/prominence assignment (on by default)");
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

        if input == "!stress" {
            let enabled = !config.assign_prominence;
            config = config.set_assign_prominence(enabled);
            adapter = Adapter::new_with_config(config.clone());
            println!(
                "* stress/prominence assignment: {}\n",
                if enabled { "ON" } else { "OFF" }
            );
            continue;
        }

        if let Some(ref mut g2p) = ipa_g2p {
            if let Ok(phonemes) = g2p.phonemize_phrase(&input, None, None, &config) {
                println!("* {phonemes}");
                let aligned_string =
                    phoneme_grapheme_alignment(tokenize_ipa(&phonemes), source_tokenizer(input));

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
                    println!("* {}\t|| {}", hyph, is_valid)
                }
                if config.assign_prominence {
                    match adapter.prominence(input, &result) {
                        Ok(Some(prominence)) => {
                            println!("* respelled:            {}", prominence.respelled);
                            println!("* with stress:          {}", prominence.with_stress);
                            println!(
                                "* with stress+syllable: {}",
                                prominence.with_stress_and_syllabified
                            );
                            println!(
                                "* english stress on penult: {}\n",
                                prominence.english_stress_on_penult
                            );
                        }
                        Ok(None) => println!("* prominent syllable: unknown (no stress info)\n"),
                        Err(_) => println!(),
                    }
                } else {
                    println!();
                }
            }
            Err(_) => (),
        }
    }
    // When main exits, `ipa_g2p` and `adapter` are dropped,
    // which cleans up the Python subprocess and deletes the temp script file.
}
