#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod chain;
mod chunks;
mod cycle;
mod debug;
mod eq;
mod fromfn;
mod impls;
mod index;
mod interleave;
mod iter;
mod map;
mod reverse;
mod slicing;
mod windows;
mod zip;

#[cfg(test)]
mod test;

use core::ops::RangeBounds;

pub use chain::Chain;
pub use chunks::{ArrayChunksBorrowed, ArrayChunksOwned, ChunksBorrowed, ChunksOwned};
pub use cycle::Cycle;
pub use fromfn::FromFn;
pub use interleave::Interleave;
pub use iter::{IterBorrowed, IterOwned};
pub use map::{MapBorrowed, MapOwned};
pub use reverse::Reverse;
pub use slicing::SliceOf;
pub use windows::{ArrayWindowsBorrowed, ArrayWindowsOwned, WindowsBorrowed, WindowsOwned};
pub use zip::Zip;

/// Clones each item on access; see [`SliceBorrowed::cloned`].
pub type Cloned<S> = MapBorrowed<S, for<'a> fn(&<S as Slice>::Output) -> <S as Slice>::Output>;

/// The base trait for [`SliceOwned`], [`SliceBorrowed`], and [`SliceMut`].
pub trait Slice {
    /// The type this slice returns; analagous to
    /// [`Index::Output`](core::ops::Index::Output).
    type Output;

    /// Returns the length of the slice.
    fn len(&self) -> usize;

    /// Returns whether or not the slice is empty.
    ///
    /// Equivalent to `slice.len() == 0`.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Call a closure with the indicated element, returning the result or
    /// `None` if the index was out-of-bounds.
    ///
    /// This allows operations that are independent of indexing method.
    fn get_with<W: FnMut(&Self::Output) -> R, R>(&self, index: usize, f: &mut W) -> Option<R>;

    /// Chains two slices together, back-to-back.
    ///
    /// Analagous to [`Iterator::chain`].
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
    fn chain<S: Slice<Output = Self::Output>>(self, other: S) -> Chain<Self, S>
    where
        Self: Sized,
    {
        Chain(self, other)
    }

    /// Cycles the slice infinitely.
    ///
    /// Analagous to [`Iterator::cycle`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::{Slice, SliceOwned};
    /// let slice = [1, 2, 3].cycle();
    /// assert_eq!(slice.get_owned(2), Some(3));
    /// assert_eq!(slice.get_owned(4), Some(2));
    /// assert_eq!(slice.get_owned(6), Some(1));
    /// ```
    fn cycle(self) -> Cycle<Self>
    where
        Self: Sized,
    {
        Cycle(self)
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
    fn interleave<S: Slice<Output = Self::Output>>(self, other: S) -> Interleave<Self, S>
    where
        Self: Sized,
    {
        Interleave(self, other)
    }

    /// Reverses the slice.
    ///
    /// Analagous to [`Iterator::rev`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3].rev();
    /// assert_eq!(slice, [3, 2, 1]);
    /// ```
    fn rev(self) -> Reverse<Self>
    where
        Self: Sized,
    {
        Reverse(self)
    }

    /// Create a sub-slice of the slice.
    ///
    /// Analagous to slicing `&[T]`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let slice = [1, 2, 3, 4, 5].slice(1..4).unwrap();
    /// assert_eq!(slice, [2, 3, 4]);
    /// ```
    fn slice<R: RangeBounds<usize>>(self, range: R) -> Option<SliceOf<Self>>
    where
        Self: Sized,
    {
        SliceOf::new(self, range)
    }

    /// Returns `(&self[..at], &self[at..])`.
    /// Returns `None` if `at` is out-of-bounds.
    ///
    /// Analagous to [`slice::split`].
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
    fn split(&self, at: usize) -> Option<(SliceOf<&Self>, SliceOf<&Self>)> {
        Some((SliceOf::new(self, ..at)?, SliceOf::new(self, at..)?))
    }
}

/// A [`Slice`] that can return owned values.
pub trait SliceOwned: Slice {
    /// Index the slice, returning an owned value.
    fn get_owned(&self, index: usize) -> Option<Self::Output>;

    /// Return an iterator over arrays covering consecutive portions of the
    /// slice.
    ///
    /// Analagous to [`slice::array_chunks`].
    ///
    /// # Panics
    ///
    /// If `N == 0`, panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::{SliceOwned};
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut iter = slice.array_chunks::<2>();
    ///
    /// assert_eq!(iter.next(), Some([1, 2]));
    /// assert_eq!(iter.next(), Some([3, 4]));
    /// assert!(iter.next().is_none());
    /// assert_eq!(iter.remainder(), [5]);
    /// ```
    fn array_chunks<const N: usize>(self) -> ArrayChunksOwned<Self, N>
    where
        Self: Sized,
    {
        ArrayChunksOwned::new(self)
    }

