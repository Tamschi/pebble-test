#![no_std]
#![feature(extern_types)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)] // Matching the SDK documentation.

pub mod prelude {
	pub use super::standard_c::prelude::*;
}

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
		pub struct GPoint {
			pub x: i16,
			pub y: i16,
		}

		#[repr(C)]
		pub struct GRect {
			pub origin: GPoint,
			pub size: GSize,
		}

		#[repr(C)]
		pub struct GSize {
			pub w: i16,
			pub h: i16,
		}

		#[repr(C)]
		pub union GColor8 {
			argb: u8,
		}

		pub type GColor = GColor8;

		extern "C" {
			pub type GBitmap;
			pub type GBitmapSequence;
			pub type GContext;
		}
	}
}

pub mod user_interface {
	pub mod clicks {
		use crate::standard_c::memory::void;

		#[repr(C)] //TODO
		pub enum ButtonId {
			_A, //TODO
		}

		#[repr(transparent)]
		pub struct ClickRecognizerRef(*mut void);
		pub type ClickHandler = extern "C" fn(recognizer: ClickRecognizerRef, context: *mut void);
		pub type ClickConfigProvider = extern "C" fn(context: *mut void);
	}

	pub mod layers {
		use super::window::Window;
		use crate::{
			graphics::graphics_types::{GContext, GPoint, GRect},
			standard_c::memory::void,
		};
		use core::ptr::NonNull;

		pub type LayerUpdateProc = extern "C" fn(layer: NonNull<Layer>, NonNull<GContext>);

		extern "C" {
			pub type Layer;

			pub fn layer_create(frame: GRect) -> *mut Layer;
			pub fn layer_create_with_data(frame: GRect, data_size: usize) -> *mut Layer;
			pub fn layer_destroy(layer: NonNull<Layer>);
			pub fn layer_mark_dirty(layer: NonNull<Layer>);
			pub fn layer_set_update_proc(
				layer: NonNull<Layer>,
				update_proc: Option<LayerUpdateProc>, //TODO: Check if this is legal!
			);
			pub fn layer_set_frame(layer: NonNull<Layer>, frame: GRect);
			pub fn layer_get_frame(layer: NonNull<Layer>) -> GRect;
			pub fn layer_set_bounds(layer: NonNull<Layer>, bounds: GRect);
			pub fn layer_get_bounds(layer: NonNull<Layer>) -> GRect;
			pub fn layer_convert_point_to_screen(layer: NonNull<Layer>, point: GPoint) -> GPoint;
			pub fn layer_convert_rect_to_screen(layer: NonNull<Layer>, rect: GRect) -> GRect;
			pub fn layer_get_window(layer: NonNull<Layer>) -> *mut Window;
			pub fn layer_remove_from_parent(child: NonNull<Layer>);
			pub fn layer_remove_child_layers(parent: NonNull<Layer>);
			pub fn layer_add_child(parent: NonNull<Layer>, child: NonNull<Layer>);
			pub fn layer_insert_below_sibling(
				layer_to_insert: NonNull<Layer>,
				below_sibling_layer: NonNull<Layer>,
			);
			pub fn layer_insert_above_sibling(
				layer_to_insert: NonNull<Layer>,
				above_sibling_layer: NonNull<Layer>,
			);
			pub fn layer_set_hidden(layer: NonNull<Layer>, hidden: bool);
			pub fn layer_get_hidden(layer: NonNull<Layer>) -> bool;
			pub fn layer_set_clips(layer: NonNull<Layer>, clips: bool);
			pub fn layer_get_clips(layer: NonNull<Layer>) -> bool;
			pub fn layer_get_data(layer: NonNull<Layer>) -> NonNull<void>;

		//TODO: #define GRect layer_get_unobstructed_bounds(const Layer* layer);
		}
	}

	pub mod window {
		use super::{
			clicks::{ButtonId, ClickConfigProvider, ClickHandler},
			layers::Layer,
		};
		use crate::{graphics::graphics_types::GColor8, standard_c::memory::void};
		use core::ptr::NonNull;

		#[repr(C)]
		pub struct WindowHandlers {
			pub load: Option<WindowHandler>,
			pub appear: Option<WindowHandler>,
			pub disappear: Option<WindowHandler>,
			pub unload: Option<WindowHandler>,
		}

		pub type WindowHandler = extern "C" fn(window: &mut Window);

