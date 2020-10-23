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
    use std::ptr::NonNull;

    pub struct Window<'a, T>(pub(crate) NonNull<pebble_sys::Window>, PhantomData<&'a T>);

    pub struct WindowHandlers<L: FnMut() -> T, A: FnMut(&mut T), D: FnMut(&mut T), U: FnMut(T), T> {
        pub load: L,
        pub appear: A,
        pub disappear: D,
        pub unload: U,
    }

    trait WindowHandlersTrait<T> {
        fn load(&mut self) -> T;
        fn appear(&mut self, data: &mut T);
        fn disappear(&mut self, data: &mut T);
        fn unload(&mut self, data: T);
    }

    impl<L: FnMut() -> T, A: FnMut(&mut T), D: FnMut(&mut T), U: FnMut(T), T> WindowHandlersTrait<T>
        for WindowHandlers<L, A, D, U, T>
    {
        fn load(&mut self) -> T {
            (self.load)()
        }

        fn appear(&mut self, data: &mut T) {
            (self.appear)(data)
        }

        fn disappear(&mut self, data: &mut T) {
            (self.disappear)(data)
        }

        fn unload(&mut self, data: T) {
            (self.unload)(data)
        }
    }

    struct WindowData<'a, T> {
        user_data: Option<T>,
        window_handlers: Box<dyn 'a + WindowHandlersTrait<T>>,
    }

    impl<'a, T: 'a> Window<'a, T> {
        pub fn new<
            L: 'a + FnMut() -> T,
            A: 'a + FnMut(&mut T),
            D: 'a + FnMut(&mut T),
            U: 'a + FnMut(T),
        >(
            window_handlers: WindowHandlers<L, A, D, U, T>,
        ) -> Result<Self, ()> {
            let raw_window = NonNull::new(unsafe { window_create() }).ok_or(())?;
            let window_data = Box::new(WindowData {
                user_data: None,
                window_handlers: Box::new(window_handlers),
            });

            extern "C" fn raw_load<T>(raw_window: NonNull<pebble_sys::Window>) {
                let window_data = unsafe {
                    window_get_user_data(raw_window)
                        .cast::<WindowData<T>>()
                        .as_mut()
                }
                .unwrap();
                window_data.user_data = Some(window_data.window_handlers.load());
            }
            extern "C" fn raw_appear<T>(raw_window: NonNull<pebble_sys::Window>) {
                let window_data = unsafe {
                    window_get_user_data(raw_window)
                        .cast::<WindowData<T>>()
                        .as_mut()
                }
                .unwrap();
                window_data
                    .window_handlers
                    .appear(window_data.user_data.as_mut().unwrap());
            }
            extern "C" fn raw_disappear<T>(raw_window: NonNull<pebble_sys::Window>) {
                let window_data = unsafe {
                    window_get_user_data(raw_window)
                        .cast::<WindowData<T>>()
                        .as_mut()
                }
                .unwrap();
                window_data
                    .window_handlers
                    .disappear(window_data.user_data.as_mut().unwrap());
            }
            extern "C" fn raw_unload<T>(raw_window: NonNull<pebble_sys::Window>) {
                let window_data = unsafe {
                    window_get_user_data(raw_window)
                        .cast::<WindowData<T>>()
                        .as_mut()
                }
                .unwrap();
                window_data
                    .window_handlers
                    .unload(window_data.user_data.take().unwrap());
            }

            unsafe {
                //SAFETY: window_data is only retrieved and destroyed in the destructor, *after* destroying the window.
                window_set_user_data(
                    raw_window,
                    (Box::leak(window_data) as *mut WindowData<T>).cast(),
                );
                window_set_window_handlers(
                    raw_window,
                    pebble_sys::WindowHandlers {
                        load: raw_load::<T>,
                        appear: raw_appear::<T>,
                        disappear: raw_disappear::<T>,
                        unload: raw_unload::<T>,
                    },
                )
            }
            Ok(Self(raw_window, PhantomData))
        }

        pub unsafe fn from_raw(raw_window: *mut pebble_sys::Window) -> Result<Self, ()> {
            Ok(Self(NonNull::new(raw_window).ok_or(())?, PhantomData))
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
        unsafe { window_stack_remove(window.0.as_ptr(), animated) }
    }

    pub fn is_empty() -> bool {
        unsafe { window_stack_get_top_window() }.is_null()
    }

    pub fn contains_window<T>(window: &Window<T>) -> bool {
        unsafe { window_stack_contains_window(window.0) }
    }
}
