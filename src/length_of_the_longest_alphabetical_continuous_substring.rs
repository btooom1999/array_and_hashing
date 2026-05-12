fn longest_continuous_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let mut res = 1;
    let mut count = 1;
    for i in 1..s.len() {
        if s[i-1] + 1 == s[i] {
            count += 1;
        } else {
            count = 1;
        }

        res = res.max(count);
    }

    res
}

pub fn main() {
    let s = "abcde".to_string();
    println!("{}", longest_continuous_substring(s));
}
