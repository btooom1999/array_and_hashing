fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
    let mut res = (0..1 << n).map(|v| v ^ (v/2)).collect::<Vec<_>>();

    if let Some(idx) = res.iter().position(|&v| v == start) {
        res.rotate_left(idx);
    }

    res
}

pub fn main() {
    let n = 2;
    let start = 3;
    println!("{:?}", circular_permutation(n, start));
}
