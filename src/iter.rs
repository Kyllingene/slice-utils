use crate::{SliceBorrowed, SliceOwned};

/// An iterator over a slice; see [`SliceOwned::iter`].
#[derive(Debug, Clone, Copy)]
pub struct IterOwned<S> {
    /// The inner slice of the iterator.
    pub data: S,

    start: usize,
    end: usize,
}

impl<S> IterOwned<S>
where
    S: SliceOwned,
{
    /// Creates a new iterator; see [`SliceOwned::iter`].
    pub fn new(data: S) -> Self {
        Self {
            start: 0,
            end: data.len(),
            data,
        }
    }
}

impl<S> Iterator for IterOwned<S>
where
    S: SliceOwned,
{
    type Item = S::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let x = self.data.get_owned(self.start)?;
            self.start += 1;
            Some(x)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.data.len(), Some(self.data.len()))
    }
}

impl<S> DoubleEndedIterator for IterOwned<S>
where
    S: SliceOwned,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            let x = self.data.get_owned(self.end)?;
            Some(x)
        }
    }
}

impl<S> ExactSizeIterator for IterOwned<S> where S: SliceOwned {}

/// An iterator over a slice; see [`SliceBorrowed::iter`].
#[derive(Debug, Clone, Copy)]
pub struct IterBorrowed<'a, S: ?Sized> {
    /// The inner slice of the iterator.
    pub data: &'a S,

    start: usize,
    end: usize,
}

impl<'a, S> IterBorrowed<'a, S>
where
    S: SliceBorrowed + ?Sized,
{
    /// Creates a new iterator; see [`SliceBorrowed::iter`].
    pub fn new(data: &'a S) -> Self {
        Self {
            data,

            start: 0,
            end: data.len(),
        }
    }
}

impl<'a, S> Iterator for IterBorrowed<'a, S>
where
    S: SliceBorrowed + ?Sized,
{
    type Item = &'a S::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let x = self.data.get(self.start)?;
            self.start += 1;
            Some(x)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.data.len(), Some(self.data.len()))
    }
}

impl<'a, S> DoubleEndedIterator for IterBorrowed<'a, S>
where
    S: SliceBorrowed,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            let x = self.data.get(self.end)?;
            Some(x)
        }
    }
}

impl<'a, S> ExactSizeIterator for IterBorrowed<'a, S> where S: SliceBorrowed {}
