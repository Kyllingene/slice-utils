use core::marker::PhantomData;

use crate::{Slice, SliceOf};

/// An iterator over overlapping windows of a [`Slice`], from [`Slice::windows`].
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Windows<'a, T, A> {
    data: &'a A,
    size: usize,
    i: usize,

    _marker: PhantomData<fn() -> &'a T>,
}

impl<'a, T, A> Windows<'a, T, A>
where
    A: Slice<T>,
{
    pub fn new(data: &'a A, size: usize) -> Self {
        if size == 0 {
            panic!("cannot call windows with size 0");
        }

        Self {
            data,
            size,
            i: 0,

            _marker: PhantomData,
        }
    }

    pub fn inner(&self) -> &'a A {
        self.data
    }
}

impl<'a, T, A> Iterator for Windows<'a, T, A>
where
    A: Slice<T> + 'a,
{
    type Item = SliceOf<T, &'a A>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.data.len() - self.size + 1 && self.data.len() >= self.size {
            let x = self.data.slice(self.i..self.i + self.size);
            self.i += 1;
            x
        } else {
            None
        }
    }
}
