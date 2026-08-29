fn is_match(s: String, p: String) -> bool {
    if s.is_empty() {
        return p.is_empty() || p.as_bytes().iter().all(|v| *v == b'*');
    }

    if p.is_empty() { return false; }

    let (m, n) = (p.len(), s.len());
    let mut dp = vec![vec![false; n+1]; m+1];
    dp[m][n] = true;

    let s = s.as_bytes();
    let p = p.as_bytes();
    for i in (0..m).rev() {
        let mut current = dp[i][n];
        for j in (0..n).rev() {
            if p[i] == b'?' || (p[i].is_ascii_lowercase() && p[i] == s[j]) {
                dp[i][j] = dp[i+1][j+1];
            } else if p[i] == b'*' {
                dp[i][j] = current || dp[i+1][j+1] || dp[i+1][j];
                if j+1 == n {
                    dp[i][j+1] = dp[i+1][j+1];
                }
            }

            current = current || dp[i][j];
        }
    }

    dp[0][0]
}

pub fn main() {
    let s = "adceb".to_string();
    let p = "*a*b".to_string();
    println!("{}", is_match(s, p));
}
