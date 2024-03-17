use slice_utils::Slice;

fn main() {
    let a = [1, 0, 0, 1, 1];
    let b = [2, 1, 1, 2, 3];

    a.cycle()
        .slice(4..11)
        .unwrap()
        .windows(3)
        .map(|x| x[0] + x[1] + x[2])
        .for_each(|x| println!(" - {x:?}"));

    dbg!(b);
}
