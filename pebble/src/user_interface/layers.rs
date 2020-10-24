use core::{marker::PhantomData, ptr::NonNull};
use pebble_sys::user_interface::layers::{Layer as sysLayer, *};

pub struct Layer<T>(pub(crate) NonNull<sysLayer>, PhantomData<T>);
