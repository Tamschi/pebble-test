#![no_std]
#![feature(extern_types)]

pub mod foundation {
	pub mod app {
		extern "C" {
			pub fn app_event_loop();
		}
	}
}

pub mod graphics {
	pub mod graphics_types {
		#[repr(C)]
		pub struct GColor8 {
			argb: u8,
		}
	}
}

pub mod user_interface {
	pub mod window {
		use crate::graphics::graphics_types::GColor8;
		use core::{ffi::c_void, ptr::NonNull};

		extern "C" {
			pub type Window;

			pub fn window_create() -> *mut Window;
			pub fn window_destroy(window: NonNull<Window>);
			pub fn window_is_loaded(window: NonNull<Window>) -> bool;

			pub fn window_set_user_data(window: NonNull<Window>, data: *mut c_void);
			pub fn window_get_user_data(window: NonNull<Window>) -> *mut c_void;

			pub fn window_set_background_color(window: NonNull<Window>, background_color: GColor8);
			pub fn window_set_window_handlers(window: NonNull<Window>, handlers: WindowHandlers);
		}

		#[repr(C)]
		pub struct WindowHandlers {
			pub load: extern "C" fn(NonNull<Window>),
			pub appear: extern "C" fn(NonNull<Window>),
			pub disappear: extern "C" fn(NonNull<Window>),
			pub unload: extern "C" fn(NonNull<Window>),
		}
	}

	pub mod window_stack {
		use super::window::Window;
		use core::ptr::NonNull;

		extern "C" {
			pub fn window_stack_push(window: NonNull<Window>, animated: bool);
			pub fn window_stack_pop(animated: bool) -> *mut Window;
			pub fn window_stack_pop_all(animated: bool);
			pub fn window_stack_remove(window: *mut Window, animated: bool) -> bool;
			pub fn window_stack_get_top_window() -> *mut Window;
			pub fn window_stack_contains_window(window: NonNull<Window>) -> bool;
		}
	}
}

pub mod standard_c {
	pub mod memory {
		use core::ffi::c_void;

		#[allow(non_camel_case_types)]
		type int = i32;

		extern "C" {
			pub fn malloc(size: usize) -> *mut c_void;
			pub fn calloc(count: usize, size: usize) -> *mut c_void;
			pub fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
			pub fn free(ptr: *mut c_void);
			pub fn memcmp(ptr1: *const c_void, ptr2: *const c_void, n: usize) -> int;
			pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
			pub fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
			pub fn memset(dest: *mut c_void, c: int, n: usize) -> *mut c_void;
		}
	}
}
