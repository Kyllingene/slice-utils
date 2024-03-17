# `slice-utils`

This is a collection of utilities for slices, similar to those found on
iterators. The goal is to be as close to feature-parity with iterators as
possible, while maintaining `no_std` compatibility.

The core of this crate is providing non-contiguous slices. For example,
`Slice::chain` allows you to join two slices together, clearly breaking
continuity. This results in a very `Iterator`-like API. Here are some
differences:

- `Slice`s can only return references, not owned values
    - This disallows methods like `map` which require ownership semantics
- `Slice`s are not lazy, and as such:
    - `Slice`s cannot perform arbitrary computation, because that would require
      allocation

These shortcomings may be alleviated by a const generic API in the future,
making these possible through statically known lengths.
