use std::collections::HashSet;

const MOD: i64 = 2i64.pow(45)-1;
const BASE: i64 = 26;

fn distinct_echo_substrings(text: String) -> i32 {
    let text = text.as_bytes();
    let n = text.len();
    let mut hashset = HashSet::new();
    let mut dp = vec![vec![-1; n]; n];
    for i in 0..n {
        let mut hashed = 0;
        for j in i..n {
            hashed = (hashed * BASE % MOD + (text[j] - b'a' + 1) as i64) % MOD;
            dp[i][j] = hashed;
            let amount = j-i+1;
            if i >= amount && dp[i-amount][i-1] == dp[i][j] {
                hashset.insert(hashed);
            }
        }
    }

    hashset.len() as i32
}

pub fn main() {
    let text = "abcdklqstcdghiklmqghijqrwyzijmnoqrsw".to_string();
    println!("{}", distinct_echo_substrings(text));
}
