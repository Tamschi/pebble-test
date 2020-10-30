#![no_std]
#![warn(clippy::pedantic)]

use debugless_unwrap::DebuglessUnwrap as _;
use pebble_skip::{
	foundation::{app, resources},
	standard_c::CStr,
	user_interface::window::number_window::{NumberWindow, NumberWindowData},
};

#[no_mangle]
pub extern "C" fn main() -> i32 {
	let label = CStr::try_from_static("PNG chonk\0").unwrap();
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

	let r#unsafe = resources::get_handle(1);
	let size = resources::size(r#unsafe);
	#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
	number_window.set_value(size as i32);
	let window = number_window.window();
	window.show(true);
	app::event_loop();
	0
}
