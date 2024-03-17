use core::ops::{Index, IndexMut};

use crate::{Chain, Cycle, Interleave, Reverse, Slice, SliceMut, SliceOf, SliceOfMut};

macro_rules! impl_index {
    ($(
        $(mut $($is_mut:lifetime)?)? $typ:ident [$($lifetimes:lifetime),* $($generics:ident),*] = $bound:ident
    ;)*) =>{$(
        impl<
            $($lifetimes,)* T, $($generics,)*
        > Index<usize> for $typ<$($lifetimes,)* T, $($generics,)*>
        where
            $($generics : $bound<T>,)*
        {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                self.get(index).unwrap_or_else(|| {
                    panic!(
                        "index out of bounds: index was {index} but the length was {}",
                        self.len()
                    )
                })
            }
        }

        impl<
            $($lifetimes,)* T, $($generics,)*
        > IndexMut<usize> for $typ<$($lifetimes,)* T, $($generics,)*>
        where
            $typ<T, $($generics,)*>: SliceMut<T>,
            $($generics : SliceMut<T>,)*
        {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                let len = self.len();
                self.get_mut(index).unwrap_or_else(|| {
                    panic!(
                        "index out of bounds: the len is {len} but the index was {index}",
                    )
                })
            }
        }
    )*};
}

impl_index! {
    Chain[A, B] = Slice;
    Cycle[A] = Slice;
    Interleave[A, B] = Slice;
    Reverse[A] = Slice;
    SliceOf[A] = Slice;
    SliceOfMut[A] = SliceMut;
}
