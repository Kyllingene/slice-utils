use crate::{Slice, SliceBorrowed, SliceOwned};

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

#[test]
#[should_panic = "index out of bounds: the len is 4 but the index is 4"]
fn index() {
    let mut slice = [1, 2, 3].cycle();
    assert_eq!(slice[3], 1);
    slice[7] = 4;
    assert_eq!(slice[7], 4);

    let slice = [1, 2, 3, 4].slice(..).unwrap();

    #[allow(unconditional_panic)]
    let _ = slice[4];
}

#[test]
fn slices() {
    fn foo<S: SliceBorrowed<Output = i32>>(s: S) {
        assert_eq!(s.get(0), Some(&1));
    }

    fn bar<S: SliceOwned<Output = i32>>(s: S) {
        assert_eq!(s.cycle().get_owned(4), Some(2));
    }

    #[allow(clippy::needless_borrows_for_generic_args)]
    {
        foo(&[1, 2, 3]);
        bar(&[1, 2, 3]);
    }

    assert_eq!((&[1, 2, 3]).rev().get(0), Some(&3));
}

#[test]
fn ranges() {
    let range = 1..5;
    assert_eq!(range.get_owned(0), Some(1));
    assert_eq!(range.get_owned(3), Some(4));
    assert_eq!(range.get_owned(4), None);
    assert_eq!(Slice::rev(range).get_owned(0), Some(4));
}
