use core::cmp::PartialEq;

use crate::{Chain, Cycle, Interleave, Reverse, Slice, SliceMut, SliceOf, SliceOfMut};

macro_rules! impl_eq {
    ($(
        $typ:ident [$($lifetimes:lifetime),* $($generics:ident),*] = $bound:ident
    ;)*) =>{$(
        impl<
            $($lifetimes,)* T, $($generics,)* const N: usize,
        > PartialEq<[T; N]> for $typ<$($lifetimes,)* T, $($generics,)*>
        where
            T: PartialEq,
            $($generics : $bound<T>,)*
        {
            fn eq(&self, other: &[T; N]) -> bool {
                if self.len() != N {
                    false
                } else {
                    for i in 0..self.len() {
                        if self.get(i) != Some(&other[i]) {
                            return false;
                        }
                    }

                    true
                }
            }
        }

        impl<
            $($lifetimes,)* T, $($generics,)*
        > PartialEq<[T]> for $typ<$($lifetimes,)* T, $($generics,)*>
        where
            T: PartialEq,
            $($generics : $bound<T>,)*
        {
            fn eq(&self, other: &[T]) -> bool {
                if self.len() != other.len() {
                    false
                } else {
                    for i in 0..self.len() {
                        if self.get(i) != Some(&other[i]) {
                            return false;
                        }
                    }

                    true
                }
            }
        }

        impl<
            'a, $($lifetimes,)* T, $($generics,)*
        > PartialEq<&'a [T]> for $typ<$($lifetimes,)* T, $($generics,)*>
        where
            T: PartialEq,
            $($generics : $bound<T>,)*
        {
            fn eq(&self, other: &&'a [T]) -> bool {
                if self.len() != other.len() {
                    false
                } else {
                    for i in 0..self.len() {
                        if self.get(i) != Some(&other[i]) {
                            return false;
                        }
                    }

                    true
                }
            }
        }
    )*};
}

impl_eq! {
    Chain[A, B] = Slice;
    Cycle[A] = Slice;
    Interleave[A, B] = Slice;
    Reverse[A] = Slice;
    SliceOf[A] = Slice;
    SliceOfMut[A] = SliceMut;
}
