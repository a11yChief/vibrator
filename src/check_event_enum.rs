use atspi_common::events::Event;

fn main() {
    let e: Event = todo!();
    match e {
        Event::Object(_) => {},
        Event::Window(_) => {},
        Event::Keyboard(_) => {},
        Event::Mouse(_) => {},
        Event::Terminal(_) => {},
        Event::Document(_) => {},
        Event::Focus(_) => {},
        // Check what else
    }
}
