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

/// An iterator over overlapping windows of a [`Slice`], from [`Slice::windows`].
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArrayWindows<'a, T, A, const N: usize> {
    data: &'a A,
    i: usize,

    _marker: PhantomData<fn() -> &'a T>,
}

impl<'a, T, A, const N: usize> ArrayWindows<'a, T, A, N>
where
    A: Slice<T>,
{
    pub fn new(data: &'a A) -> Self {
        // TODO: make this a comptime error
        if N == 0 {
            panic!("cannot call array_windows with size 0");
        }

        Self {
            data,
            i: 0,

            _marker: PhantomData,
        }
    }

    pub fn inner(&self) -> &'a A {
        self.data
    }
}

impl<'a, T, A, const N: usize> Iterator for ArrayWindows<'a, T, A, N>
where
    A: Slice<T> + 'a,
{
    type Item = [&'a T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.data.len() - N + 1 && self.data.len() >= N {
            let x = core::array::from_fn(|i| self.data.get(self.i + i).unwrap());
            self.i += 1;
            Some(x)
        } else {
            None
        }
    }
}
