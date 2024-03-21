use crate::{
    ArrayWindowsBorrowed, ArrayWindowsOwned, Chain, Cycle, FromFn, Interleave, MapBorrowed,
    MapOwned, Reverse, Slice, SliceBorrowed, SliceOf, SliceOwned, WindowsBorrowed, WindowsOwned,
    Zip,
};

macro_rules! impl_eq {
    ($(
        $typ:ident [$($generics:ident),*]
    ;)*) => {$(
        impl<
            T, S, O, V $(, $generics)*,
        > PartialEq<O> for $typ<S $(, $generics)*>
        where
            V: PartialEq<T>,
            S: SliceOwned<Output = T>,
            O: Slice<Output = V>,
            $( $generics: SliceOwned<Output = T>,)*
        {
            fn eq(&self, other: &O) -> bool {
                if self.len() != other.len() {
                    false
                } else {
                    for i in 0..self.len() {
                        if other
                            .get_with(i, &mut |x| x != &self.get_owned(i).unwrap())
                            .unwrap_or(true)
                        {
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
    Chain[S2];
    Cycle[];
    Interleave[S2];
    Reverse[];
    SliceOf[];
}

impl<T, S, O, F, U, V> PartialEq<O> for MapOwned<S, F>
where
    S: SliceOwned<Output = T>,
    O: Slice<Output = V>,
    F: Fn(T) -> U,
    V: PartialEq<U>,
{
    fn eq(&self, other: &O) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for i in 0..self.len() {
                if other
                    .get_with(i, &mut |x| x != &self.get_owned(i).unwrap())
                    .unwrap_or(true)
                {
                    return false;
                }
            }

            true
        }
    }
}

impl<T, S, O, F, U, V> PartialEq<O> for MapBorrowed<S, F>
where
    S: SliceBorrowed<Output = T>,
    O: Slice<Output = V>,
    F: Fn(&T) -> U,
    V: PartialEq<U>,
{
    fn eq(&self, other: &O) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for i in 0..self.len() {
                if other
                    .get_with(i, &mut |x| x != &self.get_owned(i).unwrap())
                    .unwrap_or(true)
                {
                    return false;
                }
            }

            true
        }
    }
}

impl<T, O, F, U> PartialEq<O> for FromFn<F>
where
    O: Slice<Output = U>,
    F: Fn(usize) -> Option<T>,
    U: PartialEq<T>,
{
    fn eq(&self, other: &O) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for i in 0..self.len() {
                if other
                    .get_with(i, &mut |x| x != &self.get_owned(i).unwrap())
                    .unwrap_or(true)
                {
                    return false;
                }
            }

            true
        }
    }
}

impl<T, O, U, S1, S2> PartialEq<O> for Zip<S1, S2>
where
    O: Slice<Output = U>,
    U: PartialEq<(T, T)>,
    S1: SliceOwned<Output = T>,
    S2: SliceOwned<Output = T>,
{
    fn eq(&self, other: &O) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for i in 0..self.len() {
                if other
                    .get_with(i, &mut |x| x != &self.get_owned(i).unwrap())
                    .unwrap_or(true)
                {
                    return false;
                }
            }

            true
        }
    }
}

impl<'a, T, U, O, S> PartialEq<O> for WindowsOwned<'a, S>
where
    O: SliceOwned<Output = U>,
    SliceOf<&'a S>: PartialEq<U>,
    S: SliceOwned<Output = T>,
{
    fn eq(&self, other: &O) -> bool {
        if Slice::len(self) != other.len() {
            false
        } else {
            for i in 0..Slice::len(self) {
                if other
                    .get_with(i, &mut |x| &self.get_owned(i).unwrap() != x)
                    .unwrap_or(true)
                {
                    return false;
                }
            }

            true
        }
    }
}

impl<'a, T, U, O, S> PartialEq<O> for WindowsBorrowed<'a, S>
where
    O: SliceBorrowed<Output = U>,
    SliceOf<&'a S>: PartialEq<U>,
    S: SliceBorrowed<Output = T>,
{
    fn eq(&self, other: &O) -> bool {
        if Slice::len(self) != other.len() {
            false
        } else {
            for i in 0..Slice::len(self) {
                if other
                    .get_with(i, &mut |x| &self.get_owned(i).unwrap() != x)
                    .unwrap_or(true)
                {
                    return false;
                }
            }

            true
        }
    }
}

impl<T, U, O, S, const N: usize> PartialEq<O> for ArrayWindowsOwned<S, N>
where
    O: SliceOwned<Output = U>,
    [T; N]: PartialEq<U>,
    S: SliceOwned<Output = T>,
{
    fn eq(&self, other: &O) -> bool {
        if Slice::len(self) != other.len() {
            false
        } else {
            for i in 0..Slice::len(self) {
                if other
                    .get_with(i, &mut |x| &self.get_owned(i).unwrap() != x)
                    .unwrap_or(true)
                {
                    return false;
                }
            }

            true
        }
    }
}

impl<'a, T: 'a, U, O, S, const N: usize> PartialEq<O> for ArrayWindowsBorrowed<'a, S, N>
where
    O: SliceBorrowed<Output = U>,
    [&'a T; N]: PartialEq<U>,
    S: SliceBorrowed<Output = T>,
{
    fn eq(&self, other: &O) -> bool {
        if Slice::len(self) != other.len() {
            false
        } else {
            for i in 0..Slice::len(self) {
                if other
                    .get_with(i, &mut |x| &self.get_owned(i).unwrap() != x)
                    .unwrap_or(true)
                {
                    return false;
                }
            }

            true
        }
    }
}
