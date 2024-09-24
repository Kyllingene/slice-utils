use core::marker::PhantomData;
use core::ops::{Bound, RangeBounds};

use crate::{Slice, SliceBorrowed, SliceMut, SliceOwned, Unique};

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

        // TODO: make this work
        // if end > data.len() {
        //     return None;
        // }

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

// SAFETY: the underlying slice is `Unique` and owned
unsafe impl<A> Unique for SliceOf<A> where A: Unique {}

/// A mutable sub-slice of a [`Slice`]; see [`SliceMut::slice_mut`].
#[derive(Clone, Copy, Hash)]
pub struct SplitMut<'a, A: ?Sized> {
    data: *mut A,

    start: Bound<usize>,
    len: usize,

    _lifetime: PhantomData<&'a mut A>,
}

impl<A> SplitMut<'_, A>
where
    A: Slice + ?Sized,
{
    /// Creates a mutable split of the slice; see [`SliceMut::split_mut`].
    pub fn new(data: &mut A, at: usize) -> Option<(Self, Self)> {
        let len = data.len();
        if at >= len {
            None
        } else {
            Some((
                Self {
                    data: data as *mut A,

                    start: Bound::Included(0),
                    len: at + 1,

                    _lifetime: PhantomData,
                },
                Self {
                    data: data as *mut A,

                    start: Bound::Excluded(at),
                    len: len - at - 1,

                    _lifetime: PhantomData,
                },
            ))
        }
    }

    fn data_imm(&self) -> &A {
        // SAFETY: lifetimes are guaranteed, `Unique` ensures no aliasing
        unsafe { &*self.data }
    }

    fn data_mut(&mut self) -> &mut A {
        // SAFETY: lifetimes are guaranteed, `Unique` ensures no aliasing
        unsafe { &mut *self.data }
    }
}

impl<A> Slice for SplitMut<'_, A>
where
    A: Slice + ?Sized,
{
    type Output = A::Output;

    fn len(&self) -> usize {
        self.len
    }

    fn get_with<W: FnMut(&Self::Output) -> R, R>(&self, index: usize, f: &mut W) -> Option<R> {
        if index > self.len() {
            None
        } else {
            self.data_imm().get_with(
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

impl<A> SliceBorrowed for SplitMut<'_, A>
where
    A: SliceBorrowed + ?Sized,
{
    fn get(&self, index: usize) -> Option<&Self::Output> {
        let i = match self.start {
            Bound::Included(s) => index + s,
            Bound::Excluded(s) => index + s + 1,
            _ => unreachable!(),
        };

        if index > self.len {
            None
        } else {
            self.data_imm().get(i)
        }
    }
}

impl<A> SliceOwned for SplitMut<'_, A>
where
    A: SliceOwned + ?Sized,
{
    fn get_owned(&self, index: usize) -> Option<Self::Output> {
        let i = match self.start {
            Bound::Included(s) => index + s,
            Bound::Excluded(s) => index + s + 1,
            _ => unreachable!(),
        };

        if index > self.len {
            None
        } else {
            self.data_imm().get_owned(i)
        }
    }
}

impl<A> SliceMut for SplitMut<'_, A>
where
    A: SliceMut + Unique,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        let i = match self.start {
            Bound::Included(s) => index + s,
            Bound::Excluded(s) => index + s + 1,
            _ => unreachable!(),
        };

        if index > self.len() {
            None
        } else {
            self.data_mut().get_mut(i)
        }
    }
}