		extern "C" {
			pub type Window;

			pub fn window_create() -> Option<&'static mut Window>;
			pub fn window_destroy(window: &'static mut Window);
			pub fn window_set_click_config_provider(
				window: &mut Window,
				click_config_provider: Option<ClickConfigProvider>,
			);
			pub fn window_set_click_config_provider_with_context(
				window: &mut Window,
				click_config_provider: Option<ClickConfigProvider>,
				context: *mut void,
			);
			pub fn window_get_click_config_provider(window: &Window)
				-> Option<ClickConfigProvider>;
			pub fn window_get_click_config_context(window: &Window) -> *mut void;
			pub fn window_set_window_handlers(window: &mut Window, handlers: WindowHandlers);

			// The watch is single-threaded and everything's on the heap, so this *should* be fine.
			pub fn window_get_root_layer(window: &Window) -> &mut Layer;

			pub fn window_set_background_color(window: &mut Window, background_color: GColor8);
			pub fn window_is_loaded(window: &Window) -> bool;
			pub fn window_set_user_data(window: &mut Window, data: *mut void);
			pub fn window_get_user_data(window: &Window) -> *mut void;
			pub fn window_single_click_subscribe(button_id: ButtonId, handler: ClickHandler);
			pub fn window_single_repeating_click_subscribe(
				button_id: ButtonId,
				repeat_interval_ms: u16,
				handler: ClickHandler,
			);
			pub fn window_multi_click_subscribe(
				button_id: ButtonId,
				min_clicks: u8,
				max_clicks: u8,
				timeout: u16,
				last_click_only: bool,
				handler: ClickHandler,
			);
			pub fn window_long_click_subscribe(
				button_id: ButtonId,
				delay_ms: u16,
				down_handler: ClickHandler,
				up_handler: ClickHandler,
			);
			pub fn window_raw_click_subscribe(
				button_id: ButtonId,
				down_handler: ClickHandler,
				up_handler: ClickHandler,
				context: Option<NonNull<void>>,
			);
			pub fn window_set_click_context(button_id: ButtonId, context: *mut void);
		}
	}

	pub mod window_stack {
		use super::window::Window;

		extern "C" {
			pub fn window_stack_push(window: &mut Window, animated: bool);
			pub fn window_stack_pop(animated: bool) -> *mut Window;
			pub fn window_stack_pop_all(animated: bool);
			pub fn window_stack_remove(window: &mut Window, animated: bool) -> bool;
			pub fn window_stack_get_top_window() -> *mut Window;
			pub fn window_stack_contains_window(window: &mut Window) -> bool;
		}
	}
}

pub mod standard_c {
	pub mod prelude {
		pub use super::memory::prelude::*;
	}

	pub mod memory {
		pub mod prelude {
			pub use super::{OptionToVoidExt, OptionVoidExt};
		}

		#[allow(non_camel_case_types)]
		type int = i32;

		extern "C" {
			/// `void` can be safely passed back across the FFI as `&void` while [`core::ffi::cvoid`] cannot.
			/// ([`c_void`] is NOT [unsized]!)
			///
			/// [`core::ffi::c_void`]: https://doc.rust-lang.org/stable/core/ffi/enum.c_void.html
			/// [`c_void`]: https://doc.rust-lang.org/stable/core/ffi/enum.c_void.html
			/// [unsized]: https://doc.rust-lang.org/stable/core/marker/trait.Sized.html
			pub type void;

			pub fn malloc(size: usize) -> Option<&'static mut void>;
			pub fn calloc(count: usize, size: usize) -> Option<&'static mut void>;
			pub fn realloc(ptr: *mut void, size: usize) -> Option<&'static mut void>;
			pub fn free(ptr: &'static mut void);
			pub fn memcmp(ptr1: &void, ptr2: &void, n: usize) -> int;
			pub fn memcpy(dest: &mut void, src: &void, n: usize) -> *mut void;
			pub fn memmove(dest: *mut void, src: *const void, n: usize) -> *mut void;
			pub fn memset(dest: &mut void, c: int, n: usize) -> *mut void;
		}

		impl<'a, T> From<&'a mut T> for &'a mut void {
			fn from(src: &'a mut T) -> Self {
				unsafe { &mut *(src as *mut _ as *mut void) }
			}
		}

		impl<'a, T> From<&'a T> for &'a void {
			fn from(src: &'a T) -> Self {
				unsafe { &*(src as *const _ as *const void) }
			}
		}

		pub trait OptionVoidExt<'a> {
			/// Casts a mutable untyped heap reference ([`&mut void]) into a typed one.
			///
			/// # Safety
			///
			/// Horribly unsafe if T doesn't point to an **initialised** instance of T.
			unsafe fn cast_unchecked<T>(self) -> Option<&'a mut T>;
		}

		pub trait OptionToVoidExt<'a> {
			fn upcast(self) -> Option<&'a mut void>;
		}

		impl<'a> OptionVoidExt<'a> for Option<&'a mut void> {
			unsafe fn cast_unchecked<T>(self) -> Option<&'a mut T> {
				self.map(|void_ref| &mut *(void_ref as *mut void).cast())
			}
		}

		impl<'a, T> OptionToVoidExt<'a> for Option<&'a mut T> {
			fn upcast(self) -> Option<&'a mut void> {
				self.map(|t_ref| t_ref.into())
			}
		}
	}
}
