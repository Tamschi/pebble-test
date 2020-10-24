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
		use core::ffi::c_void;

		#[repr(C)] //TODO
		pub enum ButtonId {
			_A, //TODO
		}

		#[repr(transparent)]
		pub struct ClickRecognizerRef(*mut c_void);
		pub type ClickHandler = extern "C" fn(recognizer: ClickRecognizerRef, context: *mut c_void);
		pub type ClickConfigProvider = extern "C" fn(context: *mut c_void);
	}

	pub mod layers {
		use super::window::Window;
		use crate::graphics::graphics_types::{GContext, GPoint, GRect};
		use core::{ffi::c_void, ptr::NonNull};

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
			pub fn layer_get_data(layer: NonNull<Layer>) -> NonNull<c_void>;

		//TODO: #define GRect layer_get_unobstructed_bounds(const Layer* layer);
		}
	}

	pub mod window {
		use super::{
			clicks::{ButtonId, ClickConfigProvider, ClickHandler},
			layers::Layer,
		};
		use crate::graphics::graphics_types::GColor8;
		use core::{ffi::c_void, ptr::NonNull};

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
				context: *mut c_void,
			);
			pub fn window_get_click_config_provider(window: &Window)
				-> Option<ClickConfigProvider>;
			pub fn window_get_click_config_context(window: &Window) -> *mut c_void;
			pub fn window_set_window_handlers(window: &mut Window, handlers: WindowHandlers);

			// The watch is single-threaded and everything's on the heap, so this *should* be fine.
			pub fn window_get_root_layer(window: &Window) -> &mut Layer;

			pub fn window_set_background_color(window: &mut Window, background_color: GColor8);
			pub fn window_is_loaded(window: &Window) -> bool;
			pub fn window_set_user_data(window: &mut Window, data: *mut c_void);
			pub fn window_get_user_data(window: &Window) -> *mut c_void;
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
				context: Option<NonNull<c_void>>,
			);
			pub fn window_set_click_context(button_id: ButtonId, context: *mut c_void);
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
