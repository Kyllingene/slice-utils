use crate::{Slice, SliceOwned};

#[test]
#[should_panic = "[1, 4, 9] [1, 2, 3, ...]"]
fn debug_impl() {
    panic!(
        "{:?} {:?}",
        SliceOwned::map([1, 2, 3], |x| x * x),
        [1, 2, 3].cycle()
    );
}

#[test]
fn mutability() {
    let mut slice = [1, 2, 3];
    let mut cycle = (&mut slice).cycle();
    cycle[2] = 0;

    assert_eq!(slice, [1, 2, 0]);
}