    /// Return an iterator over arrays covering overlapping portions of the
    /// slice.
    ///
    /// Analagous to [`slice::array_windows`].
    ///
    /// # Panics
    ///
    /// If `N == 0`, panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::{SliceOwned};
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut iter = slice.array_windows::<3>();
    ///
    /// assert_eq!(iter.next(), Some([1, 2, 3]));
    /// assert_eq!(iter.next(), Some([2, 3, 4]));
    /// assert_eq!(iter.next(), Some([3, 4, 5]));
    /// assert!(iter.next().is_none());
    /// ```
    fn array_windows<const N: usize>(self) -> ArrayWindowsOwned<Self, N>
    where
        Self: Sized,
    {
        ArrayWindowsOwned::new(self)
    }

    /// Return an iterator over slices covering consecutive portions of the
    /// slice.
    ///
    /// Analagous to [`slice::chunks`].
    ///
    /// # Panics
    ///
    /// If `size == 0`, panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::SliceOwned;
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut iter = slice.chunks(2);
    ///
    /// assert_eq!(iter.next().unwrap(), [1, 2]);
    /// assert_eq!(iter.next().unwrap(), [3, 4]);
    /// assert_eq!(iter.next().unwrap(), [5]);
    /// assert!(iter.next().is_none());
    /// ```
    fn chunks(&self, size: usize) -> ChunksOwned<Self> {
        ChunksOwned::new(self, size)
    }

    /// Call a closure on index, returning a new type.
    ///
    /// Analagous to [`Iterator::map`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::SliceOwned;
    /// let slice = [0, 1, 2].map(|x| x != 0);
    /// assert_eq!(slice, [false, true, true]);
    /// ```
    fn map<F: Fn(Self::Output) -> R, R>(self, f: F) -> MapOwned<Self, F>
    where
        Self: Sized,
    {
        MapOwned(self, f)
    }

    /// Creates an iterator over the slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::{Slice, SliceOwned};
    /// let slice = [1, 2].chain([3]);
    /// let mut iter = slice.iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert!(iter.next().is_none());
    /// ```
    fn iter(self) -> IterOwned<Self>
    where
        Self: Sized,
    {
        IterOwned::new(self)
    }

    /// Try to collect the slice into an array, failing if the lengths don't
    /// match up.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::SliceOwned;
    /// let slice = [1, 2, 3].map(|i| i * i);
    /// let arr: [i32; 3] = slice.try_array().unwrap();
    /// assert_eq!(arr, [1, 4, 9]);
    /// ```
    fn try_array<const N: usize>(&self) -> Option<[Self::Output; N]> {
        if self.len() != N {
            None
        } else {
            Some(core::array::from_fn(|i| self.get_owned(i).unwrap()))
        }
    }

    /// Return an iterator over slices covering overlapping portions of the
    /// slice. The last window may be smaller than the rest.
    ///
    /// Analagous to [`slice::windows`].
    ///
    /// # Panics
    ///
    /// If `size == 0`, panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::{SliceOwned};
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut iter = slice.windows(3);
    ///
    /// assert_eq!(iter.next().unwrap(), [1, 2, 3]);
    /// assert_eq!(iter.next().unwrap(), [2, 3, 4]);
    /// assert_eq!(iter.next().unwrap(), [3, 4, 5]);
    /// assert!(iter.next().is_none());
    /// ```
    fn windows(&self, size: usize) -> WindowsOwned<Self> {
        WindowsOwned::new(self, size)
    }

    /// Zip two slices into a single slice, where indexing returns a tuple of
    /// their items.
    ///
    /// Analagous to [`Iterator::zip`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::SliceOwned;
    /// let a = [1, 2, 3];
    /// let b = [4, 5, 6, 7];
    /// let slice = a.zip(b);
    ///
    /// assert_eq!(
    ///     slice,
    ///     [
    ///         (1, 4),
    ///         (2, 5),
    ///         (3, 6),
    ///     ]
    /// );
    /// ```
    fn zip<O: SliceOwned>(self, other: O) -> Zip<Self, O>
    where
        Self: Sized,
    {
        Zip(self, other)
    }

    /// Collect the slice into a `Vec`. Only available on feature `std`.
    ///
    /// Analagous to [`Iterator::collect`].
    #[cfg(feature = "std")]
    fn collect(&self) -> Vec<Self::Output> {
        let mut v = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            v.push(self.get_owned(i).unwrap());
        }

        v
    }
}

/// A [`Slice`] that can return borrowed values.
pub trait SliceBorrowed: Slice {
    /// Index the slice, returning a borrowed value.
    fn get(&self, index: usize) -> Option<&Self::Output>;

