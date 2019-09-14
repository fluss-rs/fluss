use std::marker::PhantomData as marker;

#[derive(Copy, Clone)]
pub struct Inlet<'a, T> {
    pub name: &'a str,
    _private: marker<T>,
}

#[derive(Copy, Clone)]
pub struct Outlet<'a, T> {
    pub name: &'a str,
    _private: marker<T>,
}

//impl<T> From<Outlet<T>> for Vec<_> {
//    fn from(out: Outlet<T>) -> Self {
//        CliError::IoError(error)
//    }
//}
