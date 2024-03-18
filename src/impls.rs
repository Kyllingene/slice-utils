use core::convert::Infallible;

use crate::Slice;

impl<'a, T, const N: usize> Slice<'a, &'a T> for [T; N] {
    type Mut = &'a mut T;

    fn get(&'a self, index: usize) -> Option<&'a T> {
        if index >= N {
            None
        } else {
            Some(&self[index])
        }
    }

    fn get_mut(&'a mut self, index: usize) -> Option<Self::Mut> {
        if index >= N {
            None
        } else {
            Some(&mut self[index])
        }
    }

    fn len(&self) -> usize {
        N
    }
}

impl<'a, T> Slice<'a, &'a T> for &'a [T] {
    type Mut = Infallible;

    fn get(&'a self, index: usize) -> Option<&'a T> {
        if index >= self.len() {
            None
        } else {
            Some(&self[index])
        }
    }

    fn len(&self) -> usize {
        (*self).len()
    }
}

impl<'a, T> Slice<'a, &'a T> for &'a mut [T] {
    type Mut = &'a mut T;

    fn get(&'a self, index: usize) -> Option<&'a T> {
        (**self).get(index)
    }

    fn get_mut(&'a mut self, index: usize) -> Option<Self::Mut> {
        (**self).get_mut(index)
    }

    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<'a, T, A> Slice<'a, T> for &'a A
where
    A: Slice<'a, T>,
{
    type Mut = Infallible;

    fn get(&'a self, index: usize) -> Option<T> {
        (*self).get(index)
    }

    fn len(&self) -> usize {
        (*self).len()
    }
}

impl<'a, T, A> Slice<'a, T> for &'a mut A
where
    A: Slice<'a, T>,
{
    type Mut = A::Mut;

    fn get(&'a self, index: usize) -> Option<T> {
        (**self).get(index)
    }

    fn get_mut(&'a mut self, index: usize) -> Option<Self::Mut> {
        (*self).get_mut(index)
    }

    fn len(&self) -> usize {
        (**self).len()
    }
}
