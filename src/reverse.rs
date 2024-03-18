use core::marker::PhantomData;

use crate::Slice;

/// A reversed slice, from [`Slice::reverse`].
#[derive(Clone, Copy, Hash)]
pub struct Reverse<T, A>(pub A, PhantomData<fn() -> T>);

impl<'a, T, A> Reverse<T, A>
where
    A: Slice<'a, T>,
{
    /// See [`Slice::reverse`].
    pub fn new(data: A) -> Self {
        Self(data, PhantomData)
    }
}

impl<'a, T, A> Slice<'a, T> for Reverse<T, A>
where
    A: Slice<'a, T>,
{
    type Mut = A::Mut;

    fn get(&'a self, index: usize) -> Option<T> {
        if index >= self.len() {
            None
        } else {
            self.0.get(self.0.len() - 1 - index)
        }
    }

    fn get_mut(&'a mut self, index: usize) -> Option<Self::Mut> {
        if index >= self.len() {
            None
        } else {
            self.0.get_mut(self.0.len() - 1 - index)
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}
