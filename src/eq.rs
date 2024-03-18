use core::cmp::PartialEq;

use crate::{Chain, Cycle, Interleave, Map, MapMut, Reverse, Slice, SliceOf};

macro_rules! impl_eq {
    ($(
        $typ:ident [$a:ident $(, $b:ident)?]
    ;)*) =>{$(
        impl<
            'a, T, $a, $($b,)?
        > PartialEq<[T]> for $typ<T, $a $(, $b)?>
        where
            T: PartialEq + Copy,
            $a: for<'b> Slice<'b, T>,
            $($b: for<'b> Slice<'b, T, Mut = <$a as Slice<'b, T>>::Mut>,)?
        {
            fn eq(&self, other: &[T]) -> bool {
                if self.len() != other.len() {
                    false
                } else {
                    for i in 0..self.len() {
                        if self.get(i) != Some(other[i]) {
                            return false;
                        }
                    }

                    true
                }
            }
        }

        impl<
            'a, T, $a, $($b,)?
        > PartialEq<[T]> for $typ<&'a T, $a $(, $b)?>
        where
            T: PartialEq,
            $a: for<'b> Slice<'b, &'b T>,
            $($b: for<'b> Slice<'b, &'b T, Mut = <$a as Slice<'b, &'b T>>::Mut>,)?
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
            'a, T, O, $a, $($b,)?
        > PartialEq<O> for $typ<T, $a $(, $b)?>
        where
            T: PartialEq,
            O: for<'b> Slice<'b, T>,
            $a: for<'b> Slice<'b, T>,
            $($b: for<'b> Slice<'b, T, Mut = <$a as Slice<'b, T>>::Mut>,)?
        {
            fn eq(&self, other: &O) -> bool {
                if self.len() != other.len() {
                    false
                } else {
                    for i in 0..self.len() {
                        if self.get(i) != other.get(i) {
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
    Chain[A, B];
    Cycle[A];
    Interleave[A, B];
    Reverse[A];
    SliceOf[A];
}

impl<'a, T, A, F, U> PartialEq<[U]> for Map<T, A, F>
where
    A: for<'b> Slice<'b, T>,
    F: Fn(T) -> U,
    U: PartialEq + Copy,
{
    fn eq(&self, other: &[U]) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for i in 0..self.len() {
                if self.get(i) != Some(other[i]) {
                    return false;
                }
            }

            true
        }
    }
}

impl<'a, T, A, F, U, O> PartialEq<O> for Map<T, A, F>
where
    A: for<'b> Slice<'b, T>,
    F: Fn(T) -> U,
    U: PartialEq,
    O: for<'b> Slice<'b, U>,
{
    fn eq(&self, other: &O) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for i in 0..self.len() {
                if self.get(i) != other.get(i) {
                    return false;
                }
            }

            true
        }
    }
}

impl<'a, T, A, F, U, M> PartialEq<[T]> for MapMut<T, A, F, U>
where
    T: PartialEq + Copy,
    A: for<'b> Slice<'b, T, Mut = M>,
    F: FnMut(M) -> U,
{
    fn eq(&self, other: &[T]) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for i in 0..self.len() {
                if self.get(i) != Some(other[i]) {
                    return false;
                }
            }

            true
        }
    }
}

impl<'a, T, A, F, U, M, O> PartialEq<O> for MapMut<T, A, F, U>
where
    T: PartialEq,
    A: for<'b> Slice<'b, T, Mut = M>,
    F: FnMut(M) -> U,
    O: for<'b> Slice<'b, T>,
{
    fn eq(&self, other: &O) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for i in 0..self.len() {
                if self.get(i) != other.get(i) {
                    return false;
                }
            }

            true
        }
    }
}
