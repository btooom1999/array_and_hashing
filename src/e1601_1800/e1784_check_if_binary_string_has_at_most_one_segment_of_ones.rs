fn check_ones_segment(s: String) -> bool {
    let s = s.as_bytes();
    !s.iter().enumerate().skip(1).any(|(i, &v)| v == b'1' && s[i-1] == b'0')
}

pub fn main() {
    let s = "1".to_string();
    println!("{}", check_ones_segment(s));
}
