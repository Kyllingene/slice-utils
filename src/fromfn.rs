use crate::{Slice, SliceOwned};

/// A slice calling a closure on index; see [`from_fn`](crate::from_fn).
#[derive(Clone, Copy, Hash)]
pub struct FromFn<F> {
    /// The inner closure of the slice.
    pub f: F,
    len: usize,
}

impl<F, T> FromFn<F>
where
    F: Fn(usize) -> Option<T>,
{
    /// Create a slice from a closure; see [`from_fn`](crate::from_fn).
    pub fn new(f: F, len: Option<usize>) -> Self {
        Self {
            f,
            len: len.unwrap_or(usize::MAX),
        }
    }
}

impl<F, T> Slice for FromFn<F>
where
    F: Fn(usize) -> Option<T>,
{
    type Output = T;

    fn len(&self) -> usize {
        self.len
    }

    fn get_with<W: FnMut(&Self::Output) -> R, R>(&self, index: usize, f: &mut W) -> Option<R> {
        Some(f(&self.get_owned(index)?))
    }
}

impl<F, T> SliceOwned for FromFn<F>
where
    F: Fn(usize) -> Option<T>,
{
    fn get_owned(&self, index: usize) -> Option<Self::Output> {
        (self.f)(index)
    }
}
