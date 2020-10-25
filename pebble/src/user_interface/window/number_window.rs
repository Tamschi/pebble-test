use crate::{Box, Handle};
use core::{
	marker::PhantomData,
	mem::{transmute_copy, ManuallyDrop},
	ops::{Deref, DerefMut},
};
#[allow(clippy::wildcard_imports)]
use pebble_sys::{
	prelude::*,
	standard_c::memory::{c_str, void},
	user_interface::window::number_window::{NumberWindow as sysNumberWindow, *},
};

use super::{WindowRef, WindowRefMut};

pub struct NumberWindow<'a, T: ?Sized>(pub(crate) Handle<'a, sysNumberWindow<'a>>, PhantomData<T>);

pub struct NumberWindowData<
	I: FnMut(&NumberWindow<void>, &mut T),
	D: FnMut(&NumberWindow<void>, &mut T),
	S: FnMut(&NumberWindow<void>, &mut T),
	T,
> {
	pub incremented: I,
	pub decremented: D,
	pub selected: S,
	pub context: T,
}

trait NumberWindowDataTrait {
	fn incremented(&mut self, number_window: &NumberWindow<void>);
	fn decremented(&mut self, number_window: &NumberWindow<void>);
	fn selected(&mut self, number_window: &NumberWindow<void>);
}

impl<
		I: FnMut(&NumberWindow<void>, &mut T),
		D: FnMut(&NumberWindow<void>, &mut T),
		S: FnMut(&NumberWindow<void>, &mut T),
		T,
	> NumberWindowDataTrait for NumberWindowData<I, D, S, T>
{
	fn incremented(&mut self, number_window: &NumberWindow<void>) {
		(self.incremented)(number_window, &mut self.context)
	}

	fn decremented(&mut self, number_window: &NumberWindow<void>) {
		(self.decremented)(number_window, &mut self.context)
	}

	fn selected(&mut self, number_window: &NumberWindow<void>) {
		(self.selected)(number_window, &mut self.context)
	}
}

struct NumberWindowDataWrapper<'a>(Box<'a, dyn 'a + NumberWindowDataTrait>);

impl<'a, T> NumberWindow<'a, T> {
	/// # Errors
	///
	/// TODO
	///
	pub fn new<
		I: 'a + FnMut(&NumberWindow<void>, &mut T),
		D: 'a + FnMut(&NumberWindow<void>, &mut T),
		S: 'a + FnMut(&NumberWindow<void>, &mut T),
	>(
		label: &'a c_str,
		number_window_data: NumberWindowData<I, D, S, T>,
	) -> Result<Self, NumberWindowData<I, D, S, T>>
	where
		T: 'a,
	{
		#![allow(clippy::items_after_statements)]

		let window_data = Box::leak(Box::new(number_window_data)?).upcast_mut() as *mut void;

		extern "C" fn raw_incremented<'a, T>(
			raw_window: &'a mut sysNumberWindow<'a>,
			context: &mut void,
		) {
			let fake_window = unsafe {
				//SAFETY: It's actually *kind of* safe to alias NumberWindow instances... But only because they store a Handle internally, which stores a pointer.
				// Actually accessing associated data would NOT be safe, so the user-provided handlers only see a NumberWindow<void> where such access is impossible.
				NumberWindow::<T>::from_raw(raw_window)
			};
			unsafe {
				context
					.cast_unchecked_mut::<NumberWindowDataWrapper>()
					.0
					.incremented(&fake_window)
			}
			fake_window.abandon();
		}
		extern "C" fn raw_decremented<'a, T>(
			raw_window: &'a mut sysNumberWindow<'a>,
			context: &mut void,
		) {
			let fake_window = unsafe {
				//SAFETY: It's actually *kind of* safe to alias NumberWindow instances... But only because they store a Handle internally, which stores a pointer.
				// Actually accessing associated data would NOT be safe, so the user-provided handlers only see a NumberWindow<void> where such access is impossible.
				NumberWindow::<T>::from_raw(raw_window)
			};
			unsafe {
				context
					.cast_unchecked_mut::<NumberWindowDataWrapper>()
					.0
					.decremented(&fake_window)
			}
			fake_window.abandon();
		}
		extern "C" fn raw_selected<'a, T>(
			raw_window: &'a mut sysNumberWindow<'a>,
			context: &mut void,
		) {
			let fake_window = unsafe {
				//SAFETY: It's actually *kind of* safe to alias NumberWindow instances... But only because they store a Handle internally, which stores a pointer.
				// Actually accessing associated data would NOT be safe, so the user-provided handlers only see a NumberWindow<void> where such access is impossible.
				NumberWindow::<T>::from_raw(raw_window)
			};
			unsafe {
				context
					.cast_unchecked_mut::<NumberWindowDataWrapper>()
					.0
					.selected(&fake_window)
			}
			fake_window.abandon();
		}

		match unsafe {
			number_window_create(
				label,
				NumberWindowCallbacks {
					incremented: Some(raw_incremented::<T>),
					decremented: Some(raw_decremented::<T>),
					selected: Some(raw_selected::<T>),
				},
				&mut *window_data,
			)
		} {
			Some(raw_window) => Ok(Self(Handle::new(raw_window), PhantomData)),
			None => Err(Box::into_inner(unsafe {
				Box::from_raw((&mut *window_data).cast_unchecked_mut())
			})),
		}
	}

	/// Assembles a new instance of [`NumberWindow`] from the given raw window handle.
	///
	/// # Safety
	///
	/// This function is only safe if `raw_window` is a raw window handle that was previously [`.leak()`]ed from the same [`NumberWindow`] variant and no other [`Window<T>`] instance has been created from it since.
	///
	/// [`.leak()`]: #method.leak
	pub unsafe fn from_raw(raw_window: &'a mut sysNumberWindow<'a>) -> Self {
		Self(Handle::new(raw_window), PhantomData)
	}

	/// Leaks the current [`NumberWindow`] instance into a raw Pebble number window handle.
	///
	/// Note that [`NumberWindow`] has associated heap instances beyond the raw window, so only destroying that would still leak memory.
	#[must_use = "Not reassembling the `NumberWindow` later causes a memory leak."]
	pub fn leak(self) -> &'a mut sysNumberWindow<'a>
	where
		T: 'a,
	{
		unsafe { ManuallyDrop::new(self).0.duplicate().unwrap() }
	}

	/// Discards this instance while skipping the destructor. Helper for aliased temporaries.
	fn abandon(self) {
		let _ = ManuallyDrop::new(self);
	}
}

impl<'a, T: ?Sized> NumberWindow<'a, T> {
	#[must_use]
	pub fn window<'b: 'a>(&'b self) -> WindowRef<'b> {
		WindowRef(Handle::new(unsafe {
			number_window_get_window_mut(self.0.as_mut_unchecked())
		}))
	}

	#[must_use]
	pub fn window_mut<'b: 'a>(&'b mut self) -> WindowRefMut<'b> {
		WindowRefMut(Handle::new(unsafe {
			number_window_get_window_mut(self.0.as_mut_unchecked())
		}))
	}
}

impl<'a, T> Deref for NumberWindow<'a, T> {
	type Target = NumberWindow<'a, void>;

	fn deref(&self) -> &Self::Target {
		unsafe {
			//SAFETY: Same memory layout, no access to data.
			transmute_copy(self)
		}
	}
}

impl<'a, T> DerefMut for NumberWindow<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {
			//SAFETY: Same memory layout, no access to data.
			transmute_copy(self)
		}
	}
}
