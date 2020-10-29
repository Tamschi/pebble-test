#![no_std]
#![warn(clippy::pedantic)]

use debugless_unwrap::DebuglessUnwrap as _;
use pebble::{
	foundation::app,
	standard_c::CStr,
	user_interface::window::number_window::{NumberWindow, NumberWindowData},
};

#[no_mangle]
pub extern "C" fn main() -> i32 {
	let label = CStr::try_from_static("miles to see you\0").unwrap();
	let number_window = NumberWindow::new(
		&label,
		NumberWindowData {
			incremented: |_, _| (),
			decremented: |_, _| (),
			selected: |_, _| (),
			context: (),
		},
	)
	.debugless_unwrap();
	number_window.set_value(10_000);
	let window = number_window.window();
	window.show(true);
	app::event_loop();
	0
}
