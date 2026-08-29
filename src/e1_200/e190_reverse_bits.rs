fn reverse_bits(mut n: i32) -> i32 {
    let mut res = 0;
    for _ in 0..32 {
        res = (res << 1) | (n & 1);
        n >>= 1;
    }

    res
}

pub fn main() {
    let n = 43261596;
    println!("{}", reverse_bits(n));
}
