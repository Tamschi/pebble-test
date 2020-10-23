#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn app_event_loop();
}

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        app_event_loop();
    }
}
