use core::marker::PhantomData;

use crate::{Slice, SliceMut};

/// Two slices joined via [`Slice::chain`].
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chain<T, A, B>(pub A, pub B, PhantomData<fn() -> T>);

impl<T, A, B> Chain<T, A, B>
where
    A: Slice<T>,
    B: Slice<T>,
{
    pub fn new(left: A, right: B) -> Self {
        Self(left, right, PhantomData)
    }
}

impl<T, A, B> Slice<T> for Chain<T, A, B>
where
    A: Slice<T>,
    B: Slice<T>,
{
    fn get(&self, index: usize) -> Option<&T> {
        self.0
            .get(index)
            .or_else(|| self.1.get(index - self.0.len()))
    }

    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }
}

impl<T, A, B> SliceMut<T> for Chain<T, A, B>
where
    A: SliceMut<T>,
    B: SliceMut<T>,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let len = self.0.len();
        if let r @ Some(_) = self.0.get_mut(index) {
            r
        } else {
            self.1.get_mut(index - len)
        }
    }
}
