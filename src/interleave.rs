use crate::{Slice, SliceBorrowed, SliceMut, SliceOwned, Unique};

/// Two interleaved slices; see [`Slice::interleave`].
pub struct Interleave<S1, S2>(pub S1, pub S2);

impl<S1, S2> Slice for Interleave<S1, S2>
where
    S1: Slice,
    S2: Slice<Output = S1::Output>,
{
    type Output = S1::Output;

    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }

    fn get_with<W: FnMut(&Self::Output) -> R, R>(&self, index: usize, f: &mut W) -> Option<R> {
        if index % 2 == 0 {
            self.0.get_with(index / 2, f)
        } else {
            self.1.get_with(index / 2, f)
        }
    }
}

impl<S1, S2> SliceOwned for Interleave<S1, S2>
where
    S1: SliceOwned,
    S2: SliceOwned<Output = S1::Output>,
{
    fn get_owned(&self, index: usize) -> Option<Self::Output> {
        if index % 2 == 0 {
            self.0.get_owned(index / 2)
        } else {
            self.1.get_owned(index / 2)
        }
    }
}

impl<S1, S2> SliceBorrowed for Interleave<S1, S2>
where
    S1: SliceBorrowed,
    S2: SliceBorrowed<Output = S1::Output>,
{
    fn get(&self, index: usize) -> Option<&Self::Output> {
        if index % 2 == 0 {
            self.0.get(index / 2)
        } else {
            self.1.get(index / 2)
        }
    }
}

impl<S1, S2> SliceMut for Interleave<S1, S2>
where
    S1: SliceMut,
    S2: SliceMut<Output = S1::Output>,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        if index % 2 == 0 {
            self.0.get_mut(index / 2)
        } else {
            self.1.get_mut(index / 2)
        }
    }
}

// SAFETY: both slices are `Unique`, and aliasing rules prevent creating two
// aliasing slices
unsafe impl<S1, S2> Unique for Interleave<S1, S2>
where
    S1: Unique,
    S2: Unique,
{}
