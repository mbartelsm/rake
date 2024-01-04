use std::ops::Deref;

#[repr(transparent)]
pub struct HandleShared<T: ?Sized>(*const T);

impl<T> HandleShared<T>
where
    T: Send + 'static,
{
    fn alloc(value: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        let boxed = Box::new(value);
        HandleShared(Box::into_raw(boxed))
    }
}

impl<T> HandleShared<T>
where
    T: ?Sized + Send + Sync,
{
    unsafe fn dealloc<R>(handle: Self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut boxed = Box::from_raw(handle.0 as *mut T);

        f(&mut *boxed)
    }
}

unsafe impl<T> Send for HandleShared<T> where T: ?Sized + Sync {}

impl<T> Deref for HandleShared<T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
