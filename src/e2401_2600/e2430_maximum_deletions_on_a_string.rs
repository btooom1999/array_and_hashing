fn delete_string(s: String) -> i32 {
    let n = s.len();
    let s = s.as_bytes();
    let mut dp = vec![1; n];
    for i in (0..n-1).rev() {
        for j in i..n {
            if j+1+(j-i) >= n { break; }
            let left = &s[i..j+1];
            let right = &s[j+1..j+(j-i)+2];
            if left == right {
                dp[i] = dp[i].max(1 + dp[j+1]);
            }
        }
    }

    dp[0]
}

pub fn main() {
    let s = "aaabaab".to_string();
    println!("{}", delete_string(s));
}
