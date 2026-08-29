const MOD: i32 = 1_000_000_007;

fn num_sub(s: String) -> i32 {
    let mut res = 0;
    let mut count = 0;
    for &b in s.as_bytes() {
        if b == b'1' {
            count += 1;
        } else {
            count = 0;
        }

        res = (res + count) % MOD;
    }

    res
}

pub fn main() {
    let s = "0110111".to_string();
    println!("{}", num_sub(s));
}
