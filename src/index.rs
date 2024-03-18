use core::ops::{Index, IndexMut};

use crate::{Chain, Cycle, Interleave, Reverse, Slice, SliceOf};

macro_rules! impl_debug {
    ($(
        $typ:ident [$a:ident $(, $b:ident)?]
    ;)*) =>{$(
        impl<
            'a, T, $a, $($b,)?
        > Index<usize> for $typ<&'a T, $a $(, $b)?>
        where
            $a: for<'b> Slice<'b, &'b T>,
            $($b: for<'b> Slice<'b, &'b T, Mut = <$a as Slice<'b, &'b T>>::Mut>,)?
        {
            type Output = T;

            fn index(&self, index: usize) -> &T {
                self.get(index)
                    .unwrap_or_else(|| panic!(
                        "index out of bounds: len is {} but index is {index}",
                        self.len(),
                    ))
            }
        }

        // FIXME: make this work
        // impl<
        //     'a, T, $a, $($b,)?
        // > IndexMut<usize> for $typ<&'a T, $a $(, $b)?>
        // where
        //     $a: for<'b> Slice<'b, &'b T, Mut = &'b mut T>,
        //     $($b: for<'b> Slice<'b, &'b T, Mut = &'b mut T>,)?
        // {
        //     fn index_mut(&mut self, index: usize) -> &mut T {
        //         let len = self.len();
        //         self.get_mut(index)
        //             .unwrap_or_else(|| panic!(
        //                 "index out of bounds: len is {len} but index is {index}"
        //             ))
        //     }
        // }
    )*};
}

impl_debug! {
    Chain[A, B];
    Cycle[A];
    Interleave[A, B];
    Reverse[A];
    SliceOf[A];
}
