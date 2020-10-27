#![no_std]
#![warn(clippy::pedantic)]

use debugless_unwrap::DebuglessUnwrap;
use pebble::{
	foundation::app,
	graphics::graphics_types::color_definitions::MELON,
	log,
	standard_c::CStr,
	user_interface::window::number_window::{NumberWindow, NumberWindowData},
};

#[no_mangle]
pub extern "C" fn main() -> i32 {
	log!(100, "started");
	let fuck = CStr::try_from_static("Fuck\0").unwrap();
	log!(100, "string created");
	let number_window = NumberWindow::new(
		&fuck,
		NumberWindowData {
			incremented: |_, _| (),
			decremented: |_, _| (),
			selected: |_, _| (),
			context: (),
		},
	)
	.debugless_unwrap();
	log!(100, "number_window created");
	number_window.set_value(2020);
	log!(100, "value set");
	let window = number_window.window();
	log!(100, "window gotten");
	window.set_background_color(MELON);
	log!(100, "background set");
	window.show(true);
	log!(100, "window shown");
	app::event_loop();
	log!(100, "window shutting down");
	0
}
