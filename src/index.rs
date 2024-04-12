use core::ops::{Index, IndexMut};

use crate::{
    Chain, Cycle, Interleave, Reverse, Slice, SliceBorrowed, SliceMut, SliceOf, SplitMut, Unique,
};

macro_rules! impl_index {
    ($(
        $typ:ident [$($lt:lifetime),* $($generics:ident),*] $(= $unique:ident)?
    ;)*) => {$(
        impl<
            $($lt,)* T, S $(, $generics)*,
        > Index<usize> for $typ<$($lt,)* S $(, $generics)*>
        where
            S: SliceBorrowed<Output = T>,
            $( $generics: SliceBorrowed<Output = T>,)*
        {
            type Output = <Self as Slice>::Output;

            fn index(&self, index: usize) -> &Self::Output {
                self.get(index).unwrap_or_else(|| {
                    panic!("index out of bounds: the len is {} but the index is {index}", self.len())
                })
            }
        }

        impl<
            $($lt,)* T, S $(, $generics)*,
        > IndexMut<usize> for $typ<$($lt,)* S $(, $generics)*>
        where
            S: SliceBorrowed<Output = T> + SliceMut $(+ $unique)?,
            $( $generics: SliceBorrowed<Output = T> + SliceMut,)*
        {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                let len = self.len();
                self.get_mut(index).unwrap_or_else(|| {
                    panic!("index out of bounds: the len is {len} but the index is {index}")
                })
            }
        }
    )*};
}

impl_index! {
    Chain[S2];
    Cycle[];
    Interleave[S2];
    Reverse[];
    SliceOf[];
    SplitMut['a] = Unique;
}
