const MOD: i64 = 1_000_000_007;

fn maximum_xor_product(mut a: i64, mut b: i64, n: i32) -> i32 {
    for i in (0..n).rev() {
        let bit_a = a >> i & 1;
        let bit_b = b >> i & 1;
        if (bit_a == bit_b && bit_b == 0) || (a < b && bit_a == 0) || (b < a && bit_b == 0) {
            a ^= 1 << i;
            b ^= 1 << i;
        }

    }

    (((a % MOD) * (b % MOD)) % MOD) as i32
}

pub fn main() {
    let a = 123456789012;
    let b = 259606810395;
    let n = 50;
    // let a = 6;
    // let b = 7;
    // let n = 5;
    println!("{}", maximum_xor_product(a, b, n));
}
