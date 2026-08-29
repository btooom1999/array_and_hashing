fn distinct_subseq_ii(s: String) -> i32 {
    const MOD: i32 = (1e9) as i32 + 7;
    let mut dp = [0;26];

    for b in s.as_bytes() {
        let k = (b - b'a') as usize;
        dp[k] = dp.iter().fold(1, |acc, &v| (acc + v) % MOD);
    }

    dp.into_iter().fold(0, |acc, v| (acc + v) % MOD)
}

pub fn main() {
    let s = "leelee".to_string();
    println!("{}", distinct_subseq_ii(s));
}
