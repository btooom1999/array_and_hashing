fn is_match(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let sn = s.len();
    let pn = p.len();
    let mut dp = vec![vec![false; sn+1]; pn+1];
    dp[pn][sn] = true;
    for i in (0..pn).rev() {
        for j in (0..sn).rev() {
            if p[i] == b'*' {
                break;
            }

            if p[i] == b'.' {
                dp[i][j] = dp[i+1][j+1];
                if i+1 < pn && p[i+1] == b'*' {
                    let mut k = j;
                    while k <= sn && !dp[i][j] {
                        dp[i][j] = dp[i+2][k];
                        k += 1;
                    }

                    if j+1 == sn {
                        dp[i][j+1] = dp[i+2][j+1];
                    }
                }
            } else if p[i] == s[j] {
                dp[i][j] = dp[i+1][j+1];

                if i+1 < pn && p[i+1] == b'*' {
                    let mut k = j;
                    while k < sn && s[k] == p[i] && !dp[i][j] {
                        dp[i][j] = dp[i+2][k];
                        k += 1;
                    }
                    dp[i][j] = dp[i][j] || dp[i+2][k];

                    if j+1 == sn {
                        dp[i][j+1] = dp[i+2][j+1];
                    }
                }
            } else if p[i] != s[j] && i+1 < pn && p[i+1] == b'*' {
                dp[i][j] = dp[i+2][j];
                if j+1 == sn {
                    dp[i][j+1] = dp[i+2][j+1];
                }
            }
        }
    }

    dp[0][0]
}

pub fn main() {
    let s = "aabccbcbacabaab".to_string();
    let p = ".*c*a*b.*a*ba*bb*".to_string();
    println!("{}", is_match(s, p));
}
