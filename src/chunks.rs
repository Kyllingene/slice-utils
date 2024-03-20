use crate::{Slice, SliceBorrowed, SliceOf, SliceOwned};

macro_rules! either {
    ( [[] $($_:tt)*][$($t:tt)*] ) => {$($t)*};
    ( [[$($_:tt)*] $($t:tt)*][$($__:tt)*] ) => {$($t)*};
}

macro_rules! def_window {
    ( $owned:ident, [$($lt:lifetime)?], $sized:ty, $item:ty, $fn:ident ) => {
        paste::paste! {
            /// An iterator over overlapping chunks of a slice; see
            #[doc = concat!("[`Slice", stringify!($owned), "::chunks`].")]
            #[derive(Debug, Clone, Copy)]
            pub struct [<Chunks $owned>]<'a, S: ?Sized> {
                /// The slice underlying the iterator.
                pub data: &'a S,
                size: usize,
                i: usize,
            }

            impl<'a, S> [<Chunks $owned>]<'a, S>
            where
                S: [<Slice $owned>] + ?Sized,
            {
                /// Create a new iterator; see
                #[doc = concat!("[`Slice", stringify!($owned), "::chunks`].")]
                pub fn new(data: &'a S, size: usize) -> Self {
                    if size == 0 {
                        panic!("cannot call `chunks` with size = 0");
                    }

                    Self {
                        data,
                        size,
                        i: 0,
                    }
                }
            }

            impl<'a, S> Iterator for [<Chunks $owned>]<'a, S>
            where
                S: [<Slice $owned>] + ?Sized,
            {
                type Item = SliceOf<&'a S>;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.i == self.data.len() {
                        None
                    } else {
                        let start = self.i;
                        self.i = self.data.len().min(self.i + self.size);

                        self.data.slice(start..self.i)
                    }
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    let len = self.data.len() / self.size;
                    (len, Some(len))
                }
            }

            impl<'a, S> ExactSizeIterator
                for [<Chunks $owned>]<'a, S>
            where
                S: [<Slice $owned>] + ?Sized {}

            type [<ArrayWindow $owned>]<$($lt,)? S> = either!([[$($lt)?] $(&$lt S)?][S]);
            /// An iterator over overlapping chunks of a slice; see
            #[doc = concat!("[`Slice", stringify!($owned), "::array_chunks`].")]
            #[derive(Debug, Clone, Copy)]
            pub struct [<ArrayChunks $owned>]<$($lt,)? S: $sized, const N: usize> {
                /// The slice underlying the iterator.
                pub data: [<ArrayWindow $owned>]<$($lt,)? S>,
                i: usize,
            }

            impl<$($lt,)? S, const N: usize> [<ArrayChunks $owned>]<$($lt,)? S, N>
            where
                S: [<Slice $owned>] + $sized,
            {
                /// Create a new iterator; see
                #[doc = concat!("[`Slice", stringify!($owned), "::array_chunks`].")]
                pub fn new(data: either!([[$($lt)?] $(&$lt S)?][S])) -> Self {
                    // TODO: make this a comptime assertion
                    if N == 0 {
                        panic!("cannot call `chunks` with size = 0");
                    }

                    Self {
                        data,
                        i: 0,
                    }
                }

                /// Returns the leftover from the end of the slice, if any.
                pub fn remainder(&self) -> SliceOf<&S> {
                    let len = self.data.len();
                    let start = len - (len % N);
                    (either!([[$($lt)?] self.data][&self.data])).slice(start..).unwrap()
                }
            }

            impl<$($lt,)? S, const N: usize> Iterator for [<ArrayChunks $owned>]<$($lt,)? S, N>
            where
                S: [<Slice $owned>] + $sized,
            {
                type Item = [$item; N];

                fn next(&mut self) -> Option<Self::Item> {
                    if self.i + N > self.data.len() {
                        None
                    } else {
                        let start = self.i;
                        self.i += N;

                        Some(core::array::from_fn(|i| self.data.$fn(start + i).unwrap()))
                    }
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    let len = self.data.len() / N;
                    (len, Some(len))
                }
            }

            impl<$($lt,)? S, const N: usize> ExactSizeIterator
                for [<ArrayChunks $owned>]<$($lt,)? S, N>
            where
                S: [<Slice $owned>] + $sized {}
        }
    };
}

def_window!(Owned, [], Sized, S::Output, get_owned);
def_window!(Borrowed, ['a], ?Sized, &'a S::Output, get);
