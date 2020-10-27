#![no_std]
#![warn(clippy::pedantic)]

use debugless_unwrap::DebuglessUnwrap;
use pebble::{
	foundation::app,
	graphics::graphics_types::color_definitions::BLUE_MOON,
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
	window.set_background_colour(BLUE_MOON);
	window.show(true);
	app::event_loop();
	0
}
