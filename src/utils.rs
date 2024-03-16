use core::marker::PhantomData;

use crate::Slice;

//== Chain ==//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chain<T, A, B>(pub A, pub B, PhantomData<fn() -> T>);

impl<T, A, B> Chain<T, A, B>
where
    A: for<'a> Slice<'a, T>,
    B: for<'a> Slice<'a, T>,
{
    pub fn new(left: A, right: B) -> Self {
        Self(left, right, PhantomData)
    }
}

impl<'a, T: 'a, A, B> Slice<'a, T> for Chain<T, A, B>
where
    A: Slice<'a, T>,
    B: Slice<'a, T>,
{
    fn index(&'a self, index: usize) -> Option<T> {
        self.0
            .index(index)
            .or_else(|| self.1.index(index - self.0.len()))
    }

    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }
}

//== Fuse ==//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fuse<T, A>(pub A, PhantomData<fn() -> T>);

impl<T, A> Fuse<T, A>
where
    A: for<'a> Slice<'a, T>,
{
    pub fn new(data: A) -> Self {
        Self(data, PhantomData)
    }
}

impl<'a, T: 'a, A> Slice<'a, T> for Fuse<T, A>
where
    A: Slice<'a, T>,
{
    fn index(&'a self, index: usize) -> Option<T> {
        self.0.index(index % self.0.len())
    }

    fn len(&self) -> usize {
        usize::MAX
    }
}
