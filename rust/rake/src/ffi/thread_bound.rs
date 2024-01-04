use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};
use std::thread::{self, ThreadId};

/// ThreadBound is a Sync-maker and Send-maker that allows accessing a value
/// of type T only from the original thread on which the ThreadBound was
/// constructed.
///
/// Refer to the [crate-level documentation] for a usage example.
///
/// [crate-level documentation]: index.html
pub struct ThreadBound<T> {
    thread_id: ThreadId,
    value: T,
}

unsafe impl<T> Sync for ThreadBound<T> {}

// Send bound requires Copy, as otherwise Drop could run in the wrong place.
//
// Today Copy and Drop are mutually exclusive so `T: Copy` implies `T: !Drop`.
// This impl needs to be revisited if that restriction is relaxed in the future.
unsafe impl<T: Copy> Send for ThreadBound<T> {}

impl<T> ThreadBound<T> {
    /// Binds a value to the current thread. The wrapper can be sent around to
    /// other threads, but no other threads will be able to access the
    /// underlying value.
    pub fn new(value: T) -> Self {
        ThreadBound {
            thread_id: thread::current().id(),
            value,
        }
    }

    /// Accesses a reference to the underlying value if this is its original
    /// thread, otherwise `None`.
    pub fn get_ref(&self) -> Option<&T> {
        if thread::current().id() == self.thread_id {
            Some(&self.value)
        } else {
            None
        }
    }

    /// Accesses a mutable reference to the underlying value if this is its
    /// original thread, otherwise `None`.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if thread::current().id() == self.thread_id {
            Some(&mut self.value)
        } else {
            None
        }
    }

    /// Extracts ownership of the underlying value if this is its original
    /// thread, otherwise `None`.
    pub fn into_inner(self) -> Option<T> {
        if thread::current().id() == self.thread_id {
            Some(self.value)
        } else {
            None
        }
    }
}

impl<T> Deref for ThreadBound<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if thread::current().id() == self.thread_id {
            &self.value
        } else {
            panic!("Value was dereferenced outside of its parent thread")
        }
    }
}

impl<T> DerefMut for ThreadBound<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if thread::current().id() == self.thread_id {
            &mut self.value
        } else {
            panic!("Value was dereferenced outside of its parent thread")
        }
    }
}

impl<T> Default for ThreadBound<T>
where
    T: Default,
{
    fn default() -> Self {
        ThreadBound::new(Default::default())
    }
}

impl<T> Debug for ThreadBound<T>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.get_ref() {
            Some(value) => Debug::fmt(value, formatter),
            None => formatter.write_str("unknown"),
        }
    }
}
