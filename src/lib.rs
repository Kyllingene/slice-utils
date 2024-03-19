#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod chain;
mod cycle;
mod debug;
mod eq;
mod impls;
mod index;
mod interleave;
mod map;
mod reverse;

#[cfg(test)]
mod test;

pub use chain::Chain;
pub use cycle::Cycle;
pub use interleave::Interleave;
pub use map::{MapBorrowed, MapOwned};
pub use reverse::Reverse;

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
}

/// A [`Slice`] that can return owned values.
pub trait SliceOwned: Slice {
    /// Index the slice, returning an owned value.
    fn get_owned(&self, index: usize) -> Option<Self::Output>;

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
}

/// A [`Slice`] that can return borrowed values.
pub trait SliceBorrowed: Slice {
    /// Index the slice, returning a borrowed value.
    fn get(&self, index: usize) -> Option<&Self::Output>;

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
}

/// A [`Slice`] that can return mutably borrowed values.
pub trait SliceMut: Slice {
    /// Index the slice, returning a mutably borrowed value.
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output>;

    // fn map<F: Fn(&mut Self::Output) -> R, R>(self, f: F) -> MapMut<Self, F>
    // where
    //     Self: Sized,
    // {
    //     MapMut(self, f)
    // }
}
