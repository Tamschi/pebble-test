use super::window_stack;
use crate::{Box, Handle};
use core::{marker::PhantomData, mem::ManuallyDrop};
use pebble_sys::{
	standard_c::memory::void,
	user_interface::window::{Window as sysWindow, WindowHandlers as sysWindowHandlers, *},
};

pub struct Window<T>(pub(crate) Handle<sysWindow>, PhantomData<T>);

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
	window_handlers: Box<'a, dyn 'a + WindowHandlersTrait<T>>,
}

pub struct WindowCreationError<L: FnMut() -> T, A: FnMut(&mut T), D: FnMut(&mut T), U: FnMut(T), T>
{
	pub window_handlers: WindowHandlers<L, A, D, U, T>,
}

impl<'a, T: 'a> Window<T> {
	pub fn new<
		L: 'a + FnMut() -> T,
		A: 'a + FnMut(&mut T),
		D: 'a + FnMut(&mut T),
		U: 'a + FnMut(T),
	>(
		window_handlers: WindowHandlers<L, A, D, U, T>,
	) -> Result<Self, WindowCreationError<L, A, D, U, T>> {
		let window_data = Box::new(WindowData {
			user_data: None,
			window_handlers: Box::new(window_handlers)
				.map_err(|window_handlers| WindowCreationError { window_handlers })?,
		})
		.map_err(|window_data| WindowCreationError::<_, _, _, _, T> {
			window_handlers: Box::into_inner(unsafe {
				Box::downcast_unchecked(window_data.window_handlers)
			}),
		})?;
		let raw_window = match unsafe { window_create() } {
			Some(raw_window) => raw_window,
			None => {
				return Err(WindowCreationError {
					window_handlers: Box::into_inner(unsafe {
						Box::downcast_unchecked(Box::into_inner(window_data).window_handlers)
					}),
				});
			}
		};

		extern "C" fn raw_load<T>(raw_window: &mut sysWindow) {
			let window_data = unsafe {
				window_get_user_data(raw_window)
					.cast::<WindowData<T>>()
					.as_mut()
			}
			.unwrap();
			window_data.user_data = Some(window_data.window_handlers.load());
		}
		extern "C" fn raw_appear<T>(raw_window: &mut sysWindow) {
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
		extern "C" fn raw_disappear<T>(raw_window: &mut sysWindow) {
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
		extern "C" fn raw_unload<T>(raw_window: &mut sysWindow) {
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
			window_set_user_data(raw_window, {
				let mem: &mut void = Box::leak(window_data).into();
				mem
			});
			window_set_window_handlers(
				raw_window,
				sysWindowHandlers {
					load: Some(raw_load::<T>),
					appear: Some(raw_appear::<T>),
					disappear: Some(raw_disappear::<T>),
					unload: Some(raw_unload::<T>),
				},
			)
		}
		Ok(Self(Handle::new(raw_window), PhantomData))
	}

	/// Assembles a new instance of `Window<T>` from the given raw window handle.
	///
	/// # Safety
	///
	/// This function is only safe if `raw_window` is a raw window handle that was previously [`.leak()`]ed from the same `Window<T>` variant and no other `Window<T>` instance has been created from it since.
	///
	/// [`null_mut()`]: https://doc.rust-lang.org/stable/std/ptr/fn.null_mut.html
	/// [`.leak()`]: #method.leak
	pub unsafe fn from_raw(raw_window: &'static mut sysWindow) -> Self {
		Self(Handle::new(raw_window), PhantomData)
	}

	pub fn is_loaded(&self) -> bool {
		unsafe { window_is_loaded(&*self.0) }
	}

	pub fn show(&self, animated: bool) {
		window_stack::push(self, animated)
	}

	pub fn hide(&self, animated: bool) -> bool {
		window_stack::remove(self, animated)
	}

	pub fn leak(self) -> &'static mut sysWindow
	where
		T: 'static,
	{
		unsafe { ManuallyDrop::new(self).0.duplicate().unwrap() }
	}
}

impl<T> Drop for Window<T> {
	fn drop(&mut self) {
		unsafe {
			//SAFETY: window_data is created and leaked in the only accessible constructor.
			//SAFETY: self.0 isn't accessed after this.
			let window_data = window_get_user_data(&*self.0).cast();
			window_destroy(self.0.duplicate().unwrap());
			Box::<WindowData<T>>::from_raw(&mut *window_data);
		}
	}
}
