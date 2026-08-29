fn has_alternating_bits(mut n: i32) -> bool {
    let mut bit = 2;

    while n > 0 {
        if bit == (n & 1) {
            return false;
        }
        bit = n & 1;
        n >>= 1;
    }

    true
}

pub fn main() {
    let n = 3;
    println!("{}", has_alternating_bits(n));
}
