#![no_std]
#![warn(clippy::pedantic)]

use debugless_unwrap::DebuglessUnwrap as _;
use pebble::{
	foundation::app,
	standard_c::c_str,
	user_interface::window::{
		number_window::{NumberWindow, NumberWindowData},
		Window, WindowHandlers,
	},
};

#[no_mangle]
pub extern "C" fn main() {
	let window = Window::new(WindowHandlers {
		load: || (),
		appear: |_| (),
		disappear: |_| (),
		unload: |_| (),
	})
	.debugless_unwrap();
	window.show(true);
	app::event_loop();
	drop(window);
}
