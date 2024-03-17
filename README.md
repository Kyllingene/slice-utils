# `::slice-utils`

See [`Slice`] and [`SliceMut`].

This is a collection of utilities for slices, similar to those found on
iterators. The goal is to be as close to feature-parity with iterators as
possible, while maintaining `no_std` compatibility.

The core of this crate is providing non-contiguous slices. For example,
`Slice::chain` allows you to join two slices together, clearly breaking
continuity. This results in an `Iterator`-like API. Here are some differences:

- `Slice`s can only return references, not owned values
    - This disallows methods like `map` which require ownership semantics
    - This may be fixable through a thorough refactor
- `Slice`s are not lazy, and as such:
    - `Slice`s cannot perform arbitrary computation, because that would require
    allocation
    - This may be alleviated by a const generic API in the future

# License

Licensed under the MIT license.

[`Slice`]: https://doc.rust-lang.org/slice-utils/latest/trait.Slice.html
[`SliceMut`]: https://doc.rust-lang.org/slice-utils/latest/trait.SliceMut.html
