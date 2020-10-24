#![no_std]
#![feature(test)]

use core::{hint::black_box, panic::PanicInfo, ptr::null_mut};

use debugless_unwrap::DebuglessUnwrap as _;
use pebble::{
	app,
	window::{Window, WindowHandlers},
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// Segfault.
	(unsafe { *black_box(null_mut::<extern "C" fn()>()) })();
	unreachable!()
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
