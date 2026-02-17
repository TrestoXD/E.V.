fn main() {
    println!("Hello, world!");
}

use std::error::Error;
use natural_tts::{*, models::parler::TtsModel};

fn main() -> Result<(), Box<dyn Error>>{

    // Create the NaturalTts struct using the builder pattern.
    let mut natural = NaturalTtsBuilder::default()
        .tts_model(TtsModel::default())
        .default_model(Model::TTS)
        .build()?;


    // Start producing an output using the default_model.
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));

    // Play the audio until it finishes
    natural.sleep_until_end();
    Ok(())    
}