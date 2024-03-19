use crate::{Slice, SliceBorrowed, SliceMut, SliceOwned};

/// A reversed slice; see [`Slice::rev`].
pub struct Reverse<S>(pub S);

impl<S> Slice for Reverse<S>
where
    S: Slice,
{
    type Output = S::Output;

    fn len(&self) -> usize {
        self.0.len()
    }

    fn get_with<F: FnMut(&Self::Output) -> U, U>(&self, index: usize, f: &mut F) -> Option<U> {
        self.0.get_with(self.len() - 1 - index, f)
    }
}

impl<S> SliceOwned for Reverse<S>
where
    S: SliceOwned,
{
    fn get_owned(&self, index: usize) -> Option<Self::Output> {
        self.0.get_owned(self.len() - 1 - index)
    }
}

impl<S> SliceBorrowed for Reverse<S>
where
    S: SliceBorrowed,
{
    fn get(&self, index: usize) -> Option<&Self::Output> {
        self.0.get(self.len() - 1 - index)
    }
}

impl<S> SliceMut for Reverse<S>
where
    S: SliceMut,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        self.0.get_mut(self.len() - 1 - index)
    }
}
