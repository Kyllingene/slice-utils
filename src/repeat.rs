use core::marker::PhantomData;

use crate::{Slice, SliceMut};

/// An infinitely looped slice, from [`Slice::cycle`].
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cycle<T, A>(pub A, PhantomData<fn() -> T>);

impl<T, A> Cycle<T, A>
where
    A: Slice<T>,
{
    pub fn new(data: A) -> Self {
        Self(data, PhantomData)
    }
}

impl<T, A> Slice<T> for Cycle<T, A>
where
    A: Slice<T>,
{
    fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index % self.0.len())
    }

    fn len(&self) -> usize {
        usize::MAX
    }
}

impl<T, A> SliceMut<T> for Cycle<T, A>
where
    A: SliceMut<T>,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.0.get_mut(index % self.0.len())
    }
}
