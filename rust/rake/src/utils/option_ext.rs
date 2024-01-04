pub trait OptionExt<T> {

    /// If `Some`, mutates the contents of the option in-place.
    /// If `None`, it does nothing.
    fn map_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut T);

    /// If `None`, it mutates the option to the value produced by the given
    /// function. If `Some`, it leaves the value as is.
    fn or_else_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce() -> Option<T>;

    /// Does nothing if the value is `None`. If it's `Some` it mutates the value
    /// to the result of the given function.
    fn and_then_mut<D, F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(T) -> Option<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn map_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut T),
    {
        match *self {
            Some(ref mut t) => f(t),
            None => (),
        }
        self
    }
    
    fn or_else_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce() -> Option<T>,
    {
        if self.is_none() {
            *self = f();
        }
        self
    }

    fn and_then_mut<D, F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(T) -> Option<T>
    {
        if let Some(v) = self {
            *self = f(*v);
        }
        self
    }
}
