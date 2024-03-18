use core::marker::PhantomData;

use crate::Slice;

/// Two interleaved slices, from [`Slice::interleave`].
pub struct Interleave<T, A, B> {
    pub left: A,
    pub right: B,
    _marker: PhantomData<fn() -> T>,
}

impl<'a, T, A, B> Interleave<T, A, B>
where
    A: Slice<'a, T>,
    B: Slice<'a, T>,
{
    /// See [`Slice::interleave`].
    pub fn new(left: A, right: B) -> Self {
        Self {
            left,
            right,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, M, A, B> Slice<'a, T> for Interleave<T, A, B>
where
    A: Slice<'a, T, Mut = M>,
    B: Slice<'a, T, Mut = M>,
{
    type Mut = A::Mut;

    fn get(&'a self, index: usize) -> Option<T> {
        if index % 2 == 0 {
            self.left.get(index / 2)
        } else {
            self.right.get(index / 2)
        }
    }

    fn get_mut(&'a mut self, index: usize) -> Option<Self::Mut> {
        if index % 2 == 0 {
            self.left.get_mut(index / 2)
        } else {
            self.right.get_mut(index / 2)
        }
    }

    fn len(&self) -> usize {
        self.left.len() + self.right.len()
    }
}
