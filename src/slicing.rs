use core::marker::PhantomData;
use core::ops::{Bound, RangeBounds};

use crate::{Slice, SliceMut};

//#===== SliceOf =====#//

pub struct SliceOf<T, A: Slice<T>> {
    /// Inclusive.
    start: usize,

    /// Inclusive.
    end: usize,

    data: A,
    _marker: PhantomData<fn() -> T>,
}

impl<T, A> SliceOf<T, A>
where
    A: Slice<T>,
{
    pub fn new<R: RangeBounds<usize>>(data: A, range: R) -> Option<Self> {
        let start = match range.start_bound().cloned() {
            Bound::Included(s) => s,
            Bound::Excluded(s) => s + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound().cloned() {
            Bound::Included(e) => e,
            Bound::Excluded(e) => e + 1,
            Bound::Unbounded => data.len(),
        };

        if start < end {
            return None;
        }

        Some(Self {
            start,
            end,
            data,
            _marker: PhantomData,
        })
    }
}

impl<T, A> Slice<T> for SliceOf<T, A>
where
    A: Slice<T>,
{
    fn get(&self, mut index: usize) -> Option<&T> {
        index += self.start;
        self.data.get(index)
    }

    fn len(&self) -> usize {
        self.end - self.start
    }
}

//#===== SliceOfMut =====#//

pub struct SliceOfMut<T, A: SliceMut<T>> {
    /// Inclusive.
    start: usize,

    /// Inclusive.
    end: usize,

    data: A,
    _marker: PhantomData<fn() -> T>,
}

impl<T, A> SliceOfMut<T, A>
where
    A: SliceMut<T>,
{
    pub fn new<R: RangeBounds<usize>>(data: A, range: R) -> Option<Self> {
        let start = match range.start_bound().cloned() {
            Bound::Included(s) => s,
            Bound::Excluded(s) => s + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound().cloned() {
            Bound::Included(e) => e,
            Bound::Excluded(e) => e + 1,
            Bound::Unbounded => data.len(),
        };

        if start < end {
            return None;
        }

        Some(Self {
            start,
            end,
            data,
            _marker: PhantomData,
        })
    }
}

impl<T, A> Slice<T> for SliceOfMut<T, A>
where
    A: SliceMut<T>,
{
    fn get(&self, mut index: usize) -> Option<&T> {
        index += self.start;
        self.data.get(index)
    }

    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl<T, A> SliceMut<T> for SliceOfMut<T, A>
where
    A: SliceMut<T>,
{
    fn get_mut(&mut self, mut index: usize) -> Option<&mut T> {
        index += self.start;
        self.data.get_mut(index)
    }
}
