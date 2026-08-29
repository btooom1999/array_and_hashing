fn partition_string(s: String) -> i32 {
    let mut mask = 0;
    let mut res = 0;
    for c in s.chars() {
        let i = c as u8 - b'a';
        if (mask & 1 << i) > 0 {
            res += 1;
            mask = 0;
        }
        mask ^= 1 << i;
    }

    res+1
}

pub fn main() {
    let s = "abacaba".to_string();
    println!("{}", partition_string(s));
}
