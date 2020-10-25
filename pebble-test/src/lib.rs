#![no_std]
#![warn(clippy::pedantic)]

use debugless_unwrap::DebuglessUnwrap as _;
use pebble::{
	foundation::app,
	standard_c::c_str,
	user_interface::window::number_window::{NumberWindow, NumberWindowData},
};

#[no_mangle]
pub extern "C" fn main() {
	let number_window = NumberWindow::new(
		unsafe {
			&*(&[
				0x48_u8, 0x65, 0x6c, 0x6c, 0x6f, 0x44, 0x72, 0x6f, 0x6e, 0x61, 0x72, 0x6f, 0x69, 0x64,
				0x21, 0x00,
			] as *const [u8] as *const _ as *const c_str)
		},
		NumberWindowData {
			incremented: |_, _| (),
			decremented: |_, _| (),
			selected: |_, _| (),
			context: (),
		},
	)
	.debugless_unwrap();
	number_window.window().show(true);
	app::event_loop();
}
