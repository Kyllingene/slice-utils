use crate::{Slice, SliceBorrowed, SliceMut, SliceOwned};

/// An infinitely looped slice; see [`Slice::cycle`].
pub struct Cycle<S>(pub S);

impl<S> Slice for Cycle<S>
where
    S: Slice,
{
    type Output = S::Output;

    fn len(&self) -> usize {
        usize::MAX
    }

    fn get_with<F: FnMut(&Self::Output) -> U, U>(&self, index: usize, f: &mut F) -> Option<U> {
        self.0.get_with(index % self.0.len(), f)
    }
}

impl<S> SliceOwned for Cycle<S>
where
    S: SliceOwned,
{
    fn get_owned(&self, index: usize) -> Option<Self::Output> {
        self.0.get_owned(index % self.0.len())
    }
}

impl<S> SliceBorrowed for Cycle<S>
where
    S: SliceBorrowed,
{
    fn get(&self, index: usize) -> Option<&Self::Output> {
        self.0.get(index % self.0.len())
    }
}

impl<S> SliceMut for Cycle<S>
where
    S: SliceMut,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        self.0.get_mut(index % self.0.len())
    }
}
