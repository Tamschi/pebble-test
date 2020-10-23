#![no_std]

use core::panic::PanicInfo;
use pebble::app;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() {
    app::event_loop();
}
