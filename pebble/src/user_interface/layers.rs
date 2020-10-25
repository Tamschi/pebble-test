use crate::Handle;
use core::marker::PhantomData;
use pebble_sys::user_interface::layers::{Layer as sysLayer, *};

pub struct Layer<T>(pub(crate) Handle<sysLayer>, PhantomData<T>);
