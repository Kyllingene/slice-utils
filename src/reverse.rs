use core::marker::PhantomData;

use crate::{Slice, SliceMut};

/// A reversed slice, from [`Slice::reverse`].
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Reverse<T, A>(pub A, PhantomData<fn() -> T>);

impl<T, A> Reverse<T, A>
where
    A: Slice<T>,
{
    pub fn new(data: A) -> Self {
        Self(data, PhantomData)
    }
}

impl<T, A> Slice<T> for Reverse<T, A>
where
    A: Slice<T>,
{
    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }

        self.0.get(self.0.len() - index - 1)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T, A> SliceMut<T> for Reverse<T, A>
where
    A: SliceMut<T>,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() {
            return None;
        }

        self.0.get_mut(self.0.len() - index - 1)
    }
}
