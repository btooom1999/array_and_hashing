fn num_distinct(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let n = s.len();
    let mut dp = vec![0; n+1];
    dp[n] = 1;

    for i in (0..t.len()).rev() {
        let mut temp = vec![0; n+1];
        let mut count = dp[n];
        for j in (0..s.len()).rev() {
            if t[i] == s[j] {
                temp[j] = count;
            }
            count += dp[j];
        }

        dp = temp;
    }

    dp.into_iter().sum()
}

pub fn main() {
    let s = "babgbag".to_string();
    let t = "bag".to_string();
    println!("{}", num_distinct(s, t));
}
