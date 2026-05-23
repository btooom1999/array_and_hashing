const MOD: i64 = 1_000_000_007;

fn count_homogenous(mut s: String) -> i32 {
    s.push('*');
    let s = s.as_bytes();

    let (mut char, mut count) = (s[0], 1i64);
    let mut res = 0;
    for i in 1..s.len() {
        if s[i] == char {
            count += 1;
        } else {
            char = s[i];
            count = 1;
        }
        res = (res + count) % MOD;
    }

    res as i32
}

pub fn main() {
    // let s = "abbcccaa".to_string();
    let s = vec!['w'; 100_000].into_iter().collect();
    println!("{}", count_homogenous(s));
}
