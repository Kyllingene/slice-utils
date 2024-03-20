use core::ops::{Bound, RangeBounds};

use crate::{Slice, SliceBorrowed, SliceMut, SliceOwned};

/// A sub-slice of a [`Slice`]; see [`Slice::slice`].
#[derive(Clone, Copy, Hash)]
pub struct SliceOf<A> {
    data: A,

    start: Bound<usize>,
    len: usize,
}

impl<A> SliceOf<A>
where
    A: Slice,
{
    /// Creates a sub-slice; see [`Slice::slice`].
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
                data,
                start,
                len: match (start, end) {
                    (Bound::Included(s), Bound::Included(e)) => e - s + 1,
                    (Bound::Included(s), Bound::Excluded(e)) => e - s,
                    (Bound::Excluded(s), Bound::Included(e)) => e - s,
                    (Bound::Excluded(s), Bound::Excluded(e)) => e - s - 1,
                    _ => unreachable!(),
                },
            }),
        }
    }
}

impl<A> Slice for SliceOf<A>
where
    A: Slice,
{
    type Output = A::Output;

    fn len(&self) -> usize {
        self.len
    }

    fn get_with<W: FnMut(&Self::Output) -> R, R>(&self, index: usize, f: &mut W) -> Option<R> {
        if index > self.len() {
            None
        } else {
            self.data.get_with(
                match self.start {
                    Bound::Included(s) => index + s,
                    Bound::Excluded(s) => index + s + 1,
                    _ => unreachable!(),
                },
                f,
            )
        }
    }
}

impl<A> SliceOwned for SliceOf<A>
where
    A: SliceOwned,
{
    fn get_owned(&self, index: usize) -> Option<Self::Output> {
        if index > self.len() {
            None
        } else {
            self.data.get_owned(match self.start {
                Bound::Included(s) => index + s,
                Bound::Excluded(s) => index + s + 1,
                _ => unreachable!(),
            })
        }
    }
}

impl<A> SliceBorrowed for SliceOf<A>
where
    A: SliceBorrowed,
{
    fn get(&self, index: usize) -> Option<&Self::Output> {
        if index > self.len() {
            None
        } else {
            self.data.get(match self.start {
                Bound::Included(s) => index + s,
                Bound::Excluded(s) => index + s + 1,
                _ => unreachable!(),
            })
        }
    }
}

impl<A> SliceMut for SliceOf<A>
where
    A: SliceMut,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        if index > self.len() {
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
