use rdev::{listen, EventType, Key};
use std::sync::{Arc, Mutex};
use crate::speech::SpeechEngine;

pub enum Command {
    Quit,
    None,
}

pub struct InputStateMachine {
    insert_pressed: bool,
}

impl InputStateMachine {
    pub fn new() -> Self {
        Self { insert_pressed: false }
    }

    pub fn process_event(&mut self, key: Key, is_press: bool) -> Command {
        match (key, is_press) {
            (Key::Insert, true) => {
                self.insert_pressed = true;
                Command::None
            }
            (Key::Insert, false) => {
                self.insert_pressed = false;
                Command::None
            }
            (Key::KeyQ, true) if self.insert_pressed => Command::Quit,
            _ => Command::None,
        }
    }
}

pub fn start_input_loop(speech: Arc<Mutex<SpeechEngine>>) {
    std::thread::spawn(move || {
        let mut state_machine = InputStateMachine::new();
        
        if let Err(error) = listen(move |event| {
            let is_press = match event.event_type {
                EventType::KeyPress(_) => true,
                EventType::KeyRelease(_) => false,
                _ => return,
            };
            
            // Extract key from event_type
            let key = match event.event_type {
                EventType::KeyPress(k) => k,
                EventType::KeyRelease(k) => k,
                _ => return,
            };

            match state_machine.process_event(key, is_press) {
                Command::Quit => {
                    println!("Quit command received.");
                    let mut s = speech.lock().unwrap();
                    let _ = s.speak("Quitting Vibrator.");
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    std::process::exit(0);
                }
                Command::None => {}
            }
        }) {
            eprintln!("Input Error: {:?}", error);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quit_command() {
        let mut sm = InputStateMachine::new();
        assert!(matches!(sm.process_event(Key::Insert, true), Command::None));
        assert!(matches!(sm.process_event(Key::KeyQ, true), Command::Quit));
    }

    #[test]
    fn test_no_quit_without_insert() {
        let mut sm = InputStateMachine::new();
        assert!(matches!(sm.process_event(Key::KeyQ, true), Command::None));
    }
}