    /// Return an iterator over arrays covering consecutive portions of the
    /// slice.
    ///
    /// Analagous to [`slice::array_chunks`].
    ///
    /// # Panics
    ///
    /// If `N == 0`, panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::SliceBorrowed;
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut iter = slice.array_chunks::<2>();
    ///
    /// assert_eq!(iter.next(), Some([&1, &2]));
    /// assert_eq!(iter.next(), Some([&3, &4]));
    /// assert!(iter.next().is_none());
    /// assert_eq!(iter.remainder(), [5]);
    /// ```
    fn array_chunks<const N: usize>(&self) -> ArrayChunksBorrowed<Self, N> {
        ArrayChunksBorrowed::new(self)
    }

    /// Return an iterator over arrays covering overlapping portions of the
    /// slice.
    ///
    /// Analagous to [`slice::array_windows`].
    ///
    /// # Panics
    ///
    /// If `N == 0`, panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::SliceBorrowed;
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut iter = slice.array_windows::<3>();
    ///
    /// assert_eq!(iter.next(), Some([&1, &2, &3]));
    /// assert_eq!(iter.next(), Some([&2, &3, &4]));
    /// assert_eq!(iter.next(), Some([&3, &4, &5]));
    /// assert!(iter.next().is_none());
    /// ```
    fn array_windows<const N: usize>(&self) -> ArrayWindowsBorrowed<Self, N> {
        ArrayWindowsBorrowed::new(self)
    }

    /// Return an iterator over slices covering consecutive portions of the
    /// slice.
    ///
    /// Analagous to [`slice::chunks`].
    ///
    /// # Panics
    ///
    /// If `size == 0`, panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::SliceBorrowed;
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut iter = slice.chunks(2);
    ///
    /// assert_eq!(iter.next().unwrap(), [1, 2]);
    /// assert_eq!(iter.next().unwrap(), [3, 4]);
    /// assert_eq!(iter.next().unwrap(), [5]);
    /// assert!(iter.next().is_none());
    /// ```
    fn chunks(&self, size: usize) -> ChunksBorrowed<Self> {
        ChunksBorrowed::new(self, size)
    }

    /// Call a closure on index, returning a new type.
    ///
    /// Analagous to [`Iterator::map`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::SliceBorrowed;
    /// let slice = [0, 1, 2].map(|x| x != 0);
    /// assert_eq!(slice, [false, true, true]);
    /// ```
    fn map<F: Fn(&Self::Output) -> R, R>(self, f: F) -> MapBorrowed<Self, F>
    where
        Self: Sized,
    {
        MapBorrowed(self, f)
    }

    /// Create a new slice that clones each value on access.
    /// Analagous to <code>self.[map](SliceBorrowed::map)([Clone::clone])</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::SliceBorrowed;
    /// # #[derive(Debug, PartialEq)]
    /// struct Foo(u32);
    /// impl Clone for Foo {
    ///     fn clone(&self) -> Foo { Foo(self.0 + 1) }
    /// }
    ///
    /// let slice = [Foo(1), Foo(2)];
    /// assert_eq!(slice.cloned(), [Foo(2), Foo(3)]);
    /// ```
    fn cloned(self) -> Cloned<Self>
    where
        Self: Sized,
        Self::Output: Clone,
    {
        MapBorrowed(self, Clone::clone)
    }

    /// Creates an iterator over the slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::{Slice, SliceBorrowed};
    /// let slice = [1, 2].chain([3]);
    /// let mut iter = slice.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert!(iter.next().is_none());
    /// ```
    fn iter(&self) -> IterBorrowed<Self> {
        IterBorrowed::new(self)
    }

    /// Return an iterator over slices covering overlapping portions of the
    /// slice. The last window may be smaller than the rest.
    ///
    /// Analagous to [`slice::windows`].
    ///
    /// # Panics
    ///
    /// If `size == 0`, panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::{SliceBorrowed};
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut iter = slice.windows(3);
    ///
    /// assert_eq!(iter.next().unwrap(), [1, 2, 3]);
    /// assert_eq!(iter.next().unwrap(), [2, 3, 4]);
    /// assert_eq!(iter.next().unwrap(), [3, 4, 5]);
    /// assert!(iter.next().is_none());
    /// ```
    fn windows(&self, size: usize) -> WindowsBorrowed<Self> {
        WindowsBorrowed::new(self, size)
    }
}

/// A [`Slice`] that can return mutably borrowed values.
pub trait SliceMut: Slice {
    /// Index the slice, returning a mutably borrowed value.
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output>;
}

/// A slice made by calling a closure on the index.
///
/// Analagous to [`core::iter::from_fn`].
///
/// # Examples
///
/// ```rust
/// # use slice_utils::SliceOwned;
/// let slice = slice_utils::from_fn(|i| Some(i * i), Some(5));
/// assert_eq!(slice, [0, 1, 4, 9, 16]);
/// ```
pub fn from_fn<F, T>(f: F, len: Option<usize>) -> FromFn<F>
where
    F: Fn(usize) -> Option<T>,
{
    FromFn::new(f, len)
}
