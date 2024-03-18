use core::marker::PhantomData;

use crate::Slice;

/// A map over a [`Slice`], wrapping [`get_mut`](Slice::get) with a closure.
/// From [`Slice::map`].
#[derive(Clone, Copy, Hash)]
pub struct Map<T, A, F> {
    data: A,
    f: F,
    _marker: PhantomData<fn() -> T>,
}

impl<'a, T, A, F, U> Map<T, A, F>
where
    A: Slice<'a, T>,
    F: Fn(T) -> U,
{
    /// See [`Slice::map`].
    pub fn new(data: A, f: F) -> Self {
        Self {
            data,
            f,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, A, F, U> Slice<'a, U> for Map<T, A, F>
where
    A: Slice<'a, T>,
    F: Fn(T) -> U,
{
    type Mut = A::Mut;

    fn get(&'a self, index: usize) -> Option<U> {
        self.data.get(index).map(&self.f)
    }

    fn get_mut(&'a mut self, index: usize) -> Option<Self::Mut> {
        self.data.get_mut(index)
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

/// A map over a [`Slice`], wrapping [`get_mut`](Slice::get_mut) with a
/// closure. From [`Slice::map_mut`].
#[derive(Clone, Copy, Hash)]
pub struct MapMut<T, A, F, U> {
    data: A,
    f: F,
    _marker: PhantomData<fn() -> (T, U)>,
}

impl<'a, T, A, F, U> MapMut<T, A, F, U>
where
    A: Slice<'a, T>,
    F: FnMut(A::Mut) -> U,
{
    /// See [`Slice::map_mut`].
    pub fn new(data: A, f: F) -> Self {
        Self {
            data,
            f,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, A, F, U> Slice<'a, T> for MapMut<T, A, F, U>
where
    A: Slice<'a, T>,
    F: FnMut(A::Mut) -> U,
{
    type Mut = U;

    fn get(&'a self, index: usize) -> Option<T> {
        self.data.get(index)
    }

    fn get_mut(&'a mut self, index: usize) -> Option<Self::Mut> {
        self.data.get_mut(index).map(&mut self.f)
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}
