#![no_std]
#![warn(clippy::pedantic)]

use debugless_unwrap::DebuglessUnwrap;
use pebble::{
	foundation::app,
	user_interface::window::{Window, WindowHandlers},
};

#[no_mangle]
pub extern "C" fn main() -> i32 {
	let window = Window::new(WindowHandlers {
		load: || (),
		appear: |()| (),
		disappear: |()| (),
		unload: |()| (),
	})
	.debugless_unwrap();
	window.show(true);
	app::event_loop();
	0
}
