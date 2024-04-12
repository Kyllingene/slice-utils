use core::fmt;

use crate::{
    ArrayWindowsBorrowed, ArrayWindowsOwned, Chain, Cycle, FromFn, Interleave, MapBorrowed,
    MapOwned, Reverse, Slice, SliceBorrowed, SliceOf, SliceOwned, SplitMut, WindowsBorrowed,
    WindowsOwned, Zip,
};

macro_rules! impl_debug {
    ($(
        $typ:ident [$($lt:lifetime),* $($generics:ident),*]
    ;)*) => {$(
        impl<
            $($lt,)* T, S $(, $generics)*,
        > fmt::Debug for $typ<$($lt,)* S $(, $generics)*>
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
    SplitMut['a];
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

impl<T, S1, S2> fmt::Debug for Zip<S1, S2>
where
    T: fmt::Debug,
    S1: SliceOwned<Output = T>,
    S2: SliceOwned<Output = T>,
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

impl<'a, S, T> fmt::Debug for WindowsOwned<'a, S>
where
    S: SliceOwned<Output = T>,
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for i in 0..Slice::len(self) {
            self.get_with(i, &mut |x| {
                list.entry(x);
            });
        }
        list.finish()
    }
}

impl<'a, S, T> fmt::Debug for WindowsBorrowed<'a, S>
where
    S: SliceBorrowed<Output = T>,
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for i in 0..Slice::len(self) {
            self.get_with(i, &mut |x| {
                list.entry(x);
            });
        }
        list.finish()
    }
}

impl<S, T, const N: usize> fmt::Debug for ArrayWindowsOwned<S, N>
where
    S: SliceOwned<Output = T>,
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for i in 0..Slice::len(self) {
            self.get_with(i, &mut |x| {
                list.entry(x);
            });
        }
        list.finish()
    }
}

impl<'a, S, T, const N: usize> fmt::Debug for ArrayWindowsBorrowed<'a, S, N>
where
    S: SliceBorrowed<Output = T>,
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for i in 0..Slice::len(self) {
            self.get_with(i, &mut |x| {
                list.entry(x);
            });
        }
        list.finish()
    }
}
