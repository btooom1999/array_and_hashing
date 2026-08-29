fn min_changes(n: i32, k: i32) -> i32 {
    if n&k != k {
        -1
    } else {
        (n^k).count_ones() as i32
    }
}

pub fn main() {
    let n = 13;
    let k = 4;
    println!("{}", min_changes(n, k));
}
