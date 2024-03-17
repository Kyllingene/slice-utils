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
fn repeat() {
    let a = [1, 2, 3].cycle();
    assert_eq!(a[1], 2);
    assert_eq!(a[3], 1);
    assert_eq!(a.get(8), Some(&3));
}

#[test]
fn slice() {
    let a = [1, 2, 3, 4, 5];

    assert_eq!(a.slice(..).expect("failed to slice .."), [1, 2, 3, 4, 5]);
    assert_eq!(a.slice(1..=3).expect("failed to slice 1..=3"), [2, 3, 4]);
    assert_eq!(a.slice(5..), None);
    assert_eq!(a.slice(..6), None);

    let b = [1, 2, 3];
    let c = [4, 5, 6];

    let d = b.chain(c).slice(1..).expect("failed to slice 1..");

    assert_eq!(d[0], 2);
    assert_eq!(d[4], 6);

    let e = b.cycle().slice(..2).expect("failed to slice ..2");

    assert_eq!(e.get(2), None);
    assert_eq!(e.get(3), None);
}

#[test]
fn split() {
    let a = [1, 2, 3].chain([4, 5, 6]);
    let (b, c) = a.split(3).expect("failed to split at 3");

    assert_eq!(b, [1, 2, 3]);
    assert_eq!(c, [4, 5, 6]);

    let d = a.cycle();
    let (e, f) = d.split(6).expect("failed to split at 6");

    assert_eq!(e[1], 2);
    assert_eq!(e.get(6), None);

    assert_eq!(f[0], 1);
    assert_eq!(f[7], 2);
}

#[test]
fn reverse() {
    let a = [1, 2, 3].reverse();

    assert_eq!(a, [3, 2, 1]);

    let b = [1, 2, 3, 4, 5, 6];
    let (c, d) = b.split(3).expect("failed to split at 3");
    let c = c.reverse();
    let d = d.reverse();
    let e = c.chain(d);

    assert_eq!(e, [3, 2, 1, 6, 5, 4]);
}

#[test]
fn interleave() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];

    assert_eq!(a.interleave(b), [1, 4, 2, 5, 3, 6]);
}

#[test]
fn chunks() {
    let slice = [1, 2, 3, 4, 5];

    let mut len = 0;
    slice.chunks(2).enumerate().for_each(|(mut i, c)| {
        i *= 2;
        assert_eq!(c, &slice[i..5.min(i + 2)]);
        len += 1;
    });
    assert_eq!(len, 3);

    assert!(slice.chunks_exact(2).is_none());
}

#[test]
fn windows() {
    let a = [1, 2, 3, 4, 5];

    let mut len = 0;
    a.windows(3).for_each(|w| {
        assert_eq!(w, &a[len..len + 3]);
        len += 1;
    });
    assert_eq!(len, 3);

    len = 0;
    a.windows(4).for_each(|w| {
        assert_eq!(w, &a[len..len + 4]);
        len += 1;
    });
    assert_eq!(len, 2);
}

#[test]
fn slice_eq() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];

    assert_eq!(a.chain(b), [1, 2, 3, 4, 5, 6]);
    assert_eq!(a.reverse(), [3, 2, 1]);
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

    let mut d = (&mut a).chain(&mut b).cycle();

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

    let d = a.cycle();
    eq(d.iter().take(8), [1, 2, 3, 1, 2, 3, 1, 2]);
}
