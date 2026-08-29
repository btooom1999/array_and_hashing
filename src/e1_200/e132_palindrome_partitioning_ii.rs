fn min_cut(s: String) -> i32 {
    let n = s.len();
    let mut dp = vec![i32::MAX; n];

    let s = s.as_bytes();
    for i in 0..n {
        let mut r = i;
        while r < n && s[i] == s[r] {
            let value = if i>0 { 1 + dp[i-1] } else { 0 };
            dp[r] = dp[r].min(value);
            r += 1;
        }

        // case: abba
        if i>0 && r<n {
            let mut l = i-1;
            let mut r = r;
            while l < n && r < n && s[l] == s[r] {
                let value = if l>0 { 1 + dp[l-1] } else { 0 };
                dp[r] = dp[r].min(value);
                r += 1;
                l = l.wrapping_sub(1);
            }
        }
    }

    dp[n-1]
}

pub fn main() {
    let s = "abba".to_string();
    println!("{}", min_cut(s));
}
