#![cfg_attr(not(test), no_std)]

pub mod chain;
mod impls;
mod index;
pub mod iter;
pub mod repeat;
pub mod slicing;

#[cfg(test)]
mod test;

use core::ops::RangeBounds;

pub use chain::Chain;
pub use repeat::Repeat;
pub use slicing::{SliceOf, SliceOfMut};

pub type SplitOf<T, A> = Option<(SliceOf<T, A>, SliceOf<T, A>)>;

/// An extension trait providing iterator-like utilities for slices.
pub trait Slice<T>: Sized {
    fn get(&self, index: usize) -> Option<&T>;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn slice<R: RangeBounds<usize>>(self, range: R) -> Option<SliceOf<T, Self>> {
        SliceOf::new(self, range)
    }

    fn chain<O: Slice<T>>(self, other: O) -> Chain<T, Self, O> {
        Chain::new(self, other)
    }

    fn repeat(self) -> Repeat<T, Self> {
        Repeat::new(self)
    }

    fn split(&self, at: usize) -> SplitOf<T, &Self> {
        Some((SliceOf::new(self, ..at)?, SliceOf::new(self, at..)?))
    }
}

pub trait SliceMut<T>: Slice<T> {
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;

    fn slice<R: RangeBounds<usize>>(self, range: R) -> Option<SliceOfMut<T, Self>> {
        SliceOfMut::new(self, range)
    }
}
