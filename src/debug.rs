use core::fmt;

use crate::{
    Chain, Cycle, FromFn, Interleave, MapBorrowed, MapOwned, Reverse, Slice, SliceBorrowed,
    SliceOf, SliceOwned,
};

macro_rules! impl_debug {
    ($(
        $typ:ident [$($generics:ident),*]
    ;)*) => {$(
        impl<
            T, S $(, $generics)*,
        > fmt::Debug for $typ<S $(, $generics)*>
        where
            T: fmt::Debug,
            S: Slice<Output = T>,
            $( $generics: Slice<Output = T>,)*
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut list = f.debug_list();
                for i in 0..self.len() {
                    self.get_with(i, &mut |x| { list.entry(x); });
                }
                list.finish()
            }
        }
    )*};
}

impl_debug! {
    Chain[S2];
    Interleave[S2];
    Reverse[];
    SliceOf[];
}

// Separate impl to avoid infinite debug printing
impl<T, S> fmt::Debug for Cycle<S>
where
    T: fmt::Debug,
    S: Slice<Output = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Ellipses;
        impl fmt::Debug for Ellipses {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "...")
            }
        }

        let mut list = f.debug_list();
        for i in 0..self.0.len() {
            self.get_with(i, &mut |x| {
                list.entry(x);
            });
        }
        list.entry(&Ellipses);
        list.finish()
    }
}

impl<T, S, F, U> fmt::Debug for MapOwned<S, F>
where
    S: SliceOwned<Output = T>,
    F: Fn(T) -> U,
    U: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut list = f.debug_list();
        for i in 0..self.len() {
            self.get_with(i, &mut |x| {
                list.entry(x);
            });
        }
        list.finish()
    }
}

impl<T, S, F, U> fmt::Debug for MapBorrowed<S, F>
where
    S: SliceBorrowed<Output = T>,
    F: Fn(&T) -> U,
    U: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut list = f.debug_list();
        for i in 0..self.len() {
            self.get_with(i, &mut |x| {
                list.entry(x);
            });
        }
        list.finish()
    }
}

impl<F, T> fmt::Debug for FromFn<F>
where
    F: Fn(usize) -> Option<T>,
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut list = f.debug_list();
        for i in 0..self.len() {
            self.get_with(i, &mut |x| {
                list.entry(x);
            });
        }
        list.finish()
    }
}
