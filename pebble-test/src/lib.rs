#![no_std]

use pebble::{app, window::Window};

#[no_mangle]
pub extern "C" fn main() {
    let window = Window::new(|| (), |_| (), |_| (), |_| ());
    window.show(true);
    app::event_loop();
}
