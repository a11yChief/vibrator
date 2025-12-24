use atspi_common::events::keyboard::KeyboardEvents;
use atspi_common::events::Event;

fn main() {
    let e: Event = unsafe { std::mem::zeroed() };
    match e {
        Event::Keyboard(ke) => {
            match ke {
                KeyboardEvents::Modifiers(m) => {},
                // What else?
            }
        },
        _ => {},
    }
}
