use crate::{Slice, SliceMut};

impl<T, const N: usize> Slice<T> for [T; N] {
    fn get(&self, index: usize) -> Option<&T> {
        if index >= N {
            None
        } else {
            Some(&self[index])
        }
    }

    fn len(&self) -> usize {
        N
    }
}

impl<T, const N: usize> SliceMut<T> for [T; N] {
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= N {
            None
        } else {
            Some(&mut self[index])
        }
    }
}

impl<'a, T> Slice<T> for &'a [T] {
    fn get(&self, index: usize) -> Option<&T> {
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

impl<'a, T> Slice<T> for &'a mut [T] {
    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            None
        } else {
            Some(&self[index])
        }
    }

    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<'a, T> SliceMut<T> for &'a mut [T] {
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() {
            None
        } else {
            Some(&mut self[index])
        }
    }
}

impl<'a, T, A> Slice<T> for &'a A
where
    A: Slice<T>,
{
    fn get(&self, index: usize) -> Option<&T> {
        (*self).get(index)
    }

    fn len(&self) -> usize {
        (*self).len()
    }
}

impl<'a, T, A> Slice<T> for &'a mut A
where
    A: Slice<T>,
{
    fn get(&self, index: usize) -> Option<&T> {
        (**self).get(index)
    }

    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<'a, T, A> SliceMut<T> for &'a mut A
where
    A: SliceMut<T>,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        (*self).get_mut(index)
    }
}
