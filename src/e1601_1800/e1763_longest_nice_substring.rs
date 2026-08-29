fn longest_nice_substring(s: String) -> String {
    let s = s.as_bytes();
    let n = s.len();
    let mut res = (0, 0);
    for i in 0..n {
        let mut bit = 0i64;
        for j in i..n {
            bit |= 1 << (s[j]-b'A');
            let mut flag = true;
            for x in 0..26 {
                if (bit >> x & 1 == 1 && bit >> (x+32) & 1 == 0)
                || (bit >> x & 1 == 0 && bit >> (x+32) & 1 == 1) {
                    flag = false;
                    break;
                }
            }
            if flag && j-i > res.1-res.0 {
                res = (i, j);
            }
        }
    }

    if res.0 == res.1 {
        return String::new();
    }

    String::from_utf8(s[res.0..=res.1].to_vec()).unwrap()
}

pub fn main() {
    let s = "YazaAay".to_string();
    println!("{}", longest_nice_substring(s));
}
