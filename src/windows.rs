use crate::{Slice, SliceBorrowed, SliceOf, SliceOwned};

macro_rules! either {
    ( [][$($t:tt)*] ) => {$($t)*};
    ( [$($t:tt)*][$($_:tt)*] ) => {$($t)*};
}

macro_rules! def_window {
    ( $owned:ident, [$($lt:lifetime)?], $sized:ty, $item:ty, $fn:ident ) => {
        paste::paste! {
            /// An iterator over overlapping windows of a slice; see
            #[doc = concat!("[`Slice", stringify!($owned), "::windows`].")]
            #[derive(Debug, Clone, Copy)]
            pub struct [<Windows $owned>]<'a, S: ?Sized> {
                /// The slice underlying the iterator.
                pub data: &'a S,
                size: usize,
                i: usize,
            }

            impl<'a, S> [<Windows $owned>]<'a, S>
            where
                S: [<Slice $owned>] + ?Sized,
            {
                /// Create a new iterator; see
                #[doc = concat!("[`Slice", stringify!($owned), "::windows`].")]
                pub fn new(data: &'a S, size: usize) -> Self {
                    if size == 0 {
                        panic!("cannot call `windows` with size = 0");
                    }

                    Self {
                        data,
                        size,
                        i: 0,
                    }
                }
            }

            impl<'a, S> Iterator for [<Windows $owned>]<'a, S>
            where
                S: [<Slice $owned>] + ?Sized,
            {
                type Item = SliceOf<&'a S>;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.i > self.data.len() {
                        None
                    } else {
                        let start = self.i;
                        self.i += 1;

                        self.data.slice(start..start + self.size)
                    }
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    let len = self.data.len() - self.size + 1;
                    (len, Some(len))
                }
            }

            impl<'a, S> ExactSizeIterator
                for [<Windows $owned>]<'a, S>
            where
                S: [<Slice $owned>] + ?Sized {}

            type [<ArrayWindow $owned>]<$($lt,)? S> = either!([$(&$lt S)?][S]);
            /// An iterator over overlapping windows of a slice; see
            #[doc = concat!("[`Slice", stringify!($owned), "::array_windows`].")]
            #[derive(Debug, Clone, Copy)]
            pub struct [<ArrayWindows $owned>]<$($lt,)? S: $sized, const N: usize> {
                /// The slice underlying the iterator.
                pub data: [<ArrayWindow $owned>]<$($lt,)? S>,
                i: usize,
            }

            impl<$($lt,)? S, const N: usize> [<ArrayWindows $owned>]<$($lt,)? S, N>
            where
                S: [<Slice $owned>] + $sized,
            {
                /// Create a new iterator; see
                #[doc = concat!("[`Slice", stringify!($owned), "::array_windows`].")]
                pub fn new(data: either!([$(&$lt S)?][S])) -> Self {
                    // TODO: make this a comptime assertion
                    if N == 0 {
                        panic!("cannot call `windows` with size = 0");
                    }

                    Self {
                        data,
                        i: 0,
                    }
                }
            }

            impl<$($lt,)? S, const N: usize> Iterator for [<ArrayWindows $owned>]<$($lt,)? S, N>
            where
                S: [<Slice $owned>] + $sized,
            {
                type Item = [$item; N];

                fn next(&mut self) -> Option<Self::Item> {
                    if self.i + N > self.data.len() {
                        None
                    } else {
                        let start = self.i;
                        self.i += 1;

                        Some(core::array::from_fn(|i| self.data.$fn(start + i).unwrap()))
                    }
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    let len = self.data.len() - N + 1;
                    (len, Some(len))
                }
            }

            impl<$($lt,)? S, const N: usize> ExactSizeIterator
                for [<ArrayWindows $owned>]<$($lt,)? S, N>
            where
                S: [<Slice $owned>] + $sized {}
        }
    };
}

def_window!(Owned, [], Sized, S::Output, get_owned);
def_window!(Borrowed, ['a], ?Sized, &'a S::Output, get);
