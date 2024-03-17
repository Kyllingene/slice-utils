use core::marker::PhantomData;

use crate::{Slice, SliceOf};

/// An iterator over fixed-sized chunks of a [`Slice`], from [`Slice::chunks`]
/// or [`Slice::chunks_exact`].
#[derive(Clone, Copy, Hash)]
pub struct Chunks<'a, T, A> {
    data: &'a A,
    size: usize,
    i: usize,
    _marker: PhantomData<fn() -> &'a T>,
}

impl<'a, T, A> Chunks<'a, T, A>
where
    T: 'a,
    A: Slice<T>,
{
    pub fn new(data: &'a A, size: usize) -> Self {
        Self {
            data,
            size,
            i: 0,
            _marker: PhantomData,
        }
    }

    pub fn new_exact(data: &'a A, size: usize) -> Option<Self> {
        if data.len() % size != 0 {
            None
        } else {
            Some(Self {
                data,
                size,
                i: 0,
                _marker: PhantomData,
            })
        }
    }

    pub fn inner(&self) -> &'a A {
        self.data
    }
}

impl<'a, T, A> Iterator for Chunks<'a, T, A>
where
    T: 'a,
    A: Slice<T>,
{
    type Item = SliceOf<T, &'a A>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.data.len() {
            let start = self.i;
            let end = (start + self.size).min(self.data.len());
            self.i = end;
            self.data.slice(start..end)
        } else {
            None
        }
    }
}

/// An iterator over const-sized chunks of a [`Slice`], from
/// [`Slice::array_chunks`] or [`Slice::array_chunks_exact`].
#[derive(Clone, Copy, Hash)]
pub struct ArrayChunks<'a, T, A, const N: usize> {
    data: &'a A,
    i: usize,
    _marker: PhantomData<fn() -> &'a T>,
}

impl<'a, T, A, const N: usize> ArrayChunks<'a, T, A, N>
where
    T: 'a,
    A: Slice<T>,
{
    pub fn new(data: &'a A) -> Self {
        Self {
            data,
            i: 0,
            _marker: PhantomData,
        }
    }

    pub fn new_exact(data: &'a A) -> Option<Self> {
        if data.len() % N != 0 {
            None
        } else {
            Some(Self {
                data,
                i: 0,
                _marker: PhantomData,
            })
        }
    }

    pub fn inner(&self) -> &'a A {
        self.data
    }
}

impl<'a, T, A, const N: usize> Iterator for ArrayChunks<'a, T, A, N>
where
    T: 'a,
    A: Slice<T>,
{
    type Item = [&'a T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.i + N < self.data.len() {
            let start = self.i;
            self.i += N;
            Some(core::array::from_fn(|i| {
                self.data
                    .get(start + i)
                    .unwrap_or_else(|| panic!("{start} {i} {}", self.i))
            }))
        } else {
            None
        }
    }
}
