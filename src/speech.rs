use tts::Tts;
use anyhow::Result;

pub struct SpeechEngine {
    tts: Tts,
}

impl SpeechEngine {
    pub fn new() -> Result<Self> {
        let tts = Tts::default()?;
        Ok(Self { tts })
    }

    pub fn speak(&mut self, text: &str) -> Result<()> {
        self.tts.speak(text, true)?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.tts.stop()?;
        Ok(())
    }
}
