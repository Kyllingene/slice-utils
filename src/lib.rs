#![cfg_attr(not(any(test, feature = "std")), no_std)]
//! See [`Slice`] and [`SliceMut`].
//!
//! This is a collection of utilities for slices, similar to those found on
//! iterators. The goal is to be as close to feature-parity with iterators as
//! possible, while maintaining `no_std` compatibility.
//!
//! The core of this crate is providing non-contiguous slices. For example,
//! `Slice::chain` allows you to join two slices together, clearly breaking
//! continuity. This results in a very `Iterator`-like API. Here are some
//! differences:
//!
//! - `Slice`s can only return references, not owned values - This disallows
//! methods like `map` which require ownership semantics - `Slice`s are not
//! lazy, and as such: - `Slice`s cannot perform arbitrary computation, because
//! that would require allocation
//!
//! These shortcomings may be alleviated by a const generic API in the future,
//! making these possible through statically known lengths.

mod chain;
mod chunks;
mod cycle;
mod debug;
mod eq;
mod impls;
mod index;
mod interleave;
mod iter;
mod reverse;
mod slicing;
mod windows;

#[cfg(test)]
mod test;

use core::ops::RangeBounds;

pub use chain::Chain;
pub use chunks::Chunks;
pub use cycle::Cycle;
pub use interleave::Interleave;
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3];
    /// assert_eq!(slice.get(2), Some(&3));
    /// ```
    fn get(&self, index: usize) -> Option<&T>;

    /// Returns the exact length of the slice.
    ///
    /// If infinite (e.g. through [`cycle`]), returns `usize::MAX`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3];
    /// assert_eq!(slice.len(), 3);
    /// let inf = slice.cycle();
    /// assert_eq!(inf.len(), usize::MAX);
    /// ```
    fn len(&self) -> usize;

    /// Whether or not the slice is empty (`self.len() == 0`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert!([0; 0].is_empty());
    /// assert!(![1, 2, 3].is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Takes a sub-slice, returning `None` if the range is out-of-bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3, 4, 5];
    /// assert_eq!(slice.slice(1..4).unwrap(), [2, 3, 4]);
    /// assert_eq!(slice.slice(0..6), None);
    /// ```
    fn slice<R: RangeBounds<usize>>(self, range: R) -> Option<SliceOf<T, Self>> {
        SliceOf::new(self, range)
    }

    /// Shortcut for [`.slice(n + 1..)`](Slice::slice)
    ///
    /// # Examples
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3, 4];
    /// assert_eq!(slice.skip(2), slice.slice(3..));
    /// ```
    fn skip(self, n: usize) -> Option<SliceOf<T, Self>> {
        SliceOf::new(self, n + 1..)
    }

    /// Shorcut for [`.slice(..=n)`](Slice::slice).
    ///
    /// # Examples
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3, 4];
    /// assert_eq!(slice.take(2), slice.slice(..=2));
    /// ```
    fn take(self, n: usize) -> Option<SliceOf<T, Self>> {
        SliceOf::new(self, ..=n)
    }

    /// Chains two slices together, back-to-back: the equivalent of
    /// [`Iterator::chain`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let a = [1, 2, 3];
    /// let b = [4, 5, 6];
    ///
    /// assert_eq!(a.chain(b), [1, 2, 3, 4, 5, 6]);
    /// ```
    fn chain<O: Slice<T>>(self, other: O) -> Chain<T, Self, O> {
        Chain::new(self, other)
    }

    /// Returns an iterator over fixed-size chunks: the equivalent of
    /// [`slice::chunks`]. The last chunk may be smaller than the rest.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut chunks = slice.chunks(2);
    /// assert_eq!(chunks.next().unwrap(), [1, 2]);
    /// assert_eq!(chunks.next().unwrap(), [3, 4]);
    /// assert_eq!(chunks.next().unwrap(), [5]);
    /// assert!(chunks.next().is_none());
    /// ```
    fn chunks(&self, size: usize) -> Chunks<T, Self> {
        Chunks::new(self, size)
    }

    /// Returns an iterator over fixed-size chunks. If `data.len()` is not
    /// divisible by `size`, returns None. Otherwise, equivalent to [`chunks`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3, 4, 5];
    /// assert!(slice.chunks_exact(2).is_none());
    /// ```
    fn chunks_exact(&self, size: usize) -> Option<Chunks<T, Self>> {
        Chunks::new_exact(self, size)
    }

    /// Repeats the slice forever: the equivalent of [`Iterator::cycle`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3].cycle();
    ///
    /// assert_eq!(slice[2], 3);
    /// assert_eq!(slice[4], 2);
    /// assert_eq!(slice[6], 1);
    /// ```
    fn cycle(self) -> Cycle<T, Self> {
        Cycle::new(self)
    }

    /// Interleaves two slices, e.g. [A, B, A, B, ...].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let a = [1, 2, 3];
    /// let b = [4, 5, 6];
    /// let c = a.interleave(b);
    ///
    /// assert_eq!(c, [1, 4, 2, 5, 3, 6]);
    /// ```
    fn interleave<O: Slice<T>>(self, other: O) -> Interleave<T, Self, O> {
        Interleave::new(self, other)
    }

    /// Reverses the slice: the equivalent of [`Iterator::rev`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3].reverse();
    /// assert_eq!(slice, [3, 2, 1]);
    /// ```
    fn reverse(self) -> Reverse<T, Self> {
        Reverse::new(self)
    }

    /// Returns an iterator over overlapping slices of length `size`:
    /// the equivalent of [`slice::windows`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut w = slice.windows(3);
    ///
    /// assert_eq!(w.next().unwrap(), [1, 2, 3]);
    /// assert_eq!(w.next().unwrap(), [2, 3, 4]);
    /// assert_eq!(w.next().unwrap(), [3, 4, 5]);
    /// assert!(w.next().is_none());
    /// ```
    fn windows(&self, size: usize) -> Windows<T, Self> {
        Windows::new(self, size)
    }

    /// Returns `(&self[..at], &self[at..])`.
    /// Returns `None` if `at` is out-of-bounds.
    ///
    /// Equivalent of [`slice::split`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3, 4, 5, 6];
    /// let (a, b) = slice.split(3).unwrap();
    ///
    /// assert_eq!(a, [1, 2, 3]);
    /// assert_eq!(b, [4, 5, 6]);
    /// ```
    fn split(&self, at: usize) -> Option<SplitOf<T, &Self>> {
        Some((SliceOf::new(self, ..at)?, SliceOf::new(self, at..)?))
    }

    /// Collects into a `Vec` by running a closure over each element.
    ///
    /// Only available on feature `std`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3];
    /// let vec = slice.collect(|x| *x + 1);
    ///
    /// assert_eq!(vec, [2, 3, 4]);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::SliceMut;
    /// let mut slice = [1, 2, 3];
    /// *slice.get_mut(2).unwrap() = 4;
    /// assert_eq!(slice, [1, 2, 4]);
    /// ```
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;

    /// Takes a mutable sub-slice, returning `None` if the range is out-of-bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::SliceMut;
    /// let mut slice = [1, 2, 3, 4, 5];
    /// let mut sliced = (&mut slice).slice_mut(2..).unwrap();
    /// sliced[0] = 0;
    ///
    /// assert_eq!(slice, [1, 2, 0, 4, 5]);
    /// ```
    fn slice_mut<R: RangeBounds<usize>>(self, range: R) -> Option<SliceOfMut<T, Self>> {
        SliceOfMut::new(self, range)
    }

    /// Calls a closure on each item, mutating it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::SliceMut;
    /// let mut slice = [1, 2, 3].slice_mut(..).unwrap();
    /// slice.map(|x| *x += 1);
    /// assert_eq!(slice, [2, 3, 4]);
    /// ```
    fn map<F>(&mut self, mut f: F)
    where
        F: for<'a> FnMut(&mut T),
    {
        for i in 0..self.len() {
            f(self.get_mut(i).unwrap());
        }
    }
}
