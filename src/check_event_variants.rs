use atspi_common::events::Event;

fn main() {
    let e: Event = unsafe { std::mem::zeroed() }; // Just for checking variants
    match e {
        Event::Object(_) => {},
        Event::Window(_) => {},
        Event::Keyboard(_) => {},
        Event::Mouse(_) => {},
        Event::Terminal(_) => {},
        Event::Document(_) => {},
        Event::Focus(_) => {},
        Event::Listener(_) => {}, // Maybe?
        // Event::Device(_) => {}, // I want to check this
        Event::Keyboard(inner) => {
            let x: i32 = inner;
        },
        _ => {},
    }
}
