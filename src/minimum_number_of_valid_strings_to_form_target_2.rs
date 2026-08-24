use std::collections::HashSet;

const MOD: i64 = 2i64.pow(45)-1;
const BASE: i64 = 26;

fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
    let mut hashset = HashSet::new();
    for word in words {
        let mut hashed = 0;
        for c in word.chars() {
            hashed = (hashed * BASE % MOD + (c as u8 + 1 - b'a') as i64) % MOD;
            hashset.insert(hashed);
        }
    }

    let n = target.len();
    let mut pow = vec![1; n+1];
    for i in 1..=n {
        pow[i] = pow[i-1] * BASE % MOD;
    }

    let target = target.as_bytes();
    let mut dp = vec![i32::MAX; n];
    let mut j = 0;
    let mut hashed = (target[0] - b'a' + 1) as i64;
    for i in 0..n {
        if i > j { return -1; }

        while j < n && hashset.contains(&hashed) {
            dp[j] = 1 + if i > 0 { dp[i-1] } else { 0 };
            j += 1;
            if j < n {
                hashed = (hashed * BASE % MOD + (target[j] - b'a' + 1) as i64) % MOD;
            }
        }

        if j == n { break; }

        hashed = (hashed + MOD - (target[i] - b'a' + 1) as i64 * pow[j-i] % MOD) % MOD;
    }

    if dp[n-1] == i32::MAX { -1 } else { dp[n-1] }
}

pub fn main() {
    let words = ["abababab"].into_iter().map(String::from).collect();
    let target = "ababaababa".to_string();
    println!("{}", min_valid_strings(words, target));
}
