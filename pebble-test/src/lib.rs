#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;
use pebble::app;

#[no_mangle]
extern "C" fn __exidx_end() {
    todo!()
}

#[no_mangle]
extern "C" fn __exidx_start() {
    todo!()
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() {
    app::event_loop();
}
