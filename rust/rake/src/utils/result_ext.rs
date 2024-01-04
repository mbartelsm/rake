pub trait ResultExt<T, E> {

    /// If `Ok`, it mutates the result value in-place
    fn map_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut T);

    /// If `Err`, it mutates the error value in-place
    fn map_err_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut E);

    /// If `Err`, it mutates the result to the value produced by the given
    /// function.
    fn or_else_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(E) -> Result<T, E>;

    /// If `Ok`, it mutates the result to the value produced by the given
    /// function.
    fn and_then_mut<D, F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(T) -> Result<T, E>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn map_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut T),
    {
        match *self {
            Ok(ref mut v) => f(v),
            Err(_) => {},
        }
        self
    }

    fn map_err_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut E),
    {
        match *self {
            Ok(_) => {},
            Err(ref mut e) => f(e),
        }
        self
    }
    
    fn or_else_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(E) -> Result<T, E>
    {
        if let Err(e) = self {
            *self = f(*e);
        }
        self
    }

    fn and_then_mut<D, F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(T) -> Result<T, E>
    {
        if let Ok(v) = self {
            *self = f(*v);
        }
        self
    }
}
