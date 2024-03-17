#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![doc = include_str!("../README.md")]

mod chain;
mod debug;
mod eq;
mod impls;
mod index;
mod interleave;
mod iter;
mod repeat;
mod reverse;
mod slicing;
mod windows;

#[cfg(test)]
mod test;

use core::ops::RangeBounds;

pub use chain::Chain;
pub use interleave::Interleave;
pub use repeat::Cycle;
pub use reverse::Reverse;
pub use slicing::{SliceOf, SliceOfMut};
pub use windows::Windows;

/// A split, returned by [`Slice::split`].
pub type SplitOf<T, A> = (SliceOf<T, A>, SliceOf<T, A>);

/// An extension trait providing iterator-like utilities for slices.
pub trait Slice<T>: Sized {
    /// Index the slice.
    ///
    /// If `index < self.len()`, this must succeed.
    fn get(&self, index: usize) -> Option<&T>;

    /// Returns the exact length of the slice.
    ///
    /// If infinite (e.g. through [`repeat`]), returns `usize::MAX`.
    fn len(&self) -> usize;

    /// Whether or not the slice is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Takes a sub-slice, returning `None` if the range is out-of-bounds.
    fn slice<R: RangeBounds<usize>>(self, range: R) -> Option<SliceOf<T, Self>> {
        SliceOf::new(self, range)
    }

    /// Shortcut for [`.slice(n + 1..)`](Slice::slice)
    fn skip(self, n: usize) -> Option<SliceOf<T, Self>> {
        SliceOf::new(self, n + 1..)
    }

    /// Shorcut for [`.slice(..=n)`](Slice::slice).
    fn take(self, n: usize) -> Option<SliceOf<T, Self>> {
        SliceOf::new(self, ..=n)
    }

    /// Chains two slices together, back-to-back: the equivalent of
    /// [`Iterator::chain`].
    fn chain<O: Slice<T>>(self, other: O) -> Chain<T, Self, O> {
        Chain::new(self, other)
    }

    /// Repeats the slice forever: the equivalent of [`Iterator::cycle`].
    fn cycle(self) -> Cycle<T, Self> {
        Cycle::new(self)
    }

    /// Interleaves two slices, e.g. [A, B, A, B, ...].
    fn interleave<O: Slice<T>>(self, other: O) -> Interleave<T, Self, O> {
        Interleave::new(self, other)
    }

    /// Reverses the slice.
    ///
    /// Note: does not actually modify the underlying data.
    /// See [`SliceMut::reverse_inplace`] for that.
    ///
    /// Equivalent of [`Iterator::rev`].
    fn reverse(self) -> Reverse<T, Self> {
        Reverse::new(self)
    }

    /// Returns an iterator over overlapping slices of length `size`:
    /// the equivalent of [`slice::windows`].
    fn windows(&self, size: usize) -> Windows<T, Self> {
        Windows::new(self, size)
    }

    /// Returns `(&self[..at], &self[at..])`.
    ///
    /// Returns `None` if `at` is out-of-bounds.
    ///
    /// Equivalent of [`slice::split`].
    fn split(&self, at: usize) -> Option<SplitOf<T, &Self>> {
        Some((SliceOf::new(self, ..at)?, SliceOf::new(self, at..)?))
    }

    /// Collects into a `Vec` by running a closure over each element.
    ///
    /// Only available on feature `std`.
    #[cfg(feature = "std")]
    fn collect<F, U>(self, f: F) -> Vec<U>
    where
        F: for<'a> FnMut(&'a T) -> U,
    {
        (0..self.len())
            .map(|i| self.get(i).unwrap())
            .map(f)
            .collect()
    }
}

/// A mutable slice. See [`Slice`] for more information.
pub trait SliceMut<T>: Slice<T> {
    /// Mutably index the slice.
    ///
    /// If `index < self.len()`, this must succeed.
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;

    /// Takes a mutable sub-slice, returning `None` if the range is out-of-bounds.
    fn slice_mut<R: RangeBounds<usize>>(self, range: R) -> Option<SliceOfMut<T, Self>> {
        SliceOfMut::new(self, range)
    }

    /// Calls a closure on each item, mutating it.
    fn map<F>(&mut self, mut f: F)
    where
        F: for<'a> FnMut(&mut T),
    {
        for i in 0..self.len() {
            f(self.get_mut(i).unwrap());
        }
    }
}
