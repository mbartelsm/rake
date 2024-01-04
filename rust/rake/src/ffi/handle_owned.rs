use std::{
    ops::{Deref, DerefMut},
    panic::{RefUnwindSafe, UnwindSafe},
};

use super::thread_bound::ThreadBound;

#[repr(transparent)]
pub struct HandleOwned<T>(*mut ThreadBound<T>);

impl<T> HandleOwned<T>
where
    T: Send + 'static,
{
    fn alloc(value: T) -> Self
    where
        T: Send + 'static,
    {
        let boxed = Box::new(ThreadBound::new(value));
        HandleOwned(Box::into_raw(boxed))
    }
}

impl<T> HandleOwned<T>
where
    T: Send,
{
    unsafe fn dealloc<R>(handle: Self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut boxed = Box::from_raw(handle.0);

        f(&mut **boxed)
    }
}

unsafe impl<T> Send for HandleOwned<T> where T: Send {}
impl<T> UnwindSafe for HandleOwned<T> where T: RefUnwindSafe {}

impl<T> Deref for HandleOwned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T> DerefMut for HandleOwned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}
