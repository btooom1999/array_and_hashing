fn has_all_codes(s: String, k: i32) -> bool {
    let k = k as usize;
    let mut hashset = vec![false; 1 << k];

    let mut num = 0;
    let mut i = 0;
    let s = s.as_bytes();
    for j in 0..s.len() {
        num = num << 1 | (s[j] == b'1') as usize;
        if j >= k {
            if s[i] == b'1' {
                num ^= 1 << k;
            }
            i += 1;
        }

        if j >= k-1 {
            hashset[num] = true;
        }
    }

    hashset.into_iter().all(|v| v)
}

pub fn main() {
    let s = "00110110".to_string();
    let k = 2;
    println!("{}", has_all_codes(s, k));
}
