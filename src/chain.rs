use core::marker::PhantomData;

use crate::Slice;

/// Two slices joined via [`Slice::chain`].
#[derive(Clone, Copy, Hash)]
pub struct Chain<T, A, B>(pub A, pub B, PhantomData<fn() -> T>);

impl<'a, T, A, B> Chain<T, A, B>
where
    A: Slice<'a, T>,
    B: Slice<'a, T>,
{
    /// See [`Slice::chain`].
    pub fn new(left: A, right: B) -> Self {
        Self(left, right, PhantomData)
    }
}

impl<'a, T, M, A, B> Slice<'a, T> for Chain<T, A, B>
where
    A: Slice<'a, T, Mut = M>,
    B: Slice<'a, T, Mut = M>,
{
    type Mut = A::Mut;

    fn get(&'a self, index: usize) -> Option<T> {
        self.0
            .get(index)
            .or_else(|| self.1.get(index - self.0.len()))
    }

    fn get_mut(&'a mut self, index: usize) -> Option<Self::Mut> {
        if let r @ Some(_) = self.0.get_mut(index) {
            r
        } else if let r @ Some(_) = self.1.get_mut(index) {
            r
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }
}
