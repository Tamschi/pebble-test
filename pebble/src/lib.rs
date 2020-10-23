#![feature(restricted_std)]

pub mod app {
    use pebble_sys::*;

    pub fn event_loop() {
        unsafe { app_event_loop() }
    }
}

pub mod window {
    use core::marker::PhantomData;
    use pebble_sys::*;

    pub struct Window<'a, T>(pub(crate) *mut pebble_sys::Window, PhantomData<&'a T>);
    struct WindowData<'a, T> {
        user_data: Option<T>,
        load: Box<dyn 'a + FnMut() -> T>,
        appear: Box<dyn 'a + FnMut(&mut T)>,
        disappear: Box<dyn 'a + FnMut(&mut T)>,
        unload: Box<dyn 'a + FnMut(T)>,
    }

    impl<'a, T> Window<'a, T> {
        pub fn new(
            load: impl 'a + FnMut() -> T,
            appear: impl 'a + FnMut(&mut T),
            disappear: impl 'a + FnMut(&mut T),
            unload: impl 'a + FnMut(T),
        ) -> Self {
            let raw_window = unsafe { window_create() };
            let window_data = Box::new(WindowData {
                user_data: None,
                load: Box::new(load),
                appear: Box::new(appear),
                disappear: Box::new(disappear),
                unload: Box::new(unload),
            });

            unsafe extern "C" fn raw_load<T>(raw_window: *mut pebble_sys::Window) {
                let window_data = window_get_user_data(raw_window)
                    .cast::<WindowData<T>>()
                    .as_mut()
                    .unwrap();
                window_data.user_data = Some((window_data.load)());
            }
            unsafe extern "C" fn raw_appear<T>(raw_window: *mut pebble_sys::Window) {
                let window_data = window_get_user_data(raw_window)
                    .cast::<WindowData<T>>()
                    .as_mut()
                    .unwrap();
                (window_data.appear)(window_data.user_data.as_mut().unwrap());
            }
            unsafe extern "C" fn raw_disappear<T>(raw_window: *mut pebble_sys::Window) {
                let window_data = window_get_user_data(raw_window)
                    .cast::<WindowData<T>>()
                    .as_mut()
                    .unwrap();
                (window_data.disappear)(window_data.user_data.as_mut().unwrap());
            }
            unsafe extern "C" fn raw_unload<T>(raw_window: *mut pebble_sys::Window) {
                let window_data = window_get_user_data(raw_window)
                    .cast::<WindowData<T>>()
                    .as_mut()
                    .unwrap();
                (window_data.unload)(window_data.user_data.take().unwrap());
            }

            unsafe {
                //SAFETY: window_data is only retrieved and destroyed in the destructor, *after* destroying the window.
                window_set_user_data(
                    raw_window,
                    (Box::leak(window_data) as *mut WindowData<T>).cast(),
                );
                window_set_window_handlers(
                    raw_window,
                    WindowHandlers {
                        load: raw_load::<T>,
                        appear: raw_appear::<T>,
                        disappear: raw_disappear::<T>,
                        unload: raw_unload::<T>,
                    },
                )
            }
            Self(raw_window, PhantomData)
        }

        pub unsafe fn from_raw(raw_window: *mut pebble_sys::Window) -> Self {
            Self(raw_window, PhantomData)
        }

        pub fn is_loaded(&self) -> bool {
            unsafe { window_is_loaded(self.0) }
        }

        pub fn show(&self, animated: bool) {
            crate::window_stack::push(self, animated)
        }

        pub fn hide(&self, animated: bool) -> bool {
            crate::window_stack::remove(self, animated)
        }
    }

    impl<'a, T> Drop for Window<'a, T> {
        fn drop(&mut self) {
            unsafe {
                //SAFETY: window_data is created and leaked in the only accessible constructor.
                let window_data = window_get_user_data(self.0);
                window_destroy(self.0);
                Box::<WindowData<T>>::from_raw(window_data.cast());
            }
        }
    }
}

pub mod window_stack {
    use crate::window::Window;
    use pebble_sys::*;

    pub fn push<T>(window: &Window<T>, animated: bool) {
        unsafe { window_stack_push(window.0, animated) }
    }

    pub fn pop(animated: bool) -> bool {
        !unsafe { window_stack_pop(animated) }.is_null()
    }

    pub fn pop_all(animated: bool) {
        unsafe { window_stack_pop_all(animated) }
    }

    pub fn remove<T>(window: &Window<T>, animated: bool) -> bool {
        unsafe { window_stack_remove(window.0, animated) }
    }

    pub fn is_empty() -> bool {
        unsafe { window_stack_get_top_window() }.is_null()
    }

    pub fn contains_window<T>(window: &Window<T>) -> bool {
        unsafe { window_stack_contains_window(window.0) }
    }
}
