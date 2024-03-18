use core::fmt;

use crate::{Chain, Cycle, Interleave, Map, MapMut, Reverse, Slice, SliceOf};

macro_rules! impl_debug {
    ($(
        $typ:ident [$a:ident $(, $b:ident)?]
    ;)*) =>{$(
        impl<
            'a, T, $a, $($b,)?
        > fmt::Debug for $typ<T, $a $(, $b)?>
        where
            T: fmt::Debug,
            $a: for<'b> Slice<'b, T>,
            $($b: for<'b> Slice<'b, T, Mut = <$a as Slice<'b, T>>::Mut>,)?
        {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                let mut builder = fmt.debug_list();

                for i in 0..self.len() {
                    builder.entry(&self.get(i).unwrap());
                }

                builder.finish()
            }
        }
    )*};
}

impl_debug! {
    Chain[A, B];
    Cycle[A];
    Interleave[A, B];
    Reverse[A];
    SliceOf[A];
}

impl<'a, T, A, F, U> fmt::Debug for Map<T, A, F>
where
    A: for<'b> Slice<'b, T>,
    F: Fn(T) -> U,
    U: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = fmt.debug_list();

        for i in 0..self.len() {
            builder.entry(&self.get(i).unwrap());
        }

        builder.finish()
    }
}

impl<'a, T, A, F, U, M> fmt::Debug for MapMut<T, A, F, U>
where
    T: fmt::Debug,
    A: for<'b> Slice<'b, T, Mut = M>,
    F: FnMut(M) -> U,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = fmt.debug_list();

        for i in 0..self.len() {
            builder.entry(&self.get(i).unwrap());
        }

        builder.finish()
    }
}
