use vibrator::speech::SpeechEngine;
use vibrator::atspi_handler::AtspiHandler;
use vibrator::input::start_input_loop;
use std::sync::{Arc, Mutex};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Vibrator Screen Reader...");

    let speech = Arc::new(Mutex::new(SpeechEngine::new()?));
    {
        let mut s = speech.lock().unwrap();
        s.speak("Vibrator started.")?;
    }

    // Start input loop in a separate thread
    start_input_loop(speech.clone());

    let atspi = AtspiHandler::new(speech.clone()).await?;
    
    println!("Connected to AT-SPI. Listening for events...");
    atspi.process_events().await?;

    Ok(())
}



