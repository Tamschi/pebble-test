use pebble_sys::*;

pub fn event_loop() {
	unsafe { app_event_loop() }
}
