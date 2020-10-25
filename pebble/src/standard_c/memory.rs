use core::{
	mem::{size_of, MaybeUninit},
	ptr::NonNull,
	slice,
};
use pebble_sys::{prelude::*, standard_c::memory as sys_memory};

/// # Errors
///
/// If the allocation fails.
pub fn malloc<'a, T>() -> Result<&'a mut MaybeUninit<T>, ()> {
	match size_of::<T>() {
		0 => Ok(unsafe { &mut *(NonNull::dangling().as_ptr()) }),
		size => match unsafe { sys_memory::malloc(size).cast_unchecked_mut() } {
			Some(uninit) => Ok(uninit),
			None => Err(()),
		},
	}
}

/// # Errors
///
/// If the allocation fails.
pub fn calloc<'a, T>(count: usize) -> Result<&'a mut [MaybeUninit<T>], ()> {
	match size_of::<T>() {
		0 => Ok(unsafe { slice::from_raw_parts_mut(NonNull::dangling().as_ptr(), count) }),
		size => match unsafe {
			sys_memory::calloc(count, size)
				.map(|mem| slice::from_raw_parts_mut(mem.cast_unchecked_mut(), count))
		} {
			Some(uninit) => Ok(uninit),
			None => Err(()),
		},
	}
}
