use crate::{Slice, SliceBorrowed, SliceMut, SliceOwned, Unique};

/// Two chained slices; see [`Slice::chain`].
pub struct Chain<S1, S2>(pub S1, pub S2);

impl<S1, S2> Slice for Chain<S1, S2>
where
    S1: Slice,
    S2: Slice<Output = S1::Output>,
{
    type Output = S1::Output;

    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }

    fn get_with<W: FnMut(&Self::Output) -> R, R>(&self, index: usize, f: &mut W) -> Option<R> {
        self.0
            .get_with(index, f)
            .or_else(|| self.1.get_with(index - self.0.len(), f))
    }
}

impl<S1, S2> SliceOwned for Chain<S1, S2>
where
    S1: SliceOwned,
    S2: SliceOwned<Output = S1::Output>,
{
    fn get_owned(&self, index: usize) -> Option<Self::Output> {
        self.0
            .get_owned(index)
            .or_else(|| self.1.get_owned(index - self.0.len()))
    }
}

impl<S1, S2> SliceBorrowed for Chain<S1, S2>
where
    S1: SliceBorrowed,
    S2: SliceBorrowed<Output = S1::Output>,
{
    fn get(&self, index: usize) -> Option<&Self::Output> {
        self.0
            .get(index)
            .or_else(|| self.1.get(index - self.0.len()))
    }
}

impl<S1, S2> SliceMut for Chain<S1, S2>
where
    S1: SliceMut,
    S2: SliceMut<Output = S1::Output>,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        let offset = self.0.len();
        if let r @ Some(_) = self.0.get_mut(index) {
            r
        } else {
            self.1.get_mut(index - offset)
        }
    }
}

// SAFETY: both slices are `Unique`, and aliasing rules prevent creating two
// aliasing slices
unsafe impl<S1, S2> Unique for Chain<S1, S2>
where
    S1: Unique,
    S2: Unique,
{}
