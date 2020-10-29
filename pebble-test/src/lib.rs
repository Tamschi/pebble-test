#![no_std]
#![warn(clippy::pedantic)]

use pebble_sys::{
	foundation::app::app_event_loop,
	standard_c::memory::{c_str, void},
	user_interface::{
		window::number_window::{
			number_window_create, number_window_get_window_mut, number_window_set_value,
			NumberWindowCallbacks,
		},
		window_stack::window_stack_push,
	},
};

#[no_mangle]
pub extern "C" fn main() -> i32 {
	static mut CONTEXT: () = ();

	unsafe {
		let label = &*("miles to see you\0" as *const _ as *const c_str);
		let number_window = number_window_create(
			label,
			NumberWindowCallbacks {
				incremented: None,
				decremented: None,
				selected: None,
			},
			&mut *(&mut CONTEXT as *mut _ as *mut void),
		)
		.unwrap();
		number_window_set_value(number_window, 10_000);
		let window = number_window_get_window_mut(number_window);
		window_stack_push(window, true);
		app_event_loop();
		0
	}
}
