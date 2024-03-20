use crate::{
    Chain, Cycle, Interleave, MapBorrowed, MapOwned, Reverse, Slice, SliceBorrowed, SliceOf,
    SliceOwned,
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
