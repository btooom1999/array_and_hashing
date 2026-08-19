fn sub_str_hash(s: String, power: i32, modulo: i32, k: i32, hash_value: i32) -> String {
    let power = power as i64;
    let modulo = modulo as i64;

    let s = s.as_bytes();
    let n = s.len();
    let k = k as usize;
    let mut value = 0;
    let mut pow = 1;
    for i in 0..k {
        value = (((s[n-k+i] - b'a' + 1) as i64 * pow as i64 % modulo + value as i64) % modulo) as i32;
        if i+1 < k { pow = (pow as i64 * power % modulo) as i32; }
    }

    let mut res = usize::MAX;
    for i in (0..=n-k).rev() {
        if i < n-k {
            value = ((value as i64 + modulo - (s[i+k] - b'a' + 1) as i64 * pow as i64 % modulo) % modulo) as i32;
            value = (value as i64 * power % modulo) as i32;
            value = ((value as i64 + (s[i] - b'a') as i64 + 1) % modulo) as i32;
        }

        if value == hash_value {
            res = i;
        }
    }

    String::from_utf8(s[res..res+k].to_vec()).unwrap()
}

pub fn main() {
    // let s = "leetcode".to_string();
    // let power = 7;
    // let modulo = 20;
    // let k = 2;
    // let hash_value = 0;
    let s = "fbxzaad".to_string();
    let power = 31;
    let modulo = 100;
    let k = 3;
    let hash_value = 32;
    println!("{}", sub_str_hash(s, power, modulo, k, hash_value));
}
