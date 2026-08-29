fn num_decodings(s: String) -> i32 {
    let mut dp = vec![0; s.len()+1];
    dp[s.len()] = 1;

    let s = s.as_bytes();
    for i in (0..s.len()).rev() {
        if s[i] == b'0' {
            dp[i] = 0;
        } else {
            dp[i] += dp[i+1];
        }

        if i+1 < s.len() && (s[i] == b'1' || (s[i] == b'2' && [b'0',b'1',b'2',b'3',b'4',b'5',b'6'].contains(&s[i+1]))) {
            dp[i] += dp[i+2];
        }
    }

    dp[0]
}

pub fn main() {
    let s = "226";
    println!("{}", num_decodings(s.to_string()));
}
