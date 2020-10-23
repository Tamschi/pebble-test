#![no_std]

use core::{ffi::c_void, ptr::NonNull};

// Opaque handles.
pub enum Window {}

#[repr(u8)]
pub enum GColor8 {
    Black = 0b_11_00_00_00_u8,
    OxfordBlue = 0b_11_00_00_01_u8,
    DukeBlue = 0b_11_00_00_10_u8,
    Blue = 0b_11_00_00_11_u8,
    DarkGreen = 0b_11_00_01_00_u8,
    //TODO: https://developer.rebble.io/developer.pebble.com/docs/c/Graphics/Graphics_Types/Color_Definitions/index.html
}

#[repr(C)]
pub struct WindowHandlers {
    pub load: unsafe extern "C" fn(NonNull<Window>),
    pub appear: unsafe extern "C" fn(NonNull<Window>),
    pub disappear: unsafe extern "C" fn(NonNull<Window>),
    pub unload: unsafe extern "C" fn(NonNull<Window>),
}

extern "C" {
    pub fn app_event_loop();

    pub fn window_create() -> *mut Window;
    pub fn window_destroy(window: NonNull<Window>);
    pub fn window_is_loaded(window: NonNull<Window>) -> bool;

    pub fn window_set_user_data(window: NonNull<Window>, data: *mut c_void);
    pub fn window_get_user_data(window: NonNull<Window>) -> *mut c_void;

    pub fn window_set_background_color(window: NonNull<Window>, background_color: GColor8);
    pub fn window_set_window_handlers(window: NonNull<Window>, handlers: WindowHandlers);

    pub fn window_stack_push(window: NonNull<Window>, animated: bool);
    pub fn window_stack_pop(animated: bool) -> *mut Window;
    pub fn window_stack_pop_all(animated: bool);
    pub fn window_stack_remove(window: *mut Window, animated: bool) -> bool;
    pub fn window_stack_get_top_window() -> *mut Window;
    pub fn window_stack_contains_window(window: NonNull<Window>) -> bool;
}
