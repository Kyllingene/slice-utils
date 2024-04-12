use core::ops::{Range, RangeFrom, RangeInclusive};

use crate::{Slice, SliceBorrowed, SliceMut, SliceOwned, Unique};

impl<T, const N: usize> Slice for [T; N] {
    type Output = T;

    fn len(&self) -> usize {
        N
    }

    fn get_with<W: FnMut(&Self::Output) -> R, R>(&self, index: usize, f: &mut W) -> Option<R> {
        Some(f(self.get(index)?))
    }
}

impl<T, const N: usize> SliceOwned for [T; N]
where
    T: Copy,
{
    fn get_owned(&self, index: usize) -> Option<Self::Output> {
        (index < N).then(|| self[index])
    }
}

impl<T, const N: usize> SliceBorrowed for [T; N] {
    fn get(&self, index: usize) -> Option<&Self::Output> {
        (index < N).then(|| &self[index])
    }
}

impl<T, const N: usize> SliceMut for [T; N] {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        (index < N).then(|| &mut self[index])
    }
}

// SAFETY: arrays are contiguous in memory
unsafe impl<T, const N: usize> Unique for [T; N] {}

impl<T> Slice for [T] {
    type Output = T;

    fn len(&self) -> usize {
        (*self).len()
    }

    fn get_with<W: FnMut(&Self::Output) -> R, R>(&self, index: usize, f: &mut W) -> Option<R> {
        Some(f(self.get(index)?))
    }
}

impl<T> SliceOwned for [T]
where
    T: Copy,
{
    fn get_owned(&self, index: usize) -> Option<Self::Output> {
        (index < self.len()).then(|| self[index])
    }
}

impl<T> SliceBorrowed for [T] {
    fn get(&self, index: usize) -> Option<&Self::Output> {
        (index < self.len()).then(|| &self[index])
    }
}

impl<T> SliceMut for [T] {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        (index < self.len()).then(|| &mut self[index])
    }
}

// SAFETY: slices are contiguous in memory
unsafe impl<T> Unique for [T] {}

impl<'a, S> Slice for &'a S
where
    S: Slice + ?Sized,
{
    type Output = S::Output;

    fn len(&self) -> usize {
        (*self).len()
    }

    fn get_with<W: FnMut(&Self::Output) -> R, R>(&self, index: usize, f: &mut W) -> Option<R> {
        (*self).get_with(index, f)
    }
}

impl<'a, S> SliceOwned for &'a S
where
    S: SliceOwned + ?Sized,
{
    fn get_owned(&self, index: usize) -> Option<Self::Output> {
        (*self).get_owned(index)
    }
}

impl<'a, S> SliceBorrowed for &'a S
where
    S: SliceBorrowed + ?Sized,
{
    fn get(&self, index: usize) -> Option<&Self::Output> {
        (**self).get(index)
    }
}

impl<'a, S> Slice for &'a mut S
where
    S: Slice + ?Sized,
{
    type Output = S::Output;

    fn len(&self) -> usize {
        (**self).len()
    }

    fn get_with<W: FnMut(&Self::Output) -> R, R>(&self, index: usize, f: &mut W) -> Option<R> {
        (**self).get_with(index, f)
    }
}

impl<'a, S> SliceOwned for &'a mut S
where
    S: SliceOwned + ?Sized,
{
    fn get_owned(&self, index: usize) -> Option<Self::Output> {
        (**self).get_owned(index)
    }
}

impl<'a, S> SliceBorrowed for &'a mut S
where
    S: SliceBorrowed + ?Sized,
{
    fn get(&self, index: usize) -> Option<&Self::Output> {
        (**self).get(index)
    }
}

impl<'a, S> SliceMut for &'a mut S
where
    S: SliceMut + ?Sized,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        (**self).get_mut(index)
    }
}

// SAFETY: the underlying slice is `Unique`
unsafe impl<'a, S> Unique for &'a mut S where S: Unique + ?Sized {}

macro_rules! impl_for_range {
    ($($range:ident),*) => {$(
        impl<T> Slice for $range<T>
        where
            T: Clone,
            $range<T>: Iterator<Item = T>,
        {
            type Output = T;

            fn len(&self) -> usize {
                self.size_hint().0
            }

            fn get_with<W: FnMut(&Self::Output) -> R, R>(&self, index: usize, f: &mut W)
                -> Option<R>
            {
                self.clone().nth(index)
                    .as_ref()
                    .map(f)
            }
        }

        impl<T> SliceOwned for $range<T>
        where
            T: Clone,
            $range<T>: Iterator<Item = T>,
        {
            fn get_owned(&self, index: usize) -> Option<T> {
                self.clone().nth(index)
            }
        }
    )*};
}

// TODO: figure out a way to include all the ranges
impl_for_range!(Range, RangeFrom, RangeInclusive);
