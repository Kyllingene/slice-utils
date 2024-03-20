use crate::{Slice, SliceBorrowed, SliceOwned};

macro_rules! map {
    ($owned:ident, $in:ty, $fn:ident) => {
        paste::paste! {
            #[doc = "Maps using a closure on index; see [`" [<Slice $owned>] "::map`]."]
            #[derive(Clone, Copy, Hash)]
            pub struct [<Map $owned>]<S, F>(pub S, pub F);

            impl<S, F, U> Slice for [<Map $owned>]<S, F>
            where
                S: [<Slice $owned>],
                F: Fn($in) -> U,
            {
                type Output = U;

                fn len(&self) -> usize {
                    self.0.len()
                }

                fn get_with<W: FnMut(&Self::Output) -> R, R>(&self, index: usize, f: &mut W) -> Option<R> {
                    Some(f(&self.get_owned(index)?))
                }
            }

            impl<S, F, U> SliceOwned for [<Map $owned>]<S, F>
            where
                S: [<Slice $owned>],
                F: Fn($in) -> U,
            {
                fn get_owned(&self, index: usize) -> Option<U> {
                    self.0.$fn(index).map(&self.1)
                }
            }
        }
    };
}

map!(Owned, S::Output, get_owned);
map!(Borrowed, &S::Output, get);
// map!(MapMut, &mut S::Output, SliceMut, get_mut);
