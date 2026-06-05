fn check_string(s: String) -> bool {
    let s = s.as_bytes();
    for i in 0..s.len()-1 {
        if s[i] == b'b' && s[i+1] == b'a' {
            return false;
        }
    }

    true
}

pub fn main() {
    let s = "aaabbb".to_string();
    println!("{}", check_string(s));
}
