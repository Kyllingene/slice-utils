use core::marker::PhantomData;

use crate::{Slice, SliceMut};

impl<T, A> Repeat<T, A>
where
    A: Slice<T>,
{
    pub fn new(data: A) -> Self {
        Self(data, PhantomData)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Repeat<T, A>(pub A, PhantomData<fn() -> T>);

impl<T, A> Slice<T> for Repeat<T, A>
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

impl<T, A> SliceMut<T> for Repeat<T, A>
where
    A: SliceMut<T>,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.0.get_mut(index % self.0.len())
    }
}
