use core::ops::{Index, IndexMut};

use crate::{Chain, Cycle, Interleave, Reverse, Slice, SliceBorrowed, SliceMut};

macro_rules! impl_index {
    ($(
        $typ:ident [$($generics:ident),*]
    ;)*) => {$(
        impl<
            T, S $(, $generics)*,
        > Index<usize> for $typ<S $(, $generics)*>
        where
            S: SliceBorrowed<Output = T>,
            $( $generics: SliceBorrowed<Output = T>,)*
        {
            type Output = <Self as Slice>::Output;

            fn index(&self, index: usize) -> &Self::Output {
                self.get(index).unwrap_or_else(|| {
                    panic!("index out of bounds: len is {} but index is {index}", self.len())
                })
            }
        }

        impl<
            T, S $(, $generics)*,
        > IndexMut<usize> for $typ<S $(, $generics)*>
        where
            S: SliceBorrowed<Output = T> + SliceMut,
            $( $generics: SliceBorrowed<Output = T> + SliceMut,)*
        {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                let len = self.len();
                self.get_mut(index).unwrap_or_else(|| {
                    panic!("index out of bounds: len is {len} but index is {index}")
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
}
