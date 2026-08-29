const MOD: i64 = (1e9 as i64) + 7;

fn count_k_subsequences_with_max_beauty(s: String, mut k: i32) -> i32 {
    let mut dp = vec![0;26];
    for b in s.as_bytes() {
        let k = (b - b'a') as usize;
        dp[k] += 1;
    }

    dp.sort_by(|a, b| b.cmp(a));

    let threshold = dp[(k.min(26)-1) as usize];
    if threshold == 0 { return 0; }
    let mut res = 1;
    let mut count = 0;
    for value in dp {
        if value > threshold {
            res *= value;
            k -= 1;
        } else if value == threshold {
            count += 1;
        }
    }

    let mut comb = 1;
    for i in 1..=k {
        comb = comb * (count - k as i64 + i as i64) / i as i64 % MOD;
    }

    let mut pow = 1;
    for _ in 0..k {
        pow = pow * threshold % MOD;
    }

    res = (res * comb) % MOD;
    res = (res * pow) % MOD;

    res as i32
}

pub fn main() {
    let s = "bcca".to_string();
    let k = 2;
    println!("{}", count_k_subsequences_with_max_beauty(s, k));
}
