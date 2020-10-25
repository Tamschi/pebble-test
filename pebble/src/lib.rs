#![no_std]
#![feature(coerce_unsized)]
#![feature(layout_for_ptr)]
#![feature(unsize)]

use core::{
	future::Future,
	intrinsics::drop_in_place,
	marker::Unsize,
	mem::{size_of, size_of_val_raw, ManuallyDrop},
	ops::{CoerceUnsized, Deref, DerefMut},
	pin::Pin,
	ptr::NonNull,
	task::{Context, Poll},
};

use pebble_sys::standard_c::memory::{free, malloc};

pub mod foundation;
pub mod user_interface;

/// Just a standard Box, more or less. The main difference is that its constructor is fallible instead of panicking.
///
/// It probably has fewer features than Rust's version, but it should be possible to add or emulate those.
struct Box<T: ?Sized>(NonNull<T>);

impl<T> Box<T> {
	pub fn new(value: T) -> Result<Self, T> {
		// No aligned_alloc 🙁
		match size_of::<T>() {
			0 => Ok(Self(NonNull::dangling())),
			size => match NonNull::new(unsafe { malloc(size).cast::<T>() }) {
				Some(ptr) => {
					unsafe { ptr.as_ptr().write(value) };
					Ok(Self(ptr))
				}
				None => Err(value),
			},
		}
	}

	pub fn into_inner(r#box: Self) -> T {
		unsafe { Box::into_raw(r#box).as_ptr().read() }
	}
}

impl<T: ?Sized> Drop for Box<T> {
	fn drop(&mut self) {
		unsafe {
			//SAFETY: ptr is always a valid pointer here that originally belonged to a sized type.
			let ptr = self.0.as_ptr();
			drop_in_place(ptr);
			match size_of_val_raw(ptr) {
				0 => (),
				_ => free(ptr.cast()),
			};
		}
	}
}

impl<T: ?Sized> Box<T> {
	pub fn leak<'a>(r#box: Self) -> &'a mut T
	where
		T: 'a,
	{
		let pointer = ManuallyDrop::new(r#box).0.as_ptr();
		unsafe { &mut *pointer }
	}

	pub unsafe fn from_raw(raw: NonNull<T>) -> Self {
		Self(raw)
	}

	pub fn into_raw(r#box: Self) -> NonNull<T> {
		Box::leak(r#box).into()
	}

	pub unsafe fn downcast_unchecked<U: Unsize<T>>(r#box: Self) -> Box<U> {
		Box::from_raw(Box::into_raw(r#box).cast())
	}
}

impl<T: ?Sized> Deref for Box<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe { self.0.as_ref() }
	}
}

impl<T: ?Sized> DerefMut for Box<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { self.0.as_mut() }
	}
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Box<U>> for Box<T> {}

impl<T: ?Sized> Unpin for Box<T> {}

impl<F: ?Sized + Future + Unpin> Future for Box<F> {
	type Output = F::Output;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		F::poll(Pin::new(&mut *self), cx)
	}
}

/// This is *sort of* like a Cell, but for constant handles. It should still allow surface-level aliasing.
///
/// Note that this is a reference wrapper and does not drop its target!
///
///TODO: Make sure this isn't Send.
struct Handle<T: ?Sized>(*mut T);

impl<T: ?Sized> Handle<T> {
	pub fn new(exclusive_handle: &'static mut T) -> Self {
		Self(exclusive_handle as *mut T)
	}

	pub fn unwrap(self) -> &'static mut T {
		unsafe { &mut *self.0 }
	}

	#[allow(clippy::mut_from_ref)]
	pub unsafe fn as_mut_unchecked(&self) -> &mut T {
		&mut *self.0
	}

	pub unsafe fn duplicate(&self) -> Self {
		Self(self.0)
	}
}

impl<T: ?Sized> Deref for Handle<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0 }
	}
}

impl<T: ?Sized> DerefMut for Handle<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *self.0 }
	}
}
