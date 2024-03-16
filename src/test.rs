use crate::Slice;

#[test]
fn chain() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];

    let c = a.chain(b);
    assert_eq!(c[2], 3);
    assert_eq!(c[4], 5);
    assert_eq!(c.get(6), None);
}

#[test]
fn fuse() {
    let a = [1, 2, 3].repeat();
    assert_eq!(a[1], 2);
    assert_eq!(a[3], 1);
    assert_eq!(a.get(8), Some(&3));
}

#[test]
fn mutability() {
    let mut a = [1, 2, 3];
    let mut b = [4, 5, 6];
    let mut c = (&mut a).chain(&mut b);

    c[2] = 0;
    c[4] = 0;

    assert_eq!(a, [1, 2, 0]);
    assert_eq!(b, [4, 0, 6]);

    let mut d = (&mut a).chain(&mut b).repeat();

    d[13] = 12;
    assert_eq!(a, [1, 12, 0]);
}

#[test]
fn iter() {
    use std::fmt::Debug;

    fn eq<'a, T: Debug + PartialEq + 'a, I: Iterator<Item = &'a T>, const N: usize>(
        mut iter: I,
        e: [T; N],
    ) {
        let mut i = 0;
        while let Some(x) = iter.next() {
            if i > N {
                panic!("expected {N} items, got {}", i + iter.count() - 1);
            }

            assert_eq!(x, &e[i]);

            i += 1;
        }

        if i < N {
            panic!("expected {N} items, got {i}");
        }
    }

    let a = [1, 2, 3];
    let b = [4, 5, 6];

    let c = a.chain(b);
    eq(c.iter(), [1, 2, 3, 4, 5, 6]);

    let d = a.repeat();
    eq(d.iter().take(8), [1, 2, 3, 1, 2, 3, 1, 2]);
}
