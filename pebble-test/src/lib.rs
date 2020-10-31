#![no_std]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_lossless)]

use debugless_unwrap::DebuglessUnwrap as _;
use pebble_skip::{
	foundation::{app, resources},
	graphics::graphics_types::Bitmap,
	user_interface::window::{Window, WindowHandlers},
};
use pebble_sys::user_interface::layers::{
	bitmap_layer::{
		bitmap_layer_create, bitmap_layer_destroy, bitmap_layer_get_layer_mut,
		bitmap_layer_set_bitmap, BitmapLayer,
	},
	layer_add_child,
};

#[no_mangle]
pub extern "C" fn main() -> i32 {
	let window = Window::new(WindowHandlers {
		load: || (),
		appear: |_| (),
		disappear: |_| (),
		unload: |_| (),
	})
	.debugless_unwrap();

	let r#unsafe = resources::get_handle(1);
	let r#unsafe = resources::load(r#unsafe).unwrap();
	let bitmap = Bitmap::from_png_data(&r#unsafe).unwrap();
	drop(r#unsafe);

	//FIXME
	let layer;
	unsafe {
		layer = bitmap_layer_create(bitmap.bounds()).unwrap() as *mut BitmapLayer;
		bitmap_layer_set_bitmap(&mut *layer, &*(bitmap.as_sys() as *const _));
		layer_add_child(
			&mut *window.root_layer().0,
			&mut *bitmap_layer_get_layer_mut(&mut *layer),
		);
	}

	window.show(true);
	app::event_loop();

	unsafe {
		drop(window);
		bitmap_layer_destroy(&mut *layer);
		drop(bitmap);
	}
	0
}
