# `::slice-utils`

See [`Slice`] and children.

This is a collection of utilities for slices, similar to those found on
iterators. The goal is to be as close to feature-parity with iterators as
possible, while maintaining `no_std` compatibility.

The core of this crate is providing non-contiguous slices. For example,
`Slice::chain` allows you to join two slices together, clearly breaking
continuity. This results in an `Iterator`-like API.

# License

Licensed under the MIT license.

[`Slice`]: https://docs.rs/slice-utils/latest/slice_utils/trait.Slice.html
