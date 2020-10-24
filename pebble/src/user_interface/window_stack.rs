use super::window::Window;
use pebble_sys::user_interface::window_stack::*;

pub fn push<T>(window: &mut Window<T>, animated: bool) {
	unsafe { window_stack_push(window.0, animated) }
}

pub fn pop(animated: bool) -> bool {
	!unsafe { window_stack_pop(animated) }.is_null()
}

pub fn pop_all(animated: bool) {
	unsafe { window_stack_pop_all(animated) }
}

pub fn remove<T>(window: &mut Window<T>, animated: bool) -> bool {
	unsafe { window_stack_remove(window.0, animated) }
}

pub fn is_empty() -> bool {
	unsafe { window_stack_get_top_window() }.is_null()
}

pub fn contains_window<T>(window: &mut Window<T>) -> bool {
	unsafe { window_stack_contains_window(window.0) }
}
