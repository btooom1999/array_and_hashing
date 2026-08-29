fn hamming_weight(mut n: i32) -> i32 {
    let one = 1;
    let mut res = 0;
    while n > 0 {
        if one & n == 1 {
            res += 1;
        }
        n >>= 1;
    }

    res
}

pub fn main() {
    let n = 11;
    println!("{:?}", hamming_weight(n));
}
