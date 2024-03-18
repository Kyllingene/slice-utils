#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod chain;
mod chunks;
mod cycle;
mod debug;
mod eq;
mod impls;
mod index;
mod interleave;
mod iter;
mod map;
mod reverse;
mod slicing;
mod windows;

#[cfg(test)]
mod test;

use core::ops::RangeBounds;

pub use chain::Chain;
pub use chunks::{ArrayChunks, Chunks};
pub use cycle::Cycle;
pub use interleave::Interleave;
pub use map::{Map, MapMut};
pub use reverse::Reverse;
pub use slicing::SliceOf;
pub use windows::{ArrayWindows, Windows};

/// A split, returned by [`Slice::split`].
pub type SplitOf<T, A> = (SliceOf<T, A>, SliceOf<T, A>);

/// An extension trait providing iterator-like utilities for slices.
pub trait Slice<'a, T>: Sized {
    type Mut;

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
    fn get(&'a self, index: usize) -> Option<T>;

    #[allow(unused_variables)]
    fn get_mut(&'a mut self, index: usize) -> Option<Self::Mut> {
        None
    }

    /// Returns the exact length of the slice.
    ///
    /// If infinite (e.g. through [`cycle`](Slice::cycle)), returns
    /// `usize::MAX`.
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
    fn split(&'a self, at: usize) -> Option<SplitOf<T, &Self>> {
        Some((SliceOf::new(self, ..at)?, SliceOf::new(self, at..)?))
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

    /// Returns an iterator over const-size chunks: the equivalent of
    /// [`slice::array_chunks`]. All the chunks are guaranteed to be the same
    /// size: items at the end will be skipped if there aren't enough to fill a
    /// chunk.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut chunks = slice.array_chunks::<2>();
    /// assert_eq!(chunks.next().unwrap(), [&1, &2]);
    /// assert_eq!(chunks.next().unwrap(), [&3, &4]);
    /// assert!(chunks.next().is_none());
    /// ```
    fn array_chunks<const N: usize>(&'a self) -> ArrayChunks<T, Self, N> {
        ArrayChunks::new(self)
    }

    /// Returns an iterator over fixed-size chunks. If `data.len()` is not
    /// divisible by `size`, returns None. Otherwise, equivalent to
    /// [`array_chunks`](Slice::array_chunks).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3, 4, 5];
    /// assert!(slice.array_chunks_exact::<2>().is_none());
    /// ```
    fn array_chunks_exact<const N: usize>(&'a self) -> Option<ArrayChunks<T, Self, N>> {
        ArrayChunks::new_exact(self)
    }

    /// Returns an iterator over overlapping slices of length `size`:
    /// the equivalent of [`slice::array_windows`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut w = slice.array_windows::<3>();
    ///
    /// assert_eq!(w.next().unwrap(), [&1, &2, &3]);
    /// assert_eq!(w.next().unwrap(), [&2, &3, &4]);
    /// assert_eq!(w.next().unwrap(), [&3, &4, &5]);
    /// assert!(w.next().is_none());
    /// ```
    fn array_windows<const N: usize>(&'a self) -> ArrayWindows<T, Self, N> {
        ArrayWindows::new(self)
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
    fn chain<O: Slice<'a, T>>(self, other: O) -> Chain<T, Self, O> {
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
    fn chunks(&'a self, size: usize) -> Chunks<T, Self> {
        Chunks::new(self, size)
    }

    /// Returns an iterator over fixed-size chunks. If `data.len()` is not
    /// divisible by `size`, returns None. Otherwise, equivalent to
    /// [`chunks`](Slice::array_chunks).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3, 4, 5];
    /// assert!(slice.chunks_exact(2).is_none());
    /// ```
    fn chunks_exact(&'a self, size: usize) -> Option<Chunks<T, Self>> {
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
    fn interleave<O: Slice<'a, T>>(self, other: O) -> Interleave<T, Self, O> {
        Interleave::new(self, other)
    }

    /// Takes a closure and creates a slice which calls that closure on index:
    /// the equivalent of [`Iterator::map`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3];
    /// assert_eq!(slice.map(|x| x == 2), [false, true, false]);
    /// ```
    fn map<F: Fn(T) -> U, U>(self, f: F) -> Map<T, Self, F> {
        Map::new(self, f)
    }

    /// Takes a closure and creates a slice which calls that closure on mutable
    /// index: the equivalent of [`Iterator::map`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let mut slice = [1, 2, 3];
    /// assert_eq!(
    ///     (&mut slice).map(|x| x == 2),
    ///     [false, true, false]
    /// );
    /// ```
    fn map_mut<F: FnMut(Self::Mut) -> U, U>(self, f: F) -> MapMut<T, Self, F, U> {
        MapMut::new(self, f)
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
    fn windows(&'a self, size: usize) -> Windows<T, Self> {
        Windows::new(self, size)
    }
}

// macro_rules! mod_with_docs {
//     ($( $mod:ident ),*) => {
//         $(
//             #[doc = "See [`" $mod:camel "`]"]
//             mod $mod;
//         )*
//     };
// }
// use mod_with_docs;
