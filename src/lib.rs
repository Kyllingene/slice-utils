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
pub use slicing::{SliceOf, SplitMut};
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::Slice;
    /// let a = [1_i32, 2, 3];
    ///
    /// // Doesn't care about ownership
    /// fn stringify_first<S>(slice: &S) -> String
    /// where
    ///     S: Slice,
    ///     S::Output: ToString,
    /// {
    ///     slice.get_with(0, &mut |x| x.to_string()).unwrap_or_default()
    /// }
    ///
    /// assert_eq!(stringify_first(&a), "1");
    /// ```
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

    /// Return a slice/iterator over arrays covering overlapping portions of the
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
    /// let mut windows = slice.array_windows::<3>();
    ///
    /// assert_eq!(windows.next(), Some([&1, &2, &3]));
    /// assert_eq!(windows.next(), Some([&2, &3, &4]));
    /// assert_eq!(windows.next(), Some([&3, &4, &5]));
    /// assert!(windows.next().is_none());
    /// # {
    /// # use slice_utils::SliceOwned;
    /// assert_eq!(windows.get_owned(1), Some([&2, &3, &4]));
    /// # }
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

    /// Return a slice/iterator over slices covering overlapping portions of the
    /// slice.
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
    /// # use slice_utils::SliceBorrowed;
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut windows = slice.windows(3);
    ///
    /// assert_eq!(windows.next().unwrap(), [1, 2, 3]);
    /// assert_eq!(windows.next().unwrap(), [2, 3, 4]);
    /// assert_eq!(windows.next().unwrap(), [3, 4, 5]);
    /// assert!(windows.next().is_none());
    /// # {
    /// # use slice_utils::SliceOwned;
    /// assert_eq!(windows.get_owned(1).unwrap(), [2, 3, 4]);
    /// # }
    /// ```
    fn windows(&self, size: usize) -> WindowsBorrowed<Self> {
        WindowsBorrowed::new(self, size)
    }
}

/// A [`Slice`] that can return mutably borrowed values.
pub trait SliceMut: Slice {
    /// Index the slice, returning a mutably borrowed value.
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output>;

    /// Returns `(&mut self[..at], &mut self[at..])`.
    /// Returns `None` if `at` is out-of-bounds.
    ///
    /// Analagous to [`slice::split_mut`].
    ///
    /// To avoid aliasing, requires <code>Self: [Unique]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::SliceMut;
    /// let mut slice = [1, 2, 3, 4, 5];
    /// let (mut left, mut right) = SliceMut::split_mut(&mut slice, 2).unwrap();
    ///
    /// assert_eq!(left, [1, 2, 3]);
    /// assert_eq!(right, [4, 5]);
    ///
    /// left[0] = 0;
    /// right[0] = 0;
    /// assert_eq!(slice, [0, 2, 3, 0, 5]);
    /// ```
    fn split_mut(&mut self, at: usize) -> Option<(SplitMut<Self>, SplitMut<Self>)>
    where
        Self: Unique,
    {
        SplitMut::new(self, at)
    }

    /// Copy all the items from `src` into `self`.
    ///
    /// Similar to [`slice::clone_from_slice`].
    ///
    /// # Panics
    ///
    /// The lengths must match. If you only want to copy a sub-slice, you can
    /// slice each side down to the desired range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::{Slice, SliceMut};
    /// let mut x = [1, 2, 3, 4, 5];
    /// let y = [6, 7];
    ///
    /// (&mut x).slice(3..).unwrap().copy_from_slice(&y);
    /// 
    /// assert_eq!(x, [1, 2, 3, 6, 7]);
    /// ```
    fn copy_from_slice<S>(&mut self, src: &S)
    where
        S: Slice<Output = Self::Output>,
        S::Output: Clone,
    {
        if self.len() != src.len() {
            panic!("source slice length ({}) does not match destination slice length ({})",
                src.len(), self.len())
        }

        for i in 0..src.len() {
            src.get_with(i, &mut |item| *self.get_mut(i).unwrap_or_else(|| {
                panic!("destination slice is shorter than source slice");
            }) = item.clone());
        }
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

    /// Return a slice/iterator over arrays covering overlapping portions of the
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
    /// let mut windows = slice.array_windows::<3>();
    ///
    /// assert_eq!(windows.next(), Some([1, 2, 3]));
    /// assert_eq!(windows.next(), Some([2, 3, 4]));
    /// assert_eq!(windows.next(), Some([3, 4, 5]));
    /// assert!(windows.next().is_none());
    /// assert_eq!(windows.get_owned(1), Some([2, 3, 4]));
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

    /// Return a slice/iterator over slices covering overlapping portions of the
    /// slice.
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
    /// # use slice_utils::SliceOwned;
    /// let slice = [1, 2, 3, 4, 5];
    /// let mut windows = slice.windows(3);
    ///
    /// assert_eq!(windows.next().unwrap(), [1, 2, 3]);
    /// assert_eq!(windows.next().unwrap(), [2, 3, 4]);
    /// assert_eq!(windows.next().unwrap(), [3, 4, 5]);
    /// assert!(windows.next().is_none());
    /// assert_eq!(windows.get_owned(1).unwrap(), [2, 3, 4]);
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

/// A [slice](SliceBorrowed) whose backing is contiguous in memory.
///
/// See also [`ContiguousMut`]. Note that there's no `ContiguousOwned`: this is
/// because Rust does not (yet) allow you to use associated constants in const
/// operations, so there is no way to return an array.
///
/// The alternative would be an "owning borrow", where you return a slice that
/// has permission to treat the referenced data like it owns it (because it
/// does), but those are still imperfect.
pub trait ContiguousBorrowed: SliceBorrowed + Unique {
    /// Get the contiguous backing of this slice.
    ///
    /// The returned slice should behave exactly like this one, e.g.
    /// `self.get(n) == self.contiguous().get(n)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::ContiguousBorrowed;
    /// let a = [1, 2, 3].to_vec();
    /// let b = a.contiguous();
    ///
    /// assert_eq!(a, b);
    /// ```
    fn contiguous(&self) -> &[Self::Output];
}

/// A [slice](SliceMut) whose backing is contiguous in memory.
///
/// See also [`ContiguousBorrowed`].
pub trait ContiguousMut: SliceMut + Unique {
    /// Get a mutable reference to the contiguous backing of this slice.
    ///
    /// The returned slice should behave exactly like this one, e.g.
    /// `self.get_mut(n) == self.contiguous_mut().get_mut(n)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use slice_utils::ContiguousMut;
    /// let mut a = [1, 2, 3].to_vec();
    ///
    /// let b = a.contiguous_mut();
    /// b.copy_from_slice(&[4, 5, 6]); // method on core::slice
    ///
    /// assert_eq!(a, [4, 5, 6]);
    /// ```
    fn contiguous_mut(&mut self) -> &mut [Self::Output];
}

/// A marker trait confirming that two indices of a [`Slice`] will never alias.
///
/// # Safety
///
/// Two calls to `get_mut` (or any other index operation) with different indices
/// must never return aliasing references.
pub unsafe trait Unique {}

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
