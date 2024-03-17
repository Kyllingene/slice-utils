use core::marker::PhantomData;

use crate::{Chain, Cycle, Reverse, SliceOf, SliceOfMut};
use crate::{Slice, SliceMut};

/// An iterator over a [`Slice`].
#[derive(Debug, Clone, Copy, Hash)]
pub struct Iter<'a, T, A> {
    start: usize,
    end: usize,
    data: &'a A,
    _marker: PhantomData<fn() -> &'a T>,
}

impl<'a, T: 'a, A: Slice<T>> Iter<'a, T, A> {
    pub fn new(data: &'a A) -> Self {
        Self {
            start: 0,
            end: data.len(),
            data,
            _marker: PhantomData,
        }
    }
}

impl<'a, T: 'a, A: Slice<T>> Iterator for Iter<'a, T, A> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.data.get(self.start);
        if item.is_some() {
            self.start += 1;
        }

        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.data.len();
        (l, Some(l))
    }
}

impl<'a, T: 'a, A: Slice<T>> DoubleEndedIterator for Iter<'a, T, A> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = self.data.get(self.end);
        if item.is_some() && self.end > 0 {
            self.end -= 1;
        }

        item
    }
}

impl<'a, T: 'a, A: Slice<T>> ExactSizeIterator for Iter<'a, T, A> {}

impl<T, A, B> Chain<T, A, B>
where
    A: Slice<T>,
    B: Slice<T>,
{
    pub fn iter(&self) -> Iter<T, Self> {
        Iter::new(self)
    }
}

impl<'a, T, A, B> IntoIterator for &'a Chain<T, A, B>
where
    A: Slice<T>,
    B: Slice<T>,
{
    type Item = &'a T;

    type IntoIter = Iter<'a, T, Chain<T, A, B>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, A> Cycle<T, A>
where
    A: Slice<T>,
{
    pub fn iter(&self) -> Iter<T, Self> {
        Iter::new(self)
    }
}

impl<'a, T, A> IntoIterator for &'a Cycle<T, A>
where
    A: Slice<T>,
{
    type Item = &'a T;

    type IntoIter = Iter<'a, T, Cycle<T, A>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, A> Reverse<T, A>
where
    A: Slice<T>,
{
    pub fn iter(&self) -> Iter<T, Self> {
        Iter::new(self)
    }
}

impl<'a, T, A> IntoIterator for &'a Reverse<T, A>
where
    A: Slice<T>,
{
    type Item = &'a T;

    type IntoIter = Iter<'a, T, Reverse<T, A>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, A> SliceOf<T, A>
where
    A: Slice<T>,
{
    pub fn iter(&self) -> Iter<T, Self> {
        Iter::new(self)
    }
}

impl<'a, T, A> IntoIterator for &'a SliceOf<T, A>
where
    A: Slice<T>,
{
    type Item = &'a T;

    type IntoIter = Iter<'a, T, SliceOf<T, A>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, A> SliceOfMut<T, A>
where
    A: SliceMut<T>,
{
    pub fn iter(&self) -> Iter<T, Self> {
        Iter::new(self)
    }
}

impl<'a, T, A> IntoIterator for &'a SliceOfMut<T, A>
where
    A: SliceMut<T>,
{
    type Item = &'a T;

    type IntoIter = Iter<'a, T, SliceOfMut<T, A>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
