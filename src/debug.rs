use core::fmt;

use crate::{Chain, Cycle, Interleave, Reverse, Slice, SliceMut, SliceOf, SliceOfMut};

macro_rules! impl_debug {
    ($(
        $typ:ident [$($lifetimes:lifetime),* $($generics:ident),*] = $bound:ident
    ;)*) =>{$(
        impl<
            $($lifetimes,)* T, $($generics,)*
        > fmt::Debug for $typ<$($lifetimes,)* T, $($generics,)*>
        where
            T: fmt::Debug,
            $($generics : $bound<T>,)*
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut builder = f.debug_list();
                for i in 0..self.len() {
                    builder.entry(&self.get(i).unwrap());
                }
                builder.finish()
            }
        }
    )*};
}

impl_debug! {
    Chain[A, B] = Slice;
    Cycle[A] = Slice;
    Interleave[A, B] = Slice;
    Reverse[A] = Slice;
    SliceOf[A] = Slice;
    SliceOfMut[A] = SliceMut;
}
