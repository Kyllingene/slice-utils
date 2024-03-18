use core::marker::PhantomData;

use crate::Slice;

/// An infinitely looped slice, from [`Slice::cycle`].
#[derive(Clone, Copy, Hash)]
pub struct Cycle<T, A>(pub A, PhantomData<fn() -> T>);

impl<'a, T, A> Cycle<T, A>
where
    A: Slice<'a, T>,
{
    /// See [`Slice::cycle`].
    pub fn new(data: A) -> Self {
        Self(data, PhantomData)
    }
}

impl<'a, T, A> Slice<'a, T> for Cycle<T, A>
where
    A: Slice<'a, T>,
{
    type Mut = A::Mut;

    fn get(&'a self, index: usize) -> Option<T> {
        self.0.get(index % self.0.len())
    }

    fn get_mut(&'a mut self, index: usize) -> Option<Self::Mut> {
        self.0.get_mut(index % self.0.len())
    }

    fn len(&self) -> usize {
        usize::MAX
    }
}
