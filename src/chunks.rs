use core::marker::PhantomData;

use crate::{Slice, SliceOf};

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
