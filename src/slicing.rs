use core::marker::PhantomData;
use core::ops::{Bound, RangeBounds};

use crate::{Slice, SliceMut};

//#===== SliceOf =====#//

/// An immutable sub-slice of a [`Slice`], from [`Slice::slice`].
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SliceOf<T, A: Slice<T>> {
    start: Bound<usize>,
    end: Bound<usize>,

    data: A,
    _marker: PhantomData<fn() -> T>,
}

impl<T, A> SliceOf<T, A>
where
    A: Slice<T>,
{
    pub fn new<R: RangeBounds<usize>>(data: A, range: R) -> Option<Self> {
        let start = match range.start_bound().cloned() {
            s @ Bound::Included(_) | s @ Bound::Excluded(_) => s,
            Bound::Unbounded => Bound::Included(0),
        };

        let end = match range.end_bound().cloned() {
            e @ Bound::Included(_) | e @ Bound::Excluded(_) => e,
            Bound::Unbounded => Bound::Included(data.len() - 1),
        };

        /*
            2= .. =2 : s > e
            2= ..  2 :
            2  .. =2 :
            2  ..  2 :
        */
        match (start, end) {
            (Bound::Included(s), Bound::Included(e)) if s > e || e >= data.len() => None,
            (Bound::Included(s), Bound::Excluded(e)) if s > e || e > data.len() => None,
            (Bound::Excluded(s), Bound::Included(e)) if s > e || e >= data.len() => None,
            (Bound::Excluded(s), Bound::Excluded(e)) if s > e || e > data.len() => None,

            _ => Some(Self {
                start,
                end,
                data,
                _marker: PhantomData,
            }),
        }
    }
}

impl<T, A> Slice<T> for SliceOf<T, A>
where
    A: Slice<T>,
{
    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            None
        } else {
            self.data.get(match self.start {
                Bound::Included(s) => index + s,
                Bound::Excluded(s) => index + s + 1,
                _ => unreachable!(),
            })
        }
    }

    fn len(&self) -> usize {
        match (self.start, self.end) {
            (Bound::Included(s), Bound::Included(e)) => e - s + 1,
            (Bound::Included(s), Bound::Excluded(e)) => e - s,
            (Bound::Excluded(s), Bound::Included(e)) => e - s,
            (Bound::Excluded(s), Bound::Excluded(e)) => e - s - 1,
            _ => unreachable!(),
        }
    }
}

//#===== SliceOfMut =====#//

/// A mutable sub-slice of a [`Slice`], from [`SliceMut::slice_mut`].
#[derive(PartialEq, Eq, Hash)]
pub struct SliceOfMut<T, A: SliceMut<T>> {
    start: Bound<usize>,
    end: Bound<usize>,

    data: A,
    _marker: PhantomData<fn() -> T>,
}

impl<T, A> SliceOfMut<T, A>
where
    A: SliceMut<T>,
{
    pub fn new<R: RangeBounds<usize>>(data: A, range: R) -> Option<Self> {
        let start = match range.start_bound().cloned() {
            s @ Bound::Included(_) | s @ Bound::Excluded(_) => s,
            Bound::Unbounded => Bound::Included(0),
        };

        let end = match range.end_bound().cloned() {
            e @ Bound::Included(_) | e @ Bound::Excluded(_) => e,
            Bound::Unbounded => Bound::Included(data.len() - 1),
        };

        match (start, end) {
            (Bound::Included(s), Bound::Included(e)) if s > e || e >= data.len() => None,
            (Bound::Included(s), Bound::Excluded(e)) if s > e || e > data.len() => None,
            (Bound::Excluded(s), Bound::Included(e)) if s > e || e >= data.len() => None,
            (Bound::Excluded(s), Bound::Excluded(e)) if s > e || e > data.len() => None,

            _ => Some(Self {
                start,
                end,
                data,
                _marker: PhantomData,
            }),
        }
    }
}

impl<T, A> Slice<T> for SliceOfMut<T, A>
where
    A: SliceMut<T>,
{
    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            None
        } else {
            self.data.get(match self.start {
                Bound::Included(s) => index + s,
                Bound::Excluded(s) => index + s + 1,
                _ => unreachable!(),
            })
        }
    }

    fn len(&self) -> usize {
        match (self.start, self.end) {
            (Bound::Included(s), Bound::Included(e)) => e - s + 1,
            (Bound::Included(s), Bound::Excluded(e)) => e - s,
            (Bound::Excluded(s), Bound::Included(e)) => e - s,
            (Bound::Excluded(s), Bound::Excluded(e)) => e - s - 1,
            _ => unreachable!(),
        }
    }
}

impl<T, A> SliceMut<T> for SliceOfMut<T, A>
where
    A: SliceMut<T>,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() {
            None
        } else {
            self.data.get_mut(match self.start {
                Bound::Included(s) => index + s,
                Bound::Excluded(s) => index + s + 1,
                _ => unreachable!(),
            })
        }
    }
}
