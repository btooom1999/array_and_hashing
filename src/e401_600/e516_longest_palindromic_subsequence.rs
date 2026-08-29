fn longest_palindrome_subseq(s: String) -> i32 {
    let n = s.len();
    let s = s.as_bytes();
    let mut dp = vec![vec![0; s.len()]; s.len()];

    for i in 0..n {
        dp[i][i] = 1;
    }

    for i in 1..n {
        for j in 0..n - i {
            if s[j] == s[j + i] {
                dp[j][j + i] = 2 + dp[j + 1][j + i - 1];
            } else {
                dp[j][j + i] = dp[j][j + i - 1].max(dp[j + 1][j + i]);
            }
        }
    }

    dp[0][n - 1]
}

pub fn main() {
    let s = "agbdba".to_string();
    println!("{}", longest_palindrome_subseq(s));
}
