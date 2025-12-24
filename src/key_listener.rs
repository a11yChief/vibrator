use zbus::dbus_interface;
use std::sync::{Arc, Mutex};
use crate::speech::SpeechEngine;
use serde::{Serialize, Deserialize};
use zbus::zvariant::Type;

pub struct KeyListener {
    pub speech: Arc<Mutex<SpeechEngine>>,
    pub insert_pressed: Arc<Mutex<bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct DeviceEvent {
    pub event_type: u32,
    pub id: i32,
    pub hw_code: u32,
    pub modifiers: u32,
    pub timestamp: i32,
    pub text: String,
    pub is_text: bool,
}

#[dbus_interface(name = "org.a11y.atspi.DeviceEventListener")]
impl KeyListener {
    fn notify_event(&self, event: DeviceEvent) -> bool {
        // event.id is the keycode/keysym
        
        println!("NotifyEvent: {:?}", event);
        
        // 0 is KeyPressed, 1 is KeyReleased
        let is_press = event.event_type == 0;
        let keysym = event.id;

        
        let mut insert = self.insert_pressed.lock().unwrap();
        
        if keysym == 65379 { // Insert
            *insert = is_press;
        } else if keysym == 113 && is_press { // Q
            if *insert {
                println!("Quit command received.");
                let mut s = self.speech.lock().unwrap();
                let _ = s.speak("Quitting Vibrator.");
                std::thread::sleep(std::time::Duration::from_millis(500));
                std::process::exit(0);
            }
        }

        false
    }
}
