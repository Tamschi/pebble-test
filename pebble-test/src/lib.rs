#![no_std]

use pebble::{
    app,
    window::{Window, WindowHandlers},
};

struct WindowState;

#[no_mangle]
pub extern "C" fn main() {
    let window = Window::new(WindowHandlers {
        // mutable closures
        load: || WindowState,
        appear: |_s| (),
        disappear: |_s| (),
        unload: |_s| (),
    });
    window.show(true);
    app::event_loop();
}
