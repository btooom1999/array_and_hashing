fn decode_at_index(s: String, k: i32) -> String {
    let mut k = k as i64;
    let s = s.as_bytes();

    loop {
        let mut idx = k;
        let mut total = 0;
        let mut last_word = s[0];
        let n = s.len();
        for j in 0..n {
            if s[j].is_ascii_digit() {
                let num = (s[j] - b'0') as i64;
                if num*total >= k {
                    k %= total;
                    if k == 0 { return (last_word as char).to_string(); }
                    break;
                }

                total *= num;
                if j+1<n && s[j+1].is_ascii_lowercase() {
                    idx = k - total;
                }
            } else {
                total += 1;
                idx -= 1;
                last_word = s[j];
            }

            if idx <= 0 {
                return (s[j] as char).to_string();
            }
        }
    }
}

// aabcaabc

pub fn main() {
    // let s = "vzpp636m8y".to_string();
    // let k = 2920;
    let s = "a2bc2c4d5e6f7g8h9".to_string();
    let k = 8;
    println!("{}", decode_at_index(s, k));
}

