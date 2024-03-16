use core::ops::{Index, IndexMut};

use crate::{Chain, Repeat, Slice, SliceMut, SliceOf, SliceOfMut};

impl<T, A, B> Index<usize> for Chain<T, A, B>
where
    A: Slice<T>,
    B: Slice<T>,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("index out of bounds: {index}"))
    }
}

impl<T, A, B> IndexMut<usize> for Chain<T, A, B>
where
    A: SliceMut<T>,
    B: SliceMut<T>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
            .unwrap_or_else(|| panic!("index out of bounds: {index}"))
    }
}

impl<T, A> Index<usize> for Repeat<T, A>
where
    A: Slice<T>,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("index out of bounds: {index}"))
    }
}

impl<T, A> IndexMut<usize> for Repeat<T, A>
where
    A: SliceMut<T>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
            .unwrap_or_else(|| panic!("index out of bounds: {index}"))
    }
}

impl<T, A> Index<usize> for SliceOf<T, A>
where
    A: Slice<T>,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("index out of bounds: {index}"))
    }
}

impl<T, A> Index<usize> for SliceOfMut<T, A>
where
    A: SliceMut<T>,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("index out of bounds: {index}"))
    }
}

impl<T, A> IndexMut<usize> for SliceOfMut<T, A>
where
    A: SliceMut<T>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
            .unwrap_or_else(|| panic!("index out of bounds: {index}"))
    }
}
