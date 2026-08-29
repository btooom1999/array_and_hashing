const MOD: i64 = 1_000_000_007;
const BASE: i64 = 26;

fn longest_decomposition(text: String) -> i32 {
    let n = text.len();
    let mut pow = vec![0; n];
    pow[0] = 1;
    for i in 1..n {
        pow[i] = pow[i-1] * BASE % MOD;
    }

    let text = text.as_bytes();
    let mut hashed_left = 0;
    let mut hashed_right = 0;
    let mut j = 0;
    let mut count = 0;
    let mut left = 0;
    let mut right = n-1;
    while left <= right && right < n {
        hashed_left = (hashed_left * BASE % MOD + (text[left] - b'a') as i64) % MOD;
        hashed_right = ((text[right] - b'a') as i64 * pow[j] % MOD + hashed_right) % MOD;
        if hashed_left == hashed_right {
            count += 1 + (left != right) as i32;
            hashed_left = 0;
            hashed_right = 0;
            j = 0;
        } else {
            j += 1;
        }

        left += 1;
        right = right.wrapping_sub(1);
    }

    count + (hashed_left > 0 || hashed_right > 0) as i32
}

pub fn main() {
    let text = "ghiabcdefhelloadamhelloabcdefghi".to_string();
    println!("{}", longest_decomposition(text));
}
