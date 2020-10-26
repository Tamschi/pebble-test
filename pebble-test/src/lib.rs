#![no_std]
#![warn(clippy::pedantic)]

use pebble::foundation::app;
use pebble_sys::{
	graphics::graphics_types::GColor8,
	user_interface::{
		window::{window_create, window_destroy},
		window_stack::window_stack_push,
	},
};

#[no_mangle]
pub extern "C" fn main() {
	unsafe {
		// let window = window_create().unwrap();
		// window_stack_push(window, true);
		// pebble_sys::user_interface::window::window_set_background_color(
		// 	window,
		// 	GColor8 {
		// 		argb: 0b_11_00_00_00,
		// 	},
		// );
		app::event_loop();
		// window_destroy(window);
	}
}
