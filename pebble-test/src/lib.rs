#![no_std]
#![warn(clippy::pedantic)]

use core::panic::PanicInfo;
use debugless_unwrap::DebuglessUnwrap as _;
use pebble::{
	foundation::app,
	user_interface::window::{Window, WindowHandlers},
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

struct WindowState;

#[no_mangle]
pub extern "C" fn main() {
	let window = Window::new(WindowHandlers {
		// mutable closures
		load: || WindowState,
		appear: |_s| (),
		disappear: |_s| (),
		unload: |_s| (),
	})
	.debugless_unwrap();
	window.show(true);
	app::event_loop();
}
