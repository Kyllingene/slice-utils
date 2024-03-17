use core::marker::PhantomData;

use crate::{Slice, SliceMut};

/// Two interleaved slices, from [`Slice::interleave`].
pub struct Interleave<T, A, B> {
    pub left: A,
    pub right: B,
    _marker: PhantomData<fn() -> T>,
}

impl<T, A, B> Interleave<T, A, B>
where
    A: Slice<T>,
    B: Slice<T>,
{
    pub fn new(left: A, right: B) -> Self {
        Self {
            left,
            right,
            _marker: PhantomData,
        }
    }
}

impl<T, A, B> Slice<T> for Interleave<T, A, B>
where
    A: Slice<T>,
    B: Slice<T>,
{
    fn get(&self, index: usize) -> Option<&T> {
        if index % 2 == 0 {
            self.left.get(index / 2)
        } else {
            self.right.get(index / 2)
        }
    }

    fn len(&self) -> usize {
        self.left.len() + self.right.len()
    }
}

impl<T, A, B> SliceMut<T> for Interleave<T, A, B>
where
    A: SliceMut<T>,
    B: SliceMut<T>,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index % 2 == 0 {
            self.left.get_mut(index / 2)
        } else {
            self.right.get_mut(index / 2 + 1)
        }
    }
}
