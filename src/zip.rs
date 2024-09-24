use crate::{Slice, SliceOwned, Unique};

/// Two slices zipped together; see [`SliceOwned::zip`].
#[derive(Clone, Copy, Hash)]
pub struct Zip<S1, S2>(pub S1, pub S2);

// TODO: can this be less strict?
impl<S1, S2> Slice for Zip<S1, S2>
where
    S1: SliceOwned,
    S2: SliceOwned,
{
    type Output = (S1::Output, S2::Output);

    fn len(&self) -> usize {
        self.0.len().min(self.1.len())
    }

    fn get_with<W: FnMut(&Self::Output) -> R, R>(&self, index: usize, f: &mut W) -> Option<R> {
        Some(f(&(self.0.get_owned(index)?, self.1.get_owned(index)?)))
    }
}

impl<S1, S2> SliceOwned for Zip<S1, S2>
where
    S1: SliceOwned,
    S2: SliceOwned,
{
    fn get_owned(&self, index: usize) -> Option<Self::Output> {
        Some((self.0.get_owned(index)?, self.1.get_owned(index)?))
    }
}

// SAFETY: both underlying slices are `Unique`
unsafe impl<S1, S2> Unique for Zip<S1, S2>
where
    S1: Unique,
    S2: Unique,
{
}
